# Prompt: Streaming CSV Parser

Write a streaming CSV parser in Rust.

Requirements:
- Parse arbitrarily large files without loading into memory
- Handle quoted fields, embedded newlines, and escaped quotes
- Configurable delimiter and comment prefix
- Return rows as iterators; yield structured errors for malformed rows
- Optional: deserialize rows into structs via derive or builder
