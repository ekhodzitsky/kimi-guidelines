# Glossary: Formal Programming Language for Kimi

> Version: 1.0.0 | Source: kimi-dotfiles
>
> Use these terms exactly in every suggestion. Consistent formal language eliminates ambiguity.

## Core Concepts

### Module
Anything with an interface and an implementation (function, struct, file, crate, package). A module is the unit of reasoning.

### Interface
Everything a caller must know to use the module: types, invariants, error modes, ordering, config. Not just the type signature.

### Implementation
The code inside the module. Hidden behind the interface.

### Depth
Leverage at the interface: a lot of behavior behind a small interface. **Deep** = high leverage. **Shallow** = interface nearly as complex as the implementation.

### Seam
Where an interface lives; a place behavior can be altered without editing in place.

### Invariant
A condition that is always true before and after the operation. If violated, the module is in an invalid state.

```rust
// Invariant: Price.value >= 0
struct Price(f64);
```

### Precondition
What must be true before the function is called.

```rust
/// Precondition: input may contain invalid items.
fn filter_valid(items: Vec<Item>) -> Vec<ValidItem>;
```

### Postcondition
What is guaranteed to be true after the function returns (success or error).

```rust
/// Postcondition: returns only elements that passed validation.
fn filter_valid(items: Vec<Item>) -> Vec<ValidItem>;
```

## Type System Concepts

### Newtype
A single-field struct wrapping a primitive to add semantic meaning and invariants.

```rust
struct Price(f64);   // not just f64 — it's a Price
```

### Typestate
Using the type system to encode state machine transitions at compile time.

```rust
struct Socket<State>;
// Socket<Disconnected> → Socket<Connected> → Socket<Authenticated>
```

### Phantom Type
A type parameter used only at compile time, with no runtime representation.

```rust
struct Meter; struct Foot;
struct Length<Unit>(f64, PhantomData<Unit>);
```

## Severity Levels

When Kimi identifies a violation, classify it as:

| Level | Definition | Action Required |
|-------|-----------|----------------|
| **CRITICAL** | Violates memory safety, type safety, or correctness guarantees. Code is broken or insecure. | Must fix before commit |
| **MAJOR** | Violates core invariants (unwrap in production, missing SAFETY comment, >40 lines without decomposition). Reduces maintainability significantly. | Must fix before PR |
| **MINOR** | Violates style or documentation conventions (missing example in doc comment, non-descriptive name). | Fix in same PR |
| **INFO** | Suggestion for improvement (could use iterator chain, could add property-based test). | Optional |

## Pipeline Stages

### Specification
Formal contract written before code: types, invariants, pre/postconditions, complexity.

### Proof Sketch
Argument why the implementation satisfies the specification. Can be a comment block or type-level encoding.

### Implementation
Code that realizes the specification.

### Verification
- Static: compiler + clippy/swiftlint + type checker
- Dynamic: unit tests + property tests + doc tests
- Review: severity classification + deletion test

## Verification: Deletion Test

> Imagine deleting the module. If complexity vanishes, it was a pass-through. If complexity reappears across N callers, it was earning its keep.

Use this to evaluate whether a module deserves to exist or should be inlined/merged.
