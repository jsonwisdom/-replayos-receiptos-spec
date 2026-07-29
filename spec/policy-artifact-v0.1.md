# RECEIPTOS-POLICY-ARTIFACT-v0.1 — FROZEN

## Identity

`ros:policy:<jurisdiction_id>:<source_hash_sha256>`

## Purpose

Observed authoritative written instrument, including statute, regulation, directive, manual, or guidance.

## Required Fields

- `id`
- `source`
- `dateObserved`
- `effectiveDate`
- `policyType`
- `citation`
- `title`
- `version`
- `contentHash`

## Relationships

- `implements` → Policy
- `amends` → Policy
- `interpretedBy` → UnderlyingDecision / Appeals / AuditFinding
- `governs` → Procurement / Contract / Budget

## Constraints

- `citation` is required.
- `title` is required.

## Registry Conformance

`RECEIPTOS-REGISTRY-v1.0`: `compatible`
