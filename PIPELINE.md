# Formal Development Pipeline for Kimi K2.6

> Version: 1.0.0 | Mathematical approach: Specification → Proof Sketch → Implementation → Verification

## Overview

Every code generation task follows a formal pipeline inspired by mathematical proof construction:

```
Specification → Proof Sketch → Implementation → Verification → Refinement
```

This replaces "idea → code" with a rigorous process where each stage has defined inputs, outputs, and acceptance criteria.

---

## Stage 1: Specification

**Input:** User requirement, domain context, existing codebase.
**Output:** Formal contract (types + invariants + pre/postconditions).

### Kimi Actions

1. **Identify the seam** — where does this module fit in the existing architecture?
2. **Define the interface** — what is the smallest surface area that provides the required leverage?
3. **Write the contract**:
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

4. **Apply deletion test** — would deleting this function concentrate complexity or move it?

**Acceptance:** Specification is self-contained. A human can implement it without asking questions.

---

## Stage 2: Proof Sketch

**Input:** Specification.
**Output:** Argument that implementation is possible and correct.

### Kimi Actions

1. **Type-level proof** — can the types express all invariants?
   - If not: refine types (Newtype, Typestate, Phantom Type).
2. **Algorithm selection** — which algorithm satisfies the complexity bound?
3. **Error path enumeration** — list all failure modes and their types.
4. **Write the sketch**:
   ```markdown
   ## Proof Sketch
   
   We iterate over bytes once (O(n)), tracking:
   - at_seen: bool (ensures exactly one '@')
   - whitespace_seen: bool (ensures no whitespace)
   
   At each step, we maintain: "all prior bytes satisfy invariant subset."
   
   Termination: i == len(input). At this point, invariant is fully satisfied
   or we return Err with the first violation found.
   ```

**Acceptance:** Sketch convincingly argues correctness. No hand-waving.

---

## Stage 3: Implementation

**Input:** Specification + Proof Sketch.
**Output:** Code satisfying the contract.

### Kimi Actions

1. **Generate code** following `AGENTS.md` rules:
   - Functions ≤ 40 lines
   - No unwrap/force-unwrap in production
   - Iterator chains > nested control flow
   - Doc comments with examples

2. **Self-review against checklist**:
   - [ ] Every public function has doc comment
   - [ ] No unwrap/expect/panic outside tests
   - [ ] No unsafe without SAFETY comment
   - [ ] Functions ≤ 40 lines
   - [ ] Nesting depth ≤ 3

**Acceptance:** Code compiles. Doc tests pass.

---

## Stage 4: Verification

**Input:** Implementation.
**Output:** Verified module with test coverage and severity report.

### Kimi Actions

1. **Static verification:**
   ```bash
   cargo check --all-features   # or swift build
   cargo clippy -- -D warnings  # or swiftlint
   cargo doc --no-deps          # verify all docs compile
   ```

2. **Dynamic verification:**
   - Unit tests for happy path
   - Unit tests for each error path
   - Property-based tests for invariants
   - Doc tests for examples

3. **Review report:**
   ```markdown
   ## Verification Report: normalize_email
   
   | Check | Status |
   |-------|--------|
   | Compiles | ✅ |
   | Clippy clean | ✅ |
   | Doc tests pass | ✅ |
   | Unit tests (happy) | ✅ |
   | Unit tests (errors) | ✅ |
   | Property tests | ✅ |
   | Functions ≤ 40 lines | ✅ |
   | No unwrap | ✅ |
   
   **Severity:** None. Module approved.
   ```

**Acceptance:** Zero CRITICAL, zero MAJOR. All tests pass.

---

## Stage 5: Refinement (Optional)

**Input:** Verified module + performance requirements.
**Output:** Optimized module with preserved invariants.

### Rules

1. **Invariants are immutable.** Performance optimizations must not violate the contract.
2. **Benchmark before optimizing.** If no benchmark, no optimization.
3. **Unsafe is a last resort.** Requires updated Proof Sketch showing why safe Rust/Swift cannot satisfy constraints.

---

## Complexity Gate

| Task Complexity | Pipeline Stages | Review Depth |
|----------------|-----------------|--------------|
| **TRIVIAL** (≤ 5 lines, no new types) | Spec → Impl | Self-check only |
| **SMALL** (1 function, 1 new type) | Spec → Proof Sketch → Impl → Verify | Quick review |
| **MEDIUM** (new module, multiple functions) | Full pipeline | Formal review with severity report |
| **LARGE** (new subsystem, multiple modules) | Full pipeline × N modules | Multi-agent review (Implementer → Verifier → Skeptic) |

---

## Agent Roles

When complexity is MEDIUM or above, assign roles:

### Specifier
Writes Specification and Proof Sketch. Focuses on correctness, not implementation details.

### Implementer
Generates code from Specification. Focuses on idiomatic code following AGENTS.md.

### Verifier
Runs verification suite, produces severity report. Does not write code, only evaluates.

### Skeptic (for LARGE tasks)
Challenges the Specification. Asks: "What if input is empty? What if this runs concurrently? What if the dependency is unavailable?"

---

## Usage with Kimi

Trigger the pipeline explicitly:

```
User: "Implement email normalization"

Kimi: "I'll follow the formal pipeline.

**Stage 1: Specification**
[Writes contract]

Do you approve the specification?"

User: "Yes"

Kimi: "**Stage 2: Proof Sketch**
[Writes argument]

**Stage 3: Implementation**
[Generates code]

**Stage 4: Verification**
[Runs tests, produces report]"
```

Or request specific stages:
- "Write a specification for X"
- "Review this implementation against its specification"
- "Run verification on module Y"
