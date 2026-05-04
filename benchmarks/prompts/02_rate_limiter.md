# Prompt: Token Bucket Rate Limiter

Implement a token bucket rate limiter in Rust.

Requirements:
- Thread-safe, async-compatible
- Configurable refill rate and burst capacity
- Support per-key limiting (e.g., per user IP)
- Return `Duration` until next request allowed when limited
- No external state — in-memory only
- Benchmark-friendly design
