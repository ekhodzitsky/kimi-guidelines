#!/usr/bin/env python3
"""
check-contracts.py — Mechanized contract verification for Rust source files.

Checks:
1. Every `pub fn` has a doc comment with Hoare triple marker `/// {`
2. No `unwrap()`, `expect()`, `panic!()` in non-test code without SAFETY comment
3. Every `unsafe` block has `// SAFETY:` comment

Usage:
    python3 scripts/check-contracts.py [path/to/src/]
    python3 scripts/check-contracts.py --strictness relaxed [path/to/src/]

Exit codes:
    0 — all contracts satisfied
    1 — violations found
"""

import argparse
import os
import re
import sys
from pathlib import Path
from typing import List, Tuple


# Severity mapping for violation types
SEVERITY_MAP = {
    "UNWRAP": "CRITICAL",
    "PANIC": "CRITICAL",
    "MISSING_SAFETY": "CRITICAL",
    "MISSING_HOARE": "MAJOR",
}


def find_rust_files(root: Path) -> List[Path]:
    """Recursively find all .rs files under root."""
    return list(root.rglob("*.rs"))


def extract_doc_comments(lines: List[str], func_line_idx: int) -> str:
    """Extract consecutive `///` or `//!` comments immediately before func_line_idx."""
    comments = []
    i = func_line_idx - 1
    while i >= 0:
        line = lines[i].strip()
        if line.startswith("///") or line.startswith("//!"):
            comments.insert(0, line)
            i -= 1
        elif line == "" or line.startswith("#["):
            # Skip blank lines and attributes
            i -= 1
        else:
            break
    return "\n".join(comments)


def is_test_context(lines: List[str], start_idx: int) -> bool:
    """Check if we're inside a `#[cfg(test)]` module or test function."""
    # Simple heuristic: look for #[cfg(test)] in preceding 20 lines
    for i in range(max(0, start_idx - 20), start_idx):
        if "#[cfg(test)]" in lines[i]:
            return True
    # Check if inside a function with #[test] attribute
    for i in range(max(0, start_idx - 5), start_idx):
        if "#[test]" in lines[i]:
            return True
    return False


def check_file(path: Path) -> List[Tuple[int, str, str, str]]:
    """Check a single Rust file for contract violations.

    Returns list of (line_number, violation_type, severity, message).
    """
    violations = []
    content = path.read_text()
    lines = content.splitlines()

    # Track whether we're in a SAFETY comment context for unsafe blocks
    in_safety_comment = False

    for idx, line in enumerate(lines, start=1):
        stripped = line.strip()

        # Check for pub fn without Hoare triple
        if re.search(r"^\s*pub\s+(?:unsafe\s+)?fn\s+\w+", stripped):
            if is_test_context(lines, idx - 1):
                continue
            doc = extract_doc_comments(lines, idx - 1)
            if "/// {" not in doc:
                violations.append((idx, "MISSING_HOARE", "MAJOR", f"pub fn without Hoare triple marker `/// {{`"))

        # Check for unwrap/expect/panic in non-test code
        if re.search(r"\.(unwrap\(\)|expect\([^)]+\))", stripped):
            if not is_test_context(lines, idx - 1):
                # Allow unwrap in const contexts or with compile-time proofs
                if "const" not in stripped and "option_env!" not in stripped:
                    violations.append((idx, "UNWRAP", "CRITICAL", f"unwrap/expect without type-level proof"))

        if re.search(r"\bpanic!\s*\(", stripped):
            if not is_test_context(lines, idx - 1):
                violations.append((idx, "PANIC", "CRITICAL", f"panic! without compile-time proof"))

        # Check for unsafe blocks without SAFETY comment
        if "unsafe" in stripped and "// SAFETY:" not in stripped:
            # Heuristic: check preceding 3 lines for SAFETY comment
            has_safety = False
            for j in range(max(0, idx - 4), idx):
                if "// SAFETY:" in lines[j]:
                    has_safety = True
                    break
            if not has_safety:
                violations.append((idx, "MISSING_SAFETY", "CRITICAL", f"unsafe without // SAFETY: comment"))

    return violations


def severity_filter(violations: List[Tuple[Path, int, str, str, str]], strictness: str) -> List[Tuple[Path, int, str, str, str]]:
    """Filter violations based on strictness level."""
    if strictness == "strict":
        return violations
    elif strictness == "standard":
        return [v for v in violations if v[3] in ("CRITICAL", "MAJOR")]
    elif strictness == "relaxed":
        return [v for v in violations if v[3] == "CRITICAL"]
    return violations


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Mechanized contract verification for Rust source files."
    )
    parser.add_argument(
        "path",
        nargs="?",
        default=".",
        help="Path to check (default: current directory)",
    )
    parser.add_argument(
        "--strictness",
        choices=["relaxed", "standard", "strict"],
        default="standard",
        help="Reporting strictness: relaxed (CRITICAL only), standard (CRITICAL+MAJOR), strict (everything). Default: standard",
    )
    args = parser.parse_args()

    target = Path(args.path)
    rust_files = find_rust_files(target)

    if not rust_files:
        print(f"No .rs files found under {target}")
        return 0

    all_violations: List[Tuple[Path, int, str, str, str]] = []
    checked = 0

    for path in rust_files:
        # Skip target/ directories
        if "target" in path.parts:
            continue
        checked += 1
        violations = check_file(path)
        for line, vtype, severity, msg in violations:
            all_violations.append((path, line, vtype, severity, msg))

    # Apply strictness filter
    filtered = severity_filter(all_violations, args.strictness)

    print(f"Checked {checked} Rust files (strictness: {args.strictness})")
    print("=" * 60)

    if not filtered:
        print("✅ All contracts satisfied.")
        return 0

    # Group by file
    current_file = None
    for path, line, vtype, severity, msg in sorted(filtered, key=lambda x: (str(x[0]), x[1])):
        if path != current_file:
            current_file = path
            print(f"\n{path}:")
        print(f"  Line {line:4d} [{vtype:15s}] [{severity:8s}] {msg}")

    print(f"\n{'=' * 60}")
    print(f"❌ Total violations: {len(filtered)}")
    return 1


if __name__ == "__main__":
    sys.exit(main())
