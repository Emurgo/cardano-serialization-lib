use hashlink::LinkedHashMap;
use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CertificatesBuilder {
    certs: LinkedHashMap<Certificate, Option<ScriptWitnessType>>,
}

#[wasm_bindgen]
impl CertificatesBuilder {
    pub fn new() -> Self {
        Self { certs: LinkedHashMap::new() }
    }

    pub fn add(&mut self, cert: &Certificate) -> Result<(), JsError> {
        if cert.has_required_script_witness() {
            return Err(JsError::from_str(
                "Your certificate has a required script witness.\
                Please use .add_with_plutus_witness or .add_with_native_script instead.",
            ));
        }

        if self.certs.contains_key(cert) {
            return Err(JsError::from_str("Certificate already exists"));
        }

        self.certs.insert(cert.clone(), None);
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

        if self.certs.contains_key(cert) {
            return Err(JsError::from_str("Certificate already exists"));
        }

        self.certs.insert(
            cert.clone(),
            Some(ScriptWitnessType::PlutusScriptWitness(witness.clone())),
        );
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

        if self.certs.contains_key(cert) {
            return Err(JsError::from_str("Certificate already exists"));
        }

        self.certs.insert(
            cert.clone(),
            Some(ScriptWitnessType::NativeScriptWitness(
                native_script_source.0.clone(),
            )),
        );
        Ok(())
    }

    pub(crate) fn get_required_signers(&self) -> Ed25519KeyHashes {
        let mut set = Ed25519KeyHashes::new();
        for (cert, script_wit) in &self.certs {
            let cert_req_signers = witness_keys_for_cert(&cert);
            set.extend_move(cert_req_signers);
            if let Some(script_wit) = script_wit {
                if let Some(signers) = script_wit.get_required_signers() {
                    set.extend(&signers);
                }
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
        TransactionInputs::from_vec(inputs)
    }

    pub fn get_native_scripts(&self) -> NativeScripts {
        let mut scripts = NativeScripts::new();
        for (_, script_wit) in self.certs.iter() {
            if let Some(ScriptWitnessType::NativeScriptWitness(
                NativeScriptSourceEnum::NativeScript(script, _),
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
                used_langs.insert(s.script.language().clone());
            }
        }
        used_langs
    }

    #[allow(unused_variables)]
    pub fn get_certificates_refund(
        &self,
        pool_deposit: &BigNum,
        key_deposit: &BigNum,
    ) -> Result<Value, JsError> {
        let mut refund = Coin::zero();
        for (cert, _) in &self.certs {
            match &cert.0 {
                CertificateEnum::StakeDeregistration(cert) => {
                    if let Some(coin) = cert.coin {
                        refund = refund.checked_add(&coin)?;
                    } else {
                        refund = refund.checked_add(&key_deposit)?;
                    }
                }
                CertificateEnum::DrepDeregistration(cert) => {
                    refund = refund.checked_add(&cert.coin)?;
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
                CertificateEnum::PoolRegistration(_) => {
                    deposit = deposit.checked_add(&pool_deposit)?;
                }
                CertificateEnum::StakeRegistration(cert) => {
                    if let Some(coin) = cert.coin {
                        deposit = deposit.checked_add(&coin)?;
                    } else {
                        deposit = deposit.checked_add(&key_deposit)?;
                    }
                }
                CertificateEnum::DrepRegistration(cert) => {
                    deposit = deposit.checked_add(&cert.coin)?;
                }
                CertificateEnum::StakeRegistrationAndDelegation(cert) => {
                    deposit = deposit.checked_add(&cert.coin)?;
                }
                CertificateEnum::VoteRegistrationAndDelegation(cert) => {
                    deposit = deposit.checked_add(&cert.coin)?;
                }
                CertificateEnum::StakeVoteRegistrationAndDelegation(cert) => {
                    deposit = deposit.checked_add(&cert.coin)?;
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
        Certificates::from_vec(certs)
    }

    //return only ref inputs that are script refs with added size
    //used for calculating the fee for the transaction
    //another ref input and also script ref input without size are filtered out
    pub(crate) fn get_script_ref_inputs_with_size(
        &self,
    ) -> impl Iterator<Item = (&TransactionInput, usize)> {
        self.certs.iter()
            .filter_map(|(_, opt_wit)| opt_wit.as_ref())
            .filter_map(|script_wit| {
                script_wit.get_script_ref_input_with_size()
            })
    }
}

// comes from witsVKeyNeeded in the Ledger spec
fn witness_keys_for_cert(cert_enum: &Certificate) -> RequiredSigners {
    let mut set = RequiredSigners::new();
    match &cert_enum.0 {
        // stake key registrations do not require a witness
        CertificateEnum::StakeRegistration(cert) => {
            if cert.coin.is_some() {
                if let Some(key) = cert.stake_credential().to_keyhash() {
                    set.add(&key);
                }
            }
        }
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
            set.extend(&cert.pool_params().pool_owners());
            set.add(&cert.pool_params().operator());
        }
        CertificateEnum::PoolRetirement(cert) => {
            set.add(&cert.pool_keyhash());
        }
        CertificateEnum::GenesisKeyDelegation(cert) => {
            set.add(&Ed25519KeyHash::from_bytes(cert.genesis_delegate_hash().to_bytes()).unwrap());
        }
        // not witness as there is no single core node or genesis key that posts the certificate
        CertificateEnum::MoveInstantaneousRewardsCert(_cert) => {}
        CertificateEnum::CommitteeHotAuth(cert) => {
            if let CredType::Key(key_hash) = &cert.committee_cold_key.0 {
                set.add(key_hash);
            }
        }
        CertificateEnum::CommitteeColdResign(cert) => {
            if let CredType::Key(key_hash) = &cert.committee_cold_key.0 {
                set.add(key_hash);
            }
        }
        CertificateEnum::DrepUpdate(cert) => {
            if let CredType::Key(key_hash) = &cert.voting_credential.0 {
                set.add(key_hash);
            }
        }
        CertificateEnum::DrepRegistration(cert) => {
            if let CredType::Key(key_hash) = &cert.voting_credential.0 {
                set.add(key_hash);
            }
        }
        CertificateEnum::DrepDeregistration(cert) => {
            if let CredType::Key(key_hash) = &cert.voting_credential.0 {
                set.add(key_hash);
            }
        }
        CertificateEnum::StakeAndVoteDelegation(cert) => {
            if let CredType::Key(key_hash) = &cert.stake_credential.0 {
                set.add(key_hash);
            }
        }
        CertificateEnum::VoteDelegation(cert) => {
            if let CredType::Key(key_hash) = &cert.stake_credential.0 {
                set.add(key_hash);
            }
        }
        CertificateEnum::StakeRegistrationAndDelegation(cert) => {
            if let CredType::Key(key_hash) = &cert.stake_credential.0 {
                set.add(key_hash);
            }
        }
        CertificateEnum::VoteRegistrationAndDelegation(cert) => {
            if let CredType::Key(key_hash) = &cert.stake_credential.0 {
                set.add(key_hash);
            }
        }
        CertificateEnum::StakeVoteRegistrationAndDelegation(cert) => {
            if let CredType::Key(key_hash) = &cert.stake_credential.0 {
                set.add(key_hash);
            }
        }
    }
    set
}
