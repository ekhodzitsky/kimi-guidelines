# Guidelines for Correct Rust with Kimi K2.6

> Version: 1.5.0 | Repository: https://github.com/ekhodzitsky/kimi-guidelines

## Meta Principle

Before applying any rule, ask: **what problem does this solve?**

A newtype, a Hoare triple, or a refactor is justified only if it prevents a concrete bug, clarifies an invariant, or removes a footgun. If the answer is "it looks better" or "the score goes up" — revert. Decoration is not engineering.

## The 5 Rules

### 1. Types Prove Invariants

Use the type system to make invalid states unrepresentable.

```rust
// BAD: any f64, negative values possible
fn process(amount: f64) -> f64 { ... }

// GOOD: invariant encoded in type
struct Price(u64); // cents, always >= 0
struct TaxRate(f64); // constructor enforces 0.0..=1.0

fn calculate(price: Price, rate: TaxRate) -> Price { ... }
```

Patterns: [FORMALISM.md](FORMALISM.md) §1–2 (Newtype, Phantom types, Typestate)

### 2. Functions Have Contracts

Every public function documents what it requires and guarantees.

```rust
/// { !items.is_empty() }
/// fn average(items: &[f64]) -> f64
/// { ret == sum(items) / items.len() }
pub fn average(items: &[f64]) -> f64 {
    debug_assert!(!items.is_empty(), "precondition: non-empty slice");
    items.iter().sum::<f64>() / items.len() as f64
}
```

Contracts are machine-checked with Kani. Write a `#[kani::proof]` harness for every non-trivial `pub fn`:

```rust
#[cfg(kani)]
#[kani::proof]
fn average_non_empty_computes_mean() {
    let items: Vec<f64> = vec![kani::any(); 3];
    kani::assume(!items.is_empty());
    let result = average(&items);
    assert_eq!(result, items.iter().sum::<f64>() / items.len() as f64);
}
```

Patterns: [FORMALISM.md](FORMALISM.md) §1 (Hoare triples, debug_assert!, Kani)

### 3. No unwrap/expect/panic Without Proof

Handle all cases. Use `Result`/`Option`. `unwrap()` is allowed only in tests or with compile-time proof.

```rust
// BAD
let port = env::var("PORT").unwrap().parse::<u16>().unwrap();

// GOOD
let port: NonZeroU16 = env::var("PORT")
    .map_err(|e| Error::ConfigMissing("PORT", e))?
    .parse()
    .map_err(Error::InvalidPort)?;
```

Enforced by: `cargo kimi check` + clippy `unwrap_used = "deny"

### 4. Verify with Property Tests

Test universal properties, not just examples.

```rust
proptest! {
    #[test]
    fn associativity(a in any::<T>(), b in any::<T>(), c in any::<T>()) {
        assert_eq!(op(a, op(b, c)), op(op(a, b), c));
    }
}
```

Patterns: [FORMALISM.md](FORMALISM.md) §4 (proptest, fuzzing)

### 5. Check Contracts Automatically

Run mechanized verification before every commit:

```bash
# Check contracts, unwrap/expect/panic, unsafe SAFETY comments
cargo kimi check

# Machine-check Hoare triples with bounded model checking
cargo kimi verify

# Full verification
cargo test
cargo clippy -- -D warnings
cargo doc --no-deps
```

---

## Reference (Optional)

For deeper theory and patterns, see:

- [FORMALISM.md](FORMALISM.md) — Concrete tools: PhantomData, Typestate, proptest, Miri, Kani
- [GLOSSARY.md](GLOSSARY.md) — Vocabulary: Invariant, Precondition, Monoid, Functor
- [PIPELINE.md](PIPELINE.md) — Development process: Spec → Type Design → Implement → Verify
- [SEVERITY.md](SEVERITY.md) — Issue classification: CRITICAL/MAJOR/MINOR/INFO

---

## Final Formula

```
Correctness = Types (prevent bugs) + Contracts (document intent)
            + Properties (verify universally) + Automation (enforce rules)
```
