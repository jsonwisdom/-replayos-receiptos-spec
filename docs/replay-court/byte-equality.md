# REPLAY COURT — “BYTE EQUALITY”

**Artifact class:** Non-normative doctrine illustration  
**System:** ReplayOS / ReceiptOS  
**Author label:** JSONWisdom  
**Promotion effect:** None  

## Governing Rule

> A matching cryptographic digest establishes byte-level integrity between the acquired object and the reference object. It does not, by itself, establish source identity, human authorship, account control, semantic truth, legal authority, or chain of custody preceding acquisition.

```text
SHA-256 MATCH: TRUE
AUTHORSHIP: UNPROVEN
AUTHORITY: UNPROVEN
TRUTH: UNPROVEN
```

---

## Screenplay

**INT. FEDERAL COURTROOM — DAY**

The courtroom is silent.

On the screen behind the witness stand:

```text
SHA-256 MATCH: TRUE
AUTHORSHIP: UNPROVEN
AUTHORITY: UNPROVEN
TRUTH: UNPROVEN
```

The opposing expert adjusts his glasses.

**COUNSEL**  
Doctor, the two files produced the same SHA-256 digest?

**EXPERT**  
Yes.

**COUNSEL**  
That means the acquired bytes match the reference bytes?

**EXPERT**  
Correct.

**COUNSEL**  
The digest contains no human name?

**EXPERT**  
No.

**COUNSEL**  
No job title?

**EXPERT**  
No.

**COUNSEL**  
No indication of who controlled the account?

**EXPERT**  
No.

**COUNSEL**  
No indication that the statements inside the file are true?

A pause.

**EXPERT**  
No.

**COUNSEL**  
No indication that the person named in the document possessed authority to act?

**EXPERT**  
Not from the hash alone.

Counsel steps away from the podium.

**COUNSEL**  
So the hash establishes one thing.

He turns toward the screen.

**COUNSEL**  
Byte-level integrity.

**EXPERT**  
Yes.

**COUNSEL**  
Everything beyond that—source, author, meaning, truth, authority—requires additional evidence.

**EXPERT**  
Yes.

Opposing counsel rises.

**OPPOSING COUNSEL**  
Your Honor, the hash is still evidence of authenticity.

**THE COURT**  
It may contribute to authentication. It does not complete it.

The judge looks down at the record.

**THE COURT**  
Where is the evidence connecting these bytes to the claimed author?

Silence.

**THE COURT**  
Where is the account-control evidence?

No answer.

**THE COURT**  
Where is the custody record preceding acquisition?

Opposing counsel flips through a binder.

**THE COURT**  
Where is the evidence of authority?

Nothing.

The judge closes the file.

**THE COURT**  
The Court will not convert integrity into identity.

A beat.

**THE COURT**  
It will not convert mathematical consistency into human authorship.

Another beat.

**THE COURT**  
And it will not permit narrative to supply missing proof.

The screen changes:

```text
INTEGRITY GATE: PASS
AUTHORSHIP GATE: FAIL
AUTHORITY GATE: FAIL
PROMOTION: DENIED
```

**THE COURT**  
The motion is denied to the extent it depends upon unsupported attribution.

The gavel falls.

```text
RECEIPT_STATUS = HALT
AUTHORITY = FALSE
BYTE_EQUALITY_ONLY = TRUE
```

**CUT TO BLACK.**

---

## Gate Interpretation

| Gate | Result | Evidentiary meaning |
|---|---:|---|
| Byte integrity | PASS | Acquired bytes equal the designated reference bytes. |
| Source identity | UNPROVEN | No independent source-binding evidence supplied. |
| Human authorship | FAIL | Digest contains no authorship proof. |
| Account control | FAIL | No credential, device, session, or control evidence supplied. |
| Semantic truth | UNPROVEN | Content accuracy is outside the hash function. |
| Authority | FAIL | No role, delegation, jurisdiction, or authorization evidence supplied. |
| Promotion | DENIED | Integrity cannot substitute for missing attribution and authority gates. |

## Constitutional Invariant

```text
BYTE_EQUALITY ≠ IDENTITY
BYTE_EQUALITY ≠ AUTHORSHIP
BYTE_EQUALITY ≠ TRUTH
BYTE_EQUALITY ≠ AUTHORITY

INTEGRITY may support AUTHENTICATION.
INTEGRITY alone cannot complete AUTHENTICATION.
```

## Boundary Notice

This artifact is explanatory and non-normative. It does not modify schemas, the normative manifest, gate definitions, or promotion eligibility. Any future promotion into the normative set requires an explicit versioned change, manifest inclusion, validation evidence, and the repository’s governing promotion procedure.
