# Custom Skill Template for Kimi

> Use this template when creating new skills in `.kimi/skills/`.

## Name

Short, concise skill name (e.g., `rust-review`, `api-client`, `git-cleanup`).

## Description

1-2 sentences about what the skill does and when to use it.

## Triggers

How the user activates the skill (keywords, commands, context).

Examples:
- "review rust code"
- "generate api client"
- "clean up git history"

## Preconditions

What must be true before running the skill:
- [ ] Project is initialized
- [ ] Dependencies are installed
- [ ] Specific files exist

## Step-by-Step Scenario

### Step 1: [Name]

Kimi's actions:
1. Read file X
2. Execute command Y
3. Verify result Z

### Step 2: [Name]

...

## Postconditions

What must be true after the skill completes:
- [ ] File is created/modified
- [ ] Tests pass
- [ ] Linters are silent

## Errors and Handling

| Error | Cause | Action |
|-------|-------|--------|
| File not found | Wrong path | Request clarification |
| Tests fail | Regression | Revert changes, report |

## Usage Example

```
User: "Apply rust-review skill to file src/parser.rs"

Kimi:
1. Reads AGENTS.md from rust/
2. Checks parser.rs against the rules
3. Produces a report with findings
```

## Dependencies

What other skills, files, or tools are needed:
- `rust/AGENTS.md`
- `cargo clippy`
- `rustfmt`

## Meta

- Author:
- Version: 1.0.0
- Last updated: YYYY-MM-DD
- Compatibility: Kimi K2.6+
