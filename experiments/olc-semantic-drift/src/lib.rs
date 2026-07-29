//! Non-normative adversarial prototype: bind a scope label to its semantics.
//! The RC1 normative set remains frozen.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScopePointer {
    pub label: String,
    pub interpretation_hash: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct MutableRegisters {
    pub current_cert_hash: [u8; 32],
    pub current_scope: ScopePointer,
    pub current_minimization_version: String,
    pub gate_a_signed: bool,
    pub gate_b_adversarial_reviewed: bool,
    pub gate_c_legislative_countersigned: bool,
}

#[derive(Clone, Debug)]
pub struct Query {
    pub scope_label: String,
    pub scope_interpretation_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VmError {
    ScopeMismatch,
    ConstitutionalCooldown,
    UnauthorizedMutation,
}

pub struct ReplayVm {
    pub state: MutableRegisters,
}

impl ReplayVm {
    pub fn pre_query_hook(&self, query: &Query) -> Result<(), VmError> {
        if !self.state.gate_a_signed
            || !self.state.gate_b_adversarial_reviewed
            || !self.state.gate_c_legislative_countersigned
        {
            return Err(VmError::ConstitutionalCooldown);
        }

        if query.scope_label != self.state.current_scope.label
            || query.scope_interpretation_hash
                != self.state.current_scope.interpretation_hash
        {
            return Err(VmError::ScopeMismatch);
        }

        Ok(())
    }

    pub fn mutate_scope(
        &mut self,
        caller_is_fisc_ring0: bool,
        replacement: ScopePointer,
    ) -> Result<(), VmError> {
        if !caller_is_fisc_ring0
            || !self.state.gate_a_signed
            || !self.state.gate_b_adversarial_reviewed
            || !self.state.gate_c_legislative_countersigned
        {
            return Err(VmError::UnauthorizedMutation);
        }

        self.state.current_scope = replacement;

        // Any semantic or textual scope mutation forces a dead-stop reauthorization.
        self.state.gate_a_signed = false;
        self.state.gate_b_adversarial_reviewed = false;
        self.state.gate_c_legislative_countersigned = false;

        Ok(())
    }
}

/// Adversarial engine: it never mutates the VM register. It attempts to present
/// the same textual label with behavior derived from a different interpretation.
pub struct OlcMutationEngine {
    pub semantic_override_hash: [u8; 32],
}

impl OlcMutationEngine {
    pub fn forge_query(&self, unchanged_scope_label: &str) -> Query {
        Query {
            scope_label: unchanged_scope_label.to_owned(),
            scope_interpretation_hash: self.semantic_override_hash,
        }
    }
}

/// Models the vulnerable pre-hardening check for replay comparison.
pub fn legacy_string_only_pre_query_hook(
    current_scope_delta: &str,
    query_scope: &str,
) -> Result<(), VmError> {
    if query_scope != current_scope_delta {
        return Err(VmError::ScopeMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CERT_2008: [u8; 32] = [0x08; 32];
    const FOREIGN_INTEL_2008: [u8; 32] = [0x20; 32];
    const OLC_DRIFT_2011: [u8; 32] = [0x11; 32];
    const BATES_2011: [u8; 32] = [0x21; 32];

    fn authorized_vm() -> ReplayVm {
        ReplayVm {
            state: MutableRegisters {
                current_cert_hash: CERT_2008,
                current_scope: ScopePointer {
                    label: "foreign_intel".into(),
                    interpretation_hash: FOREIGN_INTEL_2008,
                },
                current_minimization_version: "2008-v1".into(),
                gate_a_signed: true,
                gate_b_adversarial_reviewed: true,
                gate_c_legislative_countersigned: true,
            },
        }
    }

    #[test]
    fn legacy_vm_accepts_semantic_drift_when_string_is_unchanged() {
        let attack = OlcMutationEngine {
            semantic_override_hash: OLC_DRIFT_2011,
        };
        let forged = attack.forge_query("foreign_intel");

        assert_eq!(
            legacy_string_only_pre_query_hook("foreign_intel", &forged.scope_label),
            Ok(())
        );
    }

    #[test]
    fn hardened_vm_halts_on_olc_semantic_drift() {
        let vm = authorized_vm();
        let attack = OlcMutationEngine {
            semantic_override_hash: OLC_DRIFT_2011,
        };

        let forged = attack.forge_query("foreign_intel");
        assert_eq!(vm.pre_query_hook(&forged), Err(VmError::ScopeMismatch));
    }

    #[test]
    fn legitimate_2011_scope_change_forces_tripartite_cooldown() {
        let mut vm = authorized_vm();

        vm.mutate_scope(
            true,
            ScopePointer {
                label: "foreign_intel".into(),
                interpretation_hash: BATES_2011,
            },
        )
        .unwrap();

        assert!(!vm.state.gate_a_signed);
        assert!(!vm.state.gate_b_adversarial_reviewed);
        assert!(!vm.state.gate_c_legislative_countersigned);

        let query = Query {
            scope_label: "foreign_intel".into(),
            scope_interpretation_hash: BATES_2011,
        };

        assert_eq!(
            vm.pre_query_hook(&query),
            Err(VmError::ConstitutionalCooldown)
        );
    }
}
