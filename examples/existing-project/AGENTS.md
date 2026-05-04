# MyProject Guidelines

<!-- This example shows how to merge kimi-dotfiles with your existing rules. -->
<!-- Source: kimi-dotfiles/examples/existing-project -->

## Existing Project Rules (Keep These)

These are your original rules — they take highest priority:

- We use PostgreSQL — never SQLite
- All API responses are JSON:API spec
- Dates are ISO-8601 with timezone
- Feature flags via LaunchDarkly

---

## Included from kimi-dotfiles

<!-- @kimi-dotfiles: base@v1.0.0 -->

### Base Principles

- **Code = data** — explicit structure beats clever compression
- **One file = one responsibility**
- **Functions ≤ 40 lines**
- **Explicit error handling** — no silent failures

---

## Language-Specific Rules

### Rust Modules
<!-- @kimi-dotfiles: rust@v1.0.0 -->

Apply `kimi-dotfiles/languages/rust/AGENTS.md` to all `src/**/*.rs`:
- `//! abstract` at top of each module
- No `unwrap`/`expect` outside tests
- Iterator chains > nested `match`
- `// SAFETY:` for `unsafe`

### Swift Modules
<!-- @kimi-dotfiles: swift@v1.0.0 -->

Apply `kimi-dotfiles/languages/swift/AGENTS.md` to all `ios/**/*.swift`:
- `// MARK:` + doc comments
- No `try!`/`as!` outside tests
- `guard` > nested `if let`

---

## Conflict Resolution

If rules conflict:
1. **Project-specific rules** (top section) always win
2. **Language rules** from kimi-dotfiles apply only where not overridden
3. When in doubt — ask, don't assume

## Version Lock

<!-- kimi-dotfiles: v1.0.0 -->
<!-- Update checklist: compare with upstream templates/full when upgrading -->
