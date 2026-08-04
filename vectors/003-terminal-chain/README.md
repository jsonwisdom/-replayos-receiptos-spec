# Vector 003 — Terminal Chain

This deterministic synthetic vector closes the canonical conformance lifecycle chain.

## Expected bindings

- Sequence: `2`
- Prior receipt ID: `01928374-0001-7000-8000-000000000002`
- Prior content hash: `sha256:23f05ae86be1104a88cb304b813df25908ff1d217d5da538d08e78e4ea4f49ec`
- Terminal content hash: `sha256:cd137e96ae5fc1f1816fe15a05623a45552c96ca940f2af5cba7f0f7dfcc7c60`
- `terminal`: `true`
- Instrument: `008` (`archive`)
- Time evidence: `LOCAL_CLOCK` — unverified synthetic claim

## Verification

1. Recompute Vector 002 signed bytes as `UTF8("RAA_RECEIPT_V1") || 0x00 || JCS_UTF8(receipt_without_signature)`.
2. Confirm the result hashes to `prior-receipt-content-hash.txt`.
3. Confirm the terminal receipt binds Vector 002 by ID and content hash.
4. Confirm `sequence == 2`.
5. Confirm the signing-key validity reference still binds Vector 001 KEY_GENESIS.
6. Recompute and verify this receipt's content hash and Ed25519 signature.
7. Confirm `terminal == true`.
8. Reject every later receipt that references this terminal receipt as `POST_TERMINAL_MUTATION`.

This fixture proves terminal closure of this lifecycle chain. Corrections or disputes must begin separate chains referencing the closed history; they do not extend this chain.
