# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [1.2.1] - 2026-05-04

### Fixed

- Demo clippy clean: added `[lints.rust] check-cfg` for `cfg(kani)`
- Doc tests: added working doctest to `lib.rs`
- Version consistency: all files now reference v1.2.1
- Removed orphan build artifacts (`liblib.rlib`, `Cargo.lock`)
- Fixed rustdoc warnings (backticks for generic types)
- Rebranded "universal quantification" → "randomized property testing" in docs
- Cleaned `.gitignore` contradiction

## [1.2.0] - 2026-05-04

### Added

- **Mechanized contract verification**: `scripts/check-contracts.py`
- **Kani model checker integration**: proof harnesses in `examples/rust-demo/src/kani_proofs.rs`
- **Working demo**: compiles, passes clippy, has doc tests
- **CI verification**: contract checker + cargo test + clippy + doc
- `.cargo/config.toml` committed to repo
- Real merge example in `examples/existing-project/AGENTS.md`

### Changed

- Root AGENTS.md simplified to ~100 lines (5 rules)
- Rebranded: "mathematical proof" → "structured contracts"
- Templates fixed: `full/` is actually full
- README: honest description, before/after example

## [1.0.0] - 2026-05-04

### Added

- Initial release with Rust guidelines
- Base rules, GLOSSARY, PIPELINE, SEVERITY, FORMALISM docs
- `languages/rust/AGENTS.md`
- Templates: minimal, rust-only, full
- `install.sh` with interactive and non-interactive modes
- `CHANGELOG.md`, `LICENSE` (MIT), `.gitignore`
