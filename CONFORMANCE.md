# CONFORMANCE.md

**Status:** M7 scaffolding (Issue #2)

This document promotes and freezes the architectural invariants that all implementations and validators MUST enforce.

## Architectural Invariants (promoted)

1. **Semantic Enum Scoping**  
   All enumerations that affect decision or authorization semantics are closed and versioned. Unknown values are rejected.

2. **Artifact Independence**  
   AuthorizationReceipt, DecisionReceipt, EvidenceReceipt, and LockReceipt are independently addressable artifacts. No embedding of one inside another that would create dual sources of truth.

3. **Append-Only Evidence Graph**  
   The Lock Evidence Graph is append-only. Mutations are expressed only via new nodes/edges (including tombstones).

4. **Artifact Provenance**  
   Every artifact carries cryptographic provenance (hash + signature) sufficient for replay and independent verification.

## M7 Invariants

- **M7-001**: No Lock Without AuthorizationReceipt  
- **M7-002**: AuthorizationReceipt must reference exactly one subject_decision_id  
- **M7-003**: LockReceipt must include a complete supporting evidence set  
- **M7-004**: Lock Evidence Graph must remain acyclic  
- **M7-005**: Authorization context must be hash-stable and replay-safe  
- **M7-006**: Overrides must preserve lineage; no deletion without tombstone  
- **M7-007**: AuthorizationReceipt signature must be immutable and verified  
- **M7-008**: LockReceipt must be append-only; no mutation after issuance  

## Conformance Tests (planned)

- Artifact reference validation
- DAG integrity / acyclicity
- Authorization references decisions only
- Decision hash remains unchanged when authorization changes

See `fixtures/m7/` for positive and negative controls.
