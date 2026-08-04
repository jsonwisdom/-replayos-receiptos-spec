# Invalid Vector — 004-sequence-repeat

Expected error: `SEQUENCE_DUPLICATE`

This schema-valid, correctly signed receipt reuses sequence 1, which is already occupied by canonical Vector 002.

A conforming verifier MUST reject it with `SEQUENCE_DUPLICATE`.
