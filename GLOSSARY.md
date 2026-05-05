# Mathematical Glossary for Code

> Version: 1.3.0 | kimi-guidelines
>
> Use these terms exactly. Consistent formal language eliminates ambiguity.

## Core Concepts

### Lemma
A function. A small, provable step with explicit preconditions and postconditions. A module is composed of lemmas.

### Theorem
A module or public API. A composition of lemmas that achieves a significant result. Theorem = interface + proof (implementation + tests).

### Axiom
A type-level invariant that cannot be violated by construction. Enforced by the compiler via Newtype, PhantomData, Typestate, or const generics.

```rust
// Axiom: Price is always non-negative
#[derive(Clone, Copy)]
pub struct Price(u64);
```

### Proof
The combination of:
1. Type system (compiler checks axioms)
2. Contracts (`debug_assert!` checks preconditions)
3. Property tests (randomized property testing over input space)
4. Doc tests (executable examples)
5. Miri (absence of UB in unsafe)
6. Fuzzing (stress testing parsers)

### Invariant
A condition that is always true for a type or at a program point.

- **Type invariant:** encoded in constructor, preserved by all methods
- **Loop invariant:** stated in comment, preserved by each iteration
- **Module invariant:** documented at top of module, verified by property tests

### Precondition (`{ P }`)
What must be true before the function is called. If violated, behavior is undefined (or error is returned).

```rust
/// { !items.is_empty() }
/// fn average(items: &[f64]) -> f64
/// { ret == sum(items) / items.len() }
```

### Postcondition (`{ Q }`)
What is guaranteed true after the function returns. Must hold for all return paths.

### Hoare Triple (`{ P } C { Q }`)
If `P` holds before executing command `C`, then `Q` holds after.

```rust
/// { x >= 0 }
/// fn sqrt(x: f64) -> f64
/// { ret >= 0 && ret * ret == x }
```

---

## Type System Concepts

### Curry-Howard Correspondence
Types are propositions. Programs are proofs. A value of type `T` is evidence that proposition `T` is true.

- `Result<T, E>` — proposition "either T succeeds or E explains why"
- `Option<T>` — proposition "either T exists or nothing exists"
- `NonZeroU32` — proposition "this integer is not zero"
- `!` (never type) — proposition "this computation never returns" (absurdity)

### Newtype
A single-field struct that adds an invariant to a primitive type.

```rust
struct Price(u64);       // invariant: >= 0
struct Email(String);    // invariant: valid format
```

### Phantom Type
A type parameter used only at compile time, with no runtime representation. Used for units, dimensions, and capabilities.

```rust
struct Quantity<T>(f64, PhantomData<T>);
```

### Typestate
Using types to encode the state machine of an object. Invalid transitions are compile-time errors.

```rust
struct Socket<State>;
// Socket<Disconnected> --connect()--> Socket<Connected>
```

### Refinement Type (Lightweight)
A type with a predicate. Rust approximates this via constructor validation and Newtype.

```rust
struct Positive(i32);
impl Positive {
    fn new(n: i32) -> Option<Self> { if n > 0 { Some(Self(n)) } else { None } }
}
```

---

## Algebraic Structures

### Semigroup
A set with an associative binary operation.

- Axiom: `∀a,b,c. combine(a, combine(b, c)) == combine(combine(a, b), c)`

### Monoid
A semigroup with an identity element.

- Axioms:
  - Associativity (inherited from Semigroup)
  - Identity: `∃e. ∀a. combine(e, a) == a && combine(a, e) == a`

### Group
A monoid with inverse elements.

- Axioms: associativity, identity, inverse

### Functor
A type constructor `F` with `map` operation preserving structure.

- Axiom: `map(id) == id` and `map(f ∘ g) == map(f) ∘ map(g)`
- In Rust: `Option`, `Result`, `Vec`, `Iterator`

### Monad
A functor with `bind` (or `and_then`) and `pure` (or `Some`/`Ok`) operations.

- Laws: left identity, right identity, associativity of bind
- In Rust: `Option`, `Result`, `Iterator`, `Future`

### Homomorphism
A structure-preserving map between two algebraic structures.

- Example: `map(f ∘ g) == map(f) ∘ map(g)` — `map` is a homomorphism from functions to functions-over-F

---

## Verification Concepts

### Property-Based Testing
Randomized testing of universal properties. Instead of `assert_eq!(foo(2), 4)`, prove `∀x. foo(foo(x)) == x`.

### Fuzzing
Randomized input generation targeting edge cases, especially for parsers and deserializers.

### Model Checking (Lightweight)
Exhaustive enumeration of states for small types. In Rust: exhaustive `match` is a lightweight model checker for sum types.

### Abstract Interpretation
Analyzing program behavior without executing it. The Rust borrow checker is an abstract interpreter for ownership and lifetimes.

### Soundness
A type system is sound if "well-typed programs cannot go wrong" (Milner). Rust's borrow checker aims for soundness modulo `unsafe`.

### Completeness
A proof system is complete if all true statements are provable. Type systems are incomplete (some correct programs are rejected), but soundness is preferred.

---

## Programming Concepts

### Seam
Where an interface lives; a place behavior can be altered without editing in place. The boundary between theorem (interface) and proof (implementation).

### Depth
Leverage at the interface: a lot of behavior behind a small interface. Deep = high leverage. Shallow = interface nearly as complex as implementation.

### Locality
What maintainers get from depth: change, bugs, knowledge concentrated in one place.

### Deletion Test
Imagine deleting the module. If complexity vanishes, it was a pass-through. If complexity reappears across N callers, it was earning its keep.

---

## Severity in Mathematical Terms

| Severity | Mathematical Meaning |
|----------|---------------------|
| **CRITICAL** | Axiom violation or unsoundness. The type system or invariant is broken. |
| **MAJOR** | Proof gap. Contract is violated, precondition not checked, or property unproven. |
| **MINOR** | Presentation issue. Lemma is correct but poorly documented or formatted. |
| **INFO** | Suggest a stronger axiom or a more elegant proof. |
