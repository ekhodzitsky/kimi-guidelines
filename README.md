# kimi-dotfiles

Composable mathematical programming guidelines for **Kimi K2.6** (Moonshot AI) and **Rust**.

We treat code as mathematical proof: every function is a lemma, every module is a theorem, types are axioms.

## Philosophy

```
Specification → Type-Level Proof → Implementation → Property Verification → Refinement
```

- **Types as axioms** — encode invariants in the type system (Curry-Howard)
- **Functions as lemmas** — Hoare triples in doc comments
- **Properties as theorems** — universal quantification via proptest, not manual examples
- **Errors as codomain** — `Result` is mathematical result, not exception

## Structure

```
kimi-dotfiles/
├── AGENTS.md                    # Base rules (language-agnostic mathematical principles)
├── FORMALISM.md                 # Concrete tools: Hoare triples, Phantom types, Miri, fuzzing
├── GLOSSARY.md                  # Mathematical vocabulary
├── PIPELINE.md                  # Development pipeline
├── SEVERITY.md                  # Issue classification (axiom violation = CRITICAL)
├── README.md                    # This file
├── CHANGELOG.md                 # Version history
├── LICENSE                      # MIT
├── INSTALL.md                   # Integration guide
├── install.sh                   # Interactive installer
├── .gitignore
├── .github/workflows/lint.yml   # CI for the repo itself
├── languages/
│   └── rust/AGENTS.md           # Full Rust guidelines
├── templates/
│   ├── minimal/AGENTS.md        # Base only
│   ├── rust-only/AGENTS.md      # Base + Rust
│   └── full/AGENTS.md           # Base + Rust
├── examples/
│   ├── existing-project/        # How to merge with existing rules
│   └── rust-demo/               # Real Cargo project with Monoid, Phantom types, SortedVec
└── scripts/                     # Build tools (future)
```

## Quick Start

### Option A: Interactive installer

```bash
cd your-rust-project
bash /path/to/kimi-dotfiles/install.sh
```

### Option B: One-liner (non-interactive)

```bash
curl -sSL https://raw.githubusercontent.com/ekhodzitsky/kimi-dotfiles/main/install.sh | bash -s -- --template rust-only --yes
```

### Option C: Manual copy

```bash
cp kimi-dotfiles/templates/rust-only/AGENTS.md your-project/AGENTS.md
cp kimi-dotfiles/.cargo/config.toml your-project/.cargo/config.toml
```

### Option D: Symlink (personal use only)

```bash
ln -s ~/kimi-dotfiles/languages/rust/AGENTS.md your-project/src/AGENTS.md
```

## Integration with Existing Projects

If you already have `AGENTS.md`, copy a template and merge manually, or use `install.sh` which creates a backup.

See `examples/existing-project/AGENTS.md` for merge patterns.

## Key Documents

| Document | Purpose |
|----------|---------|
| **[FORMALISM.md](FORMALISM.md)** | Concrete patterns: Hoare triples, PhantomData, Typestate, proptest, Miri, fuzzing |
| **[GLOSSARY.md](GLOSSARY.md)** | Mathematical terms: Lemma, Theorem, Axiom, Invariant, Monad, Homomorphism |
| **[PIPELINE.md](PIPELINE.md)** | Formal development process with complexity gates |
| **[SEVERITY.md](SEVERITY.md)** | CRITICAL = axiom violation, MAJOR = proof gap, MINOR = presentation, INFO = suggestion |

## Example: What This Looks Like in Practice

```rust
/// { !items.is_empty() }
/// fn average(items: &[f64]) -> f64
/// { ret.abs_diff_eq(sum(items) / len(items), epsilon) }
pub fn average(items: &[f64]) -> f64 {
    debug_assert!(!items.is_empty(), "precondition: non-empty slice");
    items.iter().sum::<f64>() / items.len() as f64
}
```

See `examples/rust-demo/` for a complete Cargo project with:
- `Monoid` trait with property-tested axioms
- `Quantity<Meters>` via Phantom types
- `SortedVec<T>` with compile-time invariant

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
