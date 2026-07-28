use constitutional_vm_audit::{AuditError, AuditEvent, AuditLedger, Hash32, SignedAuditEntry};
use ed25519_dalek::{Signer, SigningKey};
use std::time::{Duration, UNIX_EPOCH};

fn signed_entry(
    event: AuditEvent,
    epoch_second: u64,
    signer_id: &str,
    prev_hash: Hash32,
    signing_key: &SigningKey,
) -> SignedAuditEntry {
    let mut entry = SignedAuditEntry::unsigned(
        event,
        UNIX_EPOCH + Duration::from_secs(epoch_second),
        signer_id,
        prev_hash,
    );
    entry.signature = signing_key.sign(&entry.signing_bytes().unwrap()).to_bytes().to_vec();
    entry.hash = entry.recompute_hash().unwrap();
    entry
}

#[test]
fn auditlayer_is_append_only() {
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let mut ledger = AuditLedger::new();
    ledger
        .register_signer("gate-b", signing_key.verifying_key())
        .unwrap();

    let scope_v1 = [1_u8; 32];
    let scope_v2 = [2_u8; 32];

    // Test 1: a valid chain appends successfully.
    let invocation = signed_entry(
        AuditEvent::InvocationTrace {
            invocation_id: "invoke-2008-certification".into(),
            scope_hash: scope_v1,
        },
        1,
        "gate-b",
        ledger.tail_hash(),
        &signing_key,
    );
    ledger.append(invocation).unwrap();

    let review = signed_entry(
        AuditEvent::ReviewTrace {
            review_id: "review-2011-bates-adversarial".into(),
            proposed_scope_hash: scope_v2,
            adversarial: true,
        },
        2,
        "gate-b",
        ledger.tail_hash(),
        &signing_key,
    );
    ledger.append(review).unwrap();

    let mutation = signed_entry(
        AuditEvent::ScopeMutationTrace {
            mutation_id: "mutation-2011-bates".into(),
            prior_scope_hash: scope_v1,
            proposed_scope_hash: scope_v2,
        },
        3,
        "gate-b",
        ledger.tail_hash(),
        &signing_key,
    );
    ledger.append(mutation).unwrap();
    assert_eq!(ledger.entries().len(), 3);

    // Test 2: overwrite is mechanically rejected and state remains unchanged.
    let replacement = signed_entry(
        AuditEvent::AuthorityTrace {
            authority_id: "replacement-attempt".into(),
            scope_hash: scope_v2,
        },
        4,
        "gate-b",
        ledger.tail_hash(),
        &signing_key,
    );
    let stable_tail = ledger.tail_hash();
    let stable_len = ledger.entries().len();
    assert_eq!(
        ledger.overwrite(0, replacement),
        Err(AuditError::AppendOnlyViolation("overwrite"))
    );
    assert_eq!(ledger.tail_hash(), stable_tail);
    assert_eq!(ledger.entries().len(), stable_len);

    // Test 3: a wrong previous hash is rejected before admission.
    let wrong_prev = signed_entry(
        AuditEvent::AuthorityTrace {
            authority_id: "wrong-link".into(),
            scope_hash: scope_v2,
        },
        5,
        "gate-b",
        [99_u8; 32],
        &signing_key,
    );
    assert!(matches!(
        ledger.append(wrong_prev),
        Err(AuditError::PreviousHashMismatch { .. })
    ));

    // Test 4: a cryptographically invalid signature is rejected.
    let mut invalid_signature = signed_entry(
        AuditEvent::AuthorityTrace {
            authority_id: "forged-authority".into(),
            scope_hash: scope_v2,
        },
        6,
        "gate-b",
        ledger.tail_hash(),
        &signing_key,
    );
    invalid_signature.signature[0] ^= 0x80;
    invalid_signature.hash = invalid_signature.recompute_hash().unwrap();
    assert_eq!(
        ledger.append(invalid_signature),
        Err(AuditError::InvalidSignature)
    );

    // Add a legitimate non-review event so the next mutation lacks Gate B review.
    let authority = signed_entry(
        AuditEvent::AuthorityTrace {
            authority_id: "executive-only-authorization".into(),
            scope_hash: scope_v2,
        },
        7,
        "gate-b",
        ledger.tail_hash(),
        &signing_key,
    );
    ledger.append(authority).unwrap();

    // Test 5: wormhole attempt is correctly signed and linked, but skips
    // adversarial review. Transition policy rejects it without mutating state.
    let scope_v3 = [3_u8; 32];
    let wormhole = signed_entry(
        AuditEvent::ScopeMutationTrace {
            mutation_id: "classified-semantic-drift".into(),
            prior_scope_hash: scope_v2,
            proposed_scope_hash: scope_v3,
        },
        8,
        "gate-b",
        ledger.tail_hash(),
        &signing_key,
    );
    let stable_tail = ledger.tail_hash();
    let stable_len = ledger.entries().len();
    assert_eq!(
        ledger.append(wormhole),
        Err(AuditError::MissingAdversarialReview)
    );
    assert_eq!(ledger.tail_hash(), stable_tail);
    assert_eq!(ledger.entries().len(), stable_len);

    println!(
        "constitutional audit motor green: entries={}, tail={:02x?}",
        ledger.entries().len(),
        ledger.tail_hash()
    );
}
