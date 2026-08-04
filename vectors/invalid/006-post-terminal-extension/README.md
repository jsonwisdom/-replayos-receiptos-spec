# Invalid Vector — 006-post-terminal-extension

Expected error: `POST_TERMINAL_MUTATION`

This schema-valid, correctly signed receipt references Vector 003 and increments to sequence 3. Vector 003 is `terminal=true`, so no receipt may extend it.

A conforming verifier MUST reject it with `POST_TERMINAL_MUTATION`.
