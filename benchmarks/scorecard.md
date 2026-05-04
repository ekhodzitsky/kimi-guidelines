# A/B Scorecard: kimi-dotfiles Guidelines

**Date:** 2026-05-04
**Methodology:** 10 realistic Rust coding prompts, generated twice (control without guidelines, treatment with AGENTS.md).

## Summary

| Metric | Control | Treatment | Delta |
|--------|---------|-----------|-------|
| **Average Score** | 21.5 / 100 | 91.0 / 100 | **+69.5** |
| **Improvement** | — | — | **+323%** |

## Per-Task Breakdown

| # | Task | Control | Treatment | Delta |
|---|------|---------|-----------|-------|
| 01 | Email Validator | 30 | 100 | +70 |
| 02 | Rate Limiter | 15 | 90 | +75 |
| 03 | Config Parser | 20 | 80 | +60 |
| 04 | JSON Serializer | 25 | 100 | +75 |
| 05 | Job Queue | 10 | 100 | +90 |
| 06 | URL Router | 25 | 100 | +75 |
| 07 | CSV Parser | 35 | 80 | +45 |
| 08 | Password Hasher | 25 | 80 | +55 |
| 09 | Event Bus | 20 | 80 | +60 |
| 10 | LRU Cache | 10 | 100 | +90 |

## Per-Criterion Breakdown

| Criterion | Control | Treatment | Improvement |
|-----------|---------|-----------|-------------|
| Hoare triples | 0.0 avg | 14.2 avg | +14.2 |
| No unwraps | 3.8 avg | 0.0 avg | −3.8 ✅ |
| Newtype pattern | 0% used | 90% used | +90% |
| PhantomData | 0% used | 70% used | +70% |
| Typestate | 0% used | 60% used | +60% |
| Result handling | 50% used | 90% used | +40% |
| Option handling | 50% used | 60% used | +10% |
| Function length | 7.5 avg lines | 7.1 avg lines | −0.4 |

## Key Findings

1. **Guidelines drive massive quality gains.** Code generated with AGENTS.md scored 4× higher on average.
2. **Hoare triples are the biggest differentiator.** Control group had zero contract documentation; treatment averaged 14.2 triples per file.
3. **Zero unwrap in library code.** Treatment eliminated all `unwrap()` / `expect()` / `panic!()` outside tests.
4. **Newtypes and typestate are adopted automatically.** When prompted with guidelines, Kimi consistently uses `PhantomData`, marker structs, and generic state parameters.
5. **Function length stays reasonable.** Both groups kept functions under 10 lines on average (small examples).

## Scoring Methodology

Each `.rs` file is scored 0-100 across 7 criteria:

| Criterion | Max Points | Description |
|-----------|------------|-------------|
| Hoare triples | 30 | `/// {` doc comments (up to 3 counted, 10 pts each) |
| No unwraps | 20 | Penalty of 5 pts per `unwrap()` / `expect()` / `panic!()` |
| Newtype | 10 | Uses tuple struct newtype pattern |
| PhantomData | 10 | Uses `PhantomData` for lifetime/type safety |
| Typestate | 10 | Uses marker structs + generics for typestate |
| Function length | 10 | Average function length ≤ 40 lines |
| Result handling | 10 | Returns `Result` for fallible operations |

Tool: `benchmarks/scoring/score_output.py`
