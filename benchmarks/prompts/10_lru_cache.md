# Prompt: LRU Cache

Implement a Least-Recently-Used (LRU) cache in Rust.

Requirements:
- Fixed capacity; evict least-recently-used on insert when full
- O(1) get and insert
- Thread-safe for concurrent access
- Optional TTL per entry
- Iterate entries in LRU order without consuming the cache
