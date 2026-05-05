# Full Codebase Audit Report

> Date: 2026-05-05 | Project: kimi-dotfiles | Auditor: Claude Opus 4.6

## Summary

| Severity | Found | Fixed | Deferred |
|----------|-------|-------|----------|
| CRITICAL | 1 | 1 | 0 |
| MAJOR | 6 | 6 | 0 |
| MINOR | 7 | 5 | 2 |
| INFO | 3 | 3 | 0 |
| **Total** | **17** | **15** | **2** |

**Test status before fixes:** rust-demo 13/13 PASS, rust-http-client 36/36 PASS
**Test status after fixes:** rust-demo 13/13 PASS, rust-http-client 36/36 PASS (+ clippy clean)

---

## Layer 1: Rust Code

| # | Severity | Category | File:Line | Finding | Status |
|---|----------|----------|-----------|---------|--------|
| 1 | MAJOR | correctness | `client.rs:186,249` | HTTP errors wrapped in `Error::InvalidUrl(...)` — semantically wrong variant | FIXED: Added `Error::Http(u16, String)` variant |
| 2 | MAJOR | correctness | `client.rs:158-160` | `Page` type misused as `since` cursor; wrong next-page token calculation | FIXED: Changed to `page`/`per_page` params, token increments by 1 |
| 3 | MINOR | maintainability | `types.rs:266-271` | `User` struct defined and re-exported but never used | FIXED: Removed struct and re-export |
| 4 | MINOR | correctness | `units.rs:64-72` | `velocity()` returns infinity on zero time in release mode | DEFERRED: Known f64 limitation, documented via debug_assert |
| 5 | MINOR | completeness | `fuzz/fuzz_targets/` | No fuzz target for `parse_issue_page` | FIXED: Added `parse_issues.rs` fuzz target |
| 6 | INFO | docs | `FORMALISM.md:106-113` | Velocity example used tuple phantom, code uses `MetersPerSecond` | FIXED: Updated to `MeterPerSecond` marker type |
| 7 | INFO | docs | `algebra.rs:6` vs `FORMALISM.md:166` | Semigroup bound divergence (`PartialEq`) | FIXED: Added note in FORMALISM.md about both approaches |

---

## Layer 2: Documentation

| # | Severity | Category | File:Line | Finding | Status |
|---|----------|----------|-----------|---------|--------|
| 8 | MAJOR | consistency | Multiple files | Version mismatch: AGENTS.md=1.5.0, others=1.3.0 | FIXED: All documents updated to 1.5.0 |
| 9 | MAJOR | accuracy | `FORMALISM.md:29-33` | Redundant `debug_assert!` on `NonZero` type | FIXED: Removed assert, added clarifying comment |
| 10 | MINOR | accuracy | `README.md:27` | `modular/` template listed but doesn't exist | DEFERRED: Requires decision — create dir or update README |
| 11 | MINOR | accuracy | `README.md:3-4` | Badge versions may be stale (score, cargo-kimi version) | DEFERRED: Requires crates.io verification |
| 12 | INFO | docs | `FORMALISM.md:166-183` | Doc teaches runtime axiom methods, code uses proptest | FIXED: Added note acknowledging both approaches |

---

## Layer 3: CI/Infrastructure

| # | Severity | Category | File:Line | Finding | Status |
|---|----------|----------|-----------|---------|--------|
| 13 | CRITICAL | security | `action.yml:40` | `dtolnay/rust-toolchain@stable` not pinned by SHA | FIXED: Pinned to `@3c5f7ea...` |
| 14 | MAJOR | completeness | `kani.yml:29-31` | Weekly Kani workflow missing `rust-http-client` | FIXED: Added rust-http-client step |
| 15 | MINOR | security | `action.yml:26` | `${{ inputs.strictness }}` not quoted in shell | FIXED: Quoted input |
| 16 | MINOR | maintainability | `install.sh:127-131` | Broken indentation + duplicated TEMPLATE_DIR logic | FIXED: Extracted computation, fixed indentation |
| 17 | MINOR | completeness | `.github/CODEOWNERS` | Missing coverage for key paths | DEFERRED: Low priority |

---

## Files Modified

| File | Changes |
|------|---------|
| `examples/rust-http-client/src/error.rs` | Added `Http(u16, String)` variant |
| `examples/rust-http-client/src/client.rs` | Fixed error variant, pagination URL, next-page token |
| `examples/rust-http-client/src/types.rs` | Removed unused `User` struct |
| `examples/rust-http-client/src/lib.rs` | Removed `User` re-export |
| `examples/rust-http-client/fuzz/fuzz_targets/parse_issues.rs` | New fuzz target |
| `examples/rust-http-client/fuzz/Cargo.toml` | Added `parse_issues` bin target |
| `FORMALISM.md` | Fixed velocity type, divide assert, added Semigroup note |
| `GLOSSARY.md` | Version 1.3.0 → 1.5.0 |
| `SEVERITY.md` | Version 1.3.0 → 1.5.0 |
| `PIPELINE.md` | Version 1.3.0 → 1.5.0 |
| `install.sh` | Version update, DRY refactor, indentation fix |
| `.github/actions/cargo-kimi/action.yml` | Pinned dtolnay/rust-toolchain by SHA |
| `.github/actions/kimi-dotfiles/action.yml` | Quoted input variable |
| `.github/workflows/kani.yml` | Added rust-http-client Kani step |
