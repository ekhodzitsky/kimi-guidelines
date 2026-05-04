# Prompt: HTTP URL Router

Implement a trie-based URL path router in Rust.

Requirements:
- Register routes with path parameters (`/users/:id/posts/:post_id`)
- Match incoming paths and extract parameters into a map
- Support catch-all wildcards (`/static/*path`)
- Reject duplicate route registrations
- Return the matched handler (as a type-erased function pointer or similar)
