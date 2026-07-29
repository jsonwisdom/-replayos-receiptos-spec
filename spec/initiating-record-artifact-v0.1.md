# RECEIPTOS-INITIATING-RECORD-ARTIFACT-v0.1 — FROZEN

## Identity

`ros:initiating_record:<jurisdiction_id>:<source_hash_sha256>:<record_selector_hash>`

## Purpose

Observed record that formally commences a process, including an award notice, protest filing, appeal filing, audit request, purchase order, task order, or claim.

## Required Fields

- `id`
- `source`
- `dateObserved`
- `dateFiled`
- `recordType`
- `receivingEntity`
- `docketNumber`
- `contentHash`
- `description`

## Relationships

`initiates` must reference exactly one of:

- Procurement
- Contract
- Appeals
- AuditDocument
- UnderlyingDecision

## Constraints

Exactly one `initiates` target is required.

## Registry Conformance

`RECEIPTOS-REGISTRY-v1.0`: `compatible`
