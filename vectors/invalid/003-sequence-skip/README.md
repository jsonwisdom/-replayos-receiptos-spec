# Invalid Vector — 003-sequence-skip

Expected error: `SEQUENCE_GAP`

This schema-valid, correctly signed receipt advances from canonical sequence 1 to sequence 3. Sequence 2 is absent.

A conforming verifier MUST reject it with `SEQUENCE_GAP`.
