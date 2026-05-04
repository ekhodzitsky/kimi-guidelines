# Guidelines for Correct Rust with Kimi K2.6

> Version: 1.2.0 | Repository: https://github.com/ekhodzitsky/kimi-dotfiles

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

Patterns: [FORMALISM.md](FORMALISM.md) §1 (Hoare triples, debug_assert!)

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

Enforced by: `scripts/check-contracts.py` + clippy `unwrap_used = "deny"`

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
# Check that every pub fn has a contract and no forbidden unwrap()
python3 scripts/check-contracts.py src/

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
