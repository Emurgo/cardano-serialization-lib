use super::*;
use super::fees;
use super::utils;
use std::collections::BTreeSet;

// comes from witsVKeyNeeded in the Ledger spec
fn witness_keys_for_cert(cert_enum: &Certificate, keys: &mut BTreeSet<Ed25519KeyHash>) {
    match &cert_enum.0 {
        CertificateEnum::StakeRegistration(cert) => {
            keys.insert(cert.stake_credential().to_keyhash().unwrap());
        },
        CertificateEnum::StakeDeregistration(cert) => {
            keys.insert(cert.stake_credential().to_keyhash().unwrap());
        },
        CertificateEnum::StakeDelegation(cert) => {
            keys.insert(cert.stake_credential().to_keyhash().unwrap());
        },
        CertificateEnum::PoolRegistration(cert) => {
            for owner in &cert.pool_params().pool_owners().0 {
                keys.insert(owner.clone());
            }
            keys.insert(
                Ed25519KeyHash::from_bytes(cert.pool_params().operator().to_bytes()).unwrap()
            );
        },
        CertificateEnum::PoolRetirement(cert) => {
            keys.insert(
                Ed25519KeyHash::from_bytes(cert.pool_keyhash().to_bytes()).unwrap()
            );
        },
        CertificateEnum::GenesisKeyDelegation(cert) => {
            keys.insert(
                Ed25519KeyHash::from_bytes(cert.genesis_delegate_hash().to_bytes()).unwrap()
            );
        },
        CertificateEnum::MoveInstantaneousRewardsCert(_cert) => {},
    }
}

fn estimate_fee(tx_builder: &TransactionBuilder) -> Result<Coin, JsValue> {
    let body = tx_builder.build()?;

    let fake_key_root = Bip32PrivateKey::from_bip39_entropy(
        // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
        &[0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12],
        &[]
    );

    let vkeys = match tx_builder.input_types.vkeys.len() {
        0 => None,
        x => {
            let mut result = Vkeywitnesses::new();
            let raw_key = fake_key_root.to_raw_key();
            for _i in 0..x {
                result.add(&Vkeywitness::new(
                    &Vkey::new(&raw_key.to_public()),
                    &raw_key.sign([1u8; 100].as_ref())
                ));
            }
            Some(result)
        },
    };
    let script_keys = match tx_builder.input_types.scripts.len() {
        0 => None,
        _x => {
            // TODO: figure out how to populate fake witnesses for these
            return Err(JsValue::from_str("Script inputs not supported yet"))
        },
    };
    let bootstrap_keys = match tx_builder.input_types.bootstraps.len() {
        0 => None,
        _x => {
            let mut result = BootstrapWitnesses::new();
            for addr in &tx_builder.input_types.bootstraps {
                // picking icarus over daedalus for fake witness generation shouldn't matter
                result.add(&make_icarus_bootstrap_witness(
                    &hash_transaction(&body),
                    &ByronAddress::from_bytes(addr.clone()).unwrap(),
                    &fake_key_root
                ));
            }
            Some(result)
        },
    };
    let witness_set = TransactionWitnessSet {
        vkeys: vkeys,
        scripts: script_keys,
        bootstraps: bootstrap_keys,
    };
    let full_tx = Transaction {
        body,
        witness_set,
        metadata: tx_builder.metadata.clone(),
    };
    let estimated_fee = fees::min_fee(&full_tx, &tx_builder.fee_algo);

    estimated_fee
}

// We need to know how many of each type of witness will be in the transaction so we can calculate the tx fee
#[derive(Clone, Debug)]
struct MockWitnessSet {
    vkeys: BTreeSet<Ed25519KeyHash>,
    scripts: BTreeSet<ScriptHash>,
    bootstraps: BTreeSet<Vec<u8>>,
}

#[derive(Clone, Debug)]
struct TxBuilderInput {
    input: TransactionInput,
    amount: Coin, // we need to keep track of the amount in the inputs for input selection
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilder {
    fee_algo: fees::LinearFee,
    inputs: Vec<TxBuilderInput>,
    outputs: TransactionOutputs,
    fee: Option<Coin>,
    ttl: Option<u32>, // absolute slot number
    certs: Option<Certificates>,
    withdrawals: Option<Withdrawals>,
    metadata: Option<TransactionMetadata>,
    input_types: MockWitnessSet,
}

#[wasm_bindgen]
impl TransactionBuilder {
    // We have to know what kind of inputs these are to know what kind of mock witnesses to create since
    // 1) mock witnesses have different lengths depending on the type which changes the expecting fee
    // 2) Witnesses are a set so we need to get rid of duplicates to avoid over-estimating the fee
    pub fn add_key_input(&mut self, hash: &Ed25519KeyHash, input: &TransactionInput, amount: &Coin) {
        self.inputs.push(TxBuilderInput {
            input: input.clone(),
            amount: amount.clone(),
        });
        self.input_types.vkeys.insert(hash.clone());
    }
    pub fn add_script_input(&mut self, hash: &ScriptHash, input: &TransactionInput, amount: &Coin) {
        self.inputs.push(TxBuilderInput {
            input: input.clone(),
            amount: amount.clone(),
        });
        self.input_types.scripts.insert(hash.clone());
    }
    pub fn add_bootstrap_input(&mut self, hash: &ByronAddress, input: &TransactionInput, amount: &Coin) {
        self.inputs.push(TxBuilderInput {
            input: input.clone(),
            amount: amount.clone(),
        });
        self.input_types.bootstraps.insert(hash.to_bytes());
    }

    pub fn add_output(&mut self, output: &TransactionOutput) {
        self.outputs.add(output)
    }

    pub fn set_fee(&mut self, fee: &Coin) {
        self.fee = Some(fee.clone())
    }

    pub fn set_ttl(&mut self, ttl: u32) {
        self.ttl = Some(ttl)
    }

    pub fn set_certs(&mut self, certs: &Certificates) {
        self.certs = Some(certs.clone());
        for cert in &certs.0 {
            witness_keys_for_cert(cert, &mut self.input_types.vkeys);
        };
    }

    pub fn set_withdrawals(&mut self, withdrawals: &Withdrawals) {
        self.withdrawals = Some(withdrawals.clone());
        for (withdrawal, _coin) in &withdrawals.0 {
            self.input_types.vkeys.insert(withdrawal.payment_cred().to_keyhash().unwrap().clone());
        };
    }

    pub fn set_metadata(&mut self, metadata: &TransactionMetadata) {
        self.metadata = Some(metadata.clone())
    }

    pub fn new(linear_fee: &fees::LinearFee) -> Self {
        Self {
            fee_algo: linear_fee.clone(),
            inputs: Vec::new(),
            outputs: TransactionOutputs::new(),
            fee: None,
            ttl: None,
            certs: None,
            withdrawals: None,
            metadata: None,
            input_types: MockWitnessSet {
                vkeys: BTreeSet::new(),
                scripts: BTreeSet::new(),
                bootstraps: BTreeSet::new(),
            },
        }
    }

    pub fn get_input_total(&self) -> Result<Coin, JsValue> {
        self
            .inputs
            .iter()
            .try_fold(
                Coin::new(0),
                |acc, ref tx_builder_input| acc.checked_add(&tx_builder_input.amount)
            )
    }
    pub fn get_feeless_output_total(&self) -> Result<Coin, JsValue> {
        self
            .outputs.0
            .iter()
            .try_fold(
                Coin::new(0),
                |acc, ref output| acc.checked_add(&output.amount)
            )
    }
    pub fn get_fee_or_calc(&self) -> Result<Coin, JsValue> {
        match &self.fee {
            None => self.estimate_fee(),
            Some(x) => Ok(x.clone()),
        }
    }

    pub fn add_change_if_needed(&mut self, address: &Address) -> Result<bool, JsValue> {
        let fee = match &self.fee {
            None => self.estimate_fee(),
            // generating the change output involves changing the fee
            Some(_x) => return Err(JsValue::from_str("Cannot calculate change if fee was explicitly specified")),
        }?;
        let input_total = self.get_input_total()?;
        let output_total = self.get_feeless_output_total()?;
        match input_total > output_total.checked_add(&fee)? {
            false => return Ok(false),
            true => {
                let mut copy = self.clone();
                copy.add_output(&TransactionOutput {
                    address: address.clone(),
                    // maximum possible output to maximize fee from adding this output
                    // this may over-estimate the fee by a few bytes but that's okay
                    amount: Coin::new(0x1_00_00_00_00),
                });
                let new_fee = copy.estimate_fee()?;
                match input_total > output_total.checked_add(&new_fee)? {
                    false => return Ok(false), // not enough input to covert the extra fee from adding an output
                    true => {
                        self.set_fee(&new_fee);
                        self.add_output(&TransactionOutput {
                            address: address.clone(),
                            amount: input_total.checked_sub(&output_total.checked_add(&new_fee)?)?,
                        });
                    }
                };
            },
        };
        Ok(true)
    }

    pub fn build(&self) -> Result<TransactionBody, JsValue> {
        let fee = self.fee.ok_or_else(|| JsValue::from_str("Fee not specified"))?;
        let ttl = self.ttl.ok_or_else(|| JsValue::from_str("ttl not specified"))?;
        Ok(TransactionBody {
            inputs: TransactionInputs(self.inputs.iter().map(|ref tx_builder_input| tx_builder_input.input.clone()).collect()),
            outputs: self.outputs.clone(),
            fee: fee,
            ttl: ttl,
            certs: self.certs.clone(),
            withdrawals: self.withdrawals.clone(),
            metadata_hash: match &self.metadata {
                None => None,
                Some(x) => Some(utils::hash_metadata(x)),
            },
        })
    }

    pub fn estimate_fee(&self) -> Result<Coin, JsValue> {
        match self.fee {
            // if user explicitly specified a fee already, use that one
            Some(_x) => estimate_fee(&self),
            // otherwise, use the maximum fee possible
            None => {
                let mut self_copy = self.clone();
                self_copy.set_fee(&Coin::new(0x1_00_00_00_00));
                estimate_fee(&self_copy)
            },
        }
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fees::*;

    fn genesis_id() -> TransactionHash {
        TransactionHash::from([0u8; TransactionHash::BYTE_COUNT])
    }

    fn root_key_15() -> Bip32PrivateKey {
        // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
        let entropy = [0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12];
        Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
    }

    fn harden(index: u32) -> u32 {
        index | 0x80_00_00_00
    }

    #[test]
    fn build_tx() {
        let linear_fee = LinearFee::new(&Coin::new(500), &Coin::new(2));
        let mut tx_builder = TransactionBuilder::new(&linear_fee);
        let spend = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let stake = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();

        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(0, &spend_cred, &stake_cred).to_address();
        tx_builder.add_key_input(
            &spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Coin::new(1_000_000)
        );
        tx_builder.add_output(&TransactionOutput::new(
            &addr_net_0,
            &Coin::new(10)
        ));
        tx_builder.set_ttl(1000);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(0, &change_cred, &stake_cred).to_address();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr
        );
        assert!(added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 2);
        assert_eq!(
            tx_builder.get_input_total().unwrap(),
            tx_builder.get_feeless_output_total().unwrap().checked_add(&tx_builder.get_fee_or_calc().unwrap()).unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }
}
