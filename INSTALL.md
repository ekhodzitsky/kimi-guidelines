# Installation & Integration Guide

## Scenarios

### 1. New Rust Project (no existing rules)

```bash
# Interactive installer — picks what you need
cd your-project
bash /path/to/kimi-guidelines/install.sh

# Or non-interactive:
bash /path/to/kimi-guidelines/install.sh --template rust-only --yes

# With strictness level (default: standard):
bash /path/to/kimi-guidelines/install.sh --template rust-only --strictness relaxed --yes
```

### 2. Existing Project (already has AGENTS.md)

```bash
cd your-project
bash /path/to/kimi-guidelines/install.sh
# Choose: overwrite (backup created) or save as .new
```

Or manually merge:

```markdown
<!-- My Project AGENTS.md -->
<!-- Includes kimi-guidelines: rust@v1.3.0 -->

# Project-specific conventions
- We use PostgreSQL
- API format: JSON

---

## Rust Module Rules
<!-- @kimi-guidelines: rust@v1.3.0 -->
[Copy from languages/rust/AGENTS.md]
```

### 3. Team/Organization (shared baseline)

```bash
git submodule add https://github.com/ekhodzitsky/kimi-guidelines.git .kimi/dotfiles
```

## Migration Paths

Start with `relaxed` and upgrade as your project matures:

```bash
# Phase 1: warnings only, no CI breaks
bash install.sh --strictness relaxed --yes

# Phase 2: enforce unwrap/panic bans (default)
bash install.sh --strictness standard --yes

# Phase 3: maximum rigor
bash install.sh --strictness strict --yes
```

## What to Include

| If your project has... | Include from kimi-guidelines |
|------------------------|---------------------------|
| No rules yet | `templates/full/AGENTS.md` + `.cargo/config.toml` |
| Existing AGENTS.md | Merge manually or use installer |
| Only Rust | `templates/rust-only/AGENTS.md` |
| Custom config | Installer backs up existing files |

## Version Pinning

Always pin to a tag:

```bash
cd /path/to/kimi-guidelines
git checkout v1.3.0
```

In your `AGENTS.md`:
```markdown
<!-- kimi-guidelines: v1.3.0 -->
<!-- Strictness: standard -->
```

## Updating

```bash
cd /path/to/kimi-guidelines
git fetch origin
git checkout v1.3.0  # New stable version
```

Then re-run `install.sh` in your projects.
