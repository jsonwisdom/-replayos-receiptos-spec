# RECEIPTOS-EVIDENCE-RECORD-ARTIFACT-v0.1 — FROZEN

## Identity

`ros:evidence_record:<jurisdiction_id>:<source_hash_sha256>:<record_selector_hash>`

## Purpose

Observed documentary evidence submitted in a proceeding, including an exhibit, declaration, expert report, data extract, correspondence, or physical-item description.

## Required Fields

- `id`
- `source`
- `dateObserved`
- `dateSubmitted`
- `evidenceType`
- `exhibitNumber`
- `contentHash`
- `description`
- `pageCount`
- `admissibilityStatus`

## Relationships

- `submittedIn` → Appeals / AuditFinding / UnderlyingDecision
- `supporting` → Policy / PublicRecord
- `contradictedBy` → EvidenceRecord, source-explicit only

## Constraints

At least one `submittedIn` or `supporting` relationship is required.

## Registry Conformance

`RECEIPTOS-REGISTRY-v1.0`: `compatible`
