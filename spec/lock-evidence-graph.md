# Lock Evidence Graph (LEG) Contract (M7)

**Status:** Normative draft (scaffolding for Issue #2)

## Purpose

The Lock Evidence Graph (LEG) is a directed acyclic graph (DAG) that proves:

1. A decision was locked.
2. The lock was authorized.
3. The lock was supported by evidence.
4. The lock was consistent with lineage invariants.

## Node Types

- `AuthorizationReceipt`
- `EvidenceReceipt`
- `DecisionReceipt`
- `LockReceipt` (introduced in M7)

## Edge Types

- `supports`
- `authorizes`
- `locks`
- `derives_from`
- `supersedes` (only for lock overrides)

## Required Edges (per node type)

| Node | Required outgoing edges |
|------|-------------------------|
| AuthorizationReceipt | `authorizes` → DecisionReceipt |
| LockReceipt | `locks` → DecisionReceipt, `supported_by` → AuthorizationReceipt |
| EvidenceReceipt | `supports` → AuthorizationReceipt or LockReceipt |

## Forbidden Edges

- Cycles of any kind.
- `supersedes` that deletes without a tombstone.
- AuthorizationReceipt referencing more than one DecisionReceipt.

## Invariants

- **M7-003**: LockReceipt MUST include a complete supporting evidence set.
- **M7-004**: LEG MUST remain acyclic.
- **M7-006**: Overrides MUST preserve lineage; no deletion without tombstone.
- **M7-008**: LockReceipt is append-only; no mutation after issuance.

## Related

- `schemas/authorization-receipt.schema.json`
- `CONFORMANCE.md`
- Issue #2
