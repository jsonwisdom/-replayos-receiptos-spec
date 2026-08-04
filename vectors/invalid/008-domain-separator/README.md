# Invalid Vector — 008-domain-separator

Expected error: `DOMAIN_SEPARATOR_MISMATCH`

The receipt fields are otherwise valid, but its signature was produced over `RAA_RECEIPT_V2 || 0x00 || JCS(payload)` instead of the mandatory `RAA_RECEIPT_V1` domain.

A conforming verifier MUST reject it with `DOMAIN_SEPARATOR_MISMATCH`.
