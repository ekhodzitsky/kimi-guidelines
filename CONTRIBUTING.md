# Contributing to kimi-dotfiles

Thank you for your interest in improving AI agent coding standards.

## Quick Start

```bash
git clone https://github.com/ekhodzitsky/kimi-dotfiles.git
cd kimi-dotfiles
```

## Repository Layout

| Directory | Purpose | Single Source of Truth? |
|-----------|---------|------------------------|
| `AGENTS.md` | Root guidelines (all languages) | Yes — keep under 130 lines |
| `languages/rust/AGENTS.md` | Extended Rust rules | Yes — referenced by templates |
| `languages/python/AGENTS.md` | Extended Python rules | Yes |
| `templates/rust/{minimal,rust-only,full}/` | Generated project templates | No — derived from `languages/` |
| `templates/python/` | Generated Python template | No — derived from `languages/` |
| `strictness/` | Clippy configs | Yes |
| `FORMALISM.md` | Concrete patterns | Yes |
| `GLOSSARY.md` | Vocabulary | Yes |
| `PIPELINE.md` | Development process | Yes |
| `SEVERITY.md` | Issue classification | Yes |

## Making Changes

### Documentation

1. Update the **single source of truth** first (`languages/`, root docs).
2. Regenerate templates if they are affected.
3. Bump the version in **all** changed files to keep them in sync.

### Version Policy

All documents and templates in this repository share a unified version:

```
Version: 1.6.0
```

When releasing, update every `Version:` line and the `<!-- kimi-dotfiles: vX.Y.Z -->` comment.

### Templates

Templates are **derived** from `languages/` docs. If you change `languages/rust/AGENTS.md`, mirror the change into `templates/rust/*/AGENTS.md`.

### Examples

- Every example must pass `cargo clippy -- -D warnings`, `cargo test`, and `cargo kani` (if applicable).
- Do not commit `target/` or `Cargo.lock` unless necessary for reproducibility.

## Pull Request Checklist

- [ ] `shellcheck install.sh` passes
- [ ] All links in README are valid
- [ ] Versions are consistent across modified files
- [ ] Examples build and test successfully
- [ ] CI passes on your fork

## Code of Conduct

Be constructive. This project exists to make AI-generated code reviewable by humans in 30 seconds.
