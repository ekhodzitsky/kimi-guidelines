---
name: kimi-fix
description: Auto-fix contract issues in the current Rust project using cargo-kimi fix.
version: 1.0.0
author: ekhodzitsky
commands:
  - name: fix
    description: Apply or preview auto-fixes for contract violations
    usage: /kimi.fix [--dry-run]
---

# kimi-fix

Run `cargo kimi fix` to mechanically fix contract violations in the current Rust project.

## When to use

- After `/kimi.check` shows missing Hoare triples or unwrap usage
- When you want to batch-apply mechanical fixes before manual review
- To quickly bring a file up to minimum contract standards

## Usage

```bash
# Preview changes without writing files (recommended first step)
/kimi.fix --dry-run

# Apply fixes to source files
/kimi.fix
```

## What it fixes

| Violation | Fix applied |
|-----------|-------------|
| Missing Hoare triple | Inserts `/// { TODO: precondition }` / `/// { TODO: postcondition }` above `pub fn` |
| `.unwrap()` | Replaces with `?` where return type allows |
| `.expect("msg")` | Replaces with `.map_err(\|e\| format!("msg: {e}"))?` |
| `unsafe {` block | Prepends `// SAFETY: TODO: explain why this is safe` |

## Important

- **Always review** generated Hoare triples and SAFETY comments — they are stubs (`TODO`) that require human verification.
- `unwrap() → ?` conversion only works when the enclosing function returns `Result` or `Option`. If the fix breaks compilation, revert and handle manually.
- Run `cargo check` after applying fixes to ensure the code still compiles.

## Workflow

```
/kimi.check          # See what's wrong
/kimi.fix --dry-run  # Preview fixes
/kimi.fix            # Apply fixes
cargo check          # Verify compilation
/kimi.check          # Verify improved score
```
