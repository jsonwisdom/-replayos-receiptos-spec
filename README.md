# ReplayOS / ReceiptOS Spec

**Status:** v1.0.0-rc1 (Architecture FROZEN — Promotion BLOCKED)

**Decision Formula:**
```
Decision = F(Specification, ROR, ER, ObservationBoundary)
```

**Receipt Pipeline:**
```
ROR (observation) → ER (execution) → GER (gate evaluation) → PR (final decision)
```

## Normative Artifacts (RC1)

| Path | Role |
|------|------|
| `schemas/observation-boundary.schema.json` | ObservationBoundary |
| `schemas/ror.schema.json` | RepositoryObservationReport (ROR) |
| `schemas/er.schema.json` | ExecutionReport (ER) |
| `schemas/ger.schema.json` | GateEvaluationReport (GER) |
| `schemas/promotion-receipt.schema.json` | PromotionReceipt (PR) |
| `schemas/evidence-item.schema.json` | EvidenceItem |
| `schemas/rules.schema.json` | Rules |
| `schemas/specification.schema.json` | Specification |
| `schemas/validation-report.schema.json` | ValidationReport |
| `functions/decision-function.md` | Pure Decision Function |
| `manifests/normative-set.manifest.json` | Spec Manifest |

## Promotion Gate G-001

Blocked until the following evidence is observed in the repository:

1. JCS artifact (canonical vectors)
2. Validation PASS report
3. Conformance PASS report
4. Spec Manifest SHA-256 verification record
5. Git tag `v1.0.0`
6. Release with matching hash

## Invariants

- Monotonicity (append-only evidence)
- Referential transparency
- Boundary closure
- Observation / Execution separation
- Zero side-effects in Decision function

See `manifests/normative-set.manifest.json` for the authoritative object list.
