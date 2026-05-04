# Instructions for Swift Code Generation (Kimi K2.6)

> Version: 1.0.0 | Source: kimi-dotfiles/swift/
>
> Usage: copy into the root of a Swift package or `.kimi/AGENTS.md`.

## 0. Meta Principle

Kimi K2.6 processes code as data. The more structured and explicit the code, the more accurate the generation.
Swift is a language with a powerful type system: use `struct`, `enum`, `guard`, `Result` to express intent.

---

## I. Decomposition: One File = One Responsibility

### Rule 1.1: Separation by Abstraction Level

```swift
// BAD: ApiService.swift — parses JSON, validates, and writes to CoreData

// GOOD:
// - JSONDecoder+Extensions.swift (syntax only)
// - RequestValidator.swift (semantics only)
// - CoreDataRepository.swift (persistence only)
```

### Rule 1.2: MARK and Module Documentation

```swift
// MARK: - Email Normalization
/// Implements email address normalization.
///
/// Invariant: output is always lowercase, without spaces.
/// Dependencies: Foundation
```

### Rule 1.3: Nesting Depth ≤ 3

Nested `if let` inside `if let` inside `switch` is a refactoring signal.
Use `guard`, adapter functions, and `Optional`/`Result` methods.

---

## II. Functions: One Input, One Output

### Rule 2.1: Function = Atomic Operation

Length ≤ 40 lines. If you scroll — decompose.

```swift
// BAD
func process(items: [Item]) throws {
    let valid = items.filter { $0.isValid }
    let context = persistentContainer.viewContext
    for item in valid {
        let entity = ItemEntity(context: context)
        entity.id = item.id
        try context.save()
    }
}

// GOOD
/// Precondition: input may contain invalid items.
/// Postcondition: returns only items that passed validation.
func filterValid(_ items: [Item]) -> [ValidItem] { ... }

/// Precondition: context is active.
/// Postcondition: transaction is saved or an error is thrown.
func persist(_ items: [ValidItem], in context: NSManagedObjectContext) throws { ... }
```

### Rule 2.2: Separate Pure Functions and Side Effects

```swift
// FORBIDDEN: function returns Bool AND writes to log AND mutates global state

// ALLOWED:
func isValidEmail(_ email: String) -> Bool           // pure
func logError(_ error: Error) -> Void                // effect
```

### Rule 2.3: Doc Comments as API Documentation

```swift
/// Brief description (1 line).
///
/// - Parameter input: input condition (e.g., `count > 0`, non-nil).
/// - Returns: output condition.
/// - Throws: when and why an error occurs.
///
/// - Complexity: O(n) time, O(1) extra space.
///
/// ## Example
/// ```swift
/// let result = try process([1, 2])
/// XCTAssertEqual(result, 3)
/// ```
```

---

## III. Types: The Compiler as Co-Author

### Rule 3.1: Typealiases and Wrapper Types for Semantics

Do not use raw primitives where semantics matter.

```swift
// BAD
func calculate(price: Double, taxRate: Double) -> Double

// GOOD
struct Price {
    let value: Double  // invariant: >= 0
    init?(_ value: Double) { ... }
}

struct TaxRate {
    let value: Double  // invariant: 0.0...1.0
    init?(_ value: Double) { ... }
}

func calculate(price: Price, rate: TaxRate) -> Price
```

### Rule 3.2: Result / throws Instead of fatalError

`fatalError()`, `try!`, `as!` — only in tests and extreme cases.

```swift
// BAD
let port = Int(ProcessInfo.processInfo.environment["PORT"]!)!

// GOOD
let port: UInt16 = try Environment.get("PORT")
    .flatMap { UInt16($0) }
    .mapError { .configMissing("PORT", $0) }
```

### Rule 3.3: Enum with Associated Values for States

If an object has a lifecycle phase, express it in types.

```swift
enum ConnectionState {
    case disconnected
    case connected(socket: URLSessionWebSocketTask)
    case authenticated(socket: URLSessionWebSocketTask, token: String)
}

// Impossible to call send() on .disconnected without handling at compile time.
```

---

## IV. Collections: Method Chains Instead of Nesting

### Rule 4.1: Sequence Method Composition

```swift
// BAD: Kimi gets lost in brackets
let x: Int?
if let y = maybeY {
    if let z = Int(y), z > 0 {
        x = z * 2
    } else {
        x = nil
    }
} else {
    x = nil
}

// GOOD: each step is a separate transformation
let x = maybeY
    .flatMap(Int.init)
    .filter { $0 > 0 }
    .map { $0 * 2 }
```

### Rule 4.2: Enums Instead of String Constants

```swift
// BAD
let status = "active"

// GOOD
enum Status: String, Codable {
    case active, inactive, suspended
}
```

---

## V. Methods and Runtime

### Rule 5.1: Avoid Runtime Mechanisms Unless Needed

`Mirror`, `objc_getAssociatedObject`, `performSelector` — only for interoperability.

### Rule 5.2: @objc and dynamic Only When Necessary

```swift
// FORBIDDEN: entire class @objc without reason
// ALLOWED:
@objcMembers
class PluginBridge: NSObject { ... } // only for bridge to Objective-C
```

### Rule 5.3: Minimize Global State

Avoid `static var`, singletons without protocols. Use dependency injection.

---

## VI. Naming

### Rule 6.1: Full Names in Import

```swift
// BAD
import Foundation
func helper() { ... } // what does it do?

// GOOD
import Foundation
func normalizeEmail(_ email: String) -> String // self-documenting
```

### Rule 6.2: Descriptive Names

```swift
// BAD
let n = calc(cfg, usr)

// GOOD
let totalPrice = calculateTotal(config: config, userProfile: profile)
```

### Rule 6.3: Examples in Doc Comments

Every public API must have an `## Example` section.

---

## VII. Testing

### Rule 7.1: XCTest with Descriptive Names

```swift
import XCTest
@testable import MyModule

final class ValidationTests: XCTestCase {
    func testFilterValidRejectsEmpty() {
        XCTAssertTrue(filterValid([]).isEmpty)
    }

    func testFilterValidPreservesOrder() {
        let input = [validA, invalid, validB]
        XCTAssertEqual(filterValid(input), [validA, validB])
    }
}
```

### Rule 7.2: Property-Based Tests

```swift
// Use swift-check or equivalent
property("reverse is involution") <- forAll { (xs: [Int]) in
    xs.reversed().reversed() == xs
}
```

### Rule 7.3: Error Tests

Every `throws` and `nil` path must be covered.

---

## VIII. Dependencies

### Rule 8.1: Stability > Novelty

Check:
- SemVer ≥ 1.0 for critical dependencies
- Development activity
- Size (LLM loads the API into context)

### Rule 8.2: No "Utility.swift" Files

If a file is named `Utils.swift` — abstractions are not extracted.

---

## IX. Automation

### Rule 9.1: SwiftFormat and SwiftLint Are Mandatory

```yaml
# .swiftlint.yml
disabled_rules:
  - force_try
  - force_cast
opt_in_rules:
  - empty_count
  - explicit_self
```

### Rule 9.2: CI for Every PR

```bash
swift build
swift test
swiftlint
swiftformat --lint
```

---

## X. LLM-Specific Recommendations for Kimi K2.6

### Rule 10.1: File + Doc Comment + Tests > 200 Lines

Kimi loses context when files grow too large. Decompose early.

### Rule 10.2: Avoid "Compressed" Syntax

One-liners with 5 levels of nesting, closures inside closures — Kimi generates these with errors.

**Alternative:** 3 clear lines instead of 1 "elegant" line.

### Rule 10.3: Prefer Idioms Kimi Knows Well

**Generates well:**
- `Sequence` methods (`map`, `filter`, `reduce`, `compactMap`)
- `guard let` / `guard else`
- Exhaustive `switch`
- Standard collections (`Array`, `Dictionary`, `Set`)
- Codable / Decodable

**Generates poorly:**
- Complex generic constructions with `where` clauses
- Advanced Combine/Rx patterns without clear structure
- Runtime type casting chains

---

## Pre-Generation Checklist for Kimi

- [ ] Every public function has a doc comment with examples
- [ ] No `try!`, `as!`, `fatalError` outside tests
- [ ] Every file starts with a `// MARK:` description
- [ ] `swiftlint` and `swiftformat` pass without warnings
- [ ] Functions ≤ 40 lines
- [ ] No `if let` nesting > 2 levels (use `guard`)
- [ ] All error and `nil` paths are covered by tests
- [ ] No global `static var` without a protocol
- [ ] `enum` instead of string constants for discriminated states
