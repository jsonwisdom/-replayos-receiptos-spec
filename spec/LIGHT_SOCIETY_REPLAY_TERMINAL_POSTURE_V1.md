# Light Society Replay Machine — Terminal Posture V1

Status: COMMITTED TERMINAL POSTURE
Scope: Light Society billion-agent replay audit
Authority created: false

## Terminal State

```text
TERMINAL_STATE: WAITING_FOR_BYTES
MODE_A: ENABLED
MODE_B: BLOCKED
MODE_C: BLOCKED
SCIENTIFIC_CLOSURE: OPEN
SYSTEMS_CLOSURE: OPEN
HISTORICAL_PROVENANCE: UNPROVEN
AUTHORITY_CREATED: false
MACHINE_STATE: SEALED_WAITING_FOR_BYTES
```

The corridor is not asking for interpretation. It is asking for artifacts.

## Admissible Next Events

Only these evidence-arrival events can advance the machine:

```text
RECEIPT_ARRIVED: graph_CSR_N1e9_m3
RECEIPT_ARRIVED: lookup_table_900M_uint8
RECEIPT_ARRIVED: generator_spec_exact
```

Silence is also a valid audit outcome and causes no transition.

No other event is admissible for promotion of the replay state.

## Evidence Arrival Protocol

When bytes arrive:

1. DECLARE
2. STREAM_VERIFY
3. MATERIALIZE

Only after an artifact reaches VERIFIED status may the execution router re-evaluate the available replay mode.

The following substitutions are prohibited:

```text
DIGEST_ONLY_ACCEPTANCE = false
METADATA_SUBSTITUTION = false
INFERENCE_SUBSTITUTION = false
AUTHORITY_SUBSTITUTION = false
```

## Corridor Invariant

```text
DIGEST
  -> VERIFIED_ARTIFACT
  -> REPLAY
  -> CLAIM_TEST
```

A digest is not an artifact. An inference is not a historical execution state. A replay result cannot create historical provenance without independent evidence.

## Silence Semantics

If no qualifying artifact arrives, the target claim remains suspended:

```text
CONTRADICTED = false
ACCEPTED = false
RESOLVED = false
SUSPENDED = true
```

Machine silence is therefore a meaningful forensic result, not an execution failure.

## Non-Authority Boundary

```text
AUTHORITY_CREATED = false
HISTORICAL_PROVENANCE = UNPROVEN
```

This specification records the replay machine posture. It does not identify historical truth, create scientific authority, or substitute missing artifact bytes.
