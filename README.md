# ReplayOS / ReceiptOS Spec

**Status:** v1.0.0-rc1  
**Architecture:** FROZEN  
**Promotion:** BLOCKED  
**Implementation authority:** NONE  
**Production admissibility:** FALSE

```text
ReplayOS / ReceiptOS Spec
defines what evidence must exist,
how it must be separated,
and how a promotion decision must be computed.
```

This repository is the **constitutional specification layer**.  
It is **not** the replay engine, **not** the Base implementation, and **not** a production deployment repository.

---

## Decision Model

```text
Decision = F(
    Specification,
    ROR,
    ER,
    ObservationBoundary
)
```

## Receipt Sequence

```text
ROR → ER → GER → PR
```

| Receipt | Full name | Meaning |
|---------|-----------|---------|
| **ROR** | Repository Observation Report | What files, commits, hashes, tags, and evidence were actually observed. |
| **ER**  | Execution Report | What was run, in what environment, and what happened. |
| **GER** | Gate Evaluation Report | Which rules passed, failed, or remained unresolved. |
| **PR**  | Promotion Receipt | The final bounded decision: PROMOTE, DENY, or remain BLOCKED. |

### Clean mental model

```text
OBSERVE
   ↓
EXECUTE
   ↓
EVALUATE
   ↓
DECIDE
```

The layers cannot be collapsed:

- An execution result cannot pretend it observed repository evidence.
- An observation cannot pretend code was executed.
- A gate result cannot promote itself.
- A promotion receipt cannot invent missing evidence.

---

## Where it sits in the broader system

```text
AL
governance doctrine
rules and constitutional limits
        │
        ▼
ReplayOS / ReceiptOS Spec
evidence model
receipt schemas
decision function
promotion gates
        │
        ▼
receiptos-base
canonicalization
hashing
verification
deterministic replay
        │
        ▼
JOY / EAS / Frames / external witnesses
continuity and presentation
```

| Repository | Role |
|------------|------|
| **jsonwisdom/-replayos-receiptos-spec** | Defines the constitutional evidence and decision protocol |
| **receiptos-base** | Implements receipt, hashing, verification, and replay primitives |
| **JOY / EAS / Frames** | Witnesses or presents results without creating authority |

---

## The load-bearing boundary

ReplayOS does **not** answer:

```text
“Is the claim ultimately true?”
```

It answers the narrower, safer question:

```text
“Given this specification, observation boundary,
repository evidence, execution evidence, and rules,
is promotion authorized?”
```

---

## Why promotion remains BLOCKED (Gate G-001)

Promotion is blocked until **all six** of the following are observed in this repository:

1. Canonical JCS vectors  
2. Validation PASS report  
3. Conformance PASS report  
4. Manifest SHA-256 verification record  
5. Git tag `v1.0.0`  
6. A release carrying the matching hash  

Therefore:

```text
architecture frozen ≠ production approved
schemas present ≠ conformance proven
tests described ≠ tests executed
release candidate ≠ released standard
```

See open issue [#1](https://github.com/jsonwisdom/-replayos-receiptos-spec/issues/1).

---

## Invariants

1. **Monotonicity** — new evidence may extend the record; history is not silently rewritten.  
2. **Referential transparency** — the same bounded inputs produce the same decision.  
3. **Boundary closure** — the evaluator cannot rely on hidden or undeclared evidence.  
4. **Observation / execution separation** — seeing a file and running a file are different claims.  
5. **Zero side-effects** — evaluating promotion must not mutate the system being evaluated.  

---

## What this repository is **not**

- not a Phase 2 artifact store  
- not a production replay service  
- not a blockchain witness rail  
- not an automatic truth machine  
- not an authority generator  
- not a substitute for `receiptos-base`  

---

## Current coherent posture

```text
repository:
  jsonwisdom/-replayos-receiptos-spec

role:
  NORMATIVE_SPECIFICATION_AND_PROMOTION_GATE

architecture:
  FROZEN_RC1

implementation_authority:
  NONE

production_admissibility:
  FALSE

promotion:
  BLOCKED

canonical_state_mutated:
  FALSE
```

---

## Normative artifacts (declared for RC1)

The authoritative object list lives in [`manifests/normative-set.manifest.json`](manifests/normative-set.manifest.json).

**Important honesty note:** several paths declared in the manifest and earlier README (core ROR / ER / GER / PR schemas and `functions/decision-function.md`) are **not yet present** in the tree. Their absence is itself part of the evidence record and is one reason G-001 remains blocked. See also issue [#9](https://github.com/jsonwisdom/-replayos-receiptos-spec/issues/9).

Existing material that *is* present includes:

- `schemas/authorization-receipt.schema.json` (M7)
- Multiple frozen artifact specifications under `spec/`
- Conformance scaffolding in `CONFORMANCE.md` and `fixtures/m7/`
- Vectors under `vectors/`
- Doctrine illustrations under `docs/replay-court/`

---

## Bottom line

```text
ReplayOS decides whether the evidence permits promotion.
ReceiptOS records how that decision was reached.
receiptos-base supplies the implementation primitives.
```

Neither this repository nor `receiptos-base` may declare itself authoritative, and neither may turn missing evidence into GREEN.
