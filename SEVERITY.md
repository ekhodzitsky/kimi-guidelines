# Severity Classification for Code Review

> Version: 1.0.0 | Based on formal invariants from PDF

## How to Classify Violations

When Kimi reviews code or self-reviews generated code, classify every issue using these levels.

---

## CRITICAL — Must Fix Before Commit

Violations that break correctness, safety, or security guarantees.

### Rust
- `unwrap()` / `expect()` / `panic!` in production code
- `unsafe` without `// SAFETY:` justification
- Data races or shared mutable state without synchronization
- Use-after-free or dangling references
- Integer overflow in release mode (without `checked_add` etc.)

### Swift
- `try!` / `as!` / `fatalError()` in production code
- Force-unwrapping optional without justification
- Retain cycles in closures without `[weak self]`
- Main thread violations for UI updates

### Universal
- Missing error handling for I/O or network operations
- Secrets or credentials in source code
- SQL injection vulnerabilities

---

## MAJOR — Must Fix Before PR

Violations that significantly reduce maintainability or violate core architectural invariants.

### Rust
- Function > 40 lines without decomposition
- Nesting depth > 3 without justification
- Missing doc comments on public API
- `dyn Trait` without explicit need
- Global mutable state (`static mut`, unprotected `lazy_static`)
- Missing tests for error paths

### Swift
- Function > 40 lines without decomposition
- Nesting `if let` > 2 levels (should use `guard`)
- Missing doc comments on public API
- `!` operator on implicitly unwrapped optionals
- Global `static var` without protocol abstraction
- Missing tests for `throws` or `nil` paths

### Universal
- File with multiple responsibilities (violates 1 file = 1 responsibility)
- Undocumented public interface
- "Utility" module (`utils.rs`, `Helper.swift`)
- Untested public function

---

## MINOR — Fix in Same PR

Style and documentation issues that reduce readability.

- Missing `# Examples` in doc comments
- Non-descriptive variable names (`n`, `x`, `tmp`)
- Could use iterator chain instead of `for` loop
- Missing `// MARK:` in Swift files
- Missing module-level doc (`//!` or `// MARK:`)
- Inconsistent formatting (rustfmt/swiftformat would fix)

---

## INFO — Optional Suggestions

Improvements that are not required but would be nice.

- Could use property-based testing for this invariant
- Could extract pure function from effectful one
- Could use Newtype for this primitive
- Could add complexity annotation to doc comment
- Could use exhaustive `match` / `switch` instead of `if`

---

## Review Report Template

```markdown
## Review: src/module.rs

### CRITICAL (0)
_None._

### MAJOR (1)
- **Line 34:** `unwrap()` on `env::var("PORT")` — use `?` operator.
  **Fix:** `let port = env::var("PORT")?.parse()?;`

### MINOR (2)
- **Line 12:** Missing `# Examples` in doc comment for `normalize_email`
- **Line 45:** Variable `n` is non-descriptive — rename to `total_count`

### INFO (1)
- **Line 67:** Consider adding property-based test for email invariant

### Verdict
**Conditionally approved.** Fix MAJOR issue before merge.
```

---

## Auto-Fix Policy

| Severity | Kimi Auto-Fix? | Requires User Approval? |
|----------|---------------|------------------------|
| CRITICAL | Yes | No (must fix) |
| MAJOR | Yes | Yes (suggest, wait for OK) |
| MINOR | Yes | No (fix silently) |
| INFO | No | No (mention only) |
