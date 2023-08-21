use crate::tx_builder::script_structs::*;
use crate::*;
use std::collections::BTreeMap;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VotingProposalBuilder {
    votes: BTreeMap<VotingProposal, Option<ScriptWitnessType>>,
}

#[wasm_bindgen]
impl VotingProposalBuilder {
    pub fn new() -> Self {
        Self {
            votes: BTreeMap::new(),
        }
    }

    pub fn add(
        &mut self,
        proposal: VotingProposal,
    ) -> Result<(), JsError> {

        self.votes.insert(
            proposal,
            None,
        );

        Ok(())
    }

    pub fn add_with_plutus_witness(
        &mut self,
        proposal: VotingProposal,
        witness: &PlutusWitness,
    ) -> Result<(), JsError> {

        self.votes.insert(
            proposal,
            Some(ScriptWitnessType::PlutusScriptWitness(witness.clone())),
        );
        Ok(())
    }

    pub fn get_plutus_witnesses(&self) -> PlutusWitnesses {
        let tag = RedeemerTag::new_voting_proposal();
        let mut scripts = PlutusWitnesses::new();
        for (i, (_, script_wit)) in self.votes.iter().enumerate() {
            if let Some(ScriptWitnessType::PlutusScriptWitness(s)) = script_wit {
                let index = BigNum::from(i);
                scripts.add(&s.clone_with_redeemer_index_and_tag(&index, &tag));
            }
        }
        scripts
    }

    pub fn get_ref_inputs(&self) -> TransactionInputs {
        let mut inputs = Vec::new();
        for (_, script_wit) in &self.votes {
            match script_wit {
                Some(ScriptWitnessType::NativeScriptWitness(script_source)) => {
                    if let NativeScriptSourceEnum::RefInput(input, _, _) = script_source {
                        inputs.push(input.clone());
                    }
                }
                Some(ScriptWitnessType::PlutusScriptWitness(plutus_witness)) => {
                    if let Some(DatumSourceEnum::RefInput(input)) = &plutus_witness.datum {
                        inputs.push(input.clone());
                    }
                    if let PlutusScriptSourceEnum::RefInput(input, _, _) = &plutus_witness.script {
                        inputs.push(input.clone());
                    }
                }
                None => {}
            }
        }
        TransactionInputs(inputs)
    }

    pub(crate) fn get_used_plutus_lang_versions(&self) -> BTreeSet<Language> {
        let mut used_langs = BTreeSet::new();
        for (_, script_wit) in &self.votes {
            if let Some(ScriptWitnessType::PlutusScriptWitness(s)) = script_wit {
                if let Some(lang) = s.script.language() {
                    used_langs.insert(lang.clone());
                }
            }
        }
        used_langs
    }

    pub fn has_plutus_scripts(&self) -> bool {
        for (_, script_wit) in &self.votes {
            if let Some(ScriptWitnessType::PlutusScriptWitness(_)) = script_wit {
                return true;
            }
        }
        false
    }

    pub fn build(&self) -> VotingProposals {
        let mut proposals = Vec::new();
        for (voter, _) in &self.votes {
            proposals.push(voter.clone());
        }
        VotingProposals(proposals)
    }
}
