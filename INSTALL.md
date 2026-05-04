# Installation & Integration Guide

## Scenarios

### 1. New Project (no existing rules)

```bash
# Interactive installer — picks what you need
cd your-project
bash /path/to/kimi-dotfiles/install.sh

# Or manually copy a template
cp /path/to/kimi-dotfiles/templates/rust-only/AGENTS.md your-project/AGENTS.md
```

### 2. Existing Project (already has AGENTS.md or skills)

**Strategy: Include with attribution**

At the top of your existing `AGENTS.md`, add:

```markdown
<!-- Includes kimi-dotfiles: base@v1.0.0, rust@v1.0.0 -->
<!-- Source: https://github.com/YOUR_USERNAME/kimi-dotfiles -->

# Your existing rules...
```

Then append relevant sections from this repo, or keep them as separate files:

```
your-project/
├── AGENTS.md                 # Your existing rules (highest priority)
└── .kimi/
    └── AGENTS.md             # kimi-dotfiles rules (extends above)
```

### 3. Multi-Language Project (e.g., Rust + Swift + your rules)

```markdown
<!-- My Project AGENTS.md -->
<!-- Includes kimi-dotfiles: base@v1.0.0 -->

# Project-specific conventions
- We use PostgreSQL, not SQLite
- All dates are ISO-8601

---

<!-- The following sections are imported from kimi-dotfiles -->

## Rust Module Rules
<!-- @kimi-dotfiles: rust@v1.0.0 -->
[Copy or reference rust/AGENTS.md content here]

## Swift Module Rules
<!-- @kimi-dotfiles: swift@v1.0.0 -->
[Copy or reference swift/AGENTS.md content here]
```

### 4. Team/Organization (shared baseline)

Create a submodule or fork:

```bash
git submodule add https://github.com/YOUR_USERNAME/kimi-dotfiles.git .kimi/dotfiles

# In your project's AGENTS.md:
# Reference: .kimi/dotfiles/languages/rust/AGENTS.md
```

## What to Include

| If your project has... | Include from kimi-dotfiles |
|------------------------|---------------------------|
| No rules yet | `templates/full/AGENTS.md` |
| Existing AGENTS.md | `AGENTS.md` (base) + relevant language |
| Only Rust | `templates/rust-only/AGENTS.md` |
| Only Swift | `templates/swift-only/AGENTS.md` |
| Rust + Swift | `templates/full/AGENTS.md` or compose manually |
| Custom skills | `skills/SKILL.md` + `skills/examples/` |

## Version Pinning

Always pin to a tag to avoid breaking changes:

```bash
cd kimi-dotfiles
git checkout v1.0.0  # Lock to stable version
```

In your `AGENTS.md`:
```markdown
<!-- kimi-dotfiles version: v1.0.0 -->
<!-- Update guide: https://github.com/YOUR_USERNAME/kimi-dotfiles/blob/main/CHANGELOG.md -->
```

## Updating

```bash
cd /path/to/kimi-dotfiles
git fetch origin
git checkout v1.1.0  # New stable version

# Then re-run install.sh in your projects
cd your-project
bash /path/to/kimi-dotfiles/install.sh
```
