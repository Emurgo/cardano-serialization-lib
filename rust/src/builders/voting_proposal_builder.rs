use crate::*;
use std::collections::BTreeMap;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VotingProposalBuilder {
    proposals: BTreeMap<VotingProposal, Option<ScriptWitnessType>>,
}

#[wasm_bindgen]
impl VotingProposalBuilder {
    pub fn new() -> Self {
        Self {
            proposals: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, proposal: &VotingProposal) -> Result<(), JsError> {
        if proposal.has_script_hash() {
            return Err(JsError::from_str("Proposal has a script hash. Use add_with_plutus_witness instead."));
        }
        self.proposals.insert(proposal.clone(), None);
        Ok(())
    }

    pub fn add_with_plutus_witness(
        &mut self,
        proposal: &VotingProposal,
        witness: &PlutusWitness,
    ) -> Result<(), JsError> {
        self.proposals.insert(
            proposal.clone(),
            Some(ScriptWitnessType::PlutusScriptWitness(witness.clone())),
        );
        Ok(())
    }

    pub fn get_plutus_witnesses(&self) -> PlutusWitnesses {
        let tag = RedeemerTag::new_voting_proposal();
        let mut scripts = PlutusWitnesses::new();
        for (i, (_, script_wit)) in self.proposals.iter().enumerate() {
            if let Some(ScriptWitnessType::PlutusScriptWitness(s)) = script_wit {
                let index = BigNum::from(i);
                scripts.add(&s.clone_with_redeemer_index_and_tag(&index, &tag));
            }
        }
        scripts
    }

    pub fn get_ref_inputs(&self) -> TransactionInputs {
        let mut inputs = Vec::new();
        for (_, script_wit) in &self.proposals {
            match script_wit {
                Some(script_witness) => {
                    if let Some(input) = script_witness.get_script_ref_input() {
                        inputs.push(input);
                    }
                    if let Some(input) = script_witness.get_datum_ref_input() {
                        inputs.push(input);
                    }
                }
                None => {}
            }
        }
        TransactionInputs(inputs)
    }

    pub(crate) fn get_total_deposit(&self) -> Result<Coin, JsError> {
        self.proposals.iter().fold(
            Ok(Coin::zero()),
            |acc: Result<Coin, JsError>, (proposal, _)| {
                acc.and_then(|acc| {
                    acc.checked_add(&proposal.deposit)
                        .or_else(|_| Err(JsError::from_str("Overflow when calculating total deposit")))
                })
            },
        )
    }

    pub(crate) fn get_used_plutus_lang_versions(&self) -> BTreeSet<Language> {
        let mut used_langs = BTreeSet::new();
        for (_, script_wit) in &self.proposals {
            if let Some(ScriptWitnessType::PlutusScriptWitness(s)) = script_wit {
                used_langs.insert(s.script.language());
            }
        }
        used_langs
    }

    //return only ref inputs that are script refs with added size
    //used for calculating the fee for the transaction
    //another ref input and also script ref input without size are filtered out
    pub(crate) fn get_script_ref_inputs_with_size(
        &self,
    ) -> impl Iterator<Item = (&TransactionInput, usize)> {
        self.proposals.iter()
            .filter_map(|(_, script_wit)| script_wit.as_ref())
            .filter_map(|script_wit| script_wit.get_script_ref_input_with_size())
    }

    pub fn has_plutus_scripts(&self) -> bool {
        for (_, script_wit) in &self.proposals {
            if let Some(ScriptWitnessType::PlutusScriptWitness(_)) = script_wit {
                return true;
            }
        }
        false
    }

    pub fn build(&self) -> VotingProposals {
        let mut proposals = Vec::new();
        for (voter, _) in &self.proposals {
            proposals.push(voter.clone());
        }
        VotingProposals(proposals)
    }
}
