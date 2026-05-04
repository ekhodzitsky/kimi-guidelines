# Project Guidelines

> Generated from kimi-dotfiles/templates/full
> Includes: base@v1.0.0 + rust@v1.0.0 + swift@v1.0.0

## Base Rules

- **Code = data** — explicit structure beats clever compression
- **One file = one responsibility**
- **Functions ≤ 40 lines**
- **Explicit error handling** — no silent failures
- **Standard patterns** — avoid custom DSLs

---

## Rust Module Rules

Apply to all `.rs` files:

- Module doc: `//! abstract` with invariants and dependencies
- Newtype for domain types, typestate for state machines
- `Result`/`Option` — no `unwrap`/`expect`/`panic!` outside tests
- Iterator chains instead of nested `match`
- `// SAFETY:` comment for every `unsafe` block
- `#[cfg(test)]` in same file, property tests for invariants
- clippy: `unwrap_used = "deny"`, `panic = "deny"`

Full reference: `kimi-dotfiles/languages/rust/AGENTS.md`

---

## Swift Module Rules

Apply to all `.swift` files:

- File doc: `// MARK:` + `///` with invariants and dependencies
- Wrapper types for domain semantics, enums with associated values for states
- `Result`/`throws` — no `try!`/`as!`/`fatalError` outside tests
- `guard` + method chains instead of nested `if let`
- Minimize `@objc`/`dynamic`/`Mirror` — only for interop
- `XCTest` with descriptive names, all error paths covered
- SwiftLint: `force_try` disabled, `force_cast` disabled

Full reference: `kimi-dotfiles/languages/swift/AGENTS.md`

---

## Project-Specific Rules

<!-- Add your project conventions here -->
