use crate::serialization_tools::map_names::WitnessSetNames;
use crate::tx_builder::batch_tools::cbor_calculator::CborCalculator;
use super::super::*;

#[derive(Clone)]
pub(super) struct WitnessesCalculator {
    adresses: BTreeSet<Address>,
    vkeys_count: u64,
    boostrap_count: u64,
    bootsraps: Vec<ByronAddress>,
    used_fields: HashSet<WitnessSetNames>,
    total_size: usize,
}

impl WitnessesCalculator {
    pub(super) fn new() -> Self {
        Self {
            adresses: BTreeSet::new(),
            vkeys_count: 0,
            boostrap_count: 0,
            bootsraps: Vec::new(),
            used_fields: HashSet::new(),
            total_size: 0,
        }
    }

    pub(super) fn add_address(&mut self, address: &Address) -> Result<(), JsError> {
        if self.adresses.contains(address) {
            return Ok(());
        }

        self.adresses.insert(address.clone());

        match &BaseAddress::from_address(address) {
            Some(addr) => {
                match &addr.payment_cred().to_keyhash() {
                    Some(_) => self.add_vkey(),
                    None => (),
                }
                match &addr.payment_cred().to_scripthash() {
                    Some(_) => return Err(JsError::from_str("Script input is not supported for send all")),
                    None => ()
                }
            }
            None => ()
        }
        match &EnterpriseAddress::from_address(address) {
            Some(addr) => {
                match &addr.payment_cred().to_keyhash() {
                    Some(_) => self.add_vkey(),
                    None => (),
                }
                match &addr.payment_cred().to_scripthash() {
                    Some(_) => return Err(JsError::from_str("Script input is not supported for send all")),
                    None => ()
                }
            }
            None => (),
        }
        match &PointerAddress::from_address(address) {
            Some(addr) => {
                match &addr.payment_cred().to_keyhash() {
                    Some(_) => self.add_vkey(),
                    None => (),
                }
                match &addr.payment_cred().to_scripthash() {
                    Some(_) => return Err(JsError::from_str("Script input is not supported for send all")),
                    None => ()
                }
            }
            None => (),
        }
        match &ByronAddress::from_address(address) {
            Some(addr) => self.add_boostrap(addr),
            None => (),
        }

        Ok(())
    }

    pub(super) fn get_full_size(&self) -> usize {
        self.total_size
    }

    pub(super) fn create_mock_witnesses_set(&self) -> TransactionWitnessSet {
        let fake_key_root = fake_private_key();
        let raw_key_public = fake_raw_key_public();
        let fake_sig = fake_raw_key_sig();

        // recall: this includes keys for input, certs and withdrawals
        let vkeys = match self.vkeys_count {
            0 => None,
            x => {
                let fake_vkey_witness = Vkeywitness::new(&Vkey::new(&raw_key_public), &fake_sig);
                let mut result = Vkeywitnesses::new();
                for _i in 0..x {
                    result.add(&fake_vkey_witness.clone());
                }
                Some(result)
            }
        };

        let bootstrap_keys = match self.boostrap_count {
            0 => None,
            _x => {
                let mut result = BootstrapWitnesses::new();
                for boostrap_address in &self.bootsraps {
                    // picking icarus over daedalus for fake witness generation shouldn't matter
                    result.add(&make_icarus_bootstrap_witness(
                        &TransactionHash::from([0u8; TransactionHash::BYTE_COUNT]),
                        boostrap_address,
                        &fake_key_root,
                    ));
                }
                Some(result)
            }
        };

        TransactionWitnessSet {
            vkeys,
            native_scripts: None,
            bootstraps: bootstrap_keys,
            plutus_scripts: None,
            plutus_data: None,
            redeemers: None,
        }
    }

    fn add_vkey(&mut self) {
        if self.vkeys_count == 0 {
            if self.used_fields.len() > 0 {
                self.total_size -= CborCalculator::get_wintnesses_set_struct_size(&self.used_fields);
            }

            self.used_fields.insert(WitnessSetNames::Vkeys);
            self.total_size += CborCalculator::get_wintnesses_set_struct_size(&self.used_fields);
        }

        if self.vkeys_count != 0 {
            self.total_size -= CborCalculator::get_struct_size(self.vkeys_count);
        }
        self.vkeys_count += 1;
        self.total_size += CborCalculator::get_struct_size(self.vkeys_count);
        self.total_size += CborCalculator::get_fake_vkey_size();
    }

    fn add_boostrap(&mut self, address: &ByronAddress) {
        self.bootsraps.push(address.clone());
        if self.boostrap_count == 0 {
            if self.used_fields.len() > 0 {
                self.total_size -= CborCalculator::get_wintnesses_set_struct_size(&self.used_fields);
            }

            self.used_fields.insert(WitnessSetNames::Bootstraps);
            self.total_size += CborCalculator::get_wintnesses_set_struct_size(&self.used_fields);
        }

        if self.boostrap_count != 0 {
            self.total_size -= CborCalculator::get_struct_size(self.boostrap_count);
        }
        self.boostrap_count += 1;
        self.total_size += CborCalculator::get_struct_size(self.boostrap_count);
        self.total_size += CborCalculator::get_boostrap_witness_size(address);
    }
}