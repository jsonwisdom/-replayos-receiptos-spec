# CHAMBER 2 — Declassification Timer Wormhole

**Status:** NON-NORMATIVE / ADVERSARIAL EXPERIMENT

This chamber demonstrates why `SystemTime::now()` cannot serve as the constitutional time authority for `AUDIT_APPEND`.

## Attack

A privileged host operator can supply a false wall-clock value:

- rollback: post-mutation activity appears to predate the mutation;
- fast-forward: classification or declassification deadlines appear to mature early.

The vulnerable model accepts both because the receipt's `when` field is external mutable state.

## Hardened mechanism

`AuditClock` replaces wall-clock authority with a sequential audit epoch.

A tick is accepted only when:

1. the proposed epoch is exactly `current_epoch + 1`;
2. at least two distinct constitutional branches provide valid signatures;
3. the accepted signatures and prior tick hash are committed into a SHA-256 tick receipt.

`HardenedAuditLog::append` reads only `AuditClock::now()` and chains each audit entry to its predecessor. OS time is never consulted.

## Proof tests

- vulnerable audit accepts a backward timestamp;
- hardened clock rejects rollback and fast-forward jumps;
- duplicate signatures from one branch do not satisfy threshold;
- hostile OS time cannot alter the committed audit epoch;
- tick receipts form a replayable hash chain.

## Cryptographic boundary

The experiment defines a `ThresholdVerifier` interface and uses deterministic signatures only in tests. It does **not** claim production-grade threshold cryptography, key custody, revocation, anti-equivocation, or distributed consensus. A production profile must bind branch identities to independently controlled keys and define a concrete threshold-signature protocol.

## Governance consequence

Without a valid next-epoch quorum, no new authoritative timestamp can be minted. A production VM may therefore halt new queries rather than execute without a constitutionally valid audit epoch.
