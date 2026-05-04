# Prompt: Priority Job Queue

Build an in-memory priority job queue in Rust.

Requirements:
- Jobs have a priority (u8), a payload (Vec<u8>), and a unique ID
- Workers pull highest-priority jobs first; FIFO within same priority
- Support job retry with exponential backoff
- Graceful shutdown: finish in-flight jobs, reject new ones
- Metrics: enqueued, processed, failed counts
