# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

- **Repository renamed**: `kimi-guidelines` → `kimi-dotfiles` across all files, links, and documentation
- **Build artifact cleanup**: Removed 733 MB of `examples/**/target/` artifacts from working tree; added per-example `.gitignore`
- **CONTRIBUTING.md**: Guidelines for contributors
- **Makefile**: Unified `make check` / `make test` / `make lint` entrypoints for Python projects

### Changed

- **Unified versioning**: All documents and templates now reference version **1.6.0**
- **README refresh**: Removed misleading score badge, updated all links to `kimi-dotfiles`
- **install.sh portability**: Fixed `sed -i` for macOS/BSD compatibility; corrected Python template + Rust strictness logic
- **CI hardening**: Added `Swatinem/rust-cache@v2` to all Rust workflows; deprecated legacy `kimi-dotfiles` GitHub Action
- **Kani workflow extracted**: Moved from inline in `lint.yml` to standalone `.github/workflows/kani.yml` (weekly cron + manual dispatch)
- **CI hardening**: Fixed SHA-pinned `dtolnay/rust-toolchain` action and added required `toolchain: stable` input everywhere

### Removed

- **Legacy Python scripts**: `scripts/check-contracts.py` and `benchmarks/scoring/score_output.py` — superseded by native `cargo kimi check`
- **Stale artifacts**: `.cargo/config.toml` from repo root, `example-kimi-check.yml`, `pre-commit.example.yaml`, generated build artifacts in `examples/**/target/`
- **Deprecated Action**: Removed `.github/actions/kimi-dotfiles/action.yml` in favor of `.github/actions/cargo-kimi/action.yml`
- **Score exemption**: Removed `// kimi:score-ignore=unwrap` from `kani_proofs.rs` — proof files are now handled by configuration

## [1.3.0] - 2026-05-04

### Added

- **Benchmark framework**: `benchmarks/` with 10 prompts and `score_output.py` for A/B testing
- **Migration paths**: `strictness/{relaxed,standard,strict}.toml` — gradual adoption
- **Real-world example**: `examples/rust-http-client/` — GitHub API client with reqwest, thiserror, typestate
- **Fuzz target**: `examples/rust-http-client/fuzz/` for response parsing
- **Doc tests**: 18 executable examples in rust-http-client
- **`cargo-kimi` CLI**: Cargo subcommand with `init`, `check`, `verify`, `upgrade`
- **A/B benchmark scorecard**: 10 prompts × 2 groups = +323% quality improvement with guidelines
- **Rewrite check-contracts.py in Rust**: `cargo kimi check` no longer requires Python
- **`cargo kimi generate-tests`**: Auto-generates proptest property tests for newtypes with arithmetic impls (Add, Sub, Mul), Ord, Eq, Clone
- **Workspace support**: `cargo kimi check` scans all workspace crates
- **Published on crates.io**: `cargo install cargo-kimi`
- **GitHub Action**: `.github/actions/kimi-guidelines/action.yml` for reusable CI
- **Pre-commit hook**: `pre-commit.example.yaml` for local enforcement

### Changed

- `install.sh` supports `--strictness {relaxed|standard|strict}` (default: standard)
- `scripts/check-contracts.py` filters by strictness level
- Templates include strictness annotation

## [1.2.1] - 2026-05-04

### Fixed

- Demo clippy clean
- Doc tests working
- Version consistency
- Removed orphan artifacts
- Rustdoc warnings fixed
- "universal quantification" → "randomized property testing"

## [1.2.0] - 2026-05-04

### Added

- Mechanized contract verification: `scripts/check-contracts.py`
- Kani model checker integration
- Working demo with property tests
- CI verification pipeline

### Changed

- Root AGENTS.md simplified to ~100 lines
- Rebranded: "mathematical proof" → "structured contracts"

## [1.0.0] - 2026-05-04

### Added

- Initial release with Rust guidelines
- Base rules, GLOSSARY, PIPELINE, SEVERITY, FORMALISM docs
- Templates: minimal, rust-only, full
- install.sh with interactive and non-interactive modes
