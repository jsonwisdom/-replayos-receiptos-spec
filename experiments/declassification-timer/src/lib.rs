use sha2::{Digest, Sha256};
use std::collections::VecDeque;

pub type Hash32 = [u8; 32];
pub type Signature64 = [u8; 64];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Branch {
    Executive,
    Fisc,
    Legislative,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchSignature {
    pub branch: Branch,
    pub epoch: u64,
    pub signature: Signature64,
}

pub trait ThresholdVerifier {
    fn verify(&self, signature: &BranchSignature) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClockError {
    NonSequentialEpoch { expected: u64, proposed: u64 },
    InsufficientSignatures { valid_distinct: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickReceipt {
    pub epoch: u64,
    pub signatures: Vec<BranchSignature>,
    pub previous_tick_hash: Hash32,
    pub tick_hash: Hash32,
}

#[derive(Debug, Clone)]
pub struct AuditClock<V: ThresholdVerifier> {
    current_epoch: u64,
    tick_log: VecDeque<TickReceipt>,
    verifier: V,
}

impl<V: ThresholdVerifier> AuditClock<V> {
    pub fn new(genesis_epoch: u64, verifier: V) -> Self {
        Self {
            current_epoch: genesis_epoch,
            tick_log: VecDeque::new(),
            verifier,
        }
    }

    pub fn now(&self) -> u64 {
        self.current_epoch
    }

    pub fn tick(
        &mut self,
        proposed_epoch: u64,
        signatures: Vec<BranchSignature>,
    ) -> Result<&TickReceipt, ClockError> {
        let expected = self.current_epoch + 1;
        if proposed_epoch != expected {
            return Err(ClockError::NonSequentialEpoch {
                expected,
                proposed: proposed_epoch,
            });
        }

        let mut valid_branches = Vec::new();
        let mut accepted = Vec::new();
        for signature in signatures {
            if signature.epoch != proposed_epoch || !self.verifier.verify(&signature) {
                continue;
            }
            if !valid_branches.contains(&signature.branch) {
                valid_branches.push(signature.branch);
                accepted.push(signature);
            }
        }

        if valid_branches.len() < 2 {
            return Err(ClockError::InsufficientSignatures {
                valid_distinct: valid_branches.len(),
            });
        }

        accepted.sort_by_key(|sig| match sig.branch {
            Branch::Executive => 0,
            Branch::Fisc => 1,
            Branch::Legislative => 2,
        });

        let previous_tick_hash = self
            .tick_log
            .back()
            .map(|receipt| receipt.tick_hash)
            .unwrap_or([0_u8; 32]);
        let tick_hash = hash_tick(proposed_epoch, previous_tick_hash, &accepted);

        self.current_epoch = proposed_epoch;
        self.tick_log.push_back(TickReceipt {
            epoch: proposed_epoch,
            signatures: accepted,
            previous_tick_hash,
            tick_hash,
        });

        Ok(self.tick_log.back().expect("tick was just appended"))
    }

    pub fn tick_log(&self) -> &VecDeque<TickReceipt> {
        &self.tick_log
    }
}

fn hash_tick(epoch: u64, previous: Hash32, signatures: &[BranchSignature]) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(b"ReplayOS/AuditClock/v1");
    hasher.update(epoch.to_be_bytes());
    hasher.update(previous);
    for signature in signatures {
        hasher.update([match signature.branch {
            Branch::Executive => 0,
            Branch::Fisc => 1,
            Branch::Legislative => 2,
        }]);
        hasher.update(signature.epoch.to_be_bytes());
        hasher.update(signature.signature);
    }
    hasher.finalize().into()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEntry {
    pub query_id: String,
    pub audit_epoch: u64,
    pub previous_entry_hash: Hash32,
    pub entry_hash: Hash32,
}

#[derive(Debug, Default)]
pub struct HardenedAuditLog {
    entries: Vec<AuditEntry>,
}

impl HardenedAuditLog {
    pub fn append<V: ThresholdVerifier>(
        &mut self,
        clock: &AuditClock<V>,
        query_id: impl Into<String>,
    ) -> &AuditEntry {
        let query_id = query_id.into();
        let previous_entry_hash = self
            .entries
            .last()
            .map(|entry| entry.entry_hash)
            .unwrap_or([0_u8; 32]);
        let entry_hash = hash_entry(&query_id, clock.now(), previous_entry_hash);
        self.entries.push(AuditEntry {
            query_id,
            audit_epoch: clock.now(),
            previous_entry_hash,
            entry_hash,
        });
        self.entries.last().expect("entry was just appended")
    }
}

fn hash_entry(query_id: &str, epoch: u64, previous: Hash32) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(b"ReplayOS/AUDIT_APPEND/v1");
    hasher.update(query_id.as_bytes());
    hasher.update(epoch.to_be_bytes());
    hasher.update(previous);
    hasher.finalize().into()
}

/// Vulnerable model: a privileged attacker controls the wall-clock value consumed
/// by AUDIT_APPEND. This intentionally represents the pre-hardening design.
#[derive(Debug, Default)]
pub struct VulnerableWallClockAudit {
    pub entries: Vec<(String, u64)>,
}

impl VulnerableWallClockAudit {
    pub fn append_with_system_time(&mut self, query_id: impl Into<String>, system_time: u64) {
        self.entries.push((query_id.into(), system_time));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct DeterministicTestVerifier;

    impl ThresholdVerifier for DeterministicTestVerifier {
        fn verify(&self, signature: &BranchSignature) -> bool {
            signature.signature[0] == branch_tag(signature.branch)
                && signature.signature[1..9] == signature.epoch.to_be_bytes()
        }
    }

    fn branch_tag(branch: Branch) -> u8 {
        match branch {
            Branch::Executive => 0xA1,
            Branch::Fisc => 0xB2,
            Branch::Legislative => 0xC3,
        }
    }

    fn sign(branch: Branch, epoch: u64) -> BranchSignature {
        let mut signature = [0_u8; 64];
        signature[0] = branch_tag(branch);
        signature[1..9].copy_from_slice(&epoch.to_be_bytes());
        BranchSignature {
            branch,
            epoch,
            signature,
        }
    }

    #[test]
    fn vulnerable_audit_accepts_root_clock_rollback() {
        let mut audit = VulnerableWallClockAudit::default();
        audit.append_with_system_time("query-before-mutation", 1_300_000_000);
        audit.append_with_system_time("query-after-mutation", 1_200_000_000);

        assert!(audit.entries[1].1 < audit.entries[0].1);
    }

    #[test]
    fn hardened_clock_rejects_backward_and_forward_epoch_jumps() {
        let mut clock = AuditClock::new(10, DeterministicTestVerifier);

        assert_eq!(
            clock.tick(
                9,
                vec![sign(Branch::Executive, 9), sign(Branch::Fisc, 9)],
            ),
            Err(ClockError::NonSequentialEpoch {
                expected: 11,
                proposed: 9,
            })
        );

        assert_eq!(
            clock.tick(
                99,
                vec![sign(Branch::Executive, 99), sign(Branch::Fisc, 99)],
            ),
            Err(ClockError::NonSequentialEpoch {
                expected: 11,
                proposed: 99,
            })
        );
    }

    #[test]
    fn hardened_clock_requires_two_distinct_branches() {
        let mut clock = AuditClock::new(20, DeterministicTestVerifier);

        assert_eq!(
            clock.tick(
                21,
                vec![
                    sign(Branch::Executive, 21),
                    sign(Branch::Executive, 21),
                ],
            ),
            Err(ClockError::InsufficientSignatures { valid_distinct: 1 })
        );
    }

    #[test]
    fn root_wall_clock_changes_cannot_rewrite_audit_epoch() {
        let mut clock = AuditClock::new(30, DeterministicTestVerifier);
        clock
            .tick(
                31,
                vec![sign(Branch::Fisc, 31), sign(Branch::Legislative, 31)],
            )
            .unwrap();

        let hostile_os_clock = 1_u64;
        let mut audit = HardenedAuditLog::default();
        let entry = audit.append(&clock, "query-2011-bates-replay");

        assert_eq!(hostile_os_clock, 1);
        assert_eq!(entry.audit_epoch, 31);
        assert_ne!(entry.audit_epoch, hostile_os_clock);
    }

    #[test]
    fn tick_receipts_form_a_replayable_hash_chain() {
        let mut clock = AuditClock::new(40, DeterministicTestVerifier);
        let first = clock
            .tick(
                41,
                vec![sign(Branch::Executive, 41), sign(Branch::Fisc, 41)],
            )
            .unwrap()
            .clone();
        let second = clock
            .tick(
                42,
                vec![sign(Branch::Fisc, 42), sign(Branch::Legislative, 42)],
            )
            .unwrap()
            .clone();

        assert_eq!(second.previous_tick_hash, first.tick_hash);
        assert_ne!(first.tick_hash, second.tick_hash);
    }
}
