# Severity Classification: Proof Integrity

> Version: 1.6.0 | Based on mathematical principles: axiom violation = CRITICAL, proof gap = MAJOR, presentation = MINOR, elegance = INFO

## Classification

When Kimi reviews code, classify every issue by its impact on **proof integrity**.

---

## CRITICAL — Axiom Violation / Unsoundness

The type system or invariant is broken. The code is mathematically incorrect.

### Examples
- `unwrap()` / `expect()` / `panic!` in library code without type-level proof of safety
- `unsafe` without `// SAFETY:` proof
- `unsafe` where safety proof is demonstrably false
- Data race or shared mutable state without synchronization
- Integer overflow in release mode without `checked_add` / `wrapping_add`
- Violation of type invariant (e.g., `Price(-1)` where invariant is `>= 0`)

### Action
Must fix immediately. Auto-fix if possible. No exceptions.

---

## MAJOR — Proof Gap

Contract is violated, precondition not checked, or property unproven.

### Examples
- Missing `debug_assert!` for precondition not enforced by types
- Missing Hoare triple in doc comment
- Function > 40 lines without decomposition (lemma is too complex)
- Missing property tests for algebraic axioms
- Missing tests for error paths
- `dyn Trait` without explicit need (loses type-level information)
- Global mutable state (`static mut`, unprotected `lazy_static`)
- Missing doc tests for public API

### Action
Must fix before merge. Suggest fix with code snippet.

---

## MINOR — Presentation Issue

Lemma is correct but poorly documented or formatted.

### Examples
- Missing `# Examples` in doc comment
- Non-descriptive variable names (`n`, `x`, `tmp`)
- Could use iterator chain instead of `for` loop
- Missing `// MARK:` or module-level doc
- Inconsistent formatting (rustfmt would fix)
- Missing complexity annotation

### Action
Fix in same PR. Can be auto-fixed.

---

## INFO — Suggest Stronger Proof

A more elegant proof or stronger axiom is possible.

### Examples
- Could encode invariant in type instead of runtime check
- Could use property-based test instead of manual example
- Could use `NonZeroU32` instead of `u32` + assert
- Could extract pure function from effectful one
- Could use exhaustive `match` instead of `if`

### Action
Mention only. Do not block.

---

## Review Report Template

```markdown
## Review: src/module.rs

### Axiom Violations (CRITICAL) — 0
_None._

### Proof Gaps (MAJOR) — 1
- **Line 34:** `unwrap()` on `env::var("PORT")` — no type-level proof.
  **Fix:** Use `?` operator: `let port = env::var("PORT")?.parse()?;`

### Presentation (MINOR) — 2
- **Line 12:** Missing `# Examples` in doc comment for `normalize_email`
- **Line 45:** Variable `n` is non-descriptive — rename to `total_count`

### Suggestions (INFO) — 1
- **Line 67:** Consider `NonZeroU16` for port number instead of `u16` + validation

### Verdict
**Conditionally approved.** Fix MAJOR issue before merge.
```

---

## Auto-Fix Policy

| Severity | Kimi Auto-Fix? | Requires Approval? |
|----------|---------------|-------------------|
| CRITICAL | Yes | No |
| MAJOR | Yes | Yes |
| MINOR | Yes | No |
| INFO | No | No |
