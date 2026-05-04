# Prompt: JSON Serializer/Deserializer

Implement a minimal JSON serializer and deserializer in Rust.

Requirements:
- Parse JSON strings into a tree of `Value` enum (Null, Bool, Number, String, Array, Object)
- Serialize `Value` back to compact and pretty-printed JSON
- Handle escaped characters in strings correctly
- Return precise parse errors with position
- Stream large arrays without holding everything in memory
