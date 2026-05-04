# Project Guidelines

> Generated from kimi-dotfiles/templates/rust-only
> Includes: base@v1.0.0 + rust@v1.0.0

## Base Rules

- **Types as axioms** — encode invariants in the type system
- **Functions as lemmas** — Hoare triples in doc comments
- **No unwrap/expect/panic** without compile-time proof
- **Property tests** for algebraic structures
- **Standard patterns** — no custom DSLs

## Rust-Specific Rules

### Types

- Newtype for every semantic distinction: `struct Price(u64)` not `u64`
- Phantom types for dimensions: `Quantity<Meters>`
- Typestate for state machines: `Socket<Connected>`
- `NonZeroU32`, `NonZeroU64` where applicable

### Functions

- Hoare triple in every doc comment:
```rust
/// { !items.is_empty() }
/// fn average(items: &[f64]) -> f64
/// { ret == sum(items) / items.len() }
```
- `debug_assert!` for preconditions not in types
- Max 40 lines
- No nesting > 3 levels

### Error Handling

- `Result<T, Error>` with typed errors
- `?` operator preferred
- No `unwrap` outside `#[cfg(test)]`

### Algebraic Structures

```rust
pub trait Semigroup: Clone + PartialEq {
    /// Axiom: ∀a,b,c. combine(a, combine(b, c)) == combine(combine(a, b), c)
    fn combine(&self, other: &Self) -> Self;
}

pub trait Monoid: Semigroup {
    /// Axiom: ∃e. ∀a. combine(e, a) == a && combine(a, e) == a
    fn identity() -> Self;
}
```

Property tests MUST verify all axioms.

### Unsafe

Every `unsafe` block requires `// SAFETY:` proof + Miri check.

### Testing

- `#[cfg(test)]` in same file
- `proptest` for property verification
- Doc tests for examples
- `cargo fuzz` for parsers

### Automation

```toml
[lints.clippy]
all = "deny"
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
```

Run: `cargo test`, `cargo clippy -- -D warnings`, `cargo doc --no-deps`
For unsafe: `cargo +nightly miri test`

---

## Project-Specific Rules

<!-- Add your project conventions here -->
