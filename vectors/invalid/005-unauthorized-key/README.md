# Invalid Vector — 005-unauthorized-key

Expected error: `KEY_VALIDITY_FAILED`

This receipt is structurally valid and has a valid Ed25519 signature under a distinct deterministic test key derived from `RAA_VECTOR_INVALID_005_UNAUTHORIZED_KEY_V1`. That key is not authorized by the bound key-history chain.

A conforming verifier MUST reject it with `KEY_VALIDITY_FAILED`.
