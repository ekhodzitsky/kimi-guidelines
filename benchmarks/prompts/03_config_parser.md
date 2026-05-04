# Prompt: TOML-like Config Parser

Write a minimal configuration file parser in Rust.

Requirements:
- Parse a TOML-like format: sections `[section]`, key = "value", numbers, booleans, arrays
- Return structured errors with line/column info
- Support nested sections via dot notation
- Deserialize into user-defined structs via a derive macro or builder
- Reject duplicate keys in the same section
