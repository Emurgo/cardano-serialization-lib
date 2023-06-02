use crate::tx_builder::script_structs::*;
use crate::*;
use linked_hash_map::LinkedHashMap;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WithdrawalsBuilder {
    withdrawals: LinkedHashMap<RewardAddress, (Coin, Option<ScriptWitnessType>)>,
}

#[wasm_bindgen]
impl WithdrawalsBuilder {
    pub fn new() -> Self {
        Self {
            withdrawals: LinkedHashMap::new(),
        }
    }

    pub fn add(&mut self, address: &RewardAddress, coin: &Coin) -> Result<(), JsError> {
        if address.payment_cred().has_script_hash() {
            return Err(JsError::from_str(
                "Your address has a required script witness.\
                Please use .add_with_plutus_witness or .add_with_native_script instead.",
            ));
        }

        self.withdrawals
            .insert(address.clone(), (coin.clone(), None));

        Ok(())
    }

    pub fn add_with_plutus_witness(
        &mut self,
        address: &RewardAddress,
        coin: &Coin,
        witness: &PlutusWitness,
    ) -> Result<(), JsError> {
        if !address.payment_cred().has_script_hash() {
            return Err(JsError::from_str(
                "Your address does not have a required script witness.\
                Please use .add instead.",
            ));
        }

        self.withdrawals.insert(
            address.clone(),
            (
                coin.clone(),
                Some(ScriptWitnessType::PlutusScriptWitness(witness.clone())),
            ),
        );
        Ok(())
    }

    pub fn add_with_native_script(
        &mut self,
        address: &RewardAddress,
        coin: &Coin,
        native_script_source: &NativeScriptSource,
    ) -> Result<(), JsError> {
        if !address.payment_cred().has_script_hash() {
            return Err(JsError::from_str(
                "Your address does not have a required script witness.\
                Please use .add instead.",
            ));
        }

        self.withdrawals.insert(
            address.clone(),
            (
                coin.clone(),
                Some(ScriptWitnessType::NativeScriptWitness(
                    native_script_source.0.clone(),
                )),
            ),
        );
        Ok(())
    }

    pub(crate) fn get_required_signers(&self) -> RequiredSignersSet {
        let mut set = RequiredSignersSet::new();
        for (address, (_, script_wit)) in &self.withdrawals {
            let req_signature = address.payment_cred().to_keyhash();
            if let Some(req_signature) = req_signature {
                set.insert(req_signature);
            }

            if let Some(ScriptWitnessType::NativeScriptWitness(script_source)) = script_wit {
                set.extend(script_source.required_signers());
            }
        }
        set
    }

    pub fn get_plutus_witnesses(&self) -> PlutusWitnesses {
        let tag = RedeemerTag::new_reward();
        let mut scripts = PlutusWitnesses::new();
        for (i, (_, (_, script_wit))) in self.withdrawals.iter().enumerate() {
            if let Some(ScriptWitnessType::PlutusScriptWitness(s)) = script_wit {
                let index = BigNum::from(i);
                scripts.add(&s.clone_with_redeemer_index_and_tag(&index, &tag));
            }
        }
        scripts
    }

    pub fn get_ref_inputs(&self) -> TransactionInputs {
        let mut inputs = Vec::new();
        for (_, (_, script_wit)) in self.withdrawals.iter() {
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

    pub fn get_native_scripts(&self) -> NativeScripts {
        let mut scripts = NativeScripts::new();
        for (_, (_, script_wit)) in self.withdrawals.iter() {
            if let Some(ScriptWitnessType::NativeScriptWitness(
                NativeScriptSourceEnum::NativeScript(script),
            )) = script_wit
            {
                scripts.add(script);
            }
        }
        scripts
    }

    pub(crate) fn get_used_plutus_lang_versions(&self) -> BTreeSet<Language> {
        let mut used_langs = BTreeSet::new();
        for (_, (_, script_wit)) in &self.withdrawals {
            if let Some(ScriptWitnessType::PlutusScriptWitness(s)) = script_wit {
                if let Some(lang) = s.script.language() {
                    used_langs.insert(lang.clone());
                }
            }
        }
        used_langs
    }

    pub fn get_total_withdrawals(&self) -> Result<Value, JsError> {
        let mut total = Coin::zero();
        for (_, (coin, _)) in &self.withdrawals {
            total = total.checked_add(coin)?;
        }
        Ok(Value::new(&total))
    }

    pub fn has_plutus_scripts(&self) -> bool {
        for (_, (_, script_wit)) in &self.withdrawals {
            if let Some(ScriptWitnessType::PlutusScriptWitness(_)) = script_wit {
                return true;
            }
        }
        false
    }

    pub fn build(&self) -> Withdrawals {
        let map = self
            .withdrawals
            .iter()
            .map(|(k, (v, _))| (k.clone(), v.clone()))
            .collect();
        Withdrawals(map)
    }
}
