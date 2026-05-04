# Project Guidelines

> Generated from kimi-dotfiles/templates/rust-only
> Includes: base@v1.0.0 + rust@v1.0.0

## Base Rules

- **Code = data** — explicit structure beats clever compression
- **One file = one responsibility**
- **Functions ≤ 40 lines**
- **Explicit error handling** — no silent failures
- **Standard patterns** — avoid custom DSLs

---

## Rust-Specific Rules

### Module Structure

Each module starts with `//! abstract`:
```rust
//! Implements X. Invariant: Y. Dependencies: Z.
```

### Types

- Newtype for domain semantics: `struct Price(f64)` not raw `f64`
- Typestate for lifecycles: `Socket<Disconnected>` → `Socket<Connected>`
- `Result`/`Option` everywhere — no `unwrap` outside tests

### Functions

- Pure vs effect separation
- Doc comments with `# Examples`
- Nesting depth ≤ 3 — use iterator chains

### Unsafe

Every `unsafe` block requires `// SAFETY:` justification.

### Testing

- `#[cfg(test)]` in same file
- Property-based tests for invariants
- All `Err` paths covered

### Automation

```toml
[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
```

Run: `cargo test`, `cargo clippy -- -D warnings`, `cargo doc`

---

## Project-Specific Rules

<!-- Add your project conventions here -->
