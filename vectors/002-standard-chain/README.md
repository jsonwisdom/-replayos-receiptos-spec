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

1. Retrieve Vector 001 from the canonical `main` branch.
2. Remove Vector 001's top-level `signature` member.
3. RFC 8785-canonicalize the remaining Vector 001 object to UTF-8 bytes.
4. Construct `UTF8("RAA_RECEIPT_V1") || 0x00 || vector_001_payload_bytes`.
5. SHA-256 those signed bytes and confirm the result equals `prior-receipt-content-hash.txt`.
6. Confirm `receipt.json.prior_receipt` contains the Vector 001 ID and content hash.
7. Confirm `sequence == 1`.
8. Confirm `runtime.signing_key.validity_receipt_ref` references Vector 001 by both ID and content hash.
9. Remove Vector 002's top-level `signature` member.
10. RFC 8785-canonicalize the remaining Vector 002 object to UTF-8 bytes.
11. Construct `UTF8("RAA_RECEIPT_V1") || 0x00 || vector_002_payload_bytes`.
12. SHA-256 the signed bytes and compare with `content-hash.txt`.
13. Verify the Ed25519 signature using Vector 001's published conformance public key.

## Key Reuse Notice

This vector reuses the same conformance key derivation as Vector 001. This is intentional for deterministic test reproducibility.

In production RAA deployments:

- Instrument instances should use scope-appropriate signing keys rather than copying the conformance fixture.
- Key rotation must produce a new `KEY_ROTATION` receipt before later receipts use the rotated key.
- Reuse of this published test derivation outside conformance testing is prohibited.
- Test-fixture key reuse does not establish that production key reuse is safe or authorized.

This vector proves record ordering, key-history binding, and cryptographic linkage only. It does not establish trusted time, source correctness, disclosure completeness, authority legitimacy, or real-world truth.
