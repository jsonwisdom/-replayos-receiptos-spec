# Vector 001 — KEY_GENESIS

This is a deterministic **synthetic test vector**, not a live operational receipt.

- Profile: `RAA-URP-1`
- Domain: `RAA_RECEIPT_V1`
- Context hash: `sha256:20d85fb6608c99db24c778f8eeeac7d42b7bf6604eb6e97bddb17f1fe897036c`
- Content hash: `sha256:a1c6d2b1388ffa439af48f30a60db753793d602e7cf4fe455b4c1dc259f0d10e`
- Key ID: `did:key:z6MkehRgf7yJbgaGfYsdoAsKdBPE3dj2CYhowQdcjqSJgvVd`
- Time evidence: `LOCAL_CLOCK` (unverified claim)
- Private seed: published only for deterministic conformance testing; never use it in production.

## Verification

1. Remove the top-level `signature` member from `receipt.json`.
2. RFC 8785-canonicalize the remaining object to UTF-8 bytes.
3. Prefix `UTF8("RAA_RECEIPT_V1") || 0x00`.
4. SHA-256 the resulting bytes and compare with `content-hash.txt`.
5. Verify the Ed25519 signature using `public-key.raw.base64.txt`.
6. Hash canonical `context/raa-urp-1.json` and compare with the receipt's context hash.

The illustrative anchor digest is synthetic and has no retrieval location or external inclusion proof.
