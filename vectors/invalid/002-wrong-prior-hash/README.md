# Invalid Vector — 002-wrong-prior-hash

Expected error: `PRIOR_BINDING_FAILED`

This synthetic receipt is schema-valid and correctly signed, but its `prior_receipt.content_hash` does not equal Vector 002 signed bytes.

A conforming verifier MUST reject it with `PRIOR_BINDING_FAILED` before chain admission.

Receipt validity does not establish claim truth, authority, trusted time, or disclosure completeness.
