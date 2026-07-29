use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

pub type Hash32 = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuditEvent {
    InvocationTrace {
        invocation_id: String,
        scope_hash: Hash32,
    },
    ReviewTrace {
        review_id: String,
        proposed_scope_hash: Hash32,
        adversarial: bool,
    },
    AuthorityTrace {
        authority_id: String,
        scope_hash: Hash32,
    },
    ScopeMutationTrace {
        mutation_id: String,
        prior_scope_hash: Hash32,
        proposed_scope_hash: Hash32,
    },
}

#[derive(Debug, Clone)]
pub struct SignedAuditEntry {
    pub event: AuditEvent,
    pub timestamp: SystemTime,
    pub signer_id: String,
    pub signature: Vec<u8>,
    pub prev_hash: Hash32,
    pub hash: Hash32,
}

impl SignedAuditEntry {
    pub fn unsigned(
        event: AuditEvent,
        timestamp: SystemTime,
        signer_id: impl Into<String>,
        prev_hash: Hash32,
    ) -> Self {
        Self {
            event,
            timestamp,
            signer_id: signer_id.into(),
            signature: Vec::new(),
            prev_hash,
            hash: [0; 32],
        }
    }

    pub fn signing_bytes(&self) -> Result<Vec<u8>, AuditError> {
        canonical_signing_bytes(
            &self.event,
            self.timestamp,
            &self.signer_id,
            &self.prev_hash,
        )
    }

    pub fn recompute_hash(&self) -> Result<Hash32, AuditError> {
        let signing_bytes = self.signing_bytes()?;
        let mut hasher = Sha256::new();
        hasher.update((signing_bytes.len() as u64).to_be_bytes());
        hasher.update(signing_bytes);
        hasher.update((self.signature.len() as u64).to_be_bytes());
        hasher.update(&self.signature);
        Ok(hasher.finalize().into())
    }
}

#[derive(Debug, Default)]
pub struct AuditLedger {
    entries: Vec<SignedAuditEntry>,
    tail_hash: Hash32,
    signers: HashMap<String, VerifyingKey>,
}

impl AuditLedger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_signer(
        &mut self,
        signer_id: impl Into<String>,
        verifying_key: VerifyingKey,
    ) -> Result<(), AuditError> {
        let signer_id = signer_id.into();
        if self.signers.contains_key(&signer_id) {
            return Err(AuditError::SignerAlreadyRegistered(signer_id));
        }
        self.signers.insert(signer_id, verifying_key);
        Ok(())
    }

    pub fn append(&mut self, entry: SignedAuditEntry) -> Result<(), AuditError> {
        if entry.prev_hash != self.tail_hash {
            return Err(AuditError::PreviousHashMismatch {
                expected: self.tail_hash,
                actual: entry.prev_hash,
            });
        }

        let verifying_key = self
            .signers
            .get(&entry.signer_id)
            .ok_or_else(|| AuditError::UnknownSigner(entry.signer_id.clone()))?;

        let signature = Signature::try_from(entry.signature.as_slice())
            .map_err(|_| AuditError::MalformedSignature)?;
        verifying_key
            .verify(&entry.signing_bytes()?, &signature)
            .map_err(|_| AuditError::InvalidSignature)?;

        let expected_hash = entry.recompute_hash()?;
        if entry.hash != expected_hash {
            return Err(AuditError::EntryHashMismatch {
                expected: expected_hash,
                actual: entry.hash,
            });
        }

        self.enforce_transition_invariants(&entry.event)?;

        self.tail_hash = entry.hash;
        self.entries.push(entry);
        Ok(())
    }

    pub fn entries(&self) -> &[SignedAuditEntry] {
        &self.entries
    }

    pub fn tail_hash(&self) -> Hash32 {
        self.tail_hash
    }

    pub fn overwrite(
        &mut self,
        _index: usize,
        _replacement: SignedAuditEntry,
    ) -> Result<(), AuditError> {
        Err(AuditError::AppendOnlyViolation("overwrite"))
    }

    pub fn delete(&mut self, _index: usize) -> Result<(), AuditError> {
        Err(AuditError::AppendOnlyViolation("delete"))
    }

    pub fn truncate(&mut self, _len: usize) -> Result<(), AuditError> {
        Err(AuditError::AppendOnlyViolation("truncate"))
    }

    fn enforce_transition_invariants(&self, event: &AuditEvent) -> Result<(), AuditError> {
        let AuditEvent::ScopeMutationTrace {
            proposed_scope_hash,
            ..
        } = event
        else {
            return Ok(());
        };

        match self.entries.last().map(|entry| &entry.event) {
            Some(AuditEvent::ReviewTrace {
                proposed_scope_hash: reviewed_scope_hash,
                adversarial: true,
                ..
            }) if reviewed_scope_hash == proposed_scope_hash => Ok(()),
            _ => Err(AuditError::MissingAdversarialReview),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuditError {
    #[error("append-only ledger forbids {0}")]
    AppendOnlyViolation(&'static str),

    #[error("signer already registered: {0}")]
    SignerAlreadyRegistered(String),

    #[error("unknown signer: {0}")]
    UnknownSigner(String),

    #[error("signature bytes are malformed")]
    MalformedSignature,

    #[error("signature verification failed")]
    InvalidSignature,

    #[error("previous hash mismatch")]
    PreviousHashMismatch { expected: Hash32, actual: Hash32 },

    #[error("entry hash mismatch")]
    EntryHashMismatch { expected: Hash32, actual: Hash32 },

    #[error("scope mutation requires an immediately preceding matching adversarial review")]
    MissingAdversarialReview,

    #[error("timestamp precedes UNIX_EPOCH")]
    TimestampBeforeUnixEpoch,

    #[error("audit serialization failed: {0}")]
    Serialization(String),
}

#[derive(Serialize)]
struct CanonicalUnsignedEntry<'a> {
    event: &'a AuditEvent,
    timestamp_secs: u64,
    timestamp_nanos: u32,
    signer_id: &'a str,
    prev_hash: &'a Hash32,
}

fn canonical_signing_bytes(
    event: &AuditEvent,
    timestamp: SystemTime,
    signer_id: &str,
    prev_hash: &Hash32,
) -> Result<Vec<u8>, AuditError> {
    let duration = timestamp
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AuditError::TimestampBeforeUnixEpoch)?;

    serde_json::to_vec(&CanonicalUnsignedEntry {
        event,
        timestamp_secs: duration.as_secs(),
        timestamp_nanos: duration.subsec_nanos(),
        signer_id,
        prev_hash,
    })
    .map_err(|error| AuditError::Serialization(error.to_string()))
}
