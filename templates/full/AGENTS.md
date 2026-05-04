# Project Guidelines

> Generated from kimi-dotfiles/templates/full
> Includes: base@v1.0.0 + rust@v1.0.0

## Base Rules

- **Types as axioms** — encode invariants in the type system
- **Functions as lemmas** — Hoare triples in doc comments
- **No unwrap/expect/panic** without compile-time proof
- **Property tests** for algebraic structures
- **Standard patterns** — no custom DSLs

## Rust Module Rules

Apply to all `.rs` files:

- Module doc with theorem statement and invariant
- Newtype for domain semantics, Phantom for dimensions, Typestate for lifecycles
- `Result`/`Option` — no `unwrap`/`expect`/`panic!` outside tests
- Hoare triple in every public function doc comment
- `debug_assert!` for runtime preconditions
- Iterator chains instead of nested `match`
- `// SAFETY:` comment for every `unsafe` block + Miri check
- Algebraic traits (`Semigroup`, `Monoid`) with property tests
- `#[cfg(test)]` in same file, proptest for axioms
- clippy: `unwrap_used = "deny"`, `panic = "deny"`

Full reference: `kimi-dotfiles/languages/rust/AGENTS.md`

---

## Project-Specific Rules

<!-- Add your project conventions here -->
