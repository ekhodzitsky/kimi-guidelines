# Prompt: Argon2 Password Hasher

Implement a password hashing utility in Rust using the Argon2 algorithm.

Requirements:
- Hash passwords with random salt
- Verify passwords against stored hash
- Configurable memory, iterations, and parallelism parameters
- Constant-time comparison to prevent timing attacks
- Zero sensitive data from memory after use
