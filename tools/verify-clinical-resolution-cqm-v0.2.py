#!/usr/bin/env python3
"""Structural verification harness for Clinical Resolution Oracle v0.2.

This verifies only the 12x12 one-hot CQM structure:
  12 invariant rows * 12 dimensions * 4 states = 576 binary variables
  12 invariant rows * 12 dimensions = 144 one-hot hard constraints

It does NOT determine delta_R, clinical_resolution, or any clinical outcome.
"""

from __future__ import annotations

import argparse

from dimod import Binary, ConstrainedQuadraticModel

ROWS = tuple(f"I{i}" for i in range(1, 13))
COLUMNS = tuple(f"C{i}" for i in range(1, 13))
STATES = ("PASS", "FAIL", "UNPROVEN", "CONFLICTING")

EXPECTED_CELLS = 12 * 12
EXPECTED_BINARY_VARIABLES = EXPECTED_CELLS * 4
EXPECTED_ONE_HOT_CONSTRAINTS = EXPECTED_CELLS


def build_structural_cqm():
    cqm = ConstrainedQuadraticModel()
    x = {}

    for row in ROWS:
        for column in COLUMNS:
            cell_vars = []
            for state in STATES:
                label = f"x_{row}_{column}_{state}"
                variable = Binary(label)
                x[(row, column, state)] = variable
                cell_vars.append(variable)

            # Ordinary CQM equality constraint. We intentionally do not use
            # add_discrete(), because these variables may later participate in
            # additional evidence-derived constraints.
            cqm.add_constraint(
                sum(cell_vars) == 1,
                label=f"one_hot_{row}_{column}",
            )

    # No semantic-state objective. All feasible cell states are neutral here.
    return cqm, x


def structural_receipt(cqm, x):
    return {
        "rows": len(ROWS),
        "columns": len(COLUMNS),
        "states_per_cell": len(STATES),
        "logical_cells": len(ROWS) * len(COLUMNS),
        "base_binary_variables": len(x),
        "hard_one_hot_constraints": len(cqm.constraints),
        "expected_base_binary_variables": EXPECTED_BINARY_VARIABLES,
        "expected_hard_one_hot_constraints": EXPECTED_ONE_HOT_CONSTRAINTS,
        "structural_pass": (
            len(x) == EXPECTED_BINARY_VARIABLES
            and len(cqm.constraints) == EXPECTED_ONE_HOT_CONSTRAINTS
        ),
        "clinical_authority": "NONE",
    }


def submit_to_leap(cqm):
    from dwave.system import LeapHybridCQMSampler

    sampler = LeapHybridCQMSampler()
    sampleset = sampler.sample_cqm(
        cqm,
        label="Clinical Resolution Oracle v0.2 structural verification",
    )
    feasible = sampleset.filter(lambda datum: datum.is_feasible)

    return {
        "solver_id": getattr(sampler, "solver", None).id if getattr(sampler, "solver", None) else None,
        "returned_samples": len(sampleset),
        "feasible_samples": len(feasible),
        "feasible_candidate_observed": len(feasible) > 0,
        "verification_status": "STRUCTURAL_FEASIBILITY_ONLY",
        "global_optimality_proven": False,
        "clinical_authority": "NONE",
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--submit",
        action="store_true",
        help="Submit the structural CQM to the configured D-Wave Leap hybrid CQM solver.",
    )
    args = parser.parse_args()

    cqm, x = build_structural_cqm()
    receipt = structural_receipt(cqm, x)

    print(receipt)
    if not receipt["structural_pass"]:
        raise SystemExit("STRUCTURAL_FAIL")

    if args.submit:
        print(submit_to_leap(cqm))


if __name__ == "__main__":
    main()
