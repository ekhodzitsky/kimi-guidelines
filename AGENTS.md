# Global Instructions for Kimi K2.6

> Version: 1.0.0 | Repository: https://github.com/evkhodzitsky/kimi-dotfiles

## Reference Documents

These documents form the formal foundation. Read them before generating or reviewing code:

- **[GLOSSARY.md](GLOSSARY.md)** — Formal language for specifications (Invariant, Precondition, Postcondition, Typestate, Newtype, Depth, Seam)
- **[PIPELINE.md](PIPELINE.md)** — Development pipeline: Specification → Proof Sketch → Implementation → Verification → Refinement
- **[SEVERITY.md](SEVERITY.md)** — Issue classification: CRITICAL / MAJOR / MINOR / INFO

---

## 0. Meta Principle: Code Is Data for the Model

Kimi K2.6 is an LLM. It does not "intuitively understand" code; it recognizes patterns.
The cleaner, more explicit, and more standard the structure, the more accurate the generation and review.

**Golden rule:** if a human has to strain to understand, Kimi will generate incorrectly.

---

## 1. Decomposition: One Module = One Responsibility

- Each file is responsible for **one** concept.
- If a file parses, validates, and writes to a database — split it into three.
- At the top of every module, include an "abstract": what it does, invariants, dependencies.

**Good:**
```
json_parser.rs      // syntax only
schema_validator.rs // semantics only
db_writer.rs        // persistence only
```

**Bad:**
```
parser.rs           // parses, validates, and writes
```

---

## 2. Functions: One Input, One Output, One Task

- Function length ≤ 40 lines.
- If you have to scroll — decompose.
- Separate pure functions from functions with side effects.

**Pure function:**
```rust
fn check_invariant(state: &State) -> bool;
```

**Effect function:**
```rust
fn log_violation(v: &Violation) -> io::Result<()>;
```

---

## 3. Documentation: Contracts in Comments

Every public function must include:
- Brief description (1 line)
- Input conditions (`# Arguments` / `# Parameters`)
- Output conditions (`# Returns`)
- Complexity (`# Complexity`) — if non-trivial
- Examples (`# Examples`) — executable specification

---

## 4. Naming: Name = Documentation

- Avoid single-letter names (except `i`, `j`, `k` in loops).
- Do not abbreviate without reason: `calculate_total` > `calc`.
- Module name + function name = self-documenting.

**Good:**
```rust
use crate::validation::normalize_email;
let total_price = calculate_total(&config, &user_profile);
```

**Bad:**
```rust
use crate::utils::helper; // what does helper do?
let n = calc(&cfg, &usr);
```

---

## 5. Error Handling: Explicit, No Panics

- No `unwrap()`, `expect()`, `panic!` in production code.
- Use `Result` / `Option` (Rust) or `throws` / `Result` (Swift).
- Errors must be typed and informative.

---

## 6. Testing: Proof of Correctness

- Unit tests next to the code (Rust: `#[cfg(test)]`, Swift: `XCTest`).
- Test edge cases and error paths.
- Property-based tests for invariants.

---

## 7. Automation: Linters and Formatters Are Mandatory

- Rust: `rustfmt`, `clippy`
- Swift: `swiftformat`, `swiftlint`
- CI must break the build on warnings.

---

## 8. LLM-Specific Antipatterns

**Kimi generates well:**
- Iterator / `Sequence` method chains
- `?` operator / `guard let`
- Exhaustive `match` / `switch`
- Standard collections

**Kimi generates poorly:**
- Nested `match` inside `match` inside `match`
- One-liners with 5+ closures
- Custom DSLs on top of the language
- "Smart" macros without documentation

**Rule:** if code requires mental decompression — decompose it physically.

---

## 9. Pre-Generation Checklist

- [ ] Every public function has a doc comment with examples
- [ ] No `unwrap` / `force unwrap` outside tests
- [ ] No `unsafe` without a `// SAFETY:` block
- [ ] Every module starts with an `abstract`
- [ ] Functions ≤ 40 lines
- [ ] Nesting depth ≤ 3 levels
- [ ] Linters pass without warnings
- [ ] Tests cover happy path and errors

---

## 10. Formal Pipeline

For non-trivial tasks (MEDIUM+ complexity), follow the pipeline from [PIPELINE.md](PIPELINE.md):

```
Specification → Proof Sketch → Implementation → Verification → Refinement
```

Trigger explicitly or let Kimi auto-detect complexity.

---

## Final Formula

```
Correctness = Types + Explicitness + Composition + Tests
LLM Readability = Human Readability + Standard Patterns + Self-Containment
```
