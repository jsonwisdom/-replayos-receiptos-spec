# Encounter Resolution Ledger

Status: DRAFT v0.1

## Thesis

Institutional activity is not evidence of resolution.

For every encounter, ReplayOS records the incoming state, the request, the institution's action, resources consumed, the outgoing state, and the receipt that proves whether the underlying problem actually changed.

If `STATE_IN` contains the problem and `STATE_OUT` contains the same unresolved problem, the encounter is activity, not success.

## Core transition

```text
STATE_IN
+ REQUEST
+ AGENCY
+ ACTION
+ MONEY_TIME_CONSUMED
+ REFERRAL_DENIAL_CANCELLATION
+ STATE_OUT
+ RESOLUTION_RECEIPT
= ENCOUNTER_RECEIPT
```

## Evidence rule

Missing evidence MUST NOT be replaced with narrative.

Allowed evidence states:

- `PROVEN`
- `UNPROVEN`
- `NO_RECEIPT`
- `PARTIAL_RECORD`
- `RECONSTRUCTION_REQUIRED`

`UNKNOWN` is valid. Fabricated continuity is not.

## Resolution states

- `RESOLVED`
- `PARTIALLY_RESOLVED`
- `UNRESOLVED`
- `UNKNOWN`

A resolution claim requires a receipt showing an objective state transition tied to the original request.

## Encounter schema

```text
encounter_id
subject_id
opened_at
closed_at
state_in
request
agency
actor
agency_action
money_consumed
patient_or_citizen_time_consumed
employee_time_consumed
referral
 denial
cancellation
handoff
state_out
resolution_state
resolution_receipt_ref
evidence_state
source_refs
```

## Derived audit metrics

```text
ENCOUNTERS_CONSUMED
RESOLUTIONS_PRODUCED
REPEAT_VISITS_CAUSED_BY_UNRESOLVED_STATE
DAYS_UNRESOLVED
AGENCY_HANDOFFS
DENIALS
CANCELLATIONS
DEAD_END_REFERRALS
TIME_TO_FIRST_MEANINGFUL_TRANSITION
COST_PER_RESOLUTION
ENCOUNTERS_PER_RESOLUTION
```

## Replay windows

The same ledger can be replayed over arbitrary windows:

```text
TODAY
YESTERDAY
LAST_WEEK
LAST_MONTH
LAST_QUARTER
LAST_YEAR
LIFETIME
```

Each window is computed from encounter receipts. A window with insufficient receipts is marked `RECONSTRUCTION_REQUIRED` rather than inferred.

## Audit question

> After consuming another encounter, dollar, form, appointment, appeal, employee-hour, or citizen-hour, what objectively changed?

## Generality

This model is domain-independent. Example adapters may include:

- healthcare and dental care
- veterans benefits
- disability claims
- housing assistance
- courts and administrative appeals
- insurance
- public benefits
- licensing and permitting
- customer support
- education services

The invariant is the same: **contacts, appointments, filings, referrals, and employee activity are inputs. Resolution is an independently testable output.**

## Fail-closed rules

1. No receipt -> no claimed transition.
2. No objective change -> no resolution credit.
3. A referral is not resolution unless the requested state was specifically to obtain that referral.
4. A denial is an outcome but not resolution of the underlying problem unless the request itself was only for a decision.
5. Repeated encounters caused by an unchanged problem are linked to the unresolved predecessor state.
6. Institution-defined completion does not override subject-state evidence.
7. Contradictory receipts remain visible; they are not silently collapsed.

## Minimal scoring model

```text
resolution_rate = RESOLUTIONS_PRODUCED / ENCOUNTERS_CONSUMED
repeat_burden = REPEAT_VISITS_CAUSED_BY_UNRESOLVED_STATE / ENCOUNTERS_CONSUMED
handoff_burden = AGENCY_HANDOFFS / ENCOUNTERS_CONSUMED
```

Scores are descriptive, never substitutes for the underlying receipts.

## Design objective

Turn longitudinal institutional experience into an auditable state machine:

```text
receipt -> transition -> state -> replay -> metric -> proof
```

The system should make it possible to distinguish **service delivered** from **service consumed without resolution** across days, years, agencies, and jurisdictions.
