# RECEIPTOS-CONTRACT-ARTIFACT-v0.1.1

**Status:** FROZEN ADDITIVE EXTENSION

**Base specification:** RECEIPTOS-CONTRACT-ARTIFACT-v0.1 — FROZEN

**Registry conformance:** RECEIPTOS-REGISTRY-v1.0

```text
registry_version: 1.0
registry_compatibility: compatible
```

## v0.1.1 Relationship Extensions

This extension is additive only. It does not delete, replace, reinterpret, or mutate any frozen v0.1 field or semantic rule.

```text
initiating_record_refs: []
initiating_record_link_status: linked | unlinked | disputed

evidence_record_refs: []
evidence_record_link_status: linked | unlinked | disputed

public_record_refs: []
public_record_link_status: linked | unlinked | disputed

policy_refs: []
policy_link_status: linked | unlinked | disputed
```

## Relationship Rules

- All references are reference-only.
- Reference arrays do not imply verified linkage.
- Link status must be independently evidenced.
- No referenced artifact is mutated to create reciprocal linkage.
- Initiating records may include observed award notices, purchase orders, or task orders.
- Evidence records may include observed records submitted in disputes, amendments, or modification proceedings.

## Invariant

No added relationship implies contract validity, authority, performance, acceptance, payment entitlement, compliance, legality, or liability.
