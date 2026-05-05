# kimi-dotfiles — AI Agent Coding Standards

[![cargo-kimi](https://img.shields.io/badge/cargo--kimi-v1.6.6-blue)](https://crates.io/crates/cargo-kimi)
[![CI](https://github.com/ekhodzitsky/kimi-dotfiles/actions/workflows/lint.yml/badge.svg)](https://github.com/ekhodzitsky/kimi-dotfiles/actions)

> **Making AI-generated code reviewable by humans in 30 seconds.**

**AGENTS.md** is the `.eslintrc` for AI agents.  
**[cargo-kimi](https://github.com/ekhodzitsky/cargo-kimi)** is the enforcer (Rust).  
**Score** is the quality gate.

When Kimi (or any agent) opens your repo, it reads `AGENTS.md` automatically via `${KIMI_AGENTS_MD}` and knows your invariants before it writes a single line of code.

---

## Repository Structure

```
kimi-dotfiles/
├── AGENTS.md                 # Root guidelines (applies to all subdirectories)
├── FORMALISM.md              # Concrete patterns: Hoare triples, PhantomData, Typestate
├── GLOSSARY.md               # Vocabulary: Lemma, Theorem, Axiom, Invariant, Monad
├── PIPELINE.md               # Development process with complexity gates
├── SEVERITY.md               # CRITICAL = axiom violation, MAJOR = proof gap, etc.
├── templates/                # AGENTS.md templates by project type
│   ├── rust/
│   │   ├── minimal/
│   │   ├── rust-only/
│   │   ├── full/
│   │   └── modular/
│   └── python/
├── strictness/               # Clippy configs: relaxed, standard, strict
├── examples/                 # Example projects following the guidelines
│   ├── rust-demo/
│   └── rust-http-client/
├── languages/                # Language-specific rule sets
│   ├── rust/
│   └── python/
├── benchmarks/               # Prompt benchmarks and scoring rubrics
├── skills/                   # Kimi CLI skills
└── .github/
    └── actions/
        └── cargo-kimi/       # Reusable GitHub Action for CI
```

---

## Quick Start

### Rust

```bash
# 1. Install the enforcer
cargo install cargo-kimi

# 2. Initialize AGENTS.md in your project
cargo kimi init --template rust-only --yes

# 3. Run the quality gate
cargo kimi check
# → src/error.rs   (score: 80)
# → src/ffi.rs     (score: 40)  [CRITICAL] L17: unwrap() outside test
# Average score: 60/100

# 4. Watch your team improve over time
cargo kimi trend --days 30
# → 2026-05-01  ████████░░ 45/100
# → 2026-05-04  █████████░ 47/100
```

### Python

```bash
# 1. Copy the Python guidelines
cp kimi-dotfiles/templates/python/AGENTS.md your-project/AGENTS.md

# 2. Install tools
pip install mypy ruff black hypothesis pydantic pip-audit

# 3. Run checks (or use the provided Makefile)
make check
```

> **Note:** `cargo-kimi` now lives in its own repository:  
> https://github.com/ekhodzitsky/cargo-kimi

---

## Why This Exists

AI coding assistants are fast—but left unchecked they produce:

- `unwrap()` in production paths
- Functions without documentation
- `f64` where `Price(u64)` should live
- No proof that types actually encode invariants

The result: **a code review that takes 30 minutes instead of 30 seconds.**

kimi-dotfiles fixes this by making the agent's constraints *explicit*, *measurable*, and *enforcable*.

---

## What It Does

| Layer | What | Where |
|-------|------|-------|
| **Contract** | `AGENTS.md` tells the agent your rules | Root or `.kimi/` |
| **Measure** | `cargo kimi check` scores every file 0-100 | CI or pre-commit |
| **Enforce** | Clippy + contract checker block bad commits | `.cargo/config.toml` |
| **Track** | `cargo kimi trend` shows score history | `.kimi/score-history.jsonl` |

### Honesty Policy

We do **not** claim mathematical proof. We claim:
- **Types encode invariants** — `NonZeroU64` beats `u64 > 0` comments.
- **Tests find bugs** — property tests catch edge cases humans miss.
- **Hoare triples are documentation prompts** — `/// { pre } fn foo() { post }` tells the next agent (human or AI) what the function promises.

---

## Installation Options

### Option A: Cargo subcommand (Rust, recommended)

```bash
# Install once
cargo install cargo-kimi

# Initialize in any Rust project
cargo kimi init --template rust-only --yes

# Place in .kimi/ for automatic Kimi CLI discovery
cargo kimi init --template rust-only --location .kimi --yes

# Run checks
cargo kimi check
```

### Option B: Interactive installer

```bash
cd your-rust-project
bash /path/to/kimi-dotfiles/install.sh
```

### Option C: Non-interactive

```bash
bash /path/to/kimi-dotfiles/install.sh --template rust-only --strictness relaxed --yes
```

### Option D: Manual copy

```bash
cp kimi-dotfiles/templates/rust/rust-only/AGENTS.md your-project/AGENTS.md
cp kimi-dotfiles/.cargo/config.toml your-project/.cargo/config.toml
```

---

## Tools

| Tool | Language | Repository |
|------|----------|------------|
| `cargo-kimi` | Rust | [ekhodzitsky/cargo-kimi](https://github.com/ekhodzitsky/cargo-kimi) |

---

## Kimi-Specific Integration

When you run `kimi` in a project directory, it automatically discovers and injects `AGENTS.md` into the system prompt via `${KIMI_AGENTS_MD}`.

**Zero configuration.** Place the file and Kimi follows your rules.

Supported locations (checked in order):
1. `.kimi/AGENTS.md` — project-local config (highest priority)
2. `AGENTS.md` — standard location

Files are merged root→leaf with source annotations. Deeper directories override parent rules.

---

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

---

## Migration Paths

| Strictness | Clippy | Contract Checker | Best For |
|------------|--------|-----------------|----------|
| **relaxed** | warnings only | CRITICAL only | Existing projects, gradual adoption |
| **standard** | deny unwrap/panic | CRITICAL + MAJOR | New projects, daily development (default) |
| **strict** | deny everything | all violations | Greenfield, maximum rigor |

Choose with `install.sh --strictness {relaxed|standard|strict}`. Default is `standard`.

---

## CI / Pre-commit

### GitHub Action

```yaml
- uses: ekhodzitsky/cargo-kimi/.github/actions/cargo-kimi@main
  with:
    strictness: standard
```

> The action installs `cargo-kimi` from crates.io and runs checks automatically.

### Pre-commit hook

Copy `pre-commit.example.yaml` to `.pre-commit-config.yaml` to block commits without contracts.

---

## Key Documents

| Document | Purpose |
|----------|---------|
| **[FORMALISM.md](FORMALISM.md)** | Concrete patterns: Hoare triples, PhantomData, Typestate, proptest, Miri, Kani, fuzzing |
| **[GLOSSARY.md](GLOSSARY.md)** | Vocabulary: Lemma, Theorem, Axiom, Invariant, Monad, Homomorphism |
| **[PIPELINE.md](PIPELINE.md)** | Development process with complexity gates |
| **[SEVERITY.md](SEVERITY.md)** | CRITICAL = axiom violation, MAJOR = proof gap, MINOR = style, INFO = suggestion |

---

## Roadmap

### Done
- [x] `cargo kimi check` with per-file scoring
- [x] `cargo kimi trend` for score history
- [x] Modular templates (`parts/`)
- [x] MCP server for cross-agent integration
- [x] SARIF output for GitHub Advanced Security
- [x] `cargo kimi watch` for continuous checking
- [x] Standalone `cargo-kimi` repository
- [x] Python agent guidelines

### Planned
- [ ] IDE extension (real-time score in editor)
- [ ] Custom rule DSL
- [ ] TypeScript agent guidelines (`ts-kimi`?)
- [ ] Go agent guidelines (`go-kimi`?)

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide.

Quick areas where help is especially appreciated:

- New language guidelines (TypeScript, Go, Zig)
- Additional `templates/` for frameworks (Axum, Actix, Django, FastAPI)
- Benchmarks and prompt engineering improvements
- Documentation translations

Open an issue first if the change is larger than a typo fix.

## Versioning

Pin to a tag:
```bash
git clone https://github.com/ekhodzitsky/kimi-dotfiles.git
cd kimi-dotfiles
git checkout v1.6.0  # or latest tag
```

In your project's `AGENTS.md`:
```markdown
<!-- kimi-dotfiles: v1.6.0 -->
<!-- Strictness: standard -->
```

## License

MIT
