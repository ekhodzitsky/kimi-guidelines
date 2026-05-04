# Skill Type: Verification

> Runs static and dynamic verification on code.

## When to Use

- After implementation is complete
- Before submitting a PR
- When user asks "is this correct?"

## Process

1. **Static checks**
   - Compiler: `cargo check` / `swift build`
   - Linter: `cargo clippy` / `swiftlint`
   - Formatter: `rustfmt` / `swiftformat`

2. **Dynamic checks**
   - Unit tests: happy path
   - Unit tests: each error path
   - Property-based tests: invariants
   - Doc tests: examples compile

3. **Severity review**
   - Classify each issue using `SEVERITY.md`
   - Produce structured report

## Output Format

```markdown
## Verification Report: module_name

| Check | Status | Notes |
|-------|--------|-------|
| Compiles | ✅/❌ | |
| Linter clean | ✅/❌ | |
| Doc tests | ✅/❌ | |
| Unit tests (happy) | ✅/❌ | X/Y passed |
| Unit tests (errors) | ✅/❌ | X/Y passed |
| Property tests | ✅/❌ | |

### Issues

#### [CRITICAL/MAJOR/MINOR/INFO]: Description
- **Location:** file.rs:line
- **Rule violated:** AGENTS.md rule X.Y
- **Suggested fix:** [code snippet]

### Verdict
[Approved / Conditionally approved / Rejected]
```
