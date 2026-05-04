# Rust Guidelines: Mathematical Programming

> Version: 1.2.1 | Source: kimi-dotfiles/languages/rust/
>
> These rules apply to all `.rs` files. They extend [FORMALISM.md](../../FORMALISM.md), [GLOSSARY.md](../../GLOSSARY.md), [PIPELINE.md](../../PIPELINE.md), and [SEVERITY.md](../../SEVERITY.md).

## 0. Meta Principle

Rust's type system is a theorem prover. Use it to prove correctness at compile time. What cannot be proven by types must be proven by property tests.

---

## I. Types as Axioms

### Rule 1.1: Newtype for Every Semantic Distinction

Never use raw primitives where meaning matters.

```rust
// AXIOM: Price >= 0
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(u64); // cents

// AXIOM: TaxRate in [0.0, 1.0]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TaxRate(f64);

impl TaxRate {
    /// { rate >= 0.0 && rate <= 1.0 }
    /// fn new(rate: f64) -> Option<TaxRate>
    /// { Some(_) ==> rate in [0,1], None ==> rate outside [0,1] }
    pub fn new(rate: f64) -> Option<Self> {
        if rate >= 0.0 && rate <= 1.0 { Some(Self(rate)) } else { None }
    }
}

/// { price.0 >= 0 && rate is Some }
/// fn calculate(price: Price, rate: TaxRate) -> Price
/// { ret.0 == price.0 + (price.0 as f64 * rate.0).round() as u64 }
pub fn calculate(price: Price, rate: TaxRate) -> Price {
    let tax = (price.cents() as f64 * rate.0).round() as u64;
    Price::from_cents(price.cents() + tax)
}
```

### Rule 1.2: Phantom Types for Dimensions

```rust
use std::marker::PhantomData;

pub struct Meters;
pub struct Seconds;
pub struct MetersPerSecond;

#[derive(Clone, Copy, Debug)]
pub struct Quantity<T>(f64, PhantomData<T>);

impl Quantity<Meters> {
    pub fn meters(v: f64) -> Self { Self(v, PhantomData) }
    pub fn value(&self) -> f64 { self.0 }
}

impl Quantity<Seconds> {
    pub fn seconds(v: f64) -> Self { Self(v, PhantomData) }
}

/// { time.0 != 0.0 }
/// fn velocity(dist: Quantity<Meters>, time: Quantity<Seconds>) -> Quantity<MetersPerSecond>
/// { ret.0 == dist.0 / time.0 }
pub fn velocity(
    dist: Quantity<Meters>,
    time: Quantity<Seconds>
) -> Quantity<MetersPerSecond> {
    debug_assert!(time.0 != 0.0, "time must be non-zero");
    Quantity(dist.0 / time.0, PhantomData)
}
```

### Rule 1.3: Typestate for State Machines

```rust
use std::marker::PhantomData;

pub struct Disconnected;
pub struct Connected;
pub struct Authenticated;

pub struct Session<State> {
    socket: std::net::TcpStream,
    _state: PhantomData<State>,
}

/// { true }
/// fn connect(addr: &str) -> Result<Session<Connected>, io::Error>
/// { Ok(_) ==> TCP handshake succeeded }
impl Session<Disconnected> {
    pub fn connect(addr: &str) -> std::io::Result<Session<Connected>> {
        let socket = std::net::TcpStream::connect(addr)?;
        Ok(Session { socket, _state: PhantomData })
    }
}

/// { true }
/// fn authenticate(self, token: &str) -> Result<Session<Authenticated>, io::Error>
/// { Ok(_) ==> server accepted token }
impl Session<Connected> {
    pub fn authenticate(self, _token: &str) -> std::io::Result<Session<Authenticated>> {
        // ... validate token
        Ok(Session { socket: self.socket, _state: PhantomData })
    }
}

/// { session is authenticated }
/// fn send(&mut self, data: &[u8]) -> Result<usize, io::Error>
/// { Ok(n) ==> n bytes written }
impl Session<Authenticated> {
    pub fn send(&mut self, data: &[u8]) -> std::io::Result<usize> {
        use std::io::Write;
        self.socket.write(data)
    }
}
```

---

## II. Functions as Lemmas: Hoare Logic

### Rule 2.1: Every Public Function Has a Hoare Triple

```rust
/// { !items.is_empty() }
/// fn average(items: &[f64]) -> f64
/// { ret.abs_diff_eq(sum(items) / len(items), epsilon) }
pub fn average(items: &[f64]) -> f64 {
    debug_assert!(!items.is_empty());
    items.iter().sum::<f64>() / items.len() as f64
}
```

### Rule 2.2: Precondition via debug_assert!

Use `debug_assert!` for preconditions the type system cannot enforce. Zero cost in release.

```rust
/// { divisor != 0 }
/// fn div(numerator: f64, divisor: f64) -> f64
/// { ret == numerator / divisor }
pub fn div(numerator: f64, divisor: f64) -> f64 {
    debug_assert!(divisor != 0.0, "divisor must be non-zero");
    numerator / divisor
}
```

### Rule 2.3: Function Length ≤ 40 Lines

A lemma must fit in working memory. If longer — decompose into sub-lemmas.

---

## III. Algebraic Structures as Traits

### Rule 3.1: Mathematical Structures Are Contracts with Axioms

```rust
/// Axiom: ∀a,b,c. combine(a, combine(b, c)) == combine(combine(a, b), c)
pub trait Semigroup: Clone + PartialEq {
    fn combine(&self, other: &Self) -> Self;
}

/// Axiom: ∃e. ∀a. combine(e, a) == a && combine(a, e) == a
pub trait Monoid: Semigroup {
    fn identity() -> Self;
}
```

### Rule 3.2: Axioms MUST Be Verified by Property Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn associativity(a in any::<MyMonoid>(), b in any::<MyMonoid>(), c in any::<MyMonoid>()) {
            let left = a.combine(&b.combine(&c));
            let right = a.combine(&b).combine(&c);
            assert_eq!(left, right);
        }

        #[test]
        fn left_identity(a in any::<MyMonoid>()) {
            assert_eq!(MyMonoid::identity().combine(&a), a);
        }

        #[test]
        fn right_identity(a in any::<MyMonoid>()) {
            assert_eq!(a.combine(&MyMonoid::identity()), a);
        }
    }
}
```

---

## IV. Error Handling: Errors as Codomain

### Rule 4.1: Result/Option Instead of Panic

`unwrap()`, `expect()`, `panic!` — forbidden in library code without compile-time proof.

```rust
// FORBIDDEN
let port = env::var("PORT").unwrap().parse::<u16>().unwrap();

// ALLOWED: compiler proves PORT is set (const context)
const PORT: u16 = option_env!("PORT").unwrap().parse::<u16>().unwrap();

// REQUIRED: handle all cases
let port: NonZeroU16 = env::var("PORT")
    .map_err(|e| Error::ConfigMissing("PORT", e))?
    .parse()
    .map_err(Error::InvalidPort)?;
```

---

## V. Iterators: Composition as Proof

### Rule 5.1: Iterator Chains > Nested Control Flow

```rust
// BAD: Kimi gets lost in brackets, proof is implicit
let x = match maybe_y {
    Some(y) => match y.parse::<i32>() {
        Ok(z) if z > 0 => Some(z * 2),
        _ => None,
    },
    None => None,
};

// GOOD: compositional, each step is a lemma
let x = maybe_y
    .and_then(|y| y.parse::<i32>().ok())
    .filter(|&z| z > 0)
    .map(|z| z * 2);
```

---

## VI. Unsafe: Proof Obligation

### Rule 6.1: Every unsafe Block Requires a Proof

```rust
// SAFETY: ptr is valid because:
// 1. Allocated via Box::into_raw at line 42.
// 2. len matches allocation size (invariant of Self).
// 3. ptr is not aliased (exclusive borrow).
unsafe { std::slice::from_raw_parts_mut(ptr, len) }
```

### Rule 6.2: Miri Check Mandatory

```bash
cargo +nightly miri test
```

---

## VII. Testing: Universal Quantification

### Rule 7.1: Property Tests Are Mandatory for Algebraic APIs

Every `Semigroup`, `Monoid`, `Functor` implementation MUST have property tests.

### Rule 7.2: Doc Tests Are Executable Theorems

Every public function MUST have a doc test.

### Rule 7.3: Fuzzing for Parsers

Every parser MUST have a fuzz target:

```bash
cargo fuzz run parse_target
```

---

## VIII. Automation

### Rule 8.1: rustfmt and clippy Mandatory

```toml
[lints.clippy]
all = "deny"
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
```

### Rule 8.2: CI for Every PR

```bash
cargo check --all-features
cargo clippy -- -D warnings
cargo test
cargo doc --no-deps
```

For projects with `unsafe`:
```bash
cargo +nightly miri test
```

For projects with parsers:
```bash
cargo fuzz run target_name -- -max_total_time=60
```

---

## IX. LLM-Specific: What Kimi Generates Well

**Good:**
- Explicit types (no inference in public APIs)
- Small lemmas (≤ 40 lines)
- Iterator chains (compositional proof)
- Exhaustive match (proof by cases)
- Standard collections

**Bad:**
- Nested match (> 3 levels)
- Multiple closures in one line
- Custom macros / DSLs
- Complex HRTB

**Rule:** If Kimi would need to "reason" about code, decompose it so reasoning is in types.

---

## X. Checklist

- [ ] Hoare triple in doc comment
- [ ] Invariant in type (Newtype/Phantom/Typestate) or `debug_assert!`
- [ ] No unwrap/expect/panic without compile-time proof
- [ ] Property tests for algebraic axioms
- [ ] Doc tests for examples
- [ ] unsafe has SAFETY proof + Miri check
- [ ] Function ≤ 40 lines
- [ ] No nesting > 3 levels
- [ ] clippy passes without warnings
- [ ] All tests pass (unit + property + doc)
- [ ] Fuzzing passes (if parser)
