# RECEIPTOS-UNDERLYING-DECISION-ARTIFACT-v0.1.1

**Status:** FROZEN ADDITIVE EXTENSION

**Base specification:** RECEIPTOS-UNDERLYING-DECISION-ARTIFACT-v0.1 — FROZEN

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
- These fields resolve the previously abstract InitiatingRecord and EvidenceRecord relationship roles through concrete frozen artifacts while preserving the frozen v0.1 artifact.

## Procedural Relationship

```text
InitiatingRecord → may_reference UnderlyingDecision
UnderlyingDecision → may_reference Appeal
```

The relationships are independently evidenced and do not imply authority, jurisdiction, validity, finality, enforceability, implementation, or appealability.

## Invariant

No added relationship establishes authority, jurisdiction, validity, correctness, legal sufficiency, service, entitlement, finality, enforceability, or implementation.
