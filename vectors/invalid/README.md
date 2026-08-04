# RAA Invalid Vector Suite

These eight synthetic fixtures exercise protocol-layer rejection against the canonical chain.

| ID | Expected error |
|---|---|
| 001-mutated-payload | `CONTENT_HASH_MISMATCH` |
| 002-wrong-prior-hash | `PRIOR_BINDING_FAILED` |
| 003-sequence-skip | `SEQUENCE_GAP` |
| 004-sequence-repeat | `SEQUENCE_DUPLICATE` |
| 005-unauthorized-key | `KEY_VALIDITY_FAILED` |
| 006-post-terminal-extension | `POST_TERMINAL_MUTATION` |
| 007-wrong-context | `CONTEXT_BINDING_FAILED` |
| 008-domain-separator | `DOMAIN_SEPARATOR_MISMATCH` |

Each case contains a corrupted `receipt.json`, an exact `expected-error.txt`, and a README identifying the violated invariant. These fixtures are intended to pass structural JSON validation and fail protocol verification.

The suite tests record integrity only. It does not establish trusted time, authority legitimacy, source correctness, disclosure completeness, or real-world truth.
