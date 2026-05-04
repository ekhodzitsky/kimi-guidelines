# kimi-dotfiles

Composable configuration, instructions, and skills for **Kimi K2.6** (Moonshot AI).

Designed to be **imported into existing projects** without conflicts — you pick what you need.

## Structure

```
kimi-dotfiles/
├── AGENTS.md                    # Base rules (language-agnostic)
├── languages/
│   ├── rust/AGENTS.md           # Rust-specific rules
│   └── swift/AGENTS.md          # Swift-specific rules
├── templates/
│   ├── minimal/AGENTS.md        # Base only
│   ├── rust-only/AGENTS.md      # Base + Rust
│   ├── swift-only/AGENTS.md     # Base + Swift
│   └── full/AGENTS.md           # Base + Rust + Swift
├── examples/
│   └── existing-project/        # How to merge with existing rules
├── skills/
│   ├── SKILL.md                 # Skill template
│   └── examples/                # Ready-made skills
├── install.sh                   # Interactive installer
└── INSTALL.md                   # Integration guide
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

## Versioning

Lock to a version to prevent unexpected changes:

```markdown
<!-- kimi-dotfiles: base@v1.0.0, rust@v1.0.0 -->
```

See [INSTALL.md](INSTALL.md) for detailed integration strategies.
