---
name: kimi-check
description: Run cargo-kimi contract checker on the current Rust project and report scores.
version: 1.0.0
author: ekhodzitsky
commands:
  - name: check
    description: Run contract checks with optional strictness level
    usage: /kimi.check [--strictness relaxed|standard|strict]
---

# kimi-check

Run `cargo kimi check` on the current Rust workspace and report the contract score.

## When to use

- After making changes to Rust source files
- Before committing or pushing code
- To verify contract compliance (Hoare triples, unwrap usage, newtypes, etc.)

## Usage

```bash
# Default (standard strictness)
/kimi.check

# Relaxed — only critical issues
/kimi.check --strictness relaxed

# Strict — all issues including info
/kimi.check --strictness strict
```

## What it does

1. Detects the Rust workspace (single crate or workspace)
2. Runs `cargo kimi check --strictness <level>`
3. Reports:
   - Per-file contract score (0–100)
   - Project average score
   - List of violations by severity
   - ASCII trend chart if history exists

## Score breakdown

| Category | Points | What we check |
|----------|--------|---------------|
| Hoare triples | 30 | `pub fn` has `/// { pre }` / `/// { post }` |
| No unwrap/expect/panic | 20 | No `.unwrap()`, `.expect()`, `panic!()` in production code |
| Newtype wrappers | 10 | Domain types wrapped (e.g. `Rpm(u32)`) |
| PhantomData | 10 | Markers for lifetime/raw-pointer safety |
| Typestate | 10 | Compile-state-machine pattern |
| Function length | 10 | Average ≤ 40 lines |
| Result handling | 10 | Proper `?` propagation |

## Next steps

If score is low, run `/kimi.fix` to auto-insert stubs and mechanical fixes.
