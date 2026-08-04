# Universal Receipt Protocol v1.0 Candidate

This document defines the normative verification profile for RAA Universal Receipts.

## Signed bytes

```text
domain_bytes  = UTF8("RAA_RECEIPT_V1")
payload_bytes = RFC8785_JCS_UTF8(receipt_without_signature)
signed_bytes  = domain_bytes || 0x00 || payload_bytes
content_hash  = SHA256(signed_bytes)
signature     = Ed25519_SIGN(private_key, signed_bytes)
```

## Verification order

A conforming verifier MUST evaluate receipts in this order and return the first applicable error:

1. verification-context binding
2. domain-separated signature and content binding
3. signing-key validity
4. prior-receipt binding
5. sequence uniqueness and continuity
6. terminal-state closure

## Normative error vocabulary

| Code | Meaning |
|---|---|
| `CONTEXT_BINDING_FAILED` | The bound verification context does not match `RAA-URP-1`. |
| `DOMAIN_SEPARATOR_MISMATCH` | Verification fails under mandatory domain `RAA_RECEIPT_V1` but succeeds only under another domain. |
| `CONTENT_HASH_MISMATCH` | Supplied payload bytes do not match the bound content hash or signature. |
| `KEY_VALIDITY_FAILED` | The signing key is not authorized by the referenced immutable key-history receipt. |
| `PRIOR_BINDING_FAILED` | `prior_receipt` does not bind the exact canonical predecessor ID and content hash. |
| `SEQUENCE_GAP` | Sequence is greater than predecessor sequence plus one. |
| `SEQUENCE_DUPLICATE` | Sequence is already occupied in the lifecycle chain. |
| `POST_TERMINAL_MUTATION` | A receipt attempts to extend a predecessor with `terminal=true`. |

## Truth boundary

Successful verification establishes record integrity, provenance, ordering, and declared evidentiary state only. It does not establish claim truth, source correctness, authority legitimacy, disclosure completeness, or trusted time unless separately evidenced.
