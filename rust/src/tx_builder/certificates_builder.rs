use crate::tx_builder::script_structs::*;
use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CertificatesBuilder {
    certs: Vec<(Certificate, Option<ScriptWitnessType>)>,
}

#[wasm_bindgen]
impl CertificatesBuilder {
    pub fn new() -> Self {
        Self { certs: Vec::new() }
    }

    pub fn add(&mut self, cert: &Certificate) -> Result<(), JsError> {
        if cert.has_required_script_witness() {
            return Err(JsError::from_str(
                "Your certificate has a required script witness.\
                Please use .add_with_plutus_witness or .add_with_native_script instead.",
            ));
        }

        self.certs.push((cert.clone(), None));
        Ok(())
    }

    pub fn add_with_plutus_witness(
        &mut self,
        cert: &Certificate,
        witness: &PlutusWitness,
    ) -> Result<(), JsError> {
        if !cert.has_required_script_witness() {
            return Err(JsError::from_str(
                "Your certificate does not have a required script witness.\
                Please use .add instead.",
            ));
        }

        self.certs.push((
            cert.clone(),
            Some(ScriptWitnessType::PlutusScriptWitness(witness.clone())),
        ));
        Ok(())
    }

    pub fn add_with_native_script(
        &mut self,
        cert: &Certificate,
        native_script_source: &NativeScriptSource,
    ) -> Result<(), JsError> {
        if !cert.has_required_script_witness() {
            return Err(JsError::from_str(
                "Your certificate does not have a required script witness.\
                Please use .add instead.",
            ));
        }

        self.certs.push((
            cert.clone(),
            Some(ScriptWitnessType::NativeScriptWitness(
                native_script_source.0.clone(),
            )),
        ));
        Ok(())
    }

    pub(crate) fn get_required_signers(&self) -> RequiredSignersSet {
        let mut set = RequiredSignersSet::new();
        for (cert, script_wit) in &self.certs {
            let cert_req_signers = witness_keys_for_cert(&cert);
            set.extend(cert_req_signers);
            if let Some(ScriptWitnessType::NativeScriptWitness(script_source)) = script_wit {
                set.extend(script_source.required_signers());
            }
        }
        set
    }

    pub fn get_plutus_witnesses(&self) -> PlutusWitnesses {
        let tag = RedeemerTag::new_cert();
        let mut scripts = PlutusWitnesses::new();
        for (i, (_, script_wit)) in self.certs.iter().enumerate() {
            if let Some(ScriptWitnessType::PlutusScriptWitness(s)) = script_wit {
                let index = BigNum::from(i);
                scripts.add(&s.clone_with_redeemer_index_and_tag(&index, &tag));
            }
        }
        scripts
    }

    pub fn get_ref_inputs(&self) -> TransactionInputs {
        let mut inputs = Vec::new();
        for (_, script_wit) in self.certs.iter() {
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
        for (_, script_wit) in self.certs.iter() {
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
        for (_, script_wit) in &self.certs {
            if let Some(ScriptWitnessType::PlutusScriptWitness(s)) = script_wit {
                if let Some(lang) = s.script.language() {
                    used_langs.insert(lang.clone());
                }
            }
        }
        used_langs
    }

    pub fn get_certificates_refund(
        &self,
        pool_deposit: &BigNum,
        key_deposit: &BigNum,
    ) -> Result<Value, JsError> {
        let mut refund = Coin::zero();
        for (cert, _) in &self.certs {
            match &cert.0 {
                CertificateEnum::StakeDeregistration(_cert) => {
                    refund = refund.checked_add(&key_deposit)?;
                }
                CertificateEnum::PoolRetirement(_cert) => {
                    refund = refund.checked_add(&pool_deposit)?;
                }
                _ => {}
            }
        }
        Ok(Value::new(&refund))
    }

    pub fn get_certificates_deposit(
        &self,
        pool_deposit: &BigNum,
        key_deposit: &BigNum,
    ) -> Result<Coin, JsError> {
        let mut deposit = Coin::zero();
        for (cert, _) in &self.certs {
            match &cert.0 {
                CertificateEnum::PoolRegistration(_cert) => {
                    deposit = deposit.checked_add(&pool_deposit)?;
                }
                CertificateEnum::StakeRegistration(_cert) => {
                    deposit = deposit.checked_add(&key_deposit)?;
                }
                _ => {}
            }
        }
        Ok(deposit)
    }

    pub fn has_plutus_scripts(&self) -> bool {
        for (_, script_wit) in &self.certs {
            if let Some(ScriptWitnessType::PlutusScriptWitness(_)) = script_wit {
                return true;
            }
        }
        false
    }

    pub fn build(&self) -> Certificates {
        let certs = self.certs.iter().map(|(c, _)| c.clone()).collect();
        Certificates(certs)
    }
}

// comes from witsVKeyNeeded in the Ledger spec
fn witness_keys_for_cert(cert_enum: &Certificate) -> RequiredSigners {
    let mut set = RequiredSigners::new();
    match &cert_enum.0 {
        // stake key registrations do not require a witness
        CertificateEnum::StakeRegistration(_cert) => {}
        CertificateEnum::StakeDeregistration(cert) => {
            if let Some(key) = cert.stake_credential().to_keyhash() {
                set.add(&key);
            }
        }
        CertificateEnum::StakeDelegation(cert) => {
            if let Some(key) = cert.stake_credential().to_keyhash() {
                set.add(&key);
            }
        }
        CertificateEnum::PoolRegistration(cert) => {
            for owner in &cert.pool_params().pool_owners().0 {
                set.add(&owner.clone());
            }
            set.add(&Ed25519KeyHash::from_bytes(cert.pool_params().operator().to_bytes()).unwrap());
        }
        CertificateEnum::PoolRetirement(cert) => {
            set.add(&Ed25519KeyHash::from_bytes(cert.pool_keyhash().to_bytes()).unwrap());
        }
        CertificateEnum::GenesisKeyDelegation(cert) => {
            set.add(&Ed25519KeyHash::from_bytes(cert.genesis_delegate_hash().to_bytes()).unwrap());
        }
        // not witness as there is no single core node or genesis key that posts the certificate
        CertificateEnum::MoveInstantaneousRewardsCert(_cert) => {}
    }
    set
}
