# Skill Type: Specification

> Generates formal contracts before implementation.

## When to Use

- User asks for a new function/module without defining the contract
- Before writing any non-trivial code (MEDIUM+ complexity)
- When reviewing existing code for missing contracts

## Process

1. **Identify inputs and outputs** — what types, what errors?
2. **Define invariants** — what is always true?
3. **Write preconditions** — what must caller guarantee?
4. **Write postconditions** — what does callee guarantee?
5. **Specify complexity** — time and space bounds.

## Output Format

```markdown
## Specification: function_name

**Type:** `fn(Input) -> Result<Output, Error>`

**Invariant:** [what is always true]

**Precondition:**
- [condition 1]
- [condition 2]

**Postcondition:**
- Ok(output): [condition]
- Err(Error::Variant): [when and why]

**Complexity:** O(?) time, O(?) space.
```

## Example

```markdown
## Specification: normalize_email

**Type:** `fn(String) -> Result<String, EmailError>`

**Invariant:** output is lowercase, contains exactly one '@', no whitespace.

**Precondition:** input is valid UTF-8 (enforced by String type).

**Postcondition:**
- Ok(email): satisfies RFC 5322 subset, invariant holds.
- Err(EmailError::InvalidFormat): input does not contain '@'.
- Err(EmailError::Empty): input is empty string.

**Complexity:** O(n) time, O(1) extra space.
```
