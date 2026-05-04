#!/usr/bin/env python3
"""
score_output.py

Evaluate a Rust source file against the kimi-dotfiles quality guidelines.
Produces a JSON report with per-criterion scores and a total.

Usage:
    python score_output.py path/to/file.rs
    python score_output.py path/to/file.rs --output report.json
"""

import argparse
import json
import os
import re
import sys
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Optional


@dataclass
class Criteria:
    hoare_triples: int = 0
    unwrap_count: int = 0
    newtype_used: bool = False
    phantomdata_used: bool = False
    typestate_used: bool = False
    avg_function_length: float = 0.0
    result_handling: bool = False
    option_handling: bool = False


@dataclass
class Report:
    file: str
    score: int
    criteria: dict


class RustAnalyzer:
    def __init__(self, source: str, filename: str = "output.rs"):
        self.source = source
        self.lines = source.splitlines()
        self.filename = filename

    def analyze(self) -> Criteria:
        c = Criteria()
        c.hoare_triples = self._count_hoare_triples()
        c.unwrap_count = self._count_unwraps()
        c.newtype_used = self._has_newtype()
        c.phantomdata_used = self._has_phantomdata()
        c.typestate_used = self._has_typestate()
        c.avg_function_length = self._avg_function_length()
        c.result_handling = self._uses_result()
        c.option_handling = self._uses_option()
        return c

    def _count_hoare_triples(self) -> int:
        """Doc comments that look like Hoare triples: /// { ... }"""
        # Match /// followed by optional spaces and a { at start of doc comment
        # But not inside unwrap_or or similar false positives
        pattern = re.compile(r'^\s*///\s*\{')
        return sum(1 for line in self.lines if pattern.match(line))

    def _count_unwraps(self) -> int:
        """Count unwrap(), expect(...), and panic!(...) outside doc comments and tests."""
        # Skip doc comments (///) and #[cfg(test)] blocks
        in_test_block = False
        count = 0
        unwrap_pattern = re.compile(r'\b(unwrap\(\)|expect\s*\(|panic!\s*\()')
        # Exclude unwrap_or / unwrap_or_else / unwrap_or_default
        false_positive = re.compile(r'\b(unwrap_or\(|unwrap_or_else\(|unwrap_or_default\()')
        for line in self.lines:
            stripped = line.strip()
            if stripped.startswith('///'):
                continue
            if '#[cfg(test)]' in stripped:
                in_test_block = True
                continue
            if in_test_block:
                if stripped == '}' and line.startswith('}'):
                    # Heuristic: end of test module
                    pass
                continue
            if unwrap_pattern.search(line) and not false_positive.search(line):
                count += 1
        return count

    def _has_newtype(self) -> bool:
        """Detect newtype pattern: struct Foo(T); or pub struct Bar(Baz);"""
        # Single-field tuple struct
        pattern = re.compile(r'^\s*(pub\s+)?struct\s+\w+\s*\(\s*\w+\s*\)\s*;')
        return any(pattern.match(line) for line in self.lines)

    def _has_phantomdata(self) -> bool:
        """Detect PhantomData usage."""
        return any('PhantomData' in line for line in self.lines)

    def _has_typestate(self) -> bool:
        """Detect typestate pattern: generic structs with marker types.

        Heuristic: at least two unit-struct markers (e.g., `struct Open;`)
        and at least one of them appears as a generic argument somewhere
        in angle brackets.
        """
        marker_pattern = re.compile(r'^\s*(pub\s+)?struct\s+(\w+)\s*;')
        markers = []
        for line in self.lines:
            m = marker_pattern.match(line)
            if m:
                markers.append(m.group(2))

        if len(markers) < 2:
            return False

        # Look for any marker name used inside <...> anywhere in the file
        generic_use_pattern = re.compile(r'<[^>]*\b(' + '|'.join(re.escape(m) for m in markers) + r')\b[^>]*>')
        for line in self.lines:
            if generic_use_pattern.search(line):
                return True
        return False

    def _avg_function_length(self) -> float:
        """Compute average line count of top-level functions."""
        lengths = []
        i = 0
        n = len(self.lines)
        while i < n:
            line = self.lines[i]
            # Heuristic: function starts with `fn ` at beginning (allowing visibility/attributes)
            if re.match(r'^(\s*(pub\s+)?(async\s+)?(unsafe\s+)?fn\s+)', line):
                start = i
                brace_depth = line.count('{') - line.count('}')
                i += 1
                while i < n and brace_depth > 0:
                    brace_depth += self.lines[i].count('{') - self.lines[i].count('}')
                    i += 1
                # If no braces on same line, walk until we close the function body
                if brace_depth <= 0:
                    lengths.append(i - start)
                    continue
                # Edge case: function signature only, no body (trait method)
                # Skip it.
            i += 1

        if not lengths:
            return 0.0
        return round(sum(lengths) / len(lengths), 1)

    def _uses_result(self) -> bool:
        return any('Result<' in line or '-> Result' in line for line in self.lines)

    def _uses_option(self) -> bool:
        return any('Option<' in line or '-> Option' in line for line in self.lines)


def compute_score(c: Criteria) -> int:
    """Compute a 0-100 score from criteria."""
    score = 0

    # Hoare triples: up to 30 points (3+ triples = full)
    score += min(c.hoare_triples, 3) * 10

    # Unwraps: penalize heavily. 0 unwraps = 20 pts, each loses 5, floor 0.
    score += max(0, 20 - c.unwrap_count * 5)

    # Newtype / PhantomData / Typestate: 10 pts each
    if c.newtype_used:
        score += 10
    if c.phantomdata_used:
        score += 10
    if c.typestate_used:
        score += 10

    # Avg function length: <= 40 lines = 10 pts, else 0
    if 0 < c.avg_function_length <= 40:
        score += 10

    # Result handling: 10 pts
    if c.result_handling:
        score += 10

    return min(100, max(0, score))


def analyze_file(path: Path) -> Report:
    source = path.read_text(encoding="utf-8")
    analyzer = RustAnalyzer(source, filename=str(path))
    criteria = analyzer.analyze()
    score = compute_score(criteria)
    return Report(
        file=str(path),
        score=score,
        criteria=asdict(criteria),
    )


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Score a Rust file against kimi-dotfiles quality guidelines."
    )
    parser.add_argument("file", help="Path to the .rs file")
    parser.add_argument(
        "--output", "-o", help="Optional JSON output file (default: stdout)"
    )
    args = parser.parse_args()

    target = Path(args.file)
    if not target.exists():
        print(f"Error: file not found: {target}", file=sys.stderr)
        return 1
    if target.suffix != ".rs":
        print(f"Warning: expected .rs file, got {target.suffix}", file=sys.stderr)

    report = analyze_file(target)
    report_json = json.dumps(asdict(report), indent=2)

    if args.output:
        out_path = Path(args.output)
        out_path.write_text(report_json + "\n", encoding="utf-8")
        print(f"Report written to {out_path}")
    else:
        print(report_json)

    return 0


if __name__ == "__main__":
    sys.exit(main())
