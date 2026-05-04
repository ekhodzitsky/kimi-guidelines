# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [1.0.0] - 2026-05-04

### Added

- Initial release with mathematical programming approach for Rust
- `AGENTS.md` — base rules with Hoare logic and Curry-Howard
- `FORMALISM.md` — concrete tools: proptest, Phantom types, Miri, fuzzing
- `GLOSSARY.md` — mathematical vocabulary (Lemma, Theorem, Axiom, Monad, etc.)
- `PIPELINE.md` — Specification → Type Proof → Implementation → Verification
- `SEVERITY.md` — proof-integrity based classification
- `languages/rust/AGENTS.md` — full Rust guidelines
- `templates/minimal/`, `templates/rust-only/`, `templates/full/` — composable templates
- `examples/rust-demo/` — real Cargo project with Monoid, Phantom types, SortedVec
- `install.sh` — interactive installer with non-interactive mode
- `.github/workflows/lint.yml` — CI structure validation
