# Clinical Resolution CQM v0.2

**Role:** constraint-consistency optimizer contract  
**Authority:** none over clinical outcome  
**Target compatibility:** D-Wave CQM / equivalent constrained optimizer

## Variable model

For invariant row `i ∈ {1..12}`, evaluation dimension `j ∈ {1..12}`, and cell state

```text
s ∈ {PASS, FAIL, UNPROVEN, CONFLICTING}
```

define:

```text
x[i,j,s] ∈ {0,1}
```

There are:

```text
12 * 12 = 144 logical rubric cells
144 * 4 = 576 base binary variables
```

before any auxiliary variables introduced by reformulation, presolve, hybrid decomposition, or hardware embedding.

## One-hot hard constraint

For every rubric cell:

```text
Σ_s x[i,j,s] = 1
```

No cell may be assigned two states and no cell may be left stateless.

## Hard clinical-consistency constraints

The optimizer must treat the following as feasibility constraints, not soft outcome preferences:

1. **No receipt → UNPROVEN** when critical transition evidence is absent.
2. **Independent material conflict → CONFLICTING** until reconciled.
3. **Administrative completion != clinical resolution.**
4. **Duplicate copies != independent corroboration.**
5. **Source failure must not create a false state.**
6. **Last verified clinical state survives institutional/source failure.**
7. **Evidence provenance survives transfer.**
8. **Same admissible inputs + same version → same classical audit result.**
9. **Optimizer cannot directly assign clinical outcome authority.**

## Objective

Among feasible assignments, minimize consistency penalties only:

```text
Q(X) =
  w_fail      * P_fail(X)
+ w_unproven  * P_unproven(X)
+ w_conflict  * P_conflict(X)
+ w_duplicate * P_duplicate(X)
+ w_gap       * P_provenance_gap(X)
```

The exact penalty definitions and weights are versioned inputs.

### Weight safety rule

```text
No finite objective weight may override a hard constraint.
```

There is deliberately **no clinical-outcome reward term** such as `reward(RESOLVED)` or `penalty(UNRESOLVED)`.

The optimizer therefore has no incentive or authority to manufacture a healthier state.

## Candidate solution

The optimization layer returns only:

```text
X*
```

where `X*` is a feasible candidate assignment minimizing the declared consistency objective to the extent reported by the solver.

A solver's best sample is not itself a clinical conclusion and need not constitute a mathematical proof of global optimality unless the solver and verification receipt explicitly establish that property.

## Classical deterministic resolver

The downstream resolver is separate:

```text
Resolution = G(
  X*,
  ConditionStateBefore,
  ConditionStateAfter,
  AdmissibleEvidence,
  ResolutionCriteriaVersion
)
```

and emits only:

```text
delta_R ∈ {POSITIVE, ZERO, NEGATIVE, UNPROVEN, CONFLICTING}
clinical_resolution ∈ {RESOLVED, UNRESOLVED, UNPROVEN, CONFLICTING}
last_proven_condition_state
```

`G` must be classical, deterministic, version-bounded, replayable, and independent of solver vendor.

## Mathematical boundary

Desired optimization statement:

```text
X* ∈ {0,1}^576
subject to all declared hard constraints
and Q(X*) ≤ Q(X) for every feasible X
```

However, the protocol distinguishes:

```text
CANDIDATE_OPTIMUM
VERIFIED_OPTIMUM
```

A D-Wave hybrid/QPU run may produce a strong feasible candidate without proving the universal inequality over all feasible `X`. The status `VERIFIED_OPTIMUM` therefore requires an independent verification method capable of proving optimality for the instantiated problem.

This distinction is mandatory and prevents a solver success response from being inflated into a mathematical proof.

## Multi-stakeholder tensor

For stakeholder `k`, the evidence surface may be represented as:

```text
T[k,i,j,s]
```

Pairwise comparisons may isolate cells where stakeholders disagree. The comparison layer may identify conflict loci only; it may not infer blame, motive, or clinical truth.

## Resilience invariant

Held exactly:

```text
Failure(Sourceᵢ) ⇏ Failure(Audit)
```

## Non-authority statement

```text
D-Wave optimizes constraints — never outcomes.
Optimization != Clinical Authority.
```
