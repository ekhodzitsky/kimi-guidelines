# Full Codebase Audit — Design Spec

> Date: 2026-05-05 | Project: kimi-dotfiles

## Goal

Perform a comprehensive audit of the entire kimi-dotfiles repository across three layers: Rust code, documentation, and CI/infrastructure. Produce a prioritized report of findings, then apply fixes.

## Approach

**Sequential layered audit (Approach A):** each layer is audited in order so that findings from earlier layers inform later ones. Specialized agents handle each layer.

## Severity Classification

Standard engineering scale with four levels:

| Level | Definition | Examples |
|-------|-----------|----------|
| **CRITICAL** | Bugs, vulnerabilities, data loss — immediate fix required | Logic errors, command injection, incorrect kani proofs |
| **MAJOR** | Significant quality issues — degrade maintainability or correctness | Missing error handling, dead code with side effects, doc/code mismatch, unreliable CI |
| **MINOR** | Small improvements — not broken but annoying | Unused imports, doc inaccuracies, style inconsistencies |
| **INFO** | Observations and recommendations — optional to fix | Possible simplifications, improvement ideas |

Each finding includes: severity, category (correctness/security/docs/ci), file:line, problem description, proposed fix.

## Layer 1: Rust Code

**Scope:** 12 files, ~1,356 lines in two example projects:

- `examples/rust-demo/` — algebra.rs, units.rs, sorted_vec.rs, kani_proofs.rs, lib.rs
- `examples/rust-http-client/` — client.rs, types.rs, error.rs, lib.rs, kani_proofs.rs, fuzz/fuzz_targets/parse_response.rs, tests/integration_tests.rs

**Checks:**

1. **Correctness** — logic errors, off-by-one, Hoare triple comments vs actual behavior, kani proof correctness
2. **Safety** — unwrap/expect outside tests, panics in non-test code, integer overflow, uncontrolled panic paths
3. **Error handling** — Error type design, match exhaustiveness, error context preservation
4. **API design** — public API surface (pub fn/struct), type consistency, newtype invariant enforcement
5. **Tests** — coverage, property-based test quality (proptest), integration test adequacy, fuzz target quality
6. **Dogfooding** — does the example code follow its own AGENTS.md rules (unwrap policy, Hoare triples, kani proofs for every pub fn)

**Agent:** code-reviewer (opus)

## Layer 2: Documentation

**Scope:** ~2,765 lines across ~20 markdown files:

- Core: AGENTS.md, FORMALISM.md, GLOSSARY.md, PIPELINE.md, SEVERITY.md
- Templates: rust/ (minimal, rust-only, full), python/
- Languages: rust/AGENTS.md, python/AGENTS.md
- Support: README.md, INSTALL.md, CHANGELOG.md
- Benchmarks: 10 prompts + scorecard.md + README.md
- Skills: kimi-check/SKILL.md, kimi-fix/SKILL.md

**Checks:**

1. **Internal consistency** — do documents reference the same concepts, versions, and rules without contradiction
2. **Links** — broken or outdated references (especially cargo-kimi after repo split)
3. **Code/doc alignment** — do examples in docs match patterns described in FORMALISM.md and AGENTS.md
4. **Template completeness** — do all templates cover the same rule set, or are some missing rules
5. **Freshness** — CHANGELOG, README badges/versions, install instructions

**Agent:** code-reviewer (opus) for cross-doc consistency; explore agents for link checking

## Layer 3: CI/Infrastructure

**Scope:**

- `.github/workflows/lint.yml`, `.github/workflows/kani.yml`
- `.github/actions/cargo-kimi/action.yml`, `.github/actions/kimi-dotfiles/action.yml`
- `install.sh`
- `strictness/relaxed.toml`, `strictness/standard.toml`, `strictness/strict.toml`
- `.gitignore`, `CODEOWNERS`

**Checks:**

1. **Security** — actions pinned by SHA, no secret exposure, no command injection in scripts
2. **Workflow correctness** — triggers, matrix configuration, failure handling
3. **install.sh** — injection risks, portability (bash vs zsh vs sh), error handling
4. **Strictness profiles** — consistency with documentation, completeness of lint rules
5. **Repo hygiene** — .gitignore coverage, CODEOWNERS accuracy

**Agent:** security-reviewer (opus) for CI/scripts; code-reviewer for config files

## Output Format

1. Each layer produces a list of findings
2. Findings are consolidated into `docs/superpowers/specs/2026-05-05-full-audit-report.md` (markdown table per layer)
3. Fixes are applied after report review, grouped by severity (CRITICAL first)
4. Final verification pass confirms fixes don't introduce regressions

## Execution Plan

1. Layer 1 (Rust code) → findings
2. Layer 2 (Documentation) → findings, informed by Layer 1
3. Layer 3 (CI/Infrastructure) → findings, informed by Layers 1-2
4. Consolidation → unified report with all findings
5. User reviews report
6. Apply fixes (CRITICAL → MAJOR → MINOR)
7. Verification pass
