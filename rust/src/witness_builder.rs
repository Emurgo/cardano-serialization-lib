use std::collections::HashMap;
use super::*;

/// Builder de-duplicates witnesses as they are added 
#[wasm_bindgen]
#[derive(Clone)]
pub struct TransactionWitnessSetBuilder {
    // See Alonzo spec section 3.1 which defines the keys for these types
    vkeys: HashMap<Vkey, Vkeywitness>,
    bootstraps: HashMap<Vkey, BootstrapWitness>,
    native_scripts: HashMap<ScriptHash, NativeScript>,
    plutus_scripts: HashMap<ScriptHash, PlutusScript>,
    plutus_data: HashMap<DataHash, PlutusData>,
    redeemers: HashMap<RedeemerTag, Redeemer>,
}

#[wasm_bindgen]
impl TransactionWitnessSetBuilder {
    pub fn add_vkey(&mut self, vkey: &Vkeywitness) {
        self.vkeys.insert(vkey.vkey(), vkey.clone());
    }

    pub fn add_bootstrap(&mut self, bootstrap: &BootstrapWitness) {
        self.bootstraps.insert(bootstrap.vkey(), bootstrap.clone());
    }

    pub fn add_native_script(&mut self, native_script: &NativeScript) {
        self.native_scripts.insert(native_script.hash(ScriptHashNamespace::NativeScript), native_script.clone());
    }

    pub fn add_plutus_script(&mut self, plutus_script: &PlutusScript) {
        // TODO: don't assume PlutusV1 and instead somehow calculate this
        self.plutus_scripts.insert(plutus_script.hash(ScriptHashNamespace::PlutusV1), plutus_script.clone());
    }

    pub fn add_plutus_datum(&mut self, plutus_datum: &PlutusData) {
        self.plutus_data.insert(hash_plutus_data(&plutus_datum), plutus_datum.clone());
    }

    pub fn add_redeemer(&mut self, redeemer: &Redeemer) {
        self.redeemers.insert(redeemer.tag().clone(), redeemer.clone());
    }

    pub fn new() -> Self {
        Self {
            vkeys: HashMap::new(),
            bootstraps: HashMap::new(),
            native_scripts: HashMap::new(),
            plutus_scripts: HashMap::new(),
            plutus_data: HashMap::new(),
            redeemers: HashMap::new(),
        }
    }

    pub fn from_existing(wit_set: &TransactionWitnessSet) -> Self {
        let mut builder = TransactionWitnessSetBuilder::new();
        match &wit_set.vkeys() {
            None => (),
            Some(vkeys) => vkeys.0.iter().for_each(|vkey| { builder.add_vkey(vkey); } ),
        };
        match &wit_set.bootstraps() {
            None => (),
            Some(bootstraps) => bootstraps.0.iter().for_each(|bootstrap| { builder.add_bootstrap(bootstrap); } ),
        };
        match &wit_set.native_scripts() {
            None => (),
            Some(native_scripts) => native_scripts.0.iter().for_each(|native_script| { builder.add_native_script(native_script); } ),
        };
        match &wit_set.plutus_scripts() {
            None => (),
            Some(plutus_scripts) => plutus_scripts.0.iter().for_each(|plutus_script| { builder.add_plutus_script(plutus_script); } ),
        };
        match &wit_set.plutus_data() {
            None => (),
            Some(plutus_data) => plutus_data.0.iter().for_each(|plutus_datum| { builder.add_plutus_datum(plutus_datum); } ),
        };
        match &wit_set.redeemers() {
            None => (),
            Some(redeemers) => redeemers.0.iter().for_each(|redeemer| { builder.add_redeemer(redeemer); } ),
        };

        builder
    }

    pub fn build(&self) -> TransactionWitnessSet {
        let mut result = TransactionWitnessSet::new();
        
        if self.vkeys.len() > 0 {
            result.set_vkeys(&Vkeywitnesses(self.vkeys.values().cloned().collect()));
        }
        if self.bootstraps.len() > 0 {
            result.set_bootstraps(&BootstrapWitnesses(self.bootstraps.values().cloned().collect()));
        }
        if self.native_scripts.len() > 0 {
            result.set_native_scripts(&NativeScripts(self.native_scripts.values().cloned().collect()));
        }
        if self.plutus_scripts.len() > 0 {
            result.set_plutus_scripts(&PlutusScripts(self.plutus_scripts.values().cloned().collect()));
        }
        if self.plutus_data.len() > 0 {
            result.set_plutus_data(&PlutusList(self.plutus_data.values().cloned().collect()));
        }
        if self.redeemers.len() > 0 {
            result.set_redeemers(&Redeemers(self.redeemers.values().cloned().collect()));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_raw_key_sig(id: u8) -> Ed25519Signature {
        Ed25519Signature::from_bytes(
            vec![id, 248, 153, 211, 155, 23, 253, 93, 102, 193, 146, 196, 181, 13, 52, 62, 66, 247, 35, 91, 48, 80, 76, 138, 231, 97, 159, 147, 200, 40, 220, 109, 206, 69, 104, 221, 105, 23, 124, 85, 24, 40, 73, 45, 119, 122, 103, 39, 253, 102, 194, 251, 204, 189, 168, 194, 174, 237, 146, 3, 44, 153, 121, 10]
        ).unwrap()
    }
    
    fn fake_raw_key_public(id: u8) -> PublicKey {
        PublicKey::from_bytes(
            &[id, 118, 57, 154, 33, 13, 232, 114, 14, 159, 168, 148, 228, 94, 65, 226, 154, 181, 37, 227, 11, 196, 2, 128, 28, 7, 98, 80, 209, 88, 91, 205]
        ).unwrap()
    }

    fn fake_private_key1() -> Bip32PrivateKey {
        Bip32PrivateKey::from_bytes(
            &[0xb8, 0xf2, 0xbe, 0xce, 0x9b, 0xdf, 0xe2, 0xb0, 0x28, 0x2f, 0x5b, 0xad, 0x70, 0x55, 0x62, 0xac, 0x99, 0x6e, 0xfb, 0x6a, 0xf9, 0x6b, 0x64, 0x8f,
                0x44, 0x45, 0xec, 0x44, 0xf4, 0x7a, 0xd9, 0x5c, 0x10, 0xe3, 0xd7, 0x2f, 0x26, 0xed, 0x07, 0x54, 0x22, 0xa3, 0x6e, 0xd8, 0x58, 0x5c, 0x74, 0x5a,
                0x0e, 0x11, 0x50, 0xbc, 0xce, 0xba, 0x23, 0x57, 0xd0, 0x58, 0x63, 0x69, 0x91, 0xf3, 0x8a, 0x37, 0x91, 0xe2, 0x48, 0xde, 0x50, 0x9c, 0x07, 0x0d,
                0x81, 0x2a, 0xb2, 0xfd, 0xa5, 0x78, 0x60, 0xac, 0x87, 0x6b, 0xc4, 0x89, 0x19, 0x2c, 0x1e, 0xf4, 0xce, 0x25, 0x3c, 0x19, 0x7e, 0xe2, 0x19, 0xa4]
        ).unwrap()
    }

    fn fake_private_key2() -> Bip32PrivateKey {
        Bip32PrivateKey::from_bytes(
            &hex::decode("d84c65426109a36edda5375ea67f1b738e1dacf8629f2bb5a2b0b20f3cd5075873bf5cdfa7e533482677219ac7d639e30a38e2e645ea9140855f44ff09e60c52c8b95d0d35fe75a70f9f5633a3e2439b2994b9e2bc851c49e9f91d1a5dcbb1a3").unwrap()
        ).unwrap()
    }
    

    #[test]
    fn vkey_test() {
        let mut builder = TransactionWitnessSetBuilder::new();
        
        let raw_key_public = fake_raw_key_public(0);
        let fake_sig = fake_raw_key_sig(0);

        // add the same element twice
        builder.add_vkey(&Vkeywitness::new(
            &Vkey::new(&raw_key_public),
            &fake_sig
        ));
        builder.add_vkey(&Vkeywitness::new(
            &Vkey::new(&raw_key_public),
            &fake_sig
        ));

        // add a different element
        builder.add_vkey(&Vkeywitness::new(
            &Vkey::new(&fake_raw_key_public(1)),
            &fake_raw_key_sig(1)
        ));

        let wit_set = builder.build();
        assert_eq!(
            wit_set.vkeys().unwrap().len(),
            2
        );
    }

    #[test]
    fn bootstrap_test() {
        let mut builder = TransactionWitnessSetBuilder::new();

        // add the same element twice
        let wit1 = make_icarus_bootstrap_witness(
            &TransactionHash::from([0u8; TransactionHash::BYTE_COUNT]),
            &ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap(),
            &fake_private_key1()
        );
        builder.add_bootstrap(&wit1);
        builder.add_bootstrap(&wit1);

        // add a different element
        builder.add_bootstrap(&make_icarus_bootstrap_witness(
            &TransactionHash::from([0u8; TransactionHash::BYTE_COUNT]),
            &ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap(),
            &fake_private_key2()
        ));

        let wit_set = builder.build();
        assert_eq!(
            wit_set.bootstraps().unwrap().len(),
            2
        );
    }

    #[test]
    fn native_script_test() {
        let mut builder = TransactionWitnessSetBuilder::new();

        // add the same element twice
        let wit1 = NativeScript::new_timelock_start(
            &TimelockStart::new(1),
        );
        builder.add_native_script(&wit1);
        builder.add_native_script(&wit1);

        // add a different element
        builder.add_native_script(&NativeScript::new_timelock_start(
            &TimelockStart::new(2),
        ));

        let wit_set = builder.build();
        assert_eq!(
            wit_set.native_scripts().unwrap().len(),
            2
        );
    }

    // TODO: tests for plutus_scripts, plutus_data, redeemers
    // once we have mock data for them
}