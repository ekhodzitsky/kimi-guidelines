#!/usr/bin/env python3
"""
check-contracts.py — Mechanized contract verification for Rust source files.

Checks:
1. Every `pub fn` has a doc comment with Hoare triple marker `/// {`
2. No `unwrap()`, `expect()`, `panic!()` in non-test code without SAFETY comment
3. Every `unsafe` block has `// SAFETY:` comment

Usage:
    python3 scripts/check-contracts.py [path/to/src/]

Exit codes:
    0 — all contracts satisfied
    1 — violations found
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Tuple


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


def check_file(path: Path) -> List[Tuple[int, str, str]]:
    """Check a single Rust file for contract violations.

    Returns list of (line_number, violation_type, message).
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
                violations.append((idx, "MISSING_HOARE", f"pub fn without Hoare triple marker `/// {{`"))

        # Check for unwrap/expect/panic in non-test code
        if re.search(r"\.(unwrap\(\)|expect\([^)]+\))", stripped):
            if not is_test_context(lines, idx - 1):
                # Allow unwrap in const contexts or with compile-time proofs
                if "const" not in stripped and "option_env!" not in stripped:
                    violations.append((idx, "UNWRAP", f"unwrap/expect without type-level proof"))

        if re.search(r"\bpanic!\s*\(", stripped):
            if not is_test_context(lines, idx - 1):
                violations.append((idx, "PANIC", f"panic! without compile-time proof"))

        # Check for unsafe blocks without SAFETY comment
        if "unsafe" in stripped and "// SAFETY:" not in stripped:
            # Heuristic: check preceding 3 lines for SAFETY comment
            has_safety = False
            for j in range(max(0, idx - 4), idx):
                if "// SAFETY:" in lines[j]:
                    has_safety = True
                    break
            if not has_safety:
                violations.append((idx, "MISSING_SAFETY", f"unsafe without // SAFETY: comment"))

    return violations


def main() -> int:
    target = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(".")
    rust_files = find_rust_files(target)

    if not rust_files:
        print(f"No .rs files found under {target}")
        return 0

    all_violations: List[Tuple[Path, int, str, str]] = []
    checked = 0

    for path in rust_files:
        # Skip target/ directories
        if "target" in path.parts:
            continue
        checked += 1
        violations = check_file(path)
        for line, vtype, msg in violations:
            all_violations.append((path, line, vtype, msg))

    print(f"Checked {checked} Rust files")
    print("=" * 60)

    if not all_violations:
        print("✅ All contracts satisfied.")
        return 0

    # Group by file
    current_file = None
    for path, line, vtype, msg in sorted(all_violations, key=lambda x: (str(x[0]), x[1])):
        if path != current_file:
            current_file = path
            print(f"\n{path}:")
        print(f"  Line {line:4d} [{vtype:15s}] {msg}")

    print(f"\n{'=' * 60}")
    print(f"❌ Total violations: {len(all_violations)}")
    return 1


if __name__ == "__main__":
    sys.exit(main())
