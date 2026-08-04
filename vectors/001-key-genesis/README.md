# Vector 001 — KEY_GENESIS

This is a deterministic **synthetic test vector**, not a live operational receipt.

- Profile: `RAA-URP-1`
- Domain: `RAA_RECEIPT_V1`
- Context hash: `sha256:20d85fb6608c99db24c778f8eeeac7d42b7bf6604eb6e97bddb17f1fe897036c`
- Content hash: `sha256:7ec1f57d9053774d7c595da90ab29102a3c8e5f8cd662465844a8d94ef0042f9`
- Key ID: `did:key:z6MkhdxRrX8GXyW2x9ArDxay7ioxmWr95Pfzr4Dmm7GJHuF1`
- Time evidence: `LOCAL_CLOCK` (unverified claim)
- Private seed: published only for deterministic conformance testing; never use it in production.

## Verification

1. Read `conformance-seed.txt` and decode `SEED_VALUE` as exactly 32 bytes.
2. Derive the Ed25519 keypair and confirm the raw public key and DID match the published fixtures.
3. Remove the top-level `signature` member from `receipt.json`.
4. RFC 8785-canonicalize the remaining object to UTF-8 bytes.
5. Prefix `UTF8("RAA_RECEIPT_V1") || 0x00`.
6. SHA-256 the resulting bytes and compare with `content-hash.txt`.
7. Sign the same bytes and confirm the signature matches `receipt.json`.
8. Hash canonical `context/raa-urp-1.json` and compare with the receipt's context hash.

The illustrative anchor digest is synthetic and has no retrieval location or external inclusion proof.
