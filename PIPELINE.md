# Development Pipeline: Proof Construction

> Version: 1.6.0 | Mathematical approach: Specification → Type-Level Proof → Implementation → Property Verification → Refinement

## Overview

Every code generation task follows a proof-construction pipeline. Each stage has defined inputs, outputs, and acceptance criteria.

```
Specification → Type-Level Proof → Implementation → Verification → Refinement
```

This is not a fantasy workflow. It is a disciplined sequence that Kimi can execute in a single session.

---

## Stage 1: Specification

**Input:** User requirement, domain context.
**Output:** Formal contract (types + invariants + Hoare triple + complexity).

### Actions

1. **Identify the seam** — where does this lemma fit?
2. **Define the interface** — what is the smallest surface area?
3. **Write the contract**:

```markdown
## Spec: normalize_email

**Type:** `fn(String) -> Result<String, EmailError>`

**Invariant:** output is lowercase, contains exactly one '@', no whitespace.

**Hoare triple:**
```
{ true }
fn normalize_email(s: String) -> Result<String, EmailError>
{
  Ok(r)  ==> r.is_lowercase() && r.matches('@').count() == 1 && !r.contains(' ')
  Err(EmailError::Empty)       ==> s.is_empty()
  Err(EmailError::NoAt)        ==> !s.contains('@')
  Err(EmailError::Whitespace)  ==> s.contains(' ')
}
```

**Complexity:** O(n) time, O(n) space (for allocation).
```

4. **Apply deletion test** — would deleting this function concentrate complexity?

**Acceptance:** Specification is self-contained. A human can implement it without questions.

---

## Stage 2: Type-Level Proof

**Input:** Specification.
**Output:** Type design that encodes the contract.

### Actions

1. **Can invariants be encoded in types?**
   - If yes: design Newtype / Phantom type / Typestate / const generics
   - If no: document why, use `debug_assert!` as runtime check

2. **Can errors be made unrepresentable?**
   - Example: instead of `Email(String)` with validation, use `Email` that can only be constructed via `parse`

3. **Does the type system prevent invalid transitions?**
   - Use Typestate for state machines
   - Use `&mut` vs `&` to encode uniqueness

4. **Write the type sketch:**

```rust
// Type-level proof sketch:
pub struct Email(String); // invariant enforced by constructor

impl Email {
    /// { true }
    /// fn parse(s: String) -> Result<Email, EmailError>
    /// { Ok(_) ==> invariant holds, Err(_) ==> invariant violated }
    pub fn parse(s: String) -> Result<Self, EmailError> {
        // ... validation ensures invariant
    }
}
```

**Acceptance:** The type system prevents at least one class of bugs that would otherwise need a test.

---

## Stage 3: Implementation

**Input:** Specification + Type sketch.
**Output:** Code satisfying the contract.

### Actions

1. **Generate code** following `AGENTS.md`:
   - Functions ≤ 40 lines (one lemma per screen)
   - Hoare triple in doc comment
   - `debug_assert!` for preconditions not in types
   - No `unwrap`/`expect`/`panic!` without type-level proof

2. **Self-review checklist**:
   - [ ] Hoare triple written
   - [ ] Invariant encoded in type or `debug_assert!`
   - [ ] No unwrap outside tests
   - [ ] Function ≤ 40 lines
   - [ ] Nesting depth ≤ 3

**Acceptance:** Code compiles. Doc tests pass.

---

## Stage 4: Verification

**Input:** Implementation.
**Output:** Verified module with test coverage.

### Actions

1. **Static verification:**
   ```bash
   cargo check --all-features
   cargo clippy -- -D warnings
   cargo doc --no-deps
   ```

2. **Property verification (mandatory):**
   - Identify algebraic structure (Semigroup, Monoid, Functor, etc.)
   - Write property tests for all axioms:
     ```rust
     proptest! {
         #[test]
         fn associativity(a, b, c) { assert_eq!(op(a, op(b, c)), op(op(a, b), c)); }
         #[test]
         fn identity(a) { assert_eq!(op(id, a), a); }
     }
     ```

3. **Dynamic verification:**
   - Unit tests for happy path
   - Unit tests for each error path
   - Doc tests for examples

4. **Unsafe verification (if applicable):**
   ```bash
   cargo +nightly miri test
   ```

5. **Parser verification (if applicable):**
   ```bash
   cargo fuzz run target_name
   ```

**Acceptance:**
- All checks pass
- Property tests cover all axioms
- Zero CRITICAL, zero MAJOR issues (see SEVERITY.md)

---

## Stage 5: Refinement (Optional)

**Input:** Verified module + performance requirements.
**Output:** Optimized module with preserved invariants.

### Rules

1. **Invariants are immutable.** Optimizations must not weaken the contract.
2. **Benchmark before optimizing.** If no benchmark, no optimization.
3. **Unsafe is a last resort.** Requires updated type-level proof showing why safe Rust cannot satisfy constraints.

---

## Complexity Gate

| Complexity | Pipeline | What Defines It |
|------------|----------|----------------|
| **TRIVIAL** | Spec → Impl | ≤ 5 lines, no new types, standard library only |
| **SMALL** | Spec → Type Proof → Impl → Verify | 1 function, 1 new type, no state machine |
| **MEDIUM** | Full pipeline | New module, multiple types, algebraic structure |
| **LARGE** | Full pipeline + spec file | New subsystem, multiple modules, state machines |

For LARGE tasks, write a `.spec.md` file before implementation.

---

## Single-Session Execution

Kimi executes this pipeline in one conversation. No multi-agent fantasy. One model, one session, disciplined steps:

```
User: "Implement email normalization"

Kimi:
[Stage 1: Spec]
[Stage 2: Type Proof]
[Stage 3: Impl]
[Stage 4: Verify]
Done.
```

Or step-by-step on request:
- "Write a spec for X"
- "Design types for this spec"
- "Implement from this type sketch"
- "Verify this implementation"
