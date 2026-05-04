# Prompt: Typed Event Bus

Build a type-safe event bus in Rust.

Requirements:
- Publishers emit typed events; subscribers receive only subscribed types
- Async delivery with backpressure handling
- Allow multiple subscribers per event type
- Subscribers can be dynamically added/removed
- No global state — bus is an explicit object
