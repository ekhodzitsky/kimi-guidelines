# kimi-dotfiles

Guidelines and mechanized checks for writing correct Rust with **Kimi K2.6**.

We use types to prevent bugs, contracts to document intent, property tests to verify behavior, and scripts to enforce rules automatically.

## What This Does

1. **Types prove invariants** — `Price(u64)` not `f64`, `Quantity<Meters>` not `f64`
2. **Functions have contracts** — every `pub fn` has precondition/postcondition in doc comment
3. **Property tests verify behavior** — associativity, identity, commutativity via `proptest`
4. **Scripts enforce rules** — `check-contracts.py` verifies Hoare triples and forbids `unwrap()`

## How It Works

When you run `kimi` (Kimi Code CLI) in a project directory, it automatically discovers and injects `AGENTS.md` into the system prompt via `${KIMI_AGENTS_MD}`. This means **zero configuration** — place the file and Kimi follows your rules.

Supported locations (checked in order):
1. `.kimi/AGENTS.md` — project-local config (highest priority)
2. `AGENTS.md` — standard location

Files are merged root→leaf with source annotations. Deeper directories override parent rules.

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

# With strictness level (relaxed | standard | strict):
bash /path/to/kimi-dotfiles/install.sh --template rust-only --strictness relaxed --yes
```

### Option C: Cargo subcommand (recommended)

```bash
# Install once
cargo install --git https://github.com/ekhodzitsky/kimi-dotfiles cargo-kimi

# Use in any project
cargo kimi init --template rust-only --yes
cargo kimi check

# Place in .kimi/ for automatic Kimi CLI discovery
cargo kimi init --template rust-only --location .kimi --yes
```

### Option D: Manual copy

```bash
cp kimi-dotfiles/templates/rust-only/AGENTS.md your-project/AGENTS.md
cp kimi-dotfiles/.cargo/config.toml your-project/.cargo/config.toml
```

## Structure

```
kimi-dotfiles/
├── AGENTS.md                    # The 5 rules (short, self-contained)
├── FORMALISM.md                 # Patterns: Phantom types, Hoare triples, proptest, Kani
├── GLOSSARY.md                  # Vocabulary: Invariant, Precondition, Monoid, Functor
├── PIPELINE.md                  # Development process: Spec → Type Design → Implement → Verify
├── SEVERITY.md                  # Issue classification
├── README.md                    # This file
├── CHANGELOG.md                 # Version history
├── LICENSE                      # MIT
├── INSTALL.md                   # Integration guide
├── install.sh                   # Interactive installer
├── cargo-kimi/                  # Cargo subcommand (init, check, verify, upgrade)
├── .gitignore
├── strictness/
│   ├── relaxed.toml             # Warnings only — gradual adoption
│   ├── standard.toml            # Deny unwrap/panic, warn others (default)
│   └── strict.toml              # Deny everything — maximum rigor
├── .cargo/config.toml           # Clippy rules (installed from strictness/ by install.sh)
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
# Check that every pub fn has a contract and no forbidden unwrap()
python3 scripts/check-contracts.py examples/rust-demo/src/
# ✅ All contracts satisfied.

# With strictness filtering:
python3 scripts/check-contracts.py --strictness relaxed examples/rust-demo/src/
```

CI runs this automatically on every PR.

## Formal Verification (Optional)

For critical code, use [Kani](https://github.com/model-checking/kani) — a model checker for Rust:

```bash
cargo install --locked kani-verifier
cargo kani
```

See `examples/rust-demo/` for proof harnesses.

## Key Documents

| Document | Purpose |
|----------|---------|
| **[FORMALISM.md](FORMALISM.md)** | Concrete patterns: Hoare triples, PhantomData, Typestate, proptest, Miri, Kani, fuzzing |
| **[GLOSSARY.md](GLOSSARY.md)** | Vocabulary: Lemma, Theorem, Axiom, Invariant, Monad, Homomorphism |
| **[PIPELINE.md](PIPELINE.md)** | Development process with complexity gates |
| **[SEVERITY.md](SEVERITY.md)** | CRITICAL = axiom violation, MAJOR = proof gap, MINOR = style, INFO = suggestion |

## Migration Paths

| Strictness | Clippy | Contract Checker | Best For |
|------------|--------|-----------------|----------|
| **relaxed** | warnings only | CRITICAL only | Existing projects, gradual adoption |
| **standard** | deny unwrap/panic | CRITICAL + MAJOR | New projects, daily development (default) |
| **strict** | deny everything | all violations | Greenfield, maximum rigor |

Choose with `install.sh --strictness {relaxed|standard|strict}`. Default is `standard`.

## Versioning

Pin to a tag:
```bash
git clone https://github.com/ekhodzitsky/kimi-dotfiles.git
cd kimi-dotfiles
git checkout v1.3.0
```

In your project's `AGENTS.md`:
```markdown
<!-- kimi-dotfiles: v1.3.0 -->
<!-- Strictness: standard -->
```
