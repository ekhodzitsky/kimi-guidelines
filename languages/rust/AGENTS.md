# Instructions for Rust Code Generation (Kimi K2.6)

> Version: 1.0.0 | Source: kimi-dotfiles/rust/
>
> Usage: copy into `src/AGENTS.md` or `.kimi/AGENTS.md` of a Rust project.

## 0. Meta Principle

Kimi K2.6 processes code as data. The more structured and explicit the code, the more accurate the generation.
Prefer standard Rust idioms; avoid "clever" constructions.

---

## I. Decomposition: Module = Responsibility

### Rule 1.1: One File = One Responsibility

Do not mix abstraction levels in a single file.

```rust
// BAD: parser.rs — parses JSON, validates schema, and writes to DB

// GOOD:
// - json_parser.rs (syntax only)
// - schema_validator.rs (semantics only)
// - db_writer.rs (persistence only)
```

### Rule 1.2: Module "Abstract"

At the top of every module, provide a brief description:

```rust
//! Implements Unicode string normalization while preserving identifiers.
//!
//! Invariant: output is always NFC + checked for invalid characters.
//! Dependencies: core, unicode-normalization.
```

### Rule 1.3: Nesting Depth ≤ 3

Nested `if` inside `if` inside `match` inside `loop` is a refactoring signal.
Use early returns, adapter functions, and `Option`/`Result` methods.

---

## II. Functions: One Input, One Output

### Rule 2.1: Function = Atomic Transaction

Length ≤ 40 lines. If you scroll — decompose.

```rust
// BAD: everything in one pile
fn process(data: Vec<Item>) -> Result<(), Error> {
    let cleaned = data.into_iter().filter(|x| x.valid).collect::<Vec<_>>();
    let mut db = connect_db()?;
    for item in cleaned {
        let transformed = transform(&item);
        db.insert(transformed)?;
    }
    db.commit()?;
    Ok(())
}

// GOOD: each step is a separate function with a contract
/// Precondition: input may contain invalid items.
/// Postcondition: returns only items that passed validation.
fn filter_valid(items: Vec<Item>) -> Vec<ValidItem> { ... }

/// Precondition: DB connection is active.
/// Postcondition: transaction is committed or rolled back.
fn persist_batch(conn: &mut Db, items: Vec<ValidItem>) -> Result<(), DbError> { ... }
```

### Rule 2.2: Separate Pure Functions and Side Effects

```rust
// FORBIDDEN: function returns bool AND writes to log AND mutates global state

// ALLOWED:
fn check_invariant(state: &State) -> bool;          // pure
fn log_violation(v: &Violation) -> io::Result<()>; // effect
```

### Rule 2.3: Doc Comments as API Documentation

```rust
/// Brief description (1 line).
///
/// # Arguments
/// * `input` — input condition (e.g., `len > 0`, `UTF-8`).
///
/// # Returns
/// * `Ok(x)` — output condition.
/// * `Err(e)` — when and why an error occurs.
///
/// # Complexity
/// O(n) time, O(1) extra space.
///
/// # Examples
/// ```
/// assert_eq!(foo(vec![1,2]), Ok(3));
/// ```
```

---

## III. Types: The Compiler as Co-Author

### Rule 3.1: Newtype for Domain Typing

Do not use raw primitives where semantics matter.

```rust
// BAD
fn calculate(price: f64, tax_rate: f64) -> f64;

// GOOD
struct Price(f64);     // invariant: >= 0
struct TaxRate(f64);   // invariant: 0.0..=1.0

fn calculate(price: Price, rate: TaxRate) -> Price;
```

### Rule 3.2: Result/Option Instead of Panic

`unwrap()`, `expect()`, `panic!` — only in tests and compile-time constants.

```rust
// BAD
let port = env::var("PORT").unwrap().parse::<u16>().unwrap();

// GOOD
let port: NonZeroU16 = env::var("PORT")
    .map_err(|e| Error::ConfigMissing("PORT", e))?
    .parse()
    .map_err(Error::InvalidPort)?;
```

### Rule 3.3: Typestate for State Machines

If an object has a lifecycle (Disconnected → Connected → Authenticated), express it in types.

```rust
struct Socket<State> { ... }
struct Disconnected;
struct Connected;

impl Socket<Disconnected> {
    fn connect(self, addr: &str) -> Result<Socket<Connected>, Error> { ... }
}

// Impossible to call send() on a Disconnected socket at compile time.
```

---

## IV. Iterators: Pipeline Instead of Nesting

### Rule 4.1: Iterator Method Composition

```rust
// BAD: Kimi gets lost in brackets
let x = match maybe_y {
    Some(y) => match y.parse::<i32>() {
        Ok(z) if z > 0 => Some(z * 2),
        _ => None,
    },
    None => None,
};

// GOOD: each step is a separate transformation
let x = maybe_y
    .and_then(|y| y.parse::<i32>().ok())
    .filter(|&z| z > 0)
    .map(|z| z * 2);
```

### Rule 4.2: Enums for Mutually Exclusive States

Prefer enums over boolean flags or string constants.

---

## V. Macros and Unsafe

### Rule 5.1: Macros Only for Boilerplate

`macro_rules!` and `proc_macro` are last resorts. If you can do without — do without.

### Rule 5.2: Unsafe Only with Justification

Every `unsafe` block requires a `// SAFETY:` comment:

```rust
// SAFETY: `ptr` is valid because:
// 1. Allocated via `Box::into_raw` 3 lines above.
// 2. Not used after this block.
// 3. `len` matches the actual allocation size.
unsafe { slice::from_raw_parts_mut(ptr, len) }
```

### Rule 5.3: Minimize Global State

Avoid `static mut`, `lazy_static!` unless necessary. `thread_local!` — only for FFI or loggers.

---

## VI. Naming

### Rule 6.1: Full Paths in Use

```rust
// BAD
use crate::utils::helper; // what does helper do?

// GOOD
use crate::validation::normalize_email; // module + function = self-documenting
```

### Rule 6.2: Descriptive Names

```rust
// BAD
let n = calc(&cfg, &usr);

// GOOD
let total_price = calculate_total(&config, &user_profile);
```

### Rule 6.3: Examples in Doc Tests

Every public API must have an `# Examples` section demonstrating a real scenario.

---

## VII. Testing

### Rule 7.1: Unit Tests in the Same File

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_valid_rejects_empty() {
        assert!(filter_valid(vec![]).is_empty());
    }
}
```

### Rule 7.2: Property-Based Tests for Invariants

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn reverse_is_involution(xs in prop::collection::vec(0..100i32, 0..100)) {
        assert_eq!(reverse(reverse(&xs)), xs);
    }
}
```

### Rule 7.3: Error Paths Are Tests Too

Every `Err(...)` path must be covered. A "happy path" without errors is only half the code.

---

## VIII. Dependencies

### Rule 8.1: Stability > Novelty

Do not add a crate for a single function. Check:
- semver ≥ 1.0 for critical dependencies
- Development activity
- Size (LLM loads the API into context)

### Rule 8.2: No "Utility Knife" Modules

If a module is named `utils.rs` — it is a signal that abstractions are not extracted.

---

## IX. Automation

### Rule 9.1: rustfmt and clippy Are Mandatory

```toml
# clippy.toml or .cargo/config.toml
[lints.clippy]
all = "deny"
pedantic = "warn"
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
```

### Rule 9.2: CI for Every PR

```bash
cargo check --all-features
cargo test
cargo clippy -- -D warnings
cargo doc --no-deps
```

Verify that the public API has doc comments.

---

## X. LLM-Specific Recommendations for Kimi K2.6

### Rule 10.1: Module + Doc Comment + Tests > 200 Lines

If a module grows too large, Kimi loses context. Decompose early.

### Rule 10.2: Avoid "Compressed" Syntax

One-liners with 5 levels of nesting, closures inside closures, "clever" lifetime tricks — Kimi generates these with errors.

**Alternative:** 3 clear lines instead of 1 "elegant" line.

### Rule 10.3: Prefer Idioms Kimi Knows Well

**Generates well:**
- Iterator chains
- `?` operator
- Exhaustive `match`
- Standard collections (`Vec`, `HashMap`, `BTreeMap`)

**Generates poorly:**
- Custom DSLs on macros
- Complex generic constructions / Higher-Ranked Trait Bounds
- "Smart" wrappers over `std`

---

## Pre-Generation Checklist for Kimi

- [ ] Every public function has a doc comment with examples
- [ ] No `unwrap`, `expect`, `panic!` outside tests
- [ ] No `unsafe` without `// SAFETY:`
- [ ] Every module starts with `//! abstract`
- [ ] `cargo clippy` passes without warnings
- [ ] Functions ≤ 40 lines
- [ ] Nesting depth ≤ 3 levels
- [ ] All error paths are covered by tests
- [ ] No implicit `dyn Trait` without necessity
- [ ] No redundant `mut` variables
