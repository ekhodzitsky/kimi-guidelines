# Skill: rust-review

> Ready-made skill for reviewing Rust code against kimi-dotfiles rules.

## Description

Performs a structured review of Rust files against the kimi-dotfiles Rust guidelines. Checks for: unwrap usage, unsafe blocks without SAFETY comments, function length, nesting depth, doc comments, and clippy compliance.

## Triggers

- "review this rust file"
- "check rust code quality"
- "rust-review src/parser.rs"
- "audit rust module"

## Preconditions

- [ ] File is a valid `.rs` file
- [ ] Project has `Cargo.toml` (for clippy check)

## Step-by-Step Scenario

### Step 1: Read Guidelines

1. Load `kimi-dotfiles/languages/rust/AGENTS.md`
2. Extract the checklist section

### Step 2: Analyze File

1. Read the target `.rs` file
2. Check each item from the checklist:
   - [ ] Every public function has doc comment with examples
   - [ ] No `unwrap`/`expect`/`panic!` outside tests
   - [ ] No `unsafe` without `// SAFETY:`
   - [ ] Module starts with `//! abstract`
   - [ ] Functions ≤ 40 lines
   - [ ] Nesting depth ≤ 3 levels
   - [ ] All error paths tested

### Step 3: Run External Checks

If `cargo` is available:
```bash
cargo clippy -- -D warnings
cargo test
cargo doc --no-deps
```

### Step 4: Generate Report

Produce a structured report:

```markdown
## Review: src/parser.rs

### ✅ Passed
- Function lengths OK
- Doc comments present

### ⚠️ Warnings
- Line 45: Consider using `and_then` instead of nested `match`

### ❌ Critical
- Line 78: `unwrap()` in production code — use `?` operator
- Line 92: `unsafe` block without `// SAFETY:` comment

### Suggested Fixes
[Provide code snippets]
```

## Postconditions

- [ ] Review report is generated
- [ ] Critical issues are highlighted
- [ ] Suggested fixes are provided

## Errors and Handling

| Error | Cause | Action |
|-------|-------|--------|
| File not found | Wrong path | Ask user to confirm path |
| Not a Rust file | Wrong extension | Abort with explanation |
| Clippy not installed | Missing toolchain | Run manual checks only |

## Dependencies

- `kimi-dotfiles/languages/rust/AGENTS.md`
- `cargo clippy` (optional)
- `cargo test` (optional)

## Meta

- Author: kimi-dotfiles
- Version: 1.0.0
- Compatibility: Kimi K2.6+
