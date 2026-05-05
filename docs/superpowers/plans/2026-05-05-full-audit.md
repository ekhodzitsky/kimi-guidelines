# Full Codebase Audit — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Audit all code, documentation, and CI in the kimi-dotfiles repository, produce a consolidated findings report, then fix all CRITICAL and MAJOR issues.

**Architecture:** Three sequential audit passes (Rust code → docs → CI/infra), each producing structured findings. Findings are consolidated into a single report. Fixes applied by severity: CRITICAL first, then MAJOR, then MINOR.

**Tech Stack:** Rust (examples use proptest, reqwest, serde, thiserror, kani), Bash (install.sh), GitHub Actions (YAML), Markdown (docs/templates)

---

### Task 1: Layer 1 — Audit Rust Code and Record Findings

**Files:**
- Read: `examples/rust-demo/src/algebra.rs`, `units.rs`, `sorted_vec.rs`, `kani_proofs.rs`, `lib.rs`
- Read: `examples/rust-http-client/src/client.rs`, `types.rs`, `error.rs`, `lib.rs`, `kani_proofs.rs`
- Read: `examples/rust-http-client/tests/integration_tests.rs`, `fuzz/fuzz_targets/parse_response.rs`
- Read: `examples/rust-demo/Cargo.toml`, `examples/rust-http-client/Cargo.toml`
- Create: `docs/superpowers/specs/2026-05-05-full-audit-report.md`

The following findings have been pre-identified from reading all source files. The executor must verify each one and add any additional issues discovered.

- [ ] **Step 1: Verify finding — MAJOR: Wrong error variant for HTTP failures**

In `examples/rust-http-client/src/client.rs:186` and `:249`, HTTP error responses (4xx, 5xx) are wrapped in `Error::InvalidUrl(...)`. This is semantically wrong — the URL is valid, the server returned an error. The `Error` enum in `error.rs` has no variant for HTTP/API errors.

Verify by reading `client.rs:184-188` and `client.rs:247-250`, then `error.rs`.

- [ ] **Step 2: Verify finding — MAJOR: Misuse of Page type in list_public_repos**

In `client.rs:158-160`, `page.get()` is passed as the `since` query parameter. But GitHub's `/repositories` API uses `since` as a repository ID cursor, not a page number. The `Page` newtype (which represents a 1-based page number) is semantically wrong here. Compare with `list_issues` at `:221-223` which correctly uses `page` and `per_page` query params.

Additionally at `client.rs:176`, next-page token is computed as `page.get() + per_page.get() as u32`, which makes no sense for a `since`-based API.

- [ ] **Step 3: Verify finding — MINOR: Unused User type**

`types.rs:266-271` defines `pub struct User` with `id`, `login`, `avatar_url`. It is re-exported in `lib.rs:19`. Verify it is never used:

```bash
grep -rn "User" examples/rust-http-client/src/ examples/rust-http-client/tests/ --include='*.rs' | grep -v "User-Agent" | grep -v "pub struct User" | grep -v "pub use.*User"
```

- [ ] **Step 4: Verify finding — MINOR: velocity() silent infinity on zero time**

In `examples/rust-demo/src/units.rs:64-72`, the Hoare triple says `{ time.0 != 0.0 }` but the `debug_assert!` only fires in debug builds. In release mode, `velocity(d, Quantity::seconds(0.0))` silently returns infinity. This violates the documented contract.

Note: no Kani proof exists for `velocity` — the removed proof (`d30eb39`) was for f64 division instability. Verify the precondition gap.

- [ ] **Step 5: Verify finding — INFO: FORMALISM.md velocity return type**

In `FORMALISM.md:106-113`, the example `velocity` function returns `Quantity<(Meter, Second)>`. The tuple `(Meter, Second)` is not a clear representation of "meters per second" — the actual code in `units.rs` uses a dedicated `MetersPerSecond` marker type, which is better. The doc example is weaker than the actual code.

- [ ] **Step 6: Verify finding — INFO: algebra.rs Semigroup trait lacks PartialEq bound**

In `examples/rust-demo/src/algebra.rs:6`, `Semigroup` requires `Clone + PartialEq`. But `FORMALISM.md:166-168` shows `Semigroup: Clone` without `PartialEq`. The example adds an extra bound not in the formal definition. This is actually fine (it's needed for testing `assoc`), but the doc and code diverge.

- [ ] **Step 7: Verify finding — MINOR: fuzz target only covers parse_repository_page**

In `examples/rust-http-client/fuzz/fuzz_targets/parse_response.rs`, only `parse_repository_page` is fuzz-tested. `parse_issue_page` (which is also a public parser) has no fuzz target. Per FORMALISM.md §7: "Any parser or deserializer MUST have a fuzz target."

- [ ] **Step 8: Write Layer 1 findings to report**

Create `docs/superpowers/specs/2026-05-05-full-audit-report.md` with the verified findings in a structured table:

```markdown
# Full Codebase Audit Report

> Date: 2026-05-05 | Project: kimi-dotfiles

## Layer 1: Rust Code

| # | Severity | Category | File:Line | Finding | Proposed Fix |
|---|----------|----------|-----------|---------|-------------|
| 1 | MAJOR | correctness | client.rs:186,249 | HTTP errors wrapped in Error::InvalidUrl | Add Error::Http(u16, String) variant |
| 2 | MAJOR | correctness | client.rs:158-160 | Page type misused as `since` cursor | Use separate SinceId newtype or fix API |
| 3 | MINOR | maintainability | types.rs:266-271 | User struct defined but never used | Remove or add usage |
| 4 | MINOR | correctness | units.rs:64-72 | velocity() returns infinity on zero time in release | Consider returning Result or using NonZero |
| 5 | MINOR | docs | fuzz/ | No fuzz target for parse_issue_page | Add fuzz target |
| 6 | INFO | docs | FORMALISM.md:106-113 | velocity example uses tuple, code uses MetersPerSecond | Update doc example |
| 7 | INFO | docs | algebra.rs:6 vs FORMALISM.md:166 | Semigroup bound divergence (PartialEq) | Align doc or code |
```

- [ ] **Step 9: Run tests to verify nothing is currently broken**

```bash
cd examples/rust-demo && cargo test
cd examples/rust-http-client && cargo test
```

Document pass/fail status in the report.

---

### Task 2: Layer 2 — Audit Documentation and Record Findings

**Files:**
- Read: `AGENTS.md`, `FORMALISM.md`, `GLOSSARY.md`, `PIPELINE.md`, `SEVERITY.md`
- Read: `README.md`, `INSTALL.md`, `CHANGELOG.md`
- Read: `templates/rust/minimal/AGENTS.md`, `templates/rust/rust-only/AGENTS.md`, `templates/rust/full/AGENTS.md`, `templates/python/AGENTS.md`
- Read: `languages/rust/AGENTS.md`, `languages/python/AGENTS.md`
- Read: `benchmarks/README.md`, `benchmarks/scorecard.md`
- Read: `skills/kimi-check/SKILL.md`, `skills/kimi-fix/SKILL.md`
- Modify: `docs/superpowers/specs/2026-05-05-full-audit-report.md`

The following findings have been pre-identified. Verify each one.

- [ ] **Step 1: Verify finding — MAJOR: Version inconsistency across documents**

`AGENTS.md` header says `Version: 1.5.0`. The following documents say `Version: 1.3.0`:
- `FORMALISM.md:2`
- `GLOSSARY.md:2`
- `SEVERITY.md:2`
- `PIPELINE.md:2`
- `install.sh:183` — "kimi-dotfiles: v1.3.0"

Verify each version string:

```bash
grep -rn "Version:" AGENTS.md FORMALISM.md GLOSSARY.md PIPELINE.md SEVERITY.md
grep -n "v1\." install.sh
```

- [ ] **Step 2: Verify finding — MAJOR: FORMALISM.md velocity example has weaker types than actual code**

`FORMALISM.md:106-113` shows `velocity` returning `Quantity<(Meter, Second)>` — a tuple phantom type. The actual code (`units.rs`) uses a dedicated `MetersPerSecond` struct, which is strictly better (the tuple doesn't enforce division semantics). The doc example contradicts the best practice it's supposed to teach.

- [ ] **Step 3: Verify finding — MAJOR: FORMALISM.md Semigroup uses different definition than code**

`FORMALISM.md:166-168` defines `Semigroup: Clone` with an `assoc` method. The actual code in `algebra.rs:6` defines `Semigroup: Clone + PartialEq` without `assoc`. These are materially different interfaces. A reader following the doc would write code incompatible with the example.

Also, `FORMALISM.md:167` has `fn assoc(a: &Self, b: &Self, c: &Self) -> bool` as a method on `Semigroup`, making the axiom a runtime-checkable function. The code in `algebra.rs` doesn't have this method — it tests associativity in proptest instead. The doc teaches a fundamentally different design.

- [ ] **Step 4: Verify finding — MINOR: README badge versions may be stale**

`README.md:3` shows `kimi-score-47/100` and `:4` shows `cargo-kimi-v1.6.6`. The git history shows a deprecation notice at `v1.6.5` (`1807345`). Verify whether `v1.6.6` is accurate (check crates.io reference or commit history).

```bash
grep -n "badge" README.md | head -5
git log --oneline | grep -i "1.6"
```

- [ ] **Step 5: Verify finding — MINOR: cargo-kimi references after repo split**

After commit `1807345`, cargo-kimi was moved to a separate repo (`ekhodzitsky/cargo-kimi`). Check if any docs still reference the old in-repo path or contain stale instructions:

```bash
grep -rn "cargo-kimi" --include='*.md' . | grep -v node_modules | grep -v target | grep -v "crates.io" | grep -v "github.com/ekhodzitsky/cargo-kimi"
```

- [ ] **Step 6: Verify finding — MINOR: FORMALISM.md code examples don't compile standalone**

Several FORMALISM.md examples reference types not defined in the snippet (`TcpStream`, `io::Result`, `EmailError`, `Numerator`, `Denominator`, `Quotient`). These are illustrative, but some have subtle issues:
- `FORMALISM.md:29-33`: `divide` function uses `NonZero<Denominator>` but then `debug_assert!(*d != 0)` — the assert is redundant if the type already guarantees non-zero. The comment even says so. This teaches a confusing pattern.

- [ ] **Step 7: Check template consistency**

Read all four Rust templates + Python template. Verify they cover the same core rules or document differences. Check if the "modular" template directory referenced in README exists:

```bash
ls templates/rust/
```

README line 27 references `modular/` but verify it exists.

- [ ] **Step 8: Append Layer 2 findings to report**

Add a `## Layer 2: Documentation` section to the audit report with verified findings in the same table format.

---

### Task 3: Layer 3 — Audit CI/Infrastructure and Record Findings

**Files:**
- Read: `.github/workflows/lint.yml`, `.github/workflows/kani.yml`
- Read: `.github/actions/cargo-kimi/action.yml`, `.github/actions/kimi-dotfiles/action.yml`
- Read: `install.sh`
- Read: `strictness/relaxed.toml`, `strictness/standard.toml`, `strictness/strict.toml`
- Read: `.gitignore`, `.github/CODEOWNERS`
- Modify: `docs/superpowers/specs/2026-05-05-full-audit-report.md`

The following findings have been pre-identified. Verify each one.

- [ ] **Step 1: Verify finding — CRITICAL: cargo-kimi action uses unpinned action reference**

In `.github/actions/cargo-kimi/action.yml:40`, the step uses:
```yaml
uses: dtolnay/rust-toolchain@stable
```

This is NOT pinned by SHA. All other action references in the repo are pinned (e.g., `actions/checkout@34e1148...`, `dtolnay/rust-toolchain@3c5f7ea...`). This is a supply-chain security risk.

Compare with `.github/actions/kimi-dotfiles/action.yml:16` which IS correctly pinned.

- [ ] **Step 2: Verify finding — MAJOR: kani.yml only verifies rust-demo, not rust-http-client**

`.github/workflows/kani.yml:29-31` only runs `cargo kani` on `examples/rust-demo`. But `.github/workflows/lint.yml:107-115` runs Kani on BOTH `rust-demo` and `rust-http-client`. The weekly/on-demand Kani workflow is incomplete.

- [ ] **Step 3: Verify finding — MAJOR: cargo-kimi action input injection risk**

In `.github/actions/kimi-dotfiles/action.yml:26`:
```yaml
run: cargo kimi check --strictness ${{ inputs.strictness }}
```

The `${{ inputs.strictness }}` is not quoted. While GitHub Actions inputs are generally safe from shell injection in composite actions, this is still a best-practice violation. Should be `"${{ inputs.strictness }}"`.

Similarly in `cargo-kimi/action.yml:56`:
```yaml
cargo kimi check --strictness "${{ inputs.strictness }}" --format json > /tmp/kimi-report.json 2>&1
```
This one IS properly quoted — inconsistency.

- [ ] **Step 4: Verify finding — MINOR: install.sh indentation issue in case block**

In `install.sh:127-131`, the "Save as AGENTS.md.new" case has inconsistent indentation:
```bash
            2)
                TEMPLATE_DIR="$SCRIPT_DIR/templates/rust/$TEMPLATE"
if [ "$TEMPLATE" = "python" ]; then
    TEMPLATE_DIR="$SCRIPT_DIR/templates/python"
fi
cp "$TEMPLATE_DIR/AGENTS.md" "AGENTS.md.new"
```

Lines 128-131 break out of the case indentation. This still works but is a maintainability issue.

- [ ] **Step 5: Verify finding — MINOR: install.sh duplicates TEMPLATE_DIR logic**

The TEMPLATE_DIR computation (`$SCRIPT_DIR/templates/rust/$TEMPLATE` with python override) appears at lines 127-131 AND again at lines 146-149. This violates DRY.

- [ ] **Step 6: Verify finding — MINOR: strictness profiles vs Cargo.toml mismatch**

The example projects (`examples/rust-demo/Cargo.toml`, `examples/rust-http-client/Cargo.toml`) both hardcode `lints.clippy.all = "deny"`, which matches `strictness/strict.toml` — not `strictness/standard.toml` (which uses `all = "warn"`). But CI runs them with `--strictness standard`. The examples effectively ignore the strictness system they demonstrate.

- [ ] **Step 7: Verify finding — INFO: lint.yml runs Kani in the verify job — redundant with kani.yml**

`lint.yml:91-115` has a `verify` job that runs Kani on every push/PR. `kani.yml` runs Kani weekly. Running Kani on every PR is slow and potentially redundant with the weekly check. This may be intentional (belt-and-suspenders), but worth noting.

- [ ] **Step 8: Verify finding — INFO: CODEOWNERS doesn't cover all key paths**

`.github/CODEOWNERS` covers `.github/workflows/`, `.github/actions/`, `AGENTS.md`, `templates/`, `install.sh`. Missing coverage for: `FORMALISM.md`, `GLOSSARY.md`, `PIPELINE.md`, `SEVERITY.md`, `examples/`, `languages/`, `strictness/`, `benchmarks/`.

- [ ] **Step 9: Append Layer 3 findings to report**

Add a `## Layer 3: CI/Infrastructure` section to the audit report with verified findings.

- [ ] **Step 10: Add summary section to report**

Add a summary at the top of the report:

```markdown
## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | N |
| MAJOR | N |
| MINOR | N |
| INFO | N |
| **Total** | **N** |
```

Fill in the actual counts after all findings are consolidated.

---

### Task 4: Fix CRITICAL Issues

**Files:**
- Modify: `.github/actions/cargo-kimi/action.yml`

- [ ] **Step 1: Pin dtolnay/rust-toolchain in cargo-kimi action**

In `.github/actions/cargo-kimi/action.yml:40`, replace the unpinned reference:

```yaml
# Before:
uses: dtolnay/rust-toolchain@stable
# After:
uses: dtolnay/rust-toolchain@3c5f7ea28cd621ae0bf5283f0e981fb97b8a7af9 # stable
```

Use the same SHA already used in other workflow files.

- [ ] **Step 2: Verify the fix**

```bash
grep -n "dtolnay/rust-toolchain" .github/actions/cargo-kimi/action.yml .github/actions/kimi-dotfiles/action.yml .github/workflows/lint.yml .github/workflows/kani.yml
```

All references should now use the same pinned SHA.

- [ ] **Step 3: Commit**

```bash
git add .github/actions/cargo-kimi/action.yml
git commit -m "security: pin dtolnay/rust-toolchain by SHA in cargo-kimi action"
```

---

### Task 5: Fix MAJOR Issues — Error Variant and API Semantics

**Files:**
- Modify: `examples/rust-http-client/src/error.rs`
- Modify: `examples/rust-http-client/src/client.rs`
- Modify: `examples/rust-http-client/tests/integration_tests.rs`

- [ ] **Step 1: Add HttpError variant to Error enum**

In `error.rs`, add a new variant for HTTP API errors:

```rust
/// The API returned a non-success status code.
#[error("HTTP {0}: {1}")]
Http(u16, String),
```

- [ ] **Step 2: Update client.rs to use Error::Http**

In `client.rs:184-188`, replace:
```rust
Err(Error::InvalidUrl(format!("HTTP {status}: {body}")))
```
with:
```rust
Err(Error::Http(status.as_u16(), body))
```

Same change at `client.rs:247-250`.

- [ ] **Step 3: Run tests**

```bash
cd examples/rust-http-client && cargo test
```

- [ ] **Step 4: Commit**

```bash
git add examples/rust-http-client/src/error.rs examples/rust-http-client/src/client.rs
git commit -m "fix: use dedicated Http error variant instead of InvalidUrl for API errors"
```

---

### Task 6: Fix MAJOR Issues — Kani Workflow Completeness

**Files:**
- Modify: `.github/workflows/kani.yml`

- [ ] **Step 1: Add rust-http-client to kani.yml**

In `.github/workflows/kani.yml`, after the existing `Run Kani proofs` step (lines 28-31), add:

```yaml
      - name: Run Kani proofs (rust-http-client)
        run: |
          cd examples/rust-http-client
          cargo kani
```

- [ ] **Step 2: Verify consistency with lint.yml**

```bash
grep -A2 "cargo kani" .github/workflows/kani.yml .github/workflows/lint.yml
```

Both workflows should now run Kani on both example projects.

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/kani.yml
git commit -m "ci: add rust-http-client to weekly Kani verification workflow"
```

---

### Task 7: Fix MAJOR Issues — Version Consistency

**Files:**
- Modify: `FORMALISM.md`
- Modify: `GLOSSARY.md`
- Modify: `SEVERITY.md`
- Modify: `PIPELINE.md`
- Modify: `install.sh`

- [ ] **Step 1: Update all version headers to 1.5.0**

In each file, replace `Version: 1.3.0` with `Version: 1.5.0` to match `AGENTS.md`.

In `install.sh:183`, replace `v1.3.0` with `v1.5.0`.

- [ ] **Step 2: Verify all versions are consistent**

```bash
grep -rn "Version:" AGENTS.md FORMALISM.md GLOSSARY.md PIPELINE.md SEVERITY.md
grep -n "v1\." install.sh
```

All should show `1.5.0`.

- [ ] **Step 3: Commit**

```bash
git add FORMALISM.md GLOSSARY.md SEVERITY.md PIPELINE.md install.sh
git commit -m "docs: align all document versions to 1.5.0"
```

---

### Task 8: Fix MAJOR Issues — FORMALISM.md Code/Doc Alignment

**Files:**
- Modify: `FORMALISM.md`

- [ ] **Step 1: Fix velocity example return type**

In `FORMALISM.md:106-113`, replace `Quantity<(Meter, Second)>` with a dedicated `MeterPerSecond` phantom type to match the pattern used in actual code (`units.rs`):

```rust
pub struct MeterPerSecond;

/// { true }
/// fn velocity(distance: Quantity<Meter>, time: Quantity<Second>) -> Quantity<MeterPerSecond>
/// { ret.value == distance.value / time.value }
pub fn velocity(
    distance: Quantity<Meter>,
    time: Quantity<Second>,
) -> Quantity<MeterPerSecond> {
    Quantity(distance.0 / time.0, PhantomData)
}
```

- [ ] **Step 2: Align Semigroup definition with code pattern**

In `FORMALISM.md:166-183`, the Semigroup/Monoid traits include runtime axiom-checking methods (`assoc`, `left_identity`, `right_identity`). The actual code in `algebra.rs` does NOT include these methods — it uses proptest instead. Update the doc to show the proptest approach (which is what the code actually does), or note that both patterns are valid.

Recommended: add a note below the trait definition:

```markdown
**Note:** Axiom-checking methods (`assoc`, `left_identity`) are one approach.
An alternative is to verify axioms via property-based tests (see §4).
The example projects use the proptest approach.
```

- [ ] **Step 3: Fix redundant debug_assert in divide example**

In `FORMALISM.md:29-33`, the `divide` function takes `NonZero<Denominator>` but then `debug_assert!(*d != 0)`. The assert is redundant by construction. Update to remove the assert and clarify:

```rust
pub fn divide(n: Numerator, d: NonZero<Denominator>) -> Quotient {
    // NonZero type guarantees d != 0 at compile time — no runtime check needed
    Quotient(n / d)
}
```

- [ ] **Step 4: Commit**

```bash
git add FORMALISM.md
git commit -m "docs: align FORMALISM.md examples with actual code patterns"
```

---

### Task 9: Fix MINOR Issues — CI and Infrastructure

**Files:**
- Modify: `.github/actions/kimi-dotfiles/action.yml`
- Modify: `install.sh`

- [ ] **Step 1: Quote input in kimi-dotfiles action**

In `.github/actions/kimi-dotfiles/action.yml:26`, replace:
```yaml
run: cargo kimi check --strictness ${{ inputs.strictness }}
```
with:
```yaml
run: cargo kimi check --strictness "${{ inputs.strictness }}"
```

- [ ] **Step 2: Fix install.sh indentation and DRY**

Extract the TEMPLATE_DIR computation into a function or compute it once before the case block. Fix indentation of lines 128-131 to match the surrounding case statement indentation.

- [ ] **Step 3: Commit**

```bash
git add .github/actions/kimi-dotfiles/action.yml install.sh
git commit -m "fix: quote CI inputs, fix install.sh indentation and DRY"
```

---

### Task 10: Fix MINOR Issues — Rust Code Cleanup

**Files:**
- Modify: `examples/rust-http-client/src/types.rs`
- Modify: `examples/rust-http-client/src/lib.rs`
- Create: `examples/rust-http-client/fuzz/fuzz_targets/parse_issues.rs`

- [ ] **Step 1: Remove unused User type (or add TODO)**

If `User` is truly unused (verified in Task 1 Step 3), remove it from `types.rs:265-271` and from the re-export in `lib.rs:19`.

- [ ] **Step 2: Add fuzz target for parse_issue_page**

Create `examples/rust-http-client/fuzz/fuzz_targets/parse_issues.rs`:

```rust
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = rust_http_client::parse_issue_page(data);
});
```

Also add the target to `examples/rust-http-client/fuzz/Cargo.toml` (read it first to check the format).

- [ ] **Step 3: Run tests**

```bash
cd examples/rust-http-client && cargo test
```

- [ ] **Step 4: Commit**

```bash
git add examples/rust-http-client/
git commit -m "fix: remove unused User type, add fuzz target for parse_issue_page"
```

---

### Task 11: Update Audit Report with Final Status

**Files:**
- Modify: `docs/superpowers/specs/2026-05-05-full-audit-report.md`

- [ ] **Step 1: Update each finding with fix status**

Add a `Status` column to each finding table: `FIXED`, `WONT_FIX`, or `DEFERRED`.

- [ ] **Step 2: Update summary counts**

Recalculate the summary table to reflect fixed vs remaining issues.

- [ ] **Step 3: Commit the final report**

```bash
git add docs/superpowers/specs/2026-05-05-full-audit-report.md
git commit -m "docs: finalize audit report with fix status"
```

---

### Task 12: Verification Pass

**Files:**
- Read: all modified files from Tasks 4-10

- [ ] **Step 1: Run all tests**

```bash
cd examples/rust-demo && cargo test
cd ../rust-http-client && cargo test
```

- [ ] **Step 2: Run clippy on both projects**

```bash
cd examples/rust-demo && cargo clippy -- -D warnings
cd ../rust-http-client && cargo clippy -- -D warnings
```

- [ ] **Step 3: Verify CI files parse correctly**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/lint.yml')); print('lint.yml OK')"
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/kani.yml')); print('kani.yml OK')"
python3 -c "import yaml; yaml.safe_load(open('.github/actions/cargo-kimi/action.yml')); print('cargo-kimi OK')"
python3 -c "import yaml; yaml.safe_load(open('.github/actions/kimi-dotfiles/action.yml')); print('kimi-dotfiles OK')"
```

- [ ] **Step 4: Run shellcheck on install.sh**

```bash
shellcheck install.sh
```

- [ ] **Step 5: Verify version consistency**

```bash
grep -rn "Version:" AGENTS.md FORMALISM.md GLOSSARY.md PIPELINE.md SEVERITY.md
```

All should show 1.5.0.

- [ ] **Step 6: Confirm no regressions**

Review git diff of all changes to ensure no unintended modifications. Check that all CRITICAL and MAJOR findings have been addressed.

```bash
git log --oneline HEAD~6..HEAD
```
