# kimi-dotfiles

Composable configuration, instructions, and skills for **Kimi K2.6** (Moonshot AI).

Designed to be **imported into existing projects** without conflicts — you pick what you need.
Built on a **mathematical/formal approach** to programming: contracts, invariants, proofs, verification.

## Philosophy

This repository treats code generation as **formal method construction**:

```
Specification → Proof Sketch → Implementation → Verification → Refinement
```

Every module starts with a contract. Every function has pre/postconditions. Every invariant is explicit.

## Structure

```
kimi-dotfiles/
├── AGENTS.md                    # Base rules (language-agnostic)
├── GLOSSARY.md                  # Formal vocabulary (Invariant, Typestate, Depth, Seam...)
├── PIPELINE.md                  # Development pipeline (Spec → Proof → Impl → Verify)
├── SEVERITY.md                  # Issue classification (CRITICAL/MAJOR/MINOR/INFO)
├── README.md                    # This file
├── INSTALL.md                   # Detailed integration guide
├── install.sh                   # Interactive installer
├── .github/workflows/lint.yml   # CI for the repo itself
├── languages/
│   ├── rust/AGENTS.md           # Full Rust guidelines
│   └── swift/AGENTS.md          # Full Swift guidelines
├── templates/
│   ├── minimal/AGENTS.md        # Base only
│   ├── rust-only/AGENTS.md      # Base + Rust
│   ├── swift-only/AGENTS.md     # Base + Swift
│   └── full/AGENTS.md           # Base + Rust + Swift
├── examples/
│   └── existing-project/        # How to merge with existing rules
└── skills/
    ├── SKILL.md                 # Template for new skills
    ├── examples/                # Ready-made skills
    └── types/                   # Skill categories
        ├── specification.md     # Formal contract generation
        ├── verification.md      # Static + dynamic verification
        └── refactoring.md       # Structural improvement
```

## Quick Start

### Option A: One-shot copy

```bash
# Copy everything into your project
cp -r kimi-dotfiles/languages/rust/* my-rust-project/

# Or use the interactive installer
cd my-project
bash /path/to/kimi-dotfiles/install.sh
```

### Option B: Symlink (keeps updates)

```bash
# Rust project
ln -s ~/kimi-dotfiles/languages/rust/AGENTS.md my-rust-project/src/AGENTS.md

# Swift project
ln -s ~/kimi-dotfiles/languages/swift/AGENTS.md my-swift-project/AGENTS.md
```

### Option C: Compose manually

Copy a template from `templates/` and add your project-specific rules at the bottom.

## Design Philosophy: Composable & Non-Conflicting

Every file in this repo is designed to be **mixed with existing project rules**:

- **Base rules** (`AGENTS.md`) are generic — they don't conflict with language-specific rules
- **Language rules** live in `languages/` — import only what you use
- **Templates** show how to combine layers
- **All rules are additive** — they don't override, they extend

When Kimi reads multiple `AGENTS.md` files, deeper ones override parent ones. This repo uses that hierarchy intentionally:

```
project-root/AGENTS.md          # Your project-specific rules (highest priority)
src/AGENTS.md or .kimi/AGENTS.md # Language-specific rules from this repo
~/.config/kimi/AGENTS.md         # Base rules from this repo (lowest priority)
```

## Formal Foundation

| Document | Purpose |
|----------|---------|
| **[GLOSSARY.md](GLOSSARY.md)** | Formal vocabulary: Module, Interface, Invariant, Precondition, Postcondition, Typestate, Newtype, Depth, Seam |
| **[PIPELINE.md](PIPELINE.md)** | Rigorous development pipeline with complexity gates and agent roles |
| **[SEVERITY.md](SEVERITY.md)** | How to classify violations: CRITICAL (safety), MAJOR (invariants), MINOR (style), INFO (suggestions) |

## Versioning

Lock to a version to prevent unexpected changes:

```markdown
<!-- kimi-dotfiles: base@v1.0.0, rust@v1.0.0 -->
```

See [INSTALL.md](INSTALL.md) for detailed integration strategies.
