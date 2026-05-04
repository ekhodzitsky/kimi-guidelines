# Global Instructions for Kimi K2.6: Mathematical Programming

> Version: 1.0.0 | Repository: https://github.com/ekhodzitsky/kimi-dotfiles
>
> We write code as mathematical proof. Every function is a lemma. Every module is a theorem. Types are axioms.

## Reference Documents

Read these before generating or reviewing code:

- **[FORMALISM.md](FORMALISM.md)** — Concrete tools: Hoare triples, Phantom types, property-based testing, Miri, fuzzing
- **[GLOSSARY.md](GLOSSARY.md)** — Mathematical vocabulary: Invariant, Precondition, Postcondition, Monad, Functor, Homomorphism
- **[PIPELINE.md](PIPELINE.md)** — Development process: Specification → Type-Level Proof → Implementation → Property Verification
- **[SEVERITY.md](SEVERITY.md)** — Violation classification tied to proof integrity

---

## 0. Meta Principle: Code Is Proof

Kimi K2.6 generates code that must be **correct by construction**. Not "probably correct", not "tested on examples" — but provably correct through:

1. **Types that make invalid states unrepresentable** (Curry-Howard)
2. **Contracts that are executable** (Hoare logic via `debug_assert!`)
3. **Properties that are universally quantified** (proptest, not manual examples)
4. **Invariants that are compiler-checked** (Newtype, Typestate, PhantomData)

**Golden rule:** If a property can be enforced by the type system, it MUST be. Tests are for what the compiler cannot prove.

---

## 1. Types as Axioms: Curry-Howard in Practice

### 1.1 Newtype for Every Semantic Distinction

Never use raw primitives where meaning matters.

```rust
// AXIOM: Price represents a non-negative monetary value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Price(u64); // cents, invariant: >= 0

impl Price {
    /// { cents >= 0 }
    /// fn from_cents(cents: u64) -> Price
    /// { ret.0 == cents }
    pub fn from_cents(cents: u64) -> Self { Self(cents) }
    
    pub fn cents(&self) -> u64 { self.0 }
}

// AXIOM: TaxRate is in [0.0, 1.0]
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
```

### 1.2 Phantom Types for Dimensions and Units

```rust
use std::marker::PhantomData;

pub struct Meters;
pub struct Seconds;

/// AXIOM: Quantity<T> carries a physical dimension
#[derive(Clone, Copy, Debug)]
pub struct Quantity<T>(f64, PhantomData<T>);

impl Quantity<Meters> {
    pub fn meters(v: f64) -> Self { Self(v, PhantomData) }
}

impl Quantity<Seconds> {
    pub fn seconds(v: f64) -> Self { Self(v, PhantomData) }
}

/// { time.0 != 0.0 }
/// fn velocity(dist: Quantity<Meters>, time: Quantity<Seconds>) -> Quantity<(Meters, Seconds)>
/// { ret.0 == dist.0 / time.0 }
pub fn velocity(
    dist: Quantity<Meters>,
    time: Quantity<Seconds>
) -> Quantity<(Meters, Seconds)> {
    debug_assert!(time.0 != 0.0);
    Quantity(dist.0 / time.0, PhantomData)
}
```

### 1.3 Typestate for State Machines

Encode valid transitions in the type system. Runtime checks are proofs the compiler cannot make.

See [FORMALISM.md](FORMALISM.md) §2.3 for full pattern.

---

## 2. Functions as Lemmas: Hoare Logic

Every public function is a lemma with a proof obligation.

### 2.1 Required Doc Comment Format

```rust
/// { P }
/// fn name(args) -> ReturnType
/// {
///   Ok(r)  ==> Q_ok(r)
///   Err(e) ==> Q_err(e)
/// }
///
/// # Complexity
/// O(?) time, O(?) space.
///
/// # Examples
/// ```
/// assert_eq!(foo(vec![1,2]), Ok(3));
/// ```
```

Where:
- `{ P }` — precondition (what caller guarantees)
- `Q_ok` — postcondition for success
- `Q_err` — postcondition for error

### 2.2 Runtime Preconditions via debug_assert!

```rust
/// { !items.is_empty() }
/// fn average(items: &[f64]) -> f64
/// { ret == sum(items) / len(items) }
pub fn average(items: &[f64]) -> f64 {
    debug_assert!(!items.is_empty(), "average requires non-empty slice");
    items.iter().sum::<f64>() / items.len() as f64
}
```

**Rule:** `debug_assert!` is mandatory for preconditions not enforced by types. Zero cost in release.

### 2.3 Function Length ≤ One Screen

A lemma must fit in working memory. If it does not decompose — it is not a lemma, it is a theorem that needs sub-lemmas.

**Max:** 40 lines. If longer — extract sub-functions, each with its own contract.

---

## 3. Modules as Theorems: Structure

### 3.1 One Module = One Mathematical Concept

```rust
//! Theorem: NormalizedEmail
//! 
//! Invariant: output is lowercase, contains exactly one '@', no whitespace.
//! 
//! Depends on: core, unicode-normalization.
//! Proved by: doc tests + property tests (see tests/).
```

### 3.2 Visibility = Proof Surface

Make implementation private. Expose only what has a proven contract.

```rust
mod internal {
    // Unproven helper — not public
    pub(crate) fn parse_raw(s: &str) -> Vec<char> { ... }
}

/// { s is valid UTF-8 }
/// pub fn normalize(s: &str) -> Result<String, EmailError>
/// { Ok(r) ==> r invariant holds, Err(e) ==> s violates RFC 5322 subset }
pub fn normalize(s: &str) -> Result<String, EmailError> { ... }
```

---

## 4. Testing as Mathematical Verification

### 4.1 Property Tests Are Mandatory

Unit tests check examples. Property tests prove theorems.

**Every public function with algebraic structure MUST have property tests:**

| Property | Test |
|----------|------|
| Involution | `f(f(x)) == x` |
| Idempotence | `f(f(x)) == f(x)` |
| Associativity | `f(a, f(b, c)) == f(f(a, b), c)` |
| Identity | `f(id, x) == x` |
| Homomorphism | `map(f ∘ g) == map(f) ∘ map(g)` |

See [FORMALISM.md](FORMALISM.md) §4 for proptest patterns.

### 4.2 Doc Tests Are Executable Theorems

Every doc example is a proof obligation that must compile and pass.

### 4.3 Fuzzing for Parsers

Any function that parses untrusted input MUST have a fuzz target:

```bash
cargo fuzz run parse_input
```

---

## 5. Error Handling: Errors as Mathematical Results

Errors are not exceptions. They are elements of the codomain.

```rust
/// { true }
/// fn sqrt(x: f64) -> Option<f64>
/// {
///   Some(r) ==> r >= 0 && r*r == x (within epsilon)
///   None    ==> x < 0
/// }
pub fn sqrt(x: f64) -> Option<f64> {
    if x < 0.0 { None } else { Some(x.sqrt()) }
}
```

**Forbidden in library code:** `unwrap()`, `expect()`, `panic!`.

**Allowed only where:**
- The type system has already proven the invariant (e.g., `NonZeroU32::new(n).unwrap()` where `n > 0` is compile-time constant)
- Inside `#[cfg(test)]`

---

## 6. Unsafe: Lemma with Safety Proof

Every `unsafe` block requires a proof comment:

```rust
// SAFETY: ptr is valid because:
// 1. Allocated via Box::into_raw at line 42.
// 2. len matches allocation size (invariant of Self).
// 3. ptr is not aliased (exclusive borrow guarantees this).
unsafe { std::slice::from_raw_parts_mut(ptr, len) }
```

**Mandatory:** Miri check before merge.

---

## 7. Algebraic Structures as Traits

Mathematical structures are not metaphors. They are contracts with axioms verified by property tests.

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

See [FORMALISM.md](FORMALISM.md) §3 and `examples/rust-demo/`.

---

## 8. LLM-Specific Constraints

Kimi K2.6 generates best with:

**Good:**
- Explicit types (no inference in public APIs)
- Small lemmas (≤ 40 lines)
- Iterator chains (compositional, no nesting)
- Exhaustive match (proof by cases)
- Standard collections

**Bad:**
- Nested `match` (> 3 levels)
- One-liners with multiple closures
- Custom DSLs via macros
- Complex HRTB or higher-kinded patterns

**Rule:** If Kimi would need to "reason" about the code, decompose it so the reasoning is in the types.

---

## 9. Pre-Generation Checklist

- [ ] Hoare triple written in doc comment
- [ ] Invariant encoded in type (Newtype / Phantom / Typestate)
- [ ] Precondition checked via `debug_assert!`
- [ ] No `unwrap`/`expect`/`panic!` outside tests
- [ ] Property tests for algebraic axioms
- [ ] Doc tests as executable theorems
- [ ] `unsafe` has SAFETY proof + Miri check
- [ ] Function ≤ 40 lines
- [ ] No nesting > 3 levels
- [ ] `cargo clippy` passes without warnings
- [ ] `cargo test` passes (unit + property + doc)
- [ ] `cargo fuzz` passes (if parser present)

---

## Final Formula

```
Correctness = Types (Curry-Howard) + Contracts (Hoare) + Properties (Universal Quantification) + Verification (Compiler + Miri + Fuzz)
LLM Readability = Explicit Types + Small Lemmas + Standard Combinators + Self-Documenting Contracts
```
