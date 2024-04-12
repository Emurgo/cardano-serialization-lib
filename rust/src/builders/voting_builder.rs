use crate::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct VoterVotes {
    script_witness: Option<ScriptWitnessType>,
    votes: BTreeMap<GovernanceActionId, VotingProcedure>,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VotingBuilder {
    votes: BTreeMap<Voter, VoterVotes>,
}

#[wasm_bindgen]
impl VotingBuilder {
    pub fn new() -> Self {
        Self {
            votes: BTreeMap::new(),
        }
    }

    pub fn add(
        &mut self,
        voter: &Voter,
        gov_action_id: &GovernanceActionId,
        voting_procedure: &VotingProcedure,
    ) -> Result<(), JsError> {
        if voter.has_script_credentials() {
            return Err(JsError::from_str(
                "Your voter has a required script witness.\
                Please use .add_with_plutus_witness or .add_with_native_script instead.",
            ));
        }

        let voter_votes = self.votes.entry(voter.clone()).or_insert(VoterVotes {
            script_witness: None,
            votes: BTreeMap::new(),
        });

        voter_votes
            .votes
            .insert(gov_action_id.clone(), voting_procedure.clone());

        Ok(())
    }

    pub fn add_with_plutus_witness(
        &mut self,
        voter: &Voter,
        gov_action_id: &GovernanceActionId,
        voting_procedure: &VotingProcedure,
        witness: &PlutusWitness,
    ) -> Result<(), JsError> {
        if !voter.has_script_credentials() {
            return Err(JsError::from_str(
                "Your voter does not have a required script witness.\
                Please use .add instead.",
            ));
        }

        let voter_votes = self.votes.entry(voter.clone()).or_insert(VoterVotes {
            script_witness: Some(ScriptWitnessType::PlutusScriptWitness(witness.clone())),
            votes: BTreeMap::new(),
        });

        voter_votes
            .votes
            .insert(gov_action_id.clone(), voting_procedure.clone());

        Ok(())
    }

    pub fn add_with_native_script(
        &mut self,
        voter: &Voter,
        gov_action_id: &GovernanceActionId,
        voting_procedure: &VotingProcedure,
        native_script_source: &NativeScriptSource,
    ) -> Result<(), JsError> {
        if !voter.has_script_credentials() {
            return Err(JsError::from_str(
                "Your voter does not have a required script witness.\
                Please use .add instead.",
            ));
        }

        let voter_votes = self.votes.entry(voter.clone()).or_insert(VoterVotes {
            script_witness: Some(ScriptWitnessType::NativeScriptWitness(
                native_script_source.0.clone(),
            )),
            votes: BTreeMap::new(),
        });

        voter_votes
            .votes
            .insert(gov_action_id.clone(), voting_procedure.clone());

        Ok(())
    }

    pub(crate) fn get_required_signers(&self) -> Ed25519KeyHashes {
        let mut set = Ed25519KeyHashes::new();
        for (voter, voter_votes) in &self.votes {
            let req_signature = voter.to_key_hash();
            if let Some(req_signature) = req_signature {
                set.add_move(req_signature);
            }

            if let Some(ScriptWitnessType::NativeScriptWitness(script_source)) =
                &voter_votes.script_witness
            {
                if let Some(required_signers) = script_source.required_signers() {
                    set.extend_move(required_signers);
                }
            }
        }
        set
    }

    pub fn get_plutus_witnesses(&self) -> PlutusWitnesses {
        let tag = RedeemerTag::new_vote();
        let mut scripts = PlutusWitnesses::new();
        for (i, (_, voter_votes)) in self.votes.iter().enumerate() {
            if let Some(ScriptWitnessType::PlutusScriptWitness(s)) = &voter_votes.script_witness {
                let index = BigNum::from(i);
                scripts.add(&s.clone_with_redeemer_index_and_tag(&index, &tag));
            }
        }
        scripts
    }

    pub fn get_ref_inputs(&self) -> TransactionInputs {
        let mut inputs = Vec::new();
        for (_, voter_votes) in &self.votes {
            match &voter_votes.script_witness {
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

    pub fn get_native_scripts(&self) -> NativeScripts {
        let mut scripts = NativeScripts::new();
        for (_, voter_votes) in &self.votes {
            if let Some(ScriptWitnessType::NativeScriptWitness(
                NativeScriptSourceEnum::NativeScript(script, _),
            )) = &voter_votes.script_witness
            {
                scripts.add(script);
            }
        }
        scripts
    }

    pub(crate) fn get_used_plutus_lang_versions(&self) -> BTreeSet<Language> {
        let mut used_langs = BTreeSet::new();
        for (_, voter_votes) in &self.votes {
            if let Some(ScriptWitnessType::PlutusScriptWitness(s)) = &voter_votes.script_witness {
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
        self.votes.iter()
            .filter_map(|(_, voter_votes)| voter_votes.script_witness.as_ref())
            .filter_map(|script_witness| script_witness.get_script_ref_input_with_size())
    }

    pub fn has_plutus_scripts(&self) -> bool {
        for (_, voter_votes) in &self.votes {
            if let Some(ScriptWitnessType::PlutusScriptWitness(_)) = voter_votes.script_witness {
                return true;
            }
        }
        false
    }

    pub fn build(&self) -> VotingProcedures {
        let mut voters = BTreeMap::new();
        for (voter, voter_votes) in &self.votes {
            let mut votes = BTreeMap::new();
            for (action, voting_procedure) in &voter_votes.votes {
                votes.insert(action.clone(), voting_procedure.clone());
            }
            voters.insert(voter.clone(), votes);
        }
        VotingProcedures(voters)
    }
}
