# RECEIPTOS-PUBLIC-RECORD-ARTIFACT-v0.1 — FROZEN

## Identity

`ros:public_record:<jurisdiction_id>:<source_hash_sha256>:<record_selector_hash>`

## Purpose

Observed public-records production under FOIA or state-equivalent law.

## Record Types

- Production
- Index
- WithholdingLog
- Supplemental
- Denial

## Required Fields

- `id`
- `source`
- `dateObserved`
- `dateProduced`
- `recordType`
- `trackingNumber`
- `contentHash`

## Relationships

`pertainsTo` may reference:

- UnderlyingDecision
- Appeals
- Procurement
- Contract

## Constraints

- At least one `pertainsTo` relationship is required.
- A `Denial` record requires `exemptionsCited`.

## Registry Conformance

`RECEIPTOS-REGISTRY-v1.0`: `compatible`
