# kimi-dotfiles

Mechanized guidelines for writing correct Rust with **Kimi K2.6**.

We enforce contracts through types, verify them through tests, and check them through automation.

## What This Does

1. **Types prove invariants** — `Price(u64)` not `f64`, `Quantity<Meters>` not `f64`, `Socket<Connected>` not `bool`
2. **Functions have contracts** — every `pub fn` has precondition/postcondition in doc comment
3. **Property tests verify axioms** — associativity, identity, commutativity via `proptest`
4. **Scripts enforce rules** — `check-contracts.py` greps for missing Hoare triples and forbidden `unwrap()`

## Quick Start

### Option A: Interactive installer

```bash
cd your-rust-project
bash /path/to/kimi-dotfiles/install.sh
```

### Option B: Non-interactive

```bash
cd your-rust-project
bash /path/to/kimi-dotfiles/install.sh --template rust-only --yes
```

### Option C: Manual copy

```bash
cp kimi-dotfiles/templates/rust-only/AGENTS.md your-project/AGENTS.md
cp kimi-dotfiles/.cargo/config.toml your-project/.cargo/config.toml
```

## Structure

```
kimi-dotfiles/
├── AGENTS.md                    # Base principles
├── FORMALISM.md                 # Concrete patterns: Phantom types, Hoare triples, proptest
├── GLOSSARY.md                  # Vocabulary: Invariant, Precondition, Monoid, Functor
├── PIPELINE.md                  # Development process: Spec → Type Proof → Impl → Verify
├── SEVERITY.md                  # Issue classification
├── README.md                    # This file
├── CHANGELOG.md                 # Version history
├── LICENSE                      # MIT
├── INSTALL.md                   # Integration guide
├── install.sh                   # Interactive installer
├── .gitignore
├── .cargo/config.toml           # Clippy rules (unwrap = deny, panic = deny)
├── scripts/
│   └── check-contracts.py       # Mechanized contract verification
├── .github/workflows/lint.yml   # CI: cargo test, clippy, contract checker
├── languages/
│   └── rust/AGENTS.md           # Full Rust guidelines (350+ lines)
├── templates/
│   ├── minimal/AGENTS.md        # Base only (35 lines)
│   ├── rust-only/AGENTS.md      # Base + Rust summary (85 lines)
│   └── full/AGENTS.md           # Complete ruleset
├── examples/
│   ├── existing-project/        # Merge example for existing AGENTS.md
│   └── rust-demo/               # Real Cargo project with Monoid, Phantom types, SortedVec
```

## Example: Before vs After

**Without guidelines** — Kimi generates:
```rust
fn process(amount: f64, tax: f64) -> f64 {
    amount * (1.0 + tax) // What if amount < 0? What if tax > 1.0?
}
```

**With guidelines** — Kimi generates:
```rust
/// { price.cents() >= 0 && rate.value() <= 1.0 }
/// fn calculate(price: Price, rate: TaxRate) -> Price
/// { ret.cents() == price.cents() + tax_amount }
pub fn calculate(price: Price, rate: TaxRate) -> Price {
    let tax = (price.cents() as f64 * rate.value()).round() as u64;
    Price::from_cents(price.cents() + tax)
}
```

## Mechanized Verification

```bash
# Check that every pub fn has a Hoare triple and no forbidden unwrap()
python3 scripts/check-contracts.py examples/rust-demo/src/
# ✅ All contracts satisfied.
```

CI runs this automatically on every PR.

## Key Documents

| Document | Purpose |
|----------|---------|
| **[FORMALISM.md](FORMALISM.md)** | Concrete patterns: Hoare triples, PhantomData, Typestate, proptest, Miri, fuzzing |
| **[GLOSSARY.md](GLOSSARY.md)** | Vocabulary: Lemma, Theorem, Axiom, Invariant, Monad, Homomorphism |
| **[PIPELINE.md](PIPELINE.md)** | Development process with complexity gates |
| **[SEVERITY.md](SEVERITY.md)** | CRITICAL = axiom violation, MAJOR = proof gap, MINOR = style, INFO = suggestion |

## Versioning

Pin to a tag:
```bash
git clone https://github.com/ekhodzitsky/kimi-dotfiles.git
cd kimi-dotfiles
git checkout v1.0.0
```

In your project's `AGENTS.md`:
```markdown
<!-- kimi-dotfiles: v1.0.0 -->
```
