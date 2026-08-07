# Clinical Resolution Oracle v0.2

**Role:** `ClinicalResolutionOracle`  
**Registry compatibility:** `compatible`  
**Status:** versioned extension candidate; does not mutate frozen RC1  
**Standing oracle:** **What is the last condition state we can actually prove?**

## Purpose

Clinical Resolution Oracle is a deterministic, receipt-bounded condition-transition test shared by VA RePlay, Mayo RePlay, County RePlay, Patient RePlay, and any other institution using the same versioned criteria.

It separates three questions that must never be collapsed:

1. **What evidence exists?** — ReceiptOS.
2. **What happened?** — RePlay.
3. **Did the clinical condition actually move?** — Clinical Resolution Oracle.

Administrative activity is evidence of activity only. It cannot manufacture a clinical resolution state.

## Deterministic transition function

```text
ClinicalResolution = F(
  ConditionStateBefore,
  ConditionStateAfter,
  EvidenceItems,
  IndependenceGroups,
  ResolutionCriteriaVersion
)

-> delta_R
-> clinical_resolution
-> last_proven_condition_state
```

### `delta_R`

- `POSITIVE` — verified movement toward condition-specific resolution criteria.
- `ZERO` — no verified movement.
- `NEGATIVE` — verified movement away from resolution / worsening.
- `UNPROVEN` — critical evidence is missing.
- `CONFLICTING` — independent admissible evidence materially disagrees.

### `clinical_resolution`

- `RESOLVED`
- `UNRESOLVED`
- `UNPROVEN`
- `CONFLICTING`

`delta_R = POSITIVE` **does not imply** `clinical_resolution = RESOLVED`.

## 12 x 12 evidence surface

Rows are the twelve locked invariants. Columns are twelve evaluation dimensions:

| Column | Dimension |
|---|---|
| C1 | Receipt Presence |
| C2 | Provenance Integrity |
| C3 | Source Independence |
| C4 | Corroboration |
| C5 | Temporal Ordering |
| C6 | `R_before` Integrity |
| C7 | `R_after` Integrity |
| C8 | Condition-Transition Evidence |
| C9 | Administrative Separation |
| C10 | Conflict Handling |
| C11 | Source-Failure Resilience |
| C12 | Deterministic Replay |

The Cartesian product is exactly **144 rubric cells**.

Each cell has exactly one state:

```text
PASS | FAIL | UNPROVEN | CONFLICTING
```

## Locked invariants

1. `NO RECEIPT -> UNPROVEN`
2. `CONFLICTING RECEIPTS -> CONFLICTING`
3. `AI MAY IDENTIFY GAPS ONLY`
4. `AUTHORITY MUST BE JURISDICTION + VERSION BOUNDED`
5. `SAME INPUTS + SAME VERSION -> SAME AUDIT RESULT`
6. `ADMINISTRATIVE COMPLETION != CLINICAL RESOLUTION`
7. `REDUNDANT COPIES != INDEPENDENT CORROBORATION`
8. `SOURCE FAILURE MUST NOT CREATE A FALSE STATE`
9. `LAST VERIFIED CLINICAL STATE MUST SURVIVE INSTITUTIONAL FAILURE`
10. `EVIDENCE PROVENANCE MUST SURVIVE TRANSFER`
11. `DISAGREEMENT BETWEEN INDEPENDENT SOURCES -> CONFLICTING`
12. `MISSING CRITICAL EVIDENCE -> UNPROVEN, NEVER ASSUMED`

## Resilience invariant

Held exactly:

```text
Failure(Sourceᵢ) ⇏ Failure(Audit)
```

Source loss cannot create a false clinical state, silently erase the last verified state, or terminate replay unless the missing source is genuinely required by the transition test. Missing required evidence produces `UNPROVEN`, not invention.

## D-Wave / optimization boundary

The 12 x 12 matrix may be encoded as four one-hot binary states per cell:

```text
x[i,j,s] ∈ {0,1}
Σ_s x[i,j,s] = 1
```

With 144 cells and four states, the base logical representation contains:

```text
144 * 4 = 576 binary decision variables
```

before any solver-introduced auxiliary variables or embedding overhead.

A CQM-compatible optimizer may search only for a constraint-consistent candidate:

```text
X* = argmin_X Q(X)
```

subject to the hard constraints defined by this protocol.

The optimizer has **no clinical authority**. It returns `X*` only. All mapping from an accepted candidate to `delta_R`, `clinical_resolution`, and `last_proven_condition_state` is classical, deterministic, criteria-version-bounded, and replayable.

### Hard boundary

```text
Optimization != Clinical Authority
```

A quantum, hybrid, classical, AI, or institutional system cannot override the oracle invariants by producing a lower objective value.

## Multi-stakeholder comparison

Multiple evidence producers may populate independent surfaces:

```text
T[stakeholder, invariant, dimension, state]
```

Pairwise comparison may identify disagreement loci. It does not decide which stakeholder is correct. Resolution requires admissible evidence under the same oracle version.

## Forbidden substitutions

The following may prove activity but cannot independently prove clinical resolution:

- ticket closure
- appointment completion
- prescription issuance or fill
- referral issuance
- denial or approval letter
- discharge note
- claim grant or denial
- duplicated copies of one originating source

## Determinism

```text
Same inputs + same oracle version -> same audit result
```

AI authority is limited to gap detection. Narrative authority is none.
