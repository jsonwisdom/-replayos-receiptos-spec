# Vector 002 — Standard Chain

This synthetic vector proves the first non-genesis RAA chain extension.

## Expected bindings

- Sequence: `1`
- Prior receipt ID: `01928374-0000-7000-8000-000000000000`
- Prior content hash: `sha256:7ec1f57d9053774d7c595da90ab29102a3c8e5f8cd662465844a8d94ef0042f9`
- Vector 002 content hash: `sha256:23f05ae86be1104a88cb304b813df25908ff1d217d5da538d08e78e4ea4f49ec`
- Key-validity receipt: Vector 001 KEY_GENESIS
- Time evidence: `LOCAL_CLOCK` — unverified synthetic claim

## Verification

1. Recompute Vector 001 signed bytes and confirm its content hash equals `prior-receipt-content-hash.txt`.
2. Confirm `receipt.json.prior_receipt` contains the Vector 001 ID and content hash.
3. Confirm `sequence == 1`.
4. Confirm `runtime.signing_key.validity_receipt_ref` references Vector 001 by both ID and content hash.
5. Remove Vector 002's top-level `signature` member.
6. RFC 8785-canonicalize the remaining object to UTF-8 bytes.
7. Construct `UTF8("RAA_RECEIPT_V1") || 0x00 || payload_bytes`.
8. SHA-256 the signed bytes and compare with `content-hash.txt`.
9. Verify the Ed25519 signature using Vector 001's published conformance public key.

This vector proves record ordering and cryptographic linkage only. It does not establish trusted time, source correctness, disclosure completeness, or operational authority.
