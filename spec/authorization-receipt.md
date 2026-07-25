# AuthorizationReceipt (M7)

**Status:** Normative draft (scaffolding for Issue #2)

## Purpose

An `AuthorizationReceipt` is the minimal, immutable observation that a decision was authorized under the correct conditions. It is receipt-like, not decision-like. It does not perform authorization; it records that authorization occurred and provides the cryptographic and structural hooks required for the Lock Evidence Graph.

## Schema

See `schemas/authorization-receipt.schema.json`.

### Required Fields

| Field | Type | Notes |
|-------|------|-------|
| `receipt_id` | UUIDv4 | Globally unique |
| `issued_at` | RFC3339 date-time | Issuance timestamp |
| `issuer` | string | Authorizing agent identity |
| `subject_decision_id` | string | Exactly one decision |
| `authorization_context_hash` | SHA-256 hex | Hash of justifying context |
| `authorization_type` | enum | `InitialLock` \| `ReLock` \| `Override` \| `Emergency` |
| `supporting_evidence_hashes` | array of SHA-256 hex | May be empty |
| `signature` | string | Detached, opaque |

## Invariants (subset of M7)

- **M7-001**: No Lock without a corresponding AuthorizationReceipt.
- **M7-002**: AuthorizationReceipt MUST reference exactly one `subject_decision_id`.
- **M7-005**: Authorization context MUST be hash-stable and replay-safe.
- **M7-007**: Signature MUST be immutable after issuance.

## Boundary

- Does **not** define cryptographic algorithms.
- Does **not** embed the decision itself.
- Does **not** grant authority; it only records evidence of authorization.

## Related

- `spec/lock-evidence-graph.md`
- `CONFORMANCE.md`
- Issue #2
