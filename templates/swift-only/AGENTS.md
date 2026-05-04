# Project Guidelines

> Generated from kimi-dotfiles/templates/swift-only
> Includes: base@v1.0.0 + swift@v1.0.0

## Base Rules

- **Code = data** — explicit structure beats clever compression
- **One file = one responsibility**
- **Functions ≤ 40 lines**
- **Explicit error handling** — no silent failures
- **Standard patterns** — avoid custom DSLs

---

## Swift-Specific Rules

### File Structure

Each file starts with `// MARK:` description:
```swift
// MARK: - Email Normalization
/// Implements X. Invariant: Y. Dependencies: Z.
```

### Types

- Wrapper types for semantics: `struct Price { let value: Double }` not raw `Double`
- Enums with associated values for states: `ConnectionState.connected(socket:)`
- `Result`/`throws` everywhere — no `try!`/`as!` outside tests

### Functions

- Pure vs effect separation
- Doc comments with `## Example`
- Nesting depth ≤ 3 — use `guard`, method chains

### Runtime

- Avoid `Mirror`, `objc_getAssociatedObject` unless interop
- `@objc`/`dynamic` only when bridging to Objective-C
- Minimize global `static var` — use DI

### Testing

- `XCTest` with descriptive names
- Property-based tests for invariants
- All `throws` and `nil` paths covered

### Automation

```yaml
# .swiftlint.yml
disabled_rules:
  - force_try
  - force_cast
```

Run: `swift test`, `swiftlint`, `swiftformat --lint`

---

## Project-Specific Rules

<!-- Add your project conventions here -->
