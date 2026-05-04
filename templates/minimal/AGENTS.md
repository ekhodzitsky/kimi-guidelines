# Project Guidelines

> Generated from kimi-dotfiles/templates/minimal
> Version: 1.0.0

<!-- This is the minimal template — base rules only, no language specifics. -->
<!-- Add your project-specific rules below. -->

## Core Principles

- **Code = data** — explicit structure beats clever compression
- **One file = one responsibility**
- **Functions ≤ 40 lines**
- **Explicit error handling** — no silent failures
- **Standard patterns** — avoid custom DSLs

## Documentation

Every public function must have:
- Brief description (1 line)
- Input/output contracts
- Examples (executable specs)

## Error Handling

- No unwrap/force-unwrap in production
- Typed errors, not strings
- All error paths tested

## LLM Antipatterns

Kimi generates poorly:
- Deep nesting (> 3 levels)
- One-liners with 5+ closures
- Undocumented macros/DSLs

Prefer: explicit steps, standard collections, exhaustive pattern matching.
