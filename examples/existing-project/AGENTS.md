# MyProject Guidelines

<!-- This example shows how to merge kimi-dotfiles with your existing rules. -->
<!-- Source: https://github.com/ekhodzitsky/kimi-dotfiles/tree/main/examples/existing-project -->
<!-- Includes kimi-dotfiles: rust@v1.3.0 -->

## Existing Project Rules (Highest Priority)

These are your original rules — they take precedence over everything:

- We use PostgreSQL — never SQLite
- All API responses are JSON:API spec
- Dates are ISO-8601 with timezone

---

## kimi-dotfiles Base Rules

<!-- @kimi-dotfiles: base@v1.3.0 -->

- **Types prove invariants** — encode constraints in Newtype / Phantom / Typestate
- **Functions have contracts** — Hoare triple in every doc comment
- **No unwrap/expect/panic** without compile-time proof
- **Property tests** for algebraic axioms
- **Standard patterns** — no custom DSLs

---

## Rust Module Rules
<!-- @kimi-dotfiles: rust@v1.3.0 -->

Apply `kimi-dotfiles/languages/rust/AGENTS.md` to all `src/**/*.rs`:

- `/// { precondition }` doc comment at top of every pub fn
- `debug_assert!` for runtime preconditions
- No `unwrap`/`expect` outside `#[cfg(test)]`
- Iterator chains instead of nested `match`
- `// SAFETY:` for every `unsafe` block

Full reference: https://github.com/ekhodzitsky/kimi-dotfiles/blob/main/languages/rust/AGENTS.md

---

## Conflict Resolution

If rules conflict:
1. **Project-specific rules** (top section) always win
2. **kimi-dotfiles rules** apply only where not overridden
3. When in doubt — ask, don't assume

## Version Lock

<!-- kimi-dotfiles: v1.3.0 -->
<!-- Update checklist: compare with upstream languages/rust/AGENTS.md when upgrading -->
