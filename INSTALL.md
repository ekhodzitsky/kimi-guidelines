# Installation & Integration Guide

## Scenarios

### 1. New Rust Project (no existing rules)

```bash
# Interactive installer — picks what you need
cd your-project
bash /path/to/kimi-dotfiles/install.sh

# Or non-interactive:
bash /path/to/kimi-dotfiles/install.sh --template rust-only --yes
```

### 2. Existing Project (already has AGENTS.md)

```bash
cd your-project
bash /path/to/kimi-dotfiles/install.sh
# Choose: overwrite (backup created) or save as .new
```

Or manually merge:

```markdown
<!-- My Project AGENTS.md -->
<!-- Includes kimi-dotfiles: rust@v1.0.0 -->

# Project-specific conventions
- We use PostgreSQL
- API format: JSON

---

## Rust Module Rules
<!-- @kimi-dotfiles: rust@v1.0.0 -->
[Copy from languages/rust/AGENTS.md]
```

### 3. Team/Organization (shared baseline)

```bash
git submodule add https://github.com/ekhodzitsky/kimi-dotfiles.git .kimi/dotfiles
```

## What to Include

| If your project has... | Include from kimi-dotfiles |
|------------------------|---------------------------|
| No rules yet | `templates/full/AGENTS.md` + `.cargo/config.toml` |
| Existing AGENTS.md | Merge manually or use installer |
| Only Rust | `templates/rust-only/AGENTS.md` |
| Custom config | Installer backs up existing files |

## Version Pinning

Always pin to a tag:

```bash
cd /path/to/kimi-dotfiles
git checkout v1.0.0
```

In your `AGENTS.md`:
```markdown
<!-- kimi-dotfiles: v1.0.0 -->
```

## Updating

```bash
cd /path/to/kimi-dotfiles
git fetch origin
git checkout v1.1.0  # New stable version
```

Then re-run `install.sh` in your projects.
