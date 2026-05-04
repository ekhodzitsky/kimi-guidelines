# Formal Methods in Rust: Practical Toolkit

> Version: 1.2.0 | kimi-dotfiles
>
> Concrete tools, crates, and patterns for implementing rigorous Rust code.

## 1. Hoare Logic: Contracts in Code

Hoare triple: `{P} C {Q}` — if precondition `P` holds, then after executing command `C`, postcondition `Q` holds.

### Doc Comments (Static Documentation)

Every public function MUST include a Hoare triple in its doc comment:

```rust
/// { s.is_empty() ==> Err(Empty) }
/// fn parse_u32(s: &str) -> Result<u32, ParseError>
/// {
///   Ok(n)  ==> n >= 0, n fits in u32
///   Err(e) ==> s contains non-digit chars or is empty
/// }
```

### Runtime Contracts (Dynamic Enforcement)

Use `debug_assert!` for preconditions and postconditions in debug builds. Zero cost in release.

```rust
pub fn divide(n: Numerator, d: NonZero<Denominator>) -> Quotient {
    debug_assert!(*d != 0, "Denominator must be non-zero (guaranteed by type)");
    // In release: compiler has already proven d != 0 via NonZero type
    Quotient(n / d)
}
```

For heavier runtime contracts, consider the `contracts` crate:

```toml
[dependencies]
contracts = "0.6"
```

```rust
use contracts::*;

#[pre(!s.is_empty())]
#[post(ret >= 0)]
pub fn parse_positive(s: &str) -> i32 {
    s.parse().unwrap()
}
```

**Rule:** `debug_assert!` is free and mandatory. `contracts` crate is optional but encouraged for public APIs.

---

## 2. Curry-Howard: Types as Propositions

In Rust, we use the type system to make invalid states unrepresentable.

### Newtype for Invariants

```rust
/// Invariant: value >= 0
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Natural(u32);

impl Natural {
    pub fn new(n: u32) -> Self {
        Self(n)
    }

    /// { n >= 0 }
    /// fn from_i32(n: i32) -> Option<Natural>
    /// { Some(_) ==> n >= 0, None ==> n < 0 }
    pub fn from_i32(n: i32) -> Option<Self> {
        if n >= 0 { Some(Self(n as u32)) } else { None }
    }
}
```

### Phantom Types for Units/Dimensions

```rust
use std::marker::PhantomData;

pub struct Meter;
pub struct Second;

/// Invariant: value represents length in meters
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Quantity<T>(f64, PhantomData<T>);

impl Quantity<Meter> {
    pub fn meters(v: f64) -> Self { Self(v, PhantomData) }

    /// { true }
    /// fn add(self, other: Quantity<Meter>) -> Quantity<Meter>
    /// { ret.value == self.value + other.value }
    pub fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, PhantomData)
    }
}

/// { true }
/// fn velocity(distance: Quantity<Meter>, time: Quantity<Second>) -> Quantity<(Meter, Second)>
/// { ret.value == distance.value / time.value }
pub fn velocity(
    distance: Quantity<Meter>,
    time: Quantity<Second>,
) -> Quantity<(Meter, Second)> {
    Quantity(distance.0 / time.0, PhantomData)
}
```

### Typestate for State Machines

```rust
pub struct Disconnected;
pub struct Connected;
pub struct Authenticated;

pub struct Session<State> {
    socket: TcpStream,
    _state: PhantomData<State>,
}

/// { true }
/// fn connect(addr: &str) -> Result<Session<Connected>, Error>
/// { Ok(_) ==> TCP handshake succeeded }
impl Session<Disconnected> {
    pub fn connect(addr: &str) -> io::Result<Session<Connected>> {
        let socket = TcpStream::connect(addr)?;
        Ok(Session { socket, _state: PhantomData })
    }
}

/// { true }
/// fn authenticate(self, token: &str) -> Result<Session<Authenticated>, Error>
/// { Ok(_) ==> server accepted token, Err(_) ==> token rejected }
impl Session<Connected> {
    pub fn authenticate(self, token: &str) -> io::Result<Session<Authenticated>> {
        // ... send token, verify response
        Ok(Session { socket: self.socket, _state: PhantomData })
    }
}

/// { session is authenticated }
/// fn send(self, data: &[u8]) -> Result<usize, Error>
/// { Ok(n) ==> n bytes queued }
impl Session<Authenticated> {
    pub fn send(&mut self, data: &[u8]) -> io::Result<usize> {
        self.socket.write(data)
    }
}
```

---

## 3. Algebraic Structures as Traits

Mathematical structures are contracts with axioms.

```rust
/// Axiom: assoc(a, b, c) == true for all a, b, c
pub trait Semigroup: Clone {
    fn combine(&self, other: &Self) -> Self;
    fn assoc(a: &Self, b: &Self, c: &Self) -> bool {
        a.combine(&b.combine(c)) == a.combine(b).combine(c)
    }
}

/// Axiom: left_identity(e, a) && right_identity(e, a) for all a
pub trait Monoid: Semigroup {
    fn identity() -> Self;
    fn left_identity(e: &Self, a: &Self) -> bool {
        e.combine(a) == *a
    }
    fn right_identity(e: &Self, a: &Self) -> bool {
        a.combine(e) == *a
    }
}
```

These axioms MUST be verified via property-based tests (see §4).

---

## 4. Property-Based Testing: Randomized Verification

Unit tests check examples. Property tests check invariants across random inputs.

### Obligatory Crates

```toml
[dev-dependencies]
proptest = "1.0"
```

### Required Properties for Every Public API

| Property | Meaning | Example |
|----------|---------|---------|
| **Involution** | `f(f(x)) == x` | `reverse(reverse(xs)) == xs` |
| **Idempotence** | `f(f(x)) == f(x)` | `dedup(dedup(xs)) == dedup(xs)` |
| **Homomorphism** | `f(g(x)) == f(g(x))` | `map(f ∘ g) == map(f) ∘ map(g)` |
| **Identity** | `f(id, x) == x` | `0 + x == x` |
| **Associativity** | `f(a, f(b, c)) == f(f(a, b), c)` | `append(a, append(b, c)) == append(append(a, b), c)` |
| **Commutativity** | `f(a, b) == f(b, a)` | `a + b == b + a` (if applicable) |
| **Inverse** | `f(x, inverse(x)) == id` | `x + (-x) == 0` |

### Example: Monoid Axioms

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn monoid_associativity(
            a in any::<MyMonoid>(),
            b in any::<MyMonoid>(),
            c in any::<MyMonoid>()
        ) {
            let left = a.combine(&b.combine(&c));
            let right = a.combine(&b).combine(&c);
            assert_eq!(left, right);
        }

        #[test]
        fn monoid_left_identity(a in any::<MyMonoid>()) {
            let e = MyMonoid::identity();
            assert_eq!(e.combine(&a), a);
        }

        #[test]
        fn monoid_right_identity(a in any::<MyMonoid>()) {
            let e = MyMonoid::identity();
            assert_eq!(a.combine(&e), a);
        }
    }
}
```

### Doc-Tests as Theorems

Every doc example is an executable theorem:

```rust
/// { xs is non-empty }
/// fn head<T>(xs: &[T]) -> Option<&T>
/// { Some(x) ==> x == xs[0], None ==> xs.is_empty() }
///
/// # Examples
/// ```
/// let v = vec![1, 2, 3];
/// assert_eq!(head(&v), Some(&1));
/// ```
pub fn head<T>(xs: &[T]) -> Option<&T> {
    xs.first()
}
```

---

## 5. Formal Verification: Kani Model Checker

For critical code, use [Kani](https://github.com/model-checking/kani) — a model checker for Rust that verifies properties for **all possible inputs** (not just random samples like proptest).

### Installation

```bash
cargo install --locked kani-verifier
cargo kani setup
```

### Proof Harness

```rust
#[cfg(kani)]
mod proofs {
    use crate::sorted_vec::SortedVec;

    /// Proof: Inserting any element maintains sorted invariant
    /// for all possible inputs in range [0, 100].
    #[kani::proof]
    fn insert_maintains_sorted() {
        let mut sv = SortedVec::new();
        let item: i32 = kani::any();
        kani::assume(item >= 0 && item <= 100);

        sv.insert(item);

        let slice = sv.as_slice();
        for i in 1..slice.len() {
            assert!(slice[i-1] <= slice[i]);
        }
    }
}
```

### Running Proofs

```bash
cd your-project
cargo kani
```

Kani exhaustively checks all execution paths within bounded input ranges. Unlike proptest (randomized sampling), Kani provides **mathematical guarantees** for the bounded domain.

**When to use:**
- Critical invariants that must hold for all inputs
- State machine transitions
- Parser correctness
- Security-sensitive functions

**When NOT to use:**
- IO-bound code (Kani models execution, not network)
- Large unbounded loops (state explosion)
- Code with heavy external dependencies

---

## 6. Abstract Interpretation & Static Analysis

Use the compiler and additional tools as analyzers.

### Compiler as Prover

- **Borrow checker** proves memory safety (no use-after-free, no data races)
- **Exhaustive match** proves all cases are handled
- **Type system** proves invariants at compile time (via Newtype, Typestate)

### Miri (Undefined Behavior Detector)

```bash
# Check for UB in unsafe code
MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test
```

**Rule:** Any `unsafe` block MUST be checked with Miri before merge.

### No-Panic Verification

```toml
[dependencies]
no-panic = "0.1"
```

```rust
use no_panic::no_panic;

#[no_panic]
pub fn safe_computation(x: u32) -> u32 {
    x.saturating_add(1)
}
```

If the compiler cannot prove the function never panics, it will fail to compile.

---

## 7. Fuzzing: Stress Testing Invariants

```toml
[dev-dependencies]
libfuzzer-sys = "0.4"
```

```rust
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_input(s);
    }
});
```

**Rule:** Any parser or deserializer MUST have a fuzz target.

---

## 8. Specification Files

For complex modules, write a separate `.spec.md`:

```markdown
# Spec: matrix

## Types
- `Matrix<R: Dim, C: Dim, T: Field>`
- Invariant: `data.len() == R * C`

## Operations

### multiply
- { lhs.cols == rhs.rows }
- fn multiply(lhs, rhs) -> Matrix<R, rhs.C, T>
- { ret[i][j] == sum_k(lhs[i][k] * rhs[k][j]) }

### transpose
- { true }
- fn transpose(m) -> Matrix<C, R, T>
- { ret[j][i] == m[i][j] }

## Properties
- Associativity: (A * B) * C == A * (B * C)
- Identity: I * A == A
- Transpose involution: transpose(transpose(A)) == A
```

---

## 9. Refinement Types (Lightweight)

Rust does not have full dependent types, but we can approximate:

```rust
use std::marker::PhantomData;

pub struct GreaterThan<const N: i32>(i32, PhantomData<[(); N as usize]>);

impl<const N: i32> GreaterThan<N> {
    pub fn new(value: i32) -> Option<Self> {
        if value > N {
            Some(Self(value, PhantomData))
        } else {
            None
        }
    }

    pub fn get(&self) -> i32 { self.0 }
}

// Usage: type-level proof that x > 0
let x: GreaterThan<0> = GreaterThan::new(5).unwrap();
```

---

## 10. Checklist for Every Module

- [ ] Every public function has Hoare triple in doc comment
- [ ] Every type has documented invariant
- [ ] `debug_assert!` checks preconditions in debug builds
- [ ] Property tests verify all algebraic axioms
- [ ] Doc tests provide executable examples
- [ ] No `unwrap`/`expect`/`panic!` without type-level proof of safety
- [ ] `unsafe` blocks have `// SAFETY:` proof + Miri check
- [ ] Fuzz targets for parsers/deserializers
- [ ] Kani proofs for critical invariants (optional but recommended)
- [ ] `.spec.md` for modules with >3 public types
