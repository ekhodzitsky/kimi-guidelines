# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

- **LSP server**: `cargo kimi lsp` — real-time diagnostics, code actions (Hoare triple stub, SAFETY comment insertion), hover info with score and issue count
- **MCP server**: `cargo kimi mcp` — Model Context Protocol bridge for IDE integration
- **Template restructuring**: Split into `templates/rust/{minimal,rust-only,full}/` and `templates/python/` for multi-language support
- **Shellcheck CI**: `install.sh` is now linted with shellcheck in `.github/workflows/lint.yml`
- **Auto-publish pipeline**: Release workflow includes `publish-crates-io` job (requires `CARGO_REGISTRY_TOKEN` secret)
- **Security hardening**: Path traversal protection in `cargo-kimi`, full secrets audit of both repositories

### Changed

- **Kani workflow extracted**: Moved from inline in `lint.yml` to standalone `.github/workflows/kani.yml` (weekly cron + manual dispatch)
- **deny.toml v2**: Updated to `cargo-deny` v0.19 format, added `MPL-2.0`, `CC0-1.0`, `Unicode-3.0` licenses
- **README rewrite**: Added TOC, command reference table, scoring system explanation, LSP section, FAQ
- **CI hardening**: Fixed SHA-pinned `dtolnay/rust-toolchain` action and added required `toolchain: stable` input everywhere

### Removed

- **Legacy Python scripts**: `scripts/check-contracts.py` and `benchmarks/scoring/score_output.py` — superseded by native `cargo kimi check`
- **Stale artifacts**: `.cargo/config.toml` from repo root, `example-kimi-check.yml`, `pre-commit.example.yaml`, generated build artifacts in `examples/**/target/`

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
