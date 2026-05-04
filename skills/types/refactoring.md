# Skill Type: Refactoring

> Improves code structure while preserving behavior.

## When to Use

- Code violates AGENTS.md rules (function too long, too nested, etc.)
- User asks to "clean up" or "improve" code
- After verification reveals MAJOR issues

## Process

1. **Behavior preservation check** — identify all observable effects
2. **Apply deletion test** — is this module earning its keep?
3. **Decompose** — split by responsibility
4. **Re-verify** — run full verification suite after changes

## Rules

- **Never change behavior without explicit user approval.**
- **Preserve invariants.** If invariant changes, it's not refactoring — it's redesign.
- **One change at a time.** Commit after each atomic refactor.
- **Run tests after every change.**

## Common Refactors

| Pattern | Before | After |
|---------|--------|-------|
| Extract function | 60-line function | 3 × 20-line functions |
| Replace nested match | `match { Some(y) => match { ... } }` | Iterator chain |
| Newtype introduction | `f64` for price | `struct Price(f64)` |
| Guard introduction | Nested `if let` | `guard let else` |
| Error type refinement | `String` error | Custom `enum Error` |

## Output Format

```markdown
## Refactoring Report

### Changes
1. **Extracted `filter_valid`** from `process` (lines 12-34)
   - Preserves: input/output behavior
   - Improves: testability, readability

### Verification
- [ ] All tests pass before
- [ ] All tests pass after
- [ ] No public API changes
```
