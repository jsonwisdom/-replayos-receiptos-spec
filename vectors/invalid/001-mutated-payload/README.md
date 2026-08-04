# Invalid Vector — 001-mutated-payload

Expected error: `CONTENT_HASH_MISMATCH`

This fixture preserves Vector 002's original signature while mutating the signed payload metadata to `MUTATED_AFTER_SIGNATURE`.

A conforming verifier MUST reject it with `CONTENT_HASH_MISMATCH` because the published signature and content hash no longer bind the supplied payload bytes.
