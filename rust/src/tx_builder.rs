use super::*;
use super::fees;
use super::utils;
use std::collections::{BTreeMap, BTreeSet, HashSet};

// comes from witsVKeyNeeded in the Ledger spec
fn witness_keys_for_cert(cert_enum: &Certificate, keys: &mut BTreeSet<Ed25519KeyHash>) {
    match &cert_enum.0 {
        // stake key registrations do not require a witness
        CertificateEnum::StakeRegistration(_cert) => {},
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
        // not witness as there is no single core node or genesis key that posts the certificate
        CertificateEnum::MoveInstantaneousRewardsCert(_cert) => {},
    }
}

fn fake_private_key() -> Bip32PrivateKey {
    Bip32PrivateKey::from_bytes(
        &[0xb8, 0xf2, 0xbe, 0xce, 0x9b, 0xdf, 0xe2, 0xb0, 0x28, 0x2f, 0x5b, 0xad, 0x70, 0x55, 0x62, 0xac, 0x99, 0x6e, 0xfb, 0x6a, 0xf9, 0x6b, 0x64, 0x8f,
            0x44, 0x45, 0xec, 0x44, 0xf4, 0x7a, 0xd9, 0x5c, 0x10, 0xe3, 0xd7, 0x2f, 0x26, 0xed, 0x07, 0x54, 0x22, 0xa3, 0x6e, 0xd8, 0x58, 0x5c, 0x74, 0x5a,
            0x0e, 0x11, 0x50, 0xbc, 0xce, 0xba, 0x23, 0x57, 0xd0, 0x58, 0x63, 0x69, 0x91, 0xf3, 0x8a, 0x37, 0x91, 0xe2, 0x48, 0xde, 0x50, 0x9c, 0x07, 0x0d,
            0x81, 0x2a, 0xb2, 0xfd, 0xa5, 0x78, 0x60, 0xac, 0x87, 0x6b, 0xc4, 0x89, 0x19, 0x2c, 0x1e, 0xf4, 0xce, 0x25, 0x3c, 0x19, 0x7e, 0xe2, 0x19, 0xa4]
    ).unwrap()
}

fn fake_key_hash(x: u8) -> Ed25519KeyHash {
    Ed25519KeyHash::from_bytes(
        vec![x, 239, 181, 120, 142, 135, 19, 200, 68, 223, 211, 43, 46, 145, 222, 30, 48, 159, 239, 255, 213, 85, 248, 39, 204, 158, 225, 100]
    ).unwrap()
}

// tx_body must be the result of building from tx_builder
// constructs the rest of the Transaction using fake witness data of the correct length
// for use in calculating the size of the final Transaction
fn fake_full_tx(tx_builder: &TransactionBuilder, body: TransactionBody) -> Result<Transaction, JsError> {
    let fake_key_root = fake_private_key();
    // recall: this includes keys for input, certs and withdrawals
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
    let script_keys: Option<NativeScripts> = match tx_builder.input_types.scripts.len() {
        0 => None,
        _x => {
            // TODO: figure out how to populate fake witnesses for these
            return Err(JsError::from_str("Script inputs not supported yet"))
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
    let full_script_keys = match &tx_builder.mint_scripts {
        None => script_keys,
        Some(witness_scripts) => {
            let mut ns = script_keys
                .map(|sk| { sk.clone() })
                .unwrap_or(NativeScripts::new());
            witness_scripts.0.iter().for_each(|s| { ns.add(s); });
            Some(ns)
        }
    };
    let witness_set = TransactionWitnessSet {
        vkeys: vkeys,
        native_scripts: full_script_keys,
        bootstraps: bootstrap_keys,
        // TODO: plutus support?
        plutus_scripts: None,
        plutus_data: None,
        redeemers: None,
    };
    Ok(Transaction {
        body,
        witness_set,
        is_valid: true,
        auxiliary_data: tx_builder.auxiliary_data.clone(),
    })
}

fn min_fee(tx_builder: &TransactionBuilder) -> Result<Coin, JsError> {
    if let Some(mint) = tx_builder.mint.as_ref() {
        if let Some(witness_scripts) = tx_builder.mint_scripts.as_ref() {
            let witness_hashes: HashSet<ScriptHash> = witness_scripts.0.iter().map(|script| {
                script.hash(ScriptHashNamespace::NativeScript)
            }).collect();
            for mint_hash in mint.keys().0.iter() {
                if !witness_hashes.contains(mint_hash) {
                    return Err(JsError::from_str(&format!("No witness script is found for mint policy '{:?}'! Impossible to estimate fee", hex::encode(mint_hash.to_bytes()))));
                }
            }
        } else {
            return Err(JsError::from_str("Impossible to estimate fee if mint is present in the builder, but witness scripts are not provided!"));
        }
    }
    let full_tx = fake_full_tx(tx_builder, tx_builder.build()?)?;
    fees::min_fee(&full_tx, &tx_builder.config.fee_algo)
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
    amount: Value, // we need to keep track of the amount in the inputs for input selection
}

#[wasm_bindgen]
pub enum CoinSelectionStrategyCIP2 {
    LargestFirst,
    RandomImprove,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilderConfig {
    fee_algo: fees::LinearFee,
    pool_deposit: BigNum,      // protocol parameter
    key_deposit: BigNum,       // protocol parameter
    max_value_size: u32,       // protocol parameter
    max_tx_size: u32,          // protocol parameter
    coins_per_utxo_word: Coin, // protocol parameter
    prefer_pure_change: bool,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilderConfigBuilder {
    fee_algo: Option<fees::LinearFee>,
    pool_deposit: Option<BigNum>,      // protocol parameter
    key_deposit: Option<BigNum>,       // protocol parameter
    max_value_size: Option<u32>,       // protocol parameter
    max_tx_size: Option<u32>,          // protocol parameter
    coins_per_utxo_word: Option<Coin>, // protocol parameter
    prefer_pure_change: bool,
}

#[wasm_bindgen]
impl TransactionBuilderConfigBuilder {
    pub fn new() -> Self {
        Self {
            fee_algo: None,
            pool_deposit: None,
            key_deposit: None,
            max_value_size: None,
            max_tx_size: None,
            coins_per_utxo_word: None,
            prefer_pure_change: false,
        }
    }

    pub fn fee_algo(&self, fee_algo: &fees::LinearFee) -> Self {
        let mut cfg = self.clone();
        cfg.fee_algo = Some(fee_algo.clone());
        cfg
    }

    pub fn coins_per_utxo_word(&self, coins_per_utxo_word: &Coin) -> Self {
        let mut cfg = self.clone();
        cfg.coins_per_utxo_word = Some(coins_per_utxo_word.clone());
        cfg
    }

    pub fn pool_deposit(&self, pool_deposit: &BigNum) -> Self {
        let mut cfg = self.clone();
        cfg.pool_deposit = Some(pool_deposit.clone());
        cfg
    }

    pub fn key_deposit(&self, key_deposit: &BigNum) -> Self {
        let mut cfg = self.clone();
        cfg.key_deposit = Some(key_deposit.clone());
        cfg
    }

    pub fn max_value_size(&self, max_value_size: u32) -> Self {
        let mut cfg = self.clone();
        cfg.max_value_size = Some(max_value_size);
        cfg
    }

    pub fn max_tx_size(&self, max_tx_size: u32) -> Self {
        let mut cfg = self.clone();
        cfg.max_tx_size = Some(max_tx_size);
        cfg
    }

    pub fn prefer_pure_change(&self, prefer_pure_change: bool) -> Self {
        let mut cfg = self.clone();
        cfg.prefer_pure_change = prefer_pure_change;
        cfg
    }

    pub fn build(&self) -> Result<TransactionBuilderConfig, JsError> {
        let cfg = self.clone();
        Ok(TransactionBuilderConfig {
            fee_algo: cfg.fee_algo.ok_or(JsError::from_str("uninitialized field: fee_algo"))?,
            pool_deposit: cfg.pool_deposit.ok_or(JsError::from_str("uninitialized field: pool_deposit"))?,
            key_deposit: cfg.key_deposit.ok_or(JsError::from_str("uninitialized field: key_deposit"))?,
            max_value_size: cfg.max_value_size.ok_or(JsError::from_str("uninitialized field: max_value_size"))?,
            max_tx_size: cfg.max_tx_size.ok_or(JsError::from_str("uninitialized field: max_tx_size"))?,
            coins_per_utxo_word: cfg.coins_per_utxo_word.ok_or(JsError::from_str("uninitialized field: coins_per_utxo_word"))?,
            prefer_pure_change: cfg.prefer_pure_change,
        })
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilder {
    config: TransactionBuilderConfig,
    inputs: Vec<TxBuilderInput>,
    outputs: TransactionOutputs,
    fee: Option<Coin>,
    ttl: Option<Slot>, // absolute slot number
    certs: Option<Certificates>,
    withdrawals: Option<Withdrawals>,
    auxiliary_data: Option<AuxiliaryData>,
    validity_start_interval: Option<Slot>,
    input_types: MockWitnessSet,
    mint: Option<Mint>,
    mint_scripts: Option<NativeScripts>,
    inputs_auto_added: bool,
}

#[wasm_bindgen]
impl TransactionBuilder {
    /// This automatically selects and adds inputs from {inputs} consisting of just enough to cover
    /// the outputs that have already been added.
    /// This should be called after adding all certs/outputs/etc and will be an error otherwise.
    /// Uses CIP2: https://github.com/cardano-foundation/CIPs/blob/master/CIP-0002/CIP-0002.md
    /// Adding a change output must be called after via TransactionBuilder::add_change_if_needed()
    /// This function, diverging from CIP2, takes into account fees and will attempt to add additional
    /// inputs to cover the minimum fees. This does not, however, set the txbuilder's fee.
    pub fn add_inputs_from(&mut self, inputs: &TransactionUnspentOutputs, strategy: CoinSelectionStrategyCIP2) -> Result<(), JsError> {
        let mut available_inputs = inputs.0.clone();
        let mut input_total = self.get_total_input()?;
        let mut output_total = self
            .get_explicit_output()?
            .checked_add(&Value::new(&self.get_deposit()?))?
            .checked_add(&Value::new(&self.min_fee()?))?;
        match strategy {
            CoinSelectionStrategyCIP2::LargestFirst => {
                available_inputs.sort_by_key(|input| input.output.amount.coin);
                // iterate in decreasing order of ADA-only value
                for input in available_inputs.iter().rev() {
                    if input_total >= output_total {
                        break;
                    }
                    // differing from CIP2, we include the needed fees in the targets instead of just output values
                    let input_fee = self.fee_for_input(&input.output.address, &input.input, &input.output.amount)?;
                    self.add_input(&input.output.address, &input.input, &input.output.amount);
                    input_total = input_total.checked_add(&input.output.amount)?;
                    output_total = output_total.checked_add(&Value::new(&input_fee))?;
                }
                if input_total < output_total {
                    return Err(JsError::from_str("UTxO Balance Insufficient"));
                }
            },
            CoinSelectionStrategyCIP2::RandomImprove => {
                fn add_random_input(
                    s: &mut TransactionBuilder,
                    rng: &mut rand::rngs::ThreadRng,
                    available_inputs: &mut Vec<TransactionUnspentOutput>,
                    input_total: &Value,
                    output_total: &Value,
                    added: &Value,
                    needed: &Value
                ) -> Result<(Value, Value, Value, Value, TransactionUnspentOutput), JsError> {
                    let random_index = rng.gen_range(0..available_inputs.len());
                    let input = available_inputs.swap_remove(random_index);
                    // differing from CIP2, we include the needed fees in the targets instead of just output values
                    let input_fee = s.fee_for_input(&input.output.address, &input.input, &input.output.amount)?;
                    s.add_input(&input.output.address, &input.input, &input.output.amount);
                    let new_input_total = input_total.checked_add(&input.output.amount)?;
                    let new_output_total = output_total.checked_add(&Value::new(&input_fee))?;
                    let new_needed = needed.checked_add(&Value::new(&input_fee))?;
                    let new_added = added.checked_add(&input.output.amount)?;
                    Ok((new_input_total, new_output_total, new_added, new_needed, input))
                }
                use rand::Rng;
                if self.outputs.0.iter().any(|output| output.amount.multiasset.is_some()) {
                    return Err(JsError::from_str("Multiasset values not supported by RandomImprove. Please use LargestFirst"));
                }
                // Phase 1: Random Selection
                let mut associated_inputs: BTreeMap<TransactionOutput, Vec<TransactionUnspentOutput>> = BTreeMap::new();
                let mut rng = rand::thread_rng();
                let mut outputs = self.outputs.0.clone();
                outputs.sort_by_key(|output| output.amount.coin);
                for output in outputs.iter().rev() {
                    let mut added = Value::new(&Coin::zero());
                    let mut needed = output.amount.clone();
                    while added < needed {
                        if available_inputs.is_empty() {
                            return Err(JsError::from_str("UTxO Balance Insufficient"));
                        }
                        let (new_input_total, new_output_total, new_added, new_needed, input) = add_random_input(self, &mut rng, &mut available_inputs, &input_total, &output_total, &added, &needed)?;
                        input_total = new_input_total;
                        output_total = new_output_total;
                        added = new_added;
                        needed = new_needed;
                        associated_inputs.entry(output.clone()).or_default().push(input);
                    }
                }
                if !available_inputs.is_empty() {
                    // Phase 2: Improvement
                    for output in outputs.iter_mut() {
                        let associated = associated_inputs.get_mut(output).unwrap();
                        for input in associated.iter_mut() {
                            let random_index = rng.gen_range(0..available_inputs.len());
                            let new_input = available_inputs.get_mut(random_index).unwrap();
                            let cur = from_bignum(&input.output.amount.coin);
                            let new = from_bignum(&new_input.output.amount.coin);
                            let min = from_bignum(&output.amount.coin);
                            let ideal = 2 * min;
                            let max = 3 * min;
                            let move_closer = (ideal as i128 - new as i128).abs() < (ideal as i128 - cur as i128).abs();
                            let not_exceed_max = new < max;
                            if move_closer && not_exceed_max {
                                std::mem::swap(input, new_input);
                            }
                        }
                    }
                }
                // Phase 3: add extra inputs needed for fees
                // We do this at the end because this new inputs won't be associated with
                // a specific output, so the improvement algorithm we do above does not apply here.
                if input_total < output_total {
                    let mut added = Value::new(&Coin::zero());
                    let mut remaining_amount = output_total.checked_sub(&input_total)?;
                    while added < remaining_amount {
                        if available_inputs.is_empty() {
                            return Err(JsError::from_str("UTxO Balance Insufficient"));
                        }
                        let (new_input_total, new_output_total, new_added, new_remaining_amount, _) = add_random_input(self, &mut rng, &mut available_inputs, &input_total, &output_total, &added, &remaining_amount)?;
                        input_total = new_input_total;
                        output_total = new_output_total;
                        added = new_added;
                        remaining_amount = new_remaining_amount;
                    }
                }
            },
        }

        Ok(())
    }

    /// We have to know what kind of inputs these are to know what kind of mock witnesses to create since
    /// 1) mock witnesses have different lengths depending on the type which changes the expecting fee
    /// 2) Witnesses are a set so we need to get rid of duplicates to avoid over-estimating the fee
    pub fn add_key_input(&mut self, hash: &Ed25519KeyHash, input: &TransactionInput, amount: &Value) {
        self.inputs.push(TxBuilderInput {
            input: input.clone(),
            amount: amount.clone(),
        });
        self.input_types.vkeys.insert(hash.clone());
    }
    pub fn add_script_input(&mut self, hash: &ScriptHash, input: &TransactionInput, amount: &Value) {
        self.inputs.push(TxBuilderInput {
            input: input.clone(),
            amount: amount.clone(),
        });
        self.input_types.scripts.insert(hash.clone());
    }
    pub fn add_bootstrap_input(&mut self, hash: &ByronAddress, input: &TransactionInput, amount: &Value) {
        self.inputs.push(TxBuilderInput {
            input: input.clone(),
            amount: amount.clone(),
        });
        self.input_types.bootstraps.insert(hash.to_bytes());
    }

    pub fn add_input(&mut self, address: &Address, input: &TransactionInput, amount: &Value) {
        match &BaseAddress::from_address(address) {
            Some(addr) => {
                match &addr.payment_cred().to_keyhash() {
                    Some(hash) => return self.add_key_input(hash, input, amount),
                    None => (),
                }
                match &addr.payment_cred().to_scripthash() {
                    Some(hash) => return self.add_script_input(hash, input, amount),
                    None => (),
                }
            },
            None => (),
        }
        match &EnterpriseAddress::from_address(address) {
            Some(addr) => {
                match &addr.payment_cred().to_keyhash() {
                    Some(hash) => return self.add_key_input(hash, input, amount),
                    None => (),
                }
                match &addr.payment_cred().to_scripthash() {
                    Some(hash) => return self.add_script_input(hash, input, amount),
                    None => (),
                }
            },
            None => (),
        }
        match &PointerAddress::from_address(address) {
            Some(addr) => {
                match &addr.payment_cred().to_keyhash() {
                    Some(hash) => return self.add_key_input(hash, input, amount),
                    None => (),
                }
                match &addr.payment_cred().to_scripthash() {
                    Some(hash) => return self.add_script_input(hash, input, amount),
                    None => (),
                }
            },
            None => (),
        }
        match &ByronAddress::from_address(address) {
            Some(addr) => {
                return self.add_bootstrap_input(addr, input, amount);
            },
            None => (),
        }
    }

    /// calculates how much the fee would increase if you added a given output
    pub fn fee_for_input(&self, address: &Address, input: &TransactionInput, amount: &Value) -> Result<Coin, JsError> {
        let mut self_copy = self.clone();

        // we need some value for these for it to be a a valid transaction
        // but since we're only calculating the difference between the fee of two transactions
        // it doesn't matter what these are set as, since it cancels out
        self_copy.set_fee(&to_bignum(0));

        let fee_before = min_fee(&self_copy)?;

        self_copy.add_input(&address, &input, &amount);
        let fee_after = min_fee(&self_copy)?;
        fee_after.checked_sub(&fee_before)
    }

    /// Add output by specifying the Address and Value
    pub fn add_output_amount(&mut self, address: &Address, amount: &Value) -> Result<(), JsError> {
        self.add_output(&TransactionOutput::new(address, amount))
    }

    /// Add output by specifying the Address and Coin (BigNum)
    /// Output will have no additional assets
    pub fn add_output_coin(&mut self, address: &Address, coin: &Coin) -> Result<(), JsError> {
        self.add_output_amount(address, &Value::new(coin))
    }

    /// Add output by specifying the Address, the Coin (BigNum), and the MultiAsset
    pub fn add_output_coin_and_asset(
        &mut self,
        address: &Address,
        coin: &Coin,
        multiasset: &MultiAsset,
    ) -> Result<(), JsError> {
        let mut val = Value::new(coin);
        val.set_multiasset(multiasset);
        self.add_output_amount(address, &val)
    }

    /// Add output by specifying the Address and the MultiAsset
    /// The output will be set to contain the minimum required amount of Coin
    pub fn add_output_asset_and_min_required_coin(
        &mut self,
        address: &Address,
        multiasset: &MultiAsset,
    ) -> Result<(), JsError> {
        let min_possible_coin = min_pure_ada(&self.config.coins_per_utxo_word)?;
        let mut value = Value::new(&min_possible_coin);
        value.set_multiasset(multiasset);
        let required_coin = min_ada_required(
            &value,
            false,
            &self.config.coins_per_utxo_word,
        )?;
        self.add_output_coin_and_asset(address, &required_coin, multiasset)
    }

    /// Add explicit output via a TransactionOutput object
    pub fn add_output(&mut self, output: &TransactionOutput) -> Result<(), JsError> {
        let value_size = output.amount.to_bytes().len();
        if value_size > self.config.max_value_size as usize {
            return Err(JsError::from_str(&format!(
                "Maximum value size of {} exceeded. Found: {}",
                self.config.max_value_size,
                value_size
            )));
        }
        let min_ada = min_ada_required(
            &output.amount(),
            false,
            &self.config.coins_per_utxo_word,
        )?;
        if output.amount().coin() < min_ada {
            Err(JsError::from_str(&format!(
                "Value {} less than the minimum UTXO value {}",
                from_bignum(&output.amount().coin()),
                from_bignum(&min_ada)
            )))
        } else {
            self.outputs.add(output);
            Ok(())
        }
    }

    /// calculates how much the fee would increase if you added a given output
    pub fn fee_for_output(&self, output: &TransactionOutput) -> Result<Coin, JsError> {
        let mut self_copy = self.clone();

        // we need some value for these for it to be a a valid transaction
        // but since we're only calculating the different between the fee of two transactions
        // it doesn't matter what these are set as, since it cancels out
        self_copy.set_fee(&to_bignum(0));

        let fee_before = min_fee(&self_copy)?;

        self_copy.add_output(&output)?;
        let fee_after = min_fee(&self_copy)?;
        fee_after.checked_sub(&fee_before)
    }

    pub fn set_fee(&mut self, fee: &Coin) {
        self.fee = Some(fee.clone())
    }

    pub fn set_ttl(&mut self, ttl: Slot) {
        self.ttl = Some(ttl)
    }

    pub fn set_validity_start_interval(&mut self, validity_start_interval: Slot) {
        self.validity_start_interval = Some(validity_start_interval)
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

    pub fn get_auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.auxiliary_data.clone()
    }

    /// Set explicit auxiliary data via an AuxiliaryData object
    /// It might contain some metadata plus native or Plutus scripts
    pub fn set_auxiliary_data(&mut self, auxiliary_data: &AuxiliaryData) {
        self.auxiliary_data = Some(auxiliary_data.clone())
    }

    /// Set metadata using a GeneralTransactionMetadata object
    /// It will be set to the existing or new auxiliary data in this builder
    pub fn set_metadata(&mut self, metadata: &GeneralTransactionMetadata) {
        let mut aux = self.auxiliary_data.as_ref().cloned().unwrap_or(AuxiliaryData::new());
        aux.set_metadata(metadata);
        self.set_auxiliary_data(&aux);
    }

    /// Add a single metadatum using TransactionMetadatumLabel and TransactionMetadatum objects
    /// It will be securely added to existing or new metadata in this builder
    pub fn add_metadatum(&mut self, key: &TransactionMetadatumLabel, val: &TransactionMetadatum) {
        let mut metadata = self.auxiliary_data.as_ref()
            .map(|aux| { aux.metadata().as_ref().cloned() })
            .unwrap_or(None)
            .unwrap_or(GeneralTransactionMetadata::new());
        metadata.insert(key, val);
        self.set_metadata(&metadata);
    }

    /// Add a single JSON metadatum using a TransactionMetadatumLabel and a String
    /// It will be securely added to existing or new metadata in this builder
    pub fn add_json_metadatum(
        &mut self,
        key: &TransactionMetadatumLabel,
        val: String,
    ) -> Result<(), JsError> {
        self.add_json_metadatum_with_schema(key, val, MetadataJsonSchema::NoConversions)
    }

    /// Add a single JSON metadatum using a TransactionMetadatumLabel, a String, and a MetadataJsonSchema object
    /// It will be securely added to existing or new metadata in this builder
    pub fn add_json_metadatum_with_schema(
        &mut self,
        key: &TransactionMetadatumLabel,
        val: String,
        schema: MetadataJsonSchema,
    ) -> Result<(), JsError> {
        let metadatum = encode_json_str_to_metadatum(val, schema)?;
        self.add_metadatum(key, &metadatum);
        Ok(())
    }

    /// Set explicit Mint object to this builder
    /// it will replace any previously existing mint
    /// NOTE! If you use `set_mint` manually - you must use `set_mint_scripts`
    /// to provide matching policy scripts or min-fee calculation will be rejected!
    pub fn set_mint(&mut self, mint: &Mint) {
        self.mint = Some(mint.clone());
    }

    /// Returns a copy of the current mint state in the builder
    pub fn get_mint(&self) -> Option<Mint> {
        self.mint.clone()
    }

    /// Set explicit witness set to this builder
    /// It will replace any previously existing witnesses
    /// NOTE! Use carefully! If you are using `set_mint` - then you must be using
    /// this setter as well to be able to calculate fee automatically!
    pub fn set_mint_scripts(&mut self, mint_scripts: &NativeScripts) {
        self.mint_scripts = Some(mint_scripts.clone());
    }

    /// Returns a copy of the current mint witness scripts in the builder
    pub fn get_mint_scripts(&self) -> Option<NativeScripts> {
        self.mint_scripts.clone()
    }

    fn _set_mint_asset(&mut self, policy_id: &PolicyID, policy_script: &NativeScript, mint_assets: &MintAssets) {
        let mut mint = self.mint.as_ref().cloned().unwrap_or(Mint::new());
        let is_new_policy = mint.insert(&policy_id, mint_assets).is_none();
        self.set_mint(&mint);
        if is_new_policy {
            // If policy has not been encountered before - insert the script into witnesses
            let mut witness_scripts = self.mint_scripts.as_ref().cloned()
                .unwrap_or(NativeScripts::new());
            witness_scripts.add(&policy_script.clone());
            self.set_mint_scripts(&witness_scripts);
        }
    }

    /// Add a mint entry to this builder using a PolicyID and MintAssets object
    /// It will be securely added to existing or new Mint in this builder
    /// It will replace any existing mint assets with the same PolicyID
    pub fn set_mint_asset(&mut self, policy_script: &NativeScript, mint_assets: &MintAssets) {
        let policy_id: PolicyID = policy_script.hash(ScriptHashNamespace::NativeScript);
        self._set_mint_asset(&policy_id, policy_script, mint_assets);
    }

    fn _add_mint_asset(&mut self, policy_id: &PolicyID, policy_script: &NativeScript, asset_name: &AssetName, amount: Int) {
        let mut asset = self.mint.as_ref()
            .map(|m| { m.get(&policy_id).as_ref().cloned() })
            .unwrap_or(None)
            .unwrap_or(MintAssets::new());
        asset.insert(asset_name, amount);
        self._set_mint_asset(&policy_id, policy_script, &asset);
    }

    /// Add a mint entry to this builder using a PolicyID, AssetName, and Int object for amount
    /// It will be securely added to existing or new Mint in this builder
    /// It will replace any previous existing amount same PolicyID and AssetName
    pub fn add_mint_asset(&mut self, policy_script: &NativeScript, asset_name: &AssetName, amount: Int) {
        let policy_id: PolicyID = policy_script.hash(ScriptHashNamespace::NativeScript);
        self._add_mint_asset(&policy_id, policy_script, asset_name, amount);
    }

    /// Add a mint entry together with an output to this builder
    /// Using a PolicyID, AssetName, Int for amount, Address, and Coin (BigNum) objects
    /// The asset will be securely added to existing or new Mint in this builder
    /// A new output will be added with the specified Address, the Coin value, and the minted asset
    pub fn add_mint_asset_and_output(
        &mut self,
        policy_script: &NativeScript,
        asset_name: &AssetName,
        amount: Int,
        address: &Address,
        output_coin: &Coin,
    ) -> Result<(), JsError> {
        if !amount.is_positive() {
            return Err(JsError::from_str("Output value must be positive!"));
        }
        let policy_id: PolicyID = policy_script.hash(ScriptHashNamespace::NativeScript);
        self._add_mint_asset(&policy_id, policy_script, asset_name, amount.clone());
        let multiasset = Mint::new_from_entry(
            &policy_id,
            &MintAssets::new_from_entry(asset_name, amount.clone())
        ).as_positive_multiasset();
        self.add_output_coin_and_asset(address, output_coin, &multiasset)
    }

    /// Add a mint entry together with an output to this builder
    /// Using a PolicyID, AssetName, Int for amount, and Address objects
    /// The asset will be securely added to existing or new Mint in this builder
    /// A new output will be added with the specified Address and the minted asset
    /// The output will be set to contain the minimum required amount of Coin
    pub fn add_mint_asset_and_output_min_required_coin(
        &mut self,
        policy_script: &NativeScript,
        asset_name: &AssetName,
        amount: Int,
        address: &Address,
    ) -> Result<(), JsError> {
        if !amount.is_positive() {
            return Err(JsError::from_str("Output value must be positive!"));
        }
        let policy_id: PolicyID = policy_script.hash(ScriptHashNamespace::NativeScript);
        self._add_mint_asset(&policy_id, policy_script, asset_name, amount.clone());
        let multiasset = Mint::new_from_entry(
            &policy_id,
            &MintAssets::new_from_entry(asset_name, amount.clone())
        ).as_positive_multiasset();
        self.add_output_asset_and_min_required_coin(address, &multiasset)
    }

    pub fn new(cfg: &TransactionBuilderConfig) -> Self {
        Self {
            config: cfg.clone(),
            inputs: Vec::new(),
            outputs: TransactionOutputs::new(),
            fee: None,
            ttl: None,
            certs: None,
            withdrawals: None,
            auxiliary_data: None,
            input_types: MockWitnessSet {
                vkeys: BTreeSet::new(),
                scripts: BTreeSet::new(),
                bootstraps: BTreeSet::new(),
            },
            validity_start_interval: None,
            mint: None,
            mint_scripts: None,
            inputs_auto_added: false,
        }
    }

    /// does not include refunds or withdrawals
    pub fn get_explicit_input(&self) -> Result<Value, JsError> {
        self.inputs
            .iter()
            .try_fold(Value::zero(), |acc, ref tx_builder_input| {
                acc.checked_add(&tx_builder_input.amount)
            })
    }

    /// withdrawals and refunds
    pub fn get_implicit_input(&self) -> Result<Value, JsError> {
        internal_get_implicit_input(
            &self.withdrawals,
            &self.certs,
            &self.config.pool_deposit,
            &self.config.key_deposit,
        )
    }

    /// Returns mint as tuple of (mint_value, burn_value) or two zero values
    fn get_mint_as_values(&self) -> (Value, Value) {
        self.mint.as_ref().map(|m| {
            (Value::new_from_assets(&m.as_positive_multiasset()),
             Value::new_from_assets(&m.as_negative_multiasset()))
        }).unwrap_or((Value::zero(), Value::zero()))
    }

    fn get_total_input(&self) -> Result<Value, JsError> {
        let (mint_value, burn_value) = self.get_mint_as_values();
        self.get_explicit_input()?
            .checked_add(&self.get_implicit_input()?)?
            .checked_add(&mint_value)?
            .checked_sub(&burn_value)
    }

    /// does not include fee
    pub fn get_explicit_output(&self) -> Result<Value, JsError> {
        self.outputs
            .0
            .iter()
            .try_fold(Value::new(&to_bignum(0)), |acc, ref output| {
                acc.checked_add(&output.amount())
            })
    }

    pub fn get_deposit(&self) -> Result<Coin, JsError> {
        internal_get_deposit(
            &self.certs,
            &self.config.pool_deposit,
            &self.config.key_deposit,
        )
    }

    pub fn get_fee_if_set(&self) -> Option<Coin> {
        self.fee.clone()
    }

    /// Warning: this function will mutate the /fee/ field
    /// Make sure to call this function last after setting all other tx-body properties
    /// Editing inputs, outputs, mint, etc. after change been calculated
    /// might cause a mismatch in calculated fee versus the required fee
    pub fn add_change_if_needed(&mut self, address: &Address) -> Result<bool, JsError> {
        let fee = match &self.fee {
            None => self.min_fee(),
            // generating the change output involves changing the fee
            Some(_x) => {
                return Err(JsError::from_str(
                    "Cannot calculate change if fee was explicitly specified",
                ))
            }
        }?;

        let input_total = self.get_total_input()?;

        let output_total = self
            .get_explicit_output()?
            .checked_add(&Value::new(&self.get_deposit()?))?;

        use std::cmp::Ordering;
        match &input_total.partial_cmp(&output_total.checked_add(&Value::new(&fee))?) {
            Some(Ordering::Equal) => {
                // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
                self.set_fee(&input_total.checked_sub(&output_total)?.coin());
                Ok(false)
            },
            Some(Ordering::Less) => Err(JsError::from_str("Insufficient input in transaction")),
            Some(Ordering::Greater) => {
                fn has_assets(ma: Option<MultiAsset>) -> bool {
                    ma.map(|assets| assets.len() > 0).unwrap_or(false)
                }
                let change_estimator = input_total.checked_sub(&output_total)?;
                if has_assets(change_estimator.multiasset()) {
                    fn will_adding_asset_make_output_overflow(output: &TransactionOutput, current_assets: &Assets, asset_to_add: (PolicyID, AssetName, BigNum), max_value_size: u32) -> bool {
                        let (policy, asset_name, value) = asset_to_add;
                        let mut current_assets_clone = current_assets.clone();
                        current_assets_clone.insert(&asset_name, &value);
                        let mut amount_clone = output.amount.clone();
                        let mut val = Value::new(&Coin::zero());
                        let mut ma = MultiAsset::new();

                        ma.insert(&policy, &current_assets_clone);
                        val.set_multiasset(&ma);
                        amount_clone = amount_clone.checked_add(&val).unwrap();

                        amount_clone.to_bytes().len() > max_value_size as usize
                    }
                    fn pack_nfts_for_change(max_value_size: u32, change_address: &Address, change_estimator: &Value) -> Result<Vec<MultiAsset>, JsError> {
                        // we insert the entire available ADA temporarily here since that could potentially impact the size
                        // as it could be 1, 2 3 or 4 bytes for Coin.
                        let mut change_assets: Vec<MultiAsset> = Vec::new();

                        let mut base_coin = Value::new(&change_estimator.coin());
                        base_coin.set_multiasset(&MultiAsset::new());
                        let mut output = TransactionOutput::new(change_address, &base_coin);
                        // If this becomes slow on large TXs we can optimize it like the folowing
                        // to avoid cloning + reserializing the entire output.
                        // This would probably be more relevant if we use a smarter packing algorithm
                        // which might need to compare more size differences than greedy
                        //let mut bytes_used = output.to_bytes().len();

                        // a greedy packing is done here to avoid an exponential bin-packing
                        // which in most cases likely shouldn't be the difference between
                        // having an extra change output or not unless there are gigantic
                        // differences in NFT policy sizes
                        for (policy, assets) in change_estimator.multiasset().unwrap().0.iter() {
                            // for simplicity we also don't split assets within a single policy since
                            // you would need to have a very high amoun of assets (which add 1-36 bytes each)
                            // in a single policy to make a difference. In the future if this becomes an issue
                            // we can change that here.

                            // this is the other part of the optimization but we need to take into account
                            // the difference between CBOR encoding which can change which happens in two places:
                            // a) length within assets of one policy id
                            // b) length of the entire multiasset
                            // so for simplicity we will just do it the safe, naive way unless
                            // performance becomes an issue.
                            //let extra_bytes = policy.to_bytes().len() + assets.to_bytes().len() + 2 + cbor_len_diff;
                            //if bytes_used + extra_bytes <= max_value_size as usize {
                            let mut old_amount = output.amount.clone();
                            let mut val = Value::new(&Coin::zero());
                            let mut next_nft = MultiAsset::new();

                            let asset_names = assets.keys();
                            let mut rebuilt_assets = Assets::new();
                            for n in 0..asset_names.len() {
                                let asset_name = asset_names.get(n);
                                let value = assets.get(&asset_name).unwrap();

                                if will_adding_asset_make_output_overflow(&output, &rebuilt_assets, (policy.clone(), asset_name.clone(), value), max_value_size) {
                                    // if we got here, this means we will run into a overflow error,
                                    // so we want to split into multiple outputs, for that we...

                                    // 1. insert the current assets as they are, as this won't overflow
                                    next_nft.insert(policy, &rebuilt_assets);
                                    val.set_multiasset(&next_nft);
                                    output.amount = output.amount.checked_add(&val)?;
                                    change_assets.push(output.amount.multiasset().unwrap());

                                    // 2. create a new output with the base coin value as zero
                                    base_coin = Value::new(&Coin::zero());
                                    base_coin.set_multiasset(&MultiAsset::new());
                                    output = TransactionOutput::new(change_address, &base_coin);

                                    // 3. continue building the new output from the asset we stopped
                                    old_amount = output.amount.clone();
                                    val = Value::new(&Coin::zero());
                                    next_nft = MultiAsset::new();

                                    rebuilt_assets = Assets::new();
                                }

                                rebuilt_assets.insert(&asset_name, &value);
                            }

                            next_nft.insert(policy, &rebuilt_assets);
                            val.set_multiasset(&next_nft);
                            output.amount = output.amount.checked_add(&val)?;

                            if output.amount.to_bytes().len() > max_value_size as usize {
                                output.amount = old_amount;
                                break;
                            }
                        }
                        change_assets.push(output.amount.multiasset().unwrap());
                        Ok(change_assets)
                    }
                    let mut change_left = input_total.checked_sub(&output_total)?;
                    let mut new_fee = fee.clone();
                    // we might need multiple change outputs for cases where the change has many asset types
                    // which surpass the max UTXO size limit
                    let minimum_utxo_val = min_pure_ada(&self.config.coins_per_utxo_word)?;
                    while let Some(Ordering::Greater) = change_left.multiasset.as_ref().map_or_else(|| None, |ma| ma.partial_cmp(&MultiAsset::new())) {
                        let nft_changes = pack_nfts_for_change(self.config.max_value_size, address, &change_left)?;
                        if nft_changes.len() == 0 {
                            // this likely should never happen
                            return Err(JsError::from_str("NFTs too large for change output"));
                        }
                        // we only add the minimum needed (for now) to cover this output
                        let mut change_value = Value::new(&Coin::zero());
                        for nft_change in nft_changes.iter() {
                            change_value.set_multiasset(&nft_change);
                            let min_ada = min_ada_required(&change_value, false, &self.config.coins_per_utxo_word)?;
                            change_value.set_coin(&min_ada);
                            let change_output = TransactionOutput::new(address, &change_value);
                            // increase fee
                            let fee_for_change = self.fee_for_output(&change_output)?;
                            new_fee = new_fee.checked_add(&fee_for_change)?;
                            if change_left.coin() < min_ada.checked_add(&new_fee)? {
                                return Err(JsError::from_str("Not enough ADA leftover to include non-ADA assets in a change address"));
                            }
                            change_left = change_left.checked_sub(&change_value)?;
                            self.add_output(&change_output)?;
                        }
                    }
                    change_left = change_left.checked_sub(&Value::new(&new_fee))?;
                    // add potentially a separate pure ADA change output
                    let left_above_minimum = change_left.coin.compare(&minimum_utxo_val) > 0;
                    if self.config.prefer_pure_change && left_above_minimum {
                        let pure_output = TransactionOutput::new(address, &change_left);
                        let additional_fee = self.fee_for_output(&pure_output)?;
                        let potential_pure_value = change_left.checked_sub(&Value::new(&additional_fee))?;
                        let potential_pure_above_minimum = potential_pure_value.coin.compare(&minimum_utxo_val) > 0;
                        if potential_pure_above_minimum {
                            new_fee = new_fee.checked_add(&additional_fee)?;
                            change_left = Value::zero();
                            self.add_output(&TransactionOutput::new(address, &potential_pure_value))?;
                        }
                    }
                    self.set_fee(&new_fee);
                    // add in the rest of the ADA
                    if !change_left.is_zero() {
                        self.outputs.0.last_mut().unwrap().amount = self.outputs.0.last().unwrap().amount.checked_add(&change_left)?;
                    }
                    Ok(true)
                } else {
                    let min_ada = min_ada_required(
                        &change_estimator,
                        false,
                        &self.config.coins_per_utxo_word,
                    )?;
                    // no-asset case so we have no problem burning the rest if there is no other option
                    fn burn_extra(builder: &mut TransactionBuilder, burn_amount: &BigNum) -> Result<bool, JsError> {
                        // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
                        builder.set_fee(burn_amount);
                        Ok(false) // not enough input to covert the extra fee from adding an output so we just burn whatever is left
                    }
                    match change_estimator.coin() >= min_ada {
                        false => burn_extra(self, &change_estimator.coin()),
                        true => {
                            // check how much the fee would increase if we added a change output
                            let fee_for_change = self.fee_for_output(&TransactionOutput {
                                address: address.clone(),
                                amount: change_estimator.clone(),
                                // TODO: data hash?
                                data_hash: None,
                            })?;

                            let new_fee = fee.checked_add(&fee_for_change)?;
                            match change_estimator.coin() >= min_ada.checked_add(&Value::new(&new_fee).coin())? {
                                false => burn_extra(self, &change_estimator.coin()),
                                true => {
                                    // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
                                    self.set_fee(&new_fee);

                                    self.add_output(&TransactionOutput {
                                        address: address.clone(),
                                        amount: change_estimator.checked_sub(&Value::new(&new_fee.clone()))?,
                                        data_hash: None, // TODO: How do we get DataHash?
                                    })?;

                                    Ok(true)
                                }
                            }
                        }
                    }
                }
            }
            None => Err(JsError::from_str("missing input for some native asset")),
        }
    }

    fn build_and_size(&self) -> Result<(TransactionBody, usize), JsError> {
        let fee = self.fee.ok_or_else(|| JsError::from_str("Fee not specified"))?;
        let built = TransactionBody {
            inputs: TransactionInputs(self.inputs.iter().map(|ref tx_builder_input| tx_builder_input.input.clone()).collect()),
            outputs: self.outputs.clone(),
            fee: fee,
            ttl: self.ttl,
            certs: self.certs.clone(),
            withdrawals: self.withdrawals.clone(),
            update: None,
            auxiliary_data_hash: match &self.auxiliary_data {
                None => None,
                Some(x) => Some(utils::hash_auxiliary_data(x)),
            },
            validity_start_interval: self.validity_start_interval,
            mint: self.mint.clone(),
            // TODO: update for use with Alonzo
            script_data_hash: None,
            collateral: None,
            required_signers: None,
            network_id: None,
        };
        // we must build a tx with fake data (of correct size) to check the final Transaction size
        let full_tx = fake_full_tx(self, built)?;
        let full_tx_size = full_tx.to_bytes().len();
        return Ok((full_tx.body, full_tx_size));
    }

    pub fn full_size(&self) -> Result<usize, JsError> {
        return self.build_and_size().map(|r| { r.1 });
    }

    pub fn output_sizes(&self) -> Vec<usize> {
        return self.outputs.0.iter().map(|o| { o.to_bytes().len() }).collect();
    }

    /// Returns object the body of the new transaction
    /// Auxiliary data itself is not included
    /// You can use `get_auxiliary_data` or `build_tx`
    pub fn build(&self) -> Result<TransactionBody, JsError> {
        let (body, full_tx_size) = self.build_and_size()?;
        if full_tx_size > self.config.max_tx_size as usize {
            Err(JsError::from_str(&format!(
                "Maximum transaction size of {} exceeded. Found: {}",
                self.config.max_tx_size,
                full_tx_size
            )))
        } else {
            Ok(body)
        }
    }

    fn get_witness_set(&self) -> TransactionWitnessSet {
        let mut wit = TransactionWitnessSet::new();
        if let Some(scripts) = self.mint_scripts.as_ref() {
            wit.set_native_scripts(scripts);
        }
        wit
    }

    /// Returns full Transaction object with the body and the auxiliary data
    /// NOTE: witness_set will contain all mint_scripts if any been added or set
    /// NOTE: is_valid set to true
    pub fn build_tx(&self) -> Result<Transaction, JsError> {
        Ok(Transaction {
            body: self.build()?,
            witness_set: self.get_witness_set(),
            is_valid: true,
            auxiliary_data: self.auxiliary_data.clone(),
        })
    }

    /// warning: sum of all parts of a transaction must equal 0. You cannot just set the fee to the min value and forget about it
    /// warning: min_fee may be slightly larger than the actual minimum fee (ex: a few lovelaces)
    /// this is done to simplify the library code, but can be fixed later
    pub fn min_fee(&self) -> Result<Coin, JsError> {
        let mut self_copy = self.clone();
        self_copy.set_fee(&to_bignum(0x1_00_00_00_00));
        min_fee(&self_copy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fees::*;

    const MAX_VALUE_SIZE: u32 = 4000;
    const MAX_TX_SIZE: u32 = 8000; // might be out of date but suffices for our tests
    // this is what is used in mainnet
    static COINS_PER_UTXO_WORD: u64 = 34_482;

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
    fn check_fake_private_key() {
        let fpk = fake_private_key();
        assert_eq!(
            fpk.to_bech32(),
            "xprv1hretan5mml3tq2p0twkhq4tz4jvka7m2l94kfr6yghkyfar6m9wppc7h9unw6p65y23kakzct3695rs32z7vaw3r2lg9scmfj8ec5du3ufydu5yuquxcz24jlkjhsc9vsa4ufzge9s00fn398svhacse5su2awrw",
        );
        assert_eq!(
            fpk.to_public().to_bech32(),
            "xpub1eamrnx3pph58yr5l4z2wghjpu2dt2f0rp0zq9qquqa39p52ct0xercjgmegfcpcdsy4t9ld90ps2epmtcjy3jtq77n8z20qe0m3pnfqntgrgj",
        );
    }

    fn byron_address() -> Address {
        ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address()
    }

    fn create_linear_fee(coefficient: u64, constant: u64) -> LinearFee {
        LinearFee::new(&to_bignum(coefficient), &to_bignum(constant))
    }

    fn create_default_linear_fee() -> LinearFee {
        create_linear_fee(500, 2)
    }

    fn create_tx_builder_full(
        linear_fee: &LinearFee,
        pool_deposit: u64,
        key_deposit: u64,
        max_val_size: u32,
        coins_per_utxo_word: u64,
    ) -> TransactionBuilder {
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(linear_fee)
            .pool_deposit(&to_bignum(pool_deposit))
            .key_deposit(&to_bignum(key_deposit))
            .max_value_size(max_val_size)
            .max_tx_size(MAX_TX_SIZE)
            .coins_per_utxo_word(&to_bignum(coins_per_utxo_word))
            .build()
            .unwrap();
        TransactionBuilder::new(&cfg)
    }

    fn create_tx_builder(
        linear_fee: &LinearFee,
        coins_per_utxo_word: u64,
        pool_deposit: u64,
        key_deposit: u64,
    ) -> TransactionBuilder {
        create_tx_builder_full(linear_fee, pool_deposit, key_deposit, MAX_VALUE_SIZE, coins_per_utxo_word)
    }

    fn create_reallistic_tx_builder() -> TransactionBuilder {
        create_tx_builder(
            &create_linear_fee(44, 155381),
            COINS_PER_UTXO_WORD,
            500000000,
            2000000,
        )
    }

    fn create_tx_builder_with_fee_and_val_size(linear_fee: &LinearFee, max_val_size: u32) -> TransactionBuilder {
        create_tx_builder_full(linear_fee, 1, 1, max_val_size, 1)
    }

    fn create_tx_builder_with_fee(linear_fee: &LinearFee) -> TransactionBuilder {
        create_tx_builder(linear_fee, 1, 1, 1)
    }

    fn create_tx_builder_with_fee_and_pure_change(linear_fee: &LinearFee) -> TransactionBuilder {
        TransactionBuilder::new(&TransactionBuilderConfigBuilder::new()
            .fee_algo(linear_fee)
            .pool_deposit(&to_bignum(1))
            .key_deposit(&to_bignum(1))
            .max_value_size(MAX_VALUE_SIZE)
            .max_tx_size(MAX_TX_SIZE)
            .coins_per_utxo_word(&to_bignum(1))
            .prefer_pure_change(true)
            .build()
            .unwrap())
    }

    fn create_tx_builder_with_key_deposit(deposit: u64) -> TransactionBuilder {
        create_tx_builder(&create_default_linear_fee(), 1, 1, deposit)
    }

    fn create_default_tx_builder() -> TransactionBuilder {
        create_tx_builder_with_fee(&create_default_linear_fee())
    }

    #[test]
    fn build_tx_with_change() {
        let mut tx_builder = create_default_tx_builder();
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
        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred, &stake_cred).to_address();
        tx_builder.add_key_input(
            &spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000))
        );
        tx_builder.add_output(&TransactionOutput::new(
            &addr_net_0,
            &Value::new(&to_bignum(29))
        )).unwrap();
        tx_builder.set_ttl(1000);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr
        );
        assert!(added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 2);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );
        assert_eq!(tx_builder.full_size().unwrap(), 285);
        assert_eq!(tx_builder.output_sizes(), vec![62, 65]);
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_without_change() {
        let mut tx_builder = create_default_tx_builder();
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
        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred, &stake_cred).to_address();
        tx_builder.add_key_input(
            &spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000))
        );
        tx_builder.add_output(&TransactionOutput::new(
            &addr_net_0,
            &Value::new(&to_bignum(880_000))
        )).unwrap();
        tx_builder.set_ttl(1000);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr
        );
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_with_certs() {
        let mut tx_builder = create_tx_builder_with_key_deposit(1_000_000);
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

        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        tx_builder.add_key_input(
            &spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(5_000_000))
        );
        tx_builder.set_ttl(1000);

        let mut certs = Certificates::new();
        certs.add(&Certificate::new_stake_registration(&StakeRegistration::new(&stake_cred)));
        certs.add(&Certificate::new_stake_delegation(&StakeDelegation::new(
            &stake_cred,
            &stake.to_raw_key().hash(), // in reality, this should be the pool owner's key, not ours
        )));
        tx_builder.set_certs(&certs);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        tx_builder.add_change_if_needed(
            &change_addr
        ).unwrap();
        assert_eq!(tx_builder.min_fee().unwrap().to_str(), "214002");
        assert_eq!(tx_builder.get_fee_if_set().unwrap().to_str(), "214002");
        assert_eq!(tx_builder.get_deposit().unwrap().to_str(), "1000000");
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder
                .get_explicit_output().unwrap()
                .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
                .checked_add(&Value::new(&tx_builder.get_deposit().unwrap())).unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_exact_amount() {
        // transactions where sum(input) == sum(output) exact should pass
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
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
        tx_builder.add_key_input(
            &&spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(100))
        );
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred, &stake_cred).to_address();
        tx_builder.add_output(&TransactionOutput::new(
            &addr_net_0,
            &Value::new(&to_bignum(100))
        )).unwrap();
        tx_builder.set_ttl(0);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr
        ).unwrap();
        assert_eq!(added_change, false);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 1);
    }

    #[test]
    fn build_tx_exact_change() {
        // transactions where we have exactly enough ADA to add change should pass
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
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
        tx_builder.add_key_input(
            &&spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(58))
        );
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();
        tx_builder
            .add_output(&TransactionOutput::new(
                &addr_net_0,
                &Value::new(&to_bignum(29)),
            ))
            .unwrap();
        tx_builder.set_ttl(0);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr
        ).unwrap();
        assert_eq!(added_change, true);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 2);
        assert_eq!(final_tx.outputs().get(1).amount().coin().to_str(), "29");
    }

    #[test]
    #[should_panic]
    fn build_tx_insufficient_deposit() {
        // transactions should fail with insufficient fees if a deposit is required
        let mut tx_builder = create_tx_builder_with_key_deposit(5);
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
        tx_builder.add_key_input(
            &&spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(5)),
        );
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();
        tx_builder
            .add_output(&TransactionOutput::new(
                &addr_net_0,
                &Value::new(&to_bignum(5)),
            ))
            .unwrap();
        tx_builder.set_ttl(0);

        // add a cert which requires a deposit
        let mut certs = Certificates::new();
        certs.add(&Certificate::new_stake_registration(
            &StakeRegistration::new(&stake_cred),
        ));
        tx_builder.set_certs(&certs);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        tx_builder.add_change_if_needed(&change_addr).unwrap();
    }

    #[test]
    fn build_tx_with_inputs() {
        let mut tx_builder = create_default_tx_builder();
        let spend = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
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

        {
            assert_eq!(tx_builder.fee_for_input(
                &EnterpriseAddress::new(
                    NetworkInfo::testnet().network_id(),
                    &spend_cred
                ).to_address(),
                &TransactionInput::new(&genesis_id(), 0),
                &Value::new(&to_bignum(1_000_000))
            ).unwrap().to_str(), "69500");
            tx_builder.add_input(
                &EnterpriseAddress::new(
                    NetworkInfo::testnet().network_id(),
                    &spend_cred
                ).to_address(),
                &TransactionInput::new(&genesis_id(), 0),
                &Value::new(&to_bignum(1_000_000))
            );
        }
        tx_builder.add_input(
            &BaseAddress::new(
                NetworkInfo::testnet().network_id(),
                &spend_cred,
                &stake_cred
            ).to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000))
        );
        tx_builder.add_input(
            &PointerAddress::new(
                NetworkInfo::testnet().network_id(),
                &spend_cred,
                &Pointer::new(
                    0,
                    0,
                    0
                )
            ).to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000))
        );
        tx_builder.add_input(
            &ByronAddress::icarus_from_key(
                &spend, NetworkInfo::testnet().protocol_magic()
            ).to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000))
        );

        assert_eq!(tx_builder.inputs.len(), 4);
    }

    #[test]
    fn build_tx_with_mint_all_sent() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
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

        // Input with 150 coins
        tx_builder.add_input(
            &EnterpriseAddress::new(
                NetworkInfo::testnet().network_id(),
                &spend_cred
            ).to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(150))
        );

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

        let (min_script, policy_id) = mint_script_and_policy(0);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let amount = to_bignum(1234);

        // Adding mint of the asset - which should work as an input
        tx_builder.add_mint_asset(&min_script, &name, Int::new(&amount));

        let mut ass = Assets::new();
        ass.insert(&name, &amount);
        let mut mass = MultiAsset::new();
        mass.insert(&policy_id, &ass);

        // One coin and the minted asset goes into the output
        let mut output_amount = Value::new(&to_bignum(50));
        output_amount.set_multiasset(&mass);

        tx_builder
            .add_output(&TransactionOutput::new(&addr_net_0, &output_amount))
            .unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(added_change);
        assert_eq!(tx_builder.outputs.len(), 2);

        // Change must be one remaining coin because fee is one constant coin
        let change = tx_builder.outputs.get(1).amount();
        assert_eq!(change.coin(), to_bignum(99));
        assert!(change.multiasset().is_none());
    }

    #[test]
    fn build_tx_with_mint_in_change() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
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

        // Input with 150 coins
        tx_builder.add_input(
            &EnterpriseAddress::new(
                NetworkInfo::testnet().network_id(),
                &spend_cred
            ).to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(150))
        );

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

        let (min_script, policy_id) = mint_script_and_policy(0);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let amount_minted = to_bignum(1000);
        let amount_sent = to_bignum(500);

        // Adding mint of the asset - which should work as an input
        tx_builder.add_mint_asset(&min_script, &name, Int::new(&amount_minted));

        let mut ass = Assets::new();
        ass.insert(&name, &amount_sent);
        let mut mass = MultiAsset::new();
        mass.insert(&policy_id, &ass);

        // One coin and the minted asset goes into the output
        let mut output_amount = Value::new(&to_bignum(50));
        output_amount.set_multiasset(&mass);

        tx_builder
            .add_output(&TransactionOutput::new(&addr_net_0, &output_amount))
            .unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(added_change);
        assert_eq!(tx_builder.outputs.len(), 2);

        // Change must be one remaining coin because fee is one constant coin
        let change = tx_builder.outputs.get(1).amount();
        assert_eq!(change.coin(), to_bignum(99));
        assert!(change.multiasset().is_some());

        let change_assets = change.multiasset().unwrap();
        let change_asset = change_assets.get(&policy_id).unwrap();
        assert_eq!(
            change_asset.get(&name).unwrap(),
            amount_minted.checked_sub(&amount_sent).unwrap(),
        );
    }

    #[test]
    fn build_tx_with_native_assets_change() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
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

        let policy_id = &PolicyID::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let ma_input1 = 100;
        let ma_input2 = 200;
        let ma_output1 = 60;

        let multiassets = [ma_input1, ma_input2, ma_output1]
            .iter()
            .map(|input| {
                let mut multiasset = MultiAsset::new();
                multiasset.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(*input));
                    assets
                });
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets
            .iter()
            .zip([100u64, 100].iter().cloned().map(to_bignum))
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            tx_builder.add_key_input(
                &&spend.to_raw_key().hash(),
                &TransactionInput::new(&genesis_id(), 0),
                &input_amount,
            );
        }

        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

        let mut output_amount = Value::new(&to_bignum(100));
        output_amount.set_multiasset(&multiassets[2]);

        tx_builder
            .add_output(&TransactionOutput::new(&addr_net_0, &output_amount))
            .unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert_eq!(added_change, true);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 2);
        assert_eq!(
            final_tx
                .outputs()
                .get(1)
                .amount()
                .multiasset()
                .unwrap()
                .get(policy_id)
                .unwrap()
                .get(&name)
                .unwrap(),
            to_bignum(ma_input1 + ma_input2 - ma_output1)
        );
        assert_eq!(
            final_tx.outputs().get(1).amount().coin(),
            to_bignum(99)
        );
    }

    #[test]
    fn build_tx_with_native_assets_change_and_purification() {
        let coin_per_utxo_word = to_bignum(1);
        // Prefer pure change!
        let mut tx_builder = create_tx_builder_with_fee_and_pure_change(&create_linear_fee(0, 1));
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

        let policy_id = &PolicyID::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let ma_input1 = 100;
        let ma_input2 = 200;
        let ma_output1 = 60;

        let multiassets = [ma_input1, ma_input2, ma_output1]
            .iter()
            .map(|input| {
                let mut multiasset = MultiAsset::new();
                multiasset.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(*input));
                    assets
                });
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets
            .iter()
            .zip([100u64, 100].iter().cloned().map(to_bignum))
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            tx_builder.add_key_input(
                &&spend.to_raw_key().hash(),
                &TransactionInput::new(&genesis_id(), 0),
                &input_amount,
            );
        }

        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

        let mut output_amount = Value::new(&to_bignum(50));
        output_amount.set_multiasset(&multiassets[2]);

        tx_builder
            .add_output(&TransactionOutput::new(&addr_net_0, &output_amount))
            .unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert_eq!(added_change, true);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 3);
        assert_eq!(
            final_tx.outputs().get(0).amount().coin(),
            to_bignum(50)
        );
        assert_eq!(
            final_tx
                .outputs()
                .get(1)
                .amount()
                .multiasset()
                .unwrap()
                .get(policy_id)
                .unwrap()
                .get(&name)
                .unwrap(),
            to_bignum(ma_input1 + ma_input2 - ma_output1)
        );
        // The first change output that contains all the tokens contain minimum required Coin
        let min_coin_for_dirty_change = min_ada_required(
            &final_tx.outputs().get(1).amount(),
            false,
            &coin_per_utxo_word,
        ).unwrap();
        assert_eq!(
            final_tx.outputs().get(1).amount().coin(),
            min_coin_for_dirty_change
        );
        assert_eq!(
            final_tx.outputs().get(2).amount().coin(),
            to_bignum(110)
        );
        assert_eq!(
            final_tx.outputs().get(2).amount().multiasset(),
            None
        );
    }

    #[test]
    fn build_tx_with_native_assets_change_and_no_purification_cuz_not_enough_pure_coin() {
        // Prefer pure change!
        let mut tx_builder = create_tx_builder_with_fee_and_pure_change(&create_linear_fee(1, 1));
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

        let policy_id = &PolicyID::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let ma_input1 = 100;
        let ma_input2 = 200;
        let ma_output1 = 60;

        let multiassets = [ma_input1, ma_input2, ma_output1]
            .iter()
            .map(|input| {
                let mut multiasset = MultiAsset::new();
                multiasset.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(*input));
                    assets
                });
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets
            .iter()
            .zip([300u64, 300].iter().cloned().map(to_bignum))
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            tx_builder.add_key_input(
                &&spend.to_raw_key().hash(),
                &TransactionInput::new(&genesis_id(), 0),
                &input_amount,
            );
        }

        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

        let mut output_amount = Value::new(&to_bignum(100));
        output_amount.set_multiasset(&multiassets[2]);

        tx_builder
            .add_output(&TransactionOutput::new(&addr_net_0, &output_amount))
            .unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert_eq!(added_change, true);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 2);
        assert_eq!(
            final_tx.outputs().get(0).amount().coin(),
            to_bignum(100)
        );
        assert_eq!(
            final_tx
                .outputs()
                .get(1)
                .amount()
                .multiasset()
                .unwrap()
                .get(policy_id)
                .unwrap()
                .get(&name)
                .unwrap(),
            to_bignum(ma_input1 + ma_input2 - ma_output1)
        );
        // The single change output contains more Coin then minimal utxo value
        // But not enough to cover the additional fee for a separate output
        assert_eq!(
            final_tx.outputs().get(1).amount().coin(),
            to_bignum(101)
        );
    }

    #[test]
    #[should_panic]
    fn build_tx_leftover_assets() {
        let mut tx_builder = create_default_tx_builder();
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
        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred, &stake_cred).to_address();

        // add an input that contains an asset not present in the output
        let policy_id = &PolicyID::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let mut input_amount = Value::new(&to_bignum(1_000_000));
        let mut input_multiasset = MultiAsset::new();
        input_multiasset.insert(policy_id, &{
            let mut assets = Assets::new();
            assets.insert(&name, &to_bignum(100));
            assets
        });
        input_amount.set_multiasset(&input_multiasset);
        tx_builder.add_key_input(
            &spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &input_amount
        );

        tx_builder.add_output(&TransactionOutput::new(
            &addr_net_0,
            &Value::new(&to_bignum(880_000))
        )).unwrap();
        tx_builder.set_ttl(1000);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr
        );
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_burn_less_than_min_ada() {
        // with this mainnet value we should end up with a final min_ada_required of just under 1_000_000
        let mut tx_builder = create_reallistic_tx_builder();

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap();
        tx_builder.add_output(&TransactionOutput::new(
            &output_addr.to_address(),
            &Value::new(&to_bignum(2_000_000))
        )).unwrap();

        tx_builder.add_input(
            &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address(),
            &TransactionInput::new(
                &genesis_id(),
                0
            ),
            &Value::new(&to_bignum(2_400_000))
        );

        tx_builder.set_ttl(1);

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr.to_address()
        );
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_burn_empty_assets() {
        let mut tx_builder = create_reallistic_tx_builder();

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap();
        tx_builder.add_output(&TransactionOutput::new(
            &output_addr.to_address(),
            &Value::new(&to_bignum(2_000_000))
        )).unwrap();

        let mut input_value = Value::new(&to_bignum(2_400_000));
        input_value.set_multiasset(&MultiAsset::new());
        tx_builder.add_input(
            &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address(),
            &TransactionInput::new(
                &genesis_id(),
                0
            ),
            &input_value
        );

        tx_builder.set_ttl(1);

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr.to_address()
        );
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap().coin(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap().coin()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_no_useless_multiasset() {
        let mut tx_builder = create_reallistic_tx_builder();

        let policy_id = &PolicyID::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        // add an output that uses up all the token but leaves ADA
        let mut input_amount = Value::new(&to_bignum(5_000_000));
        let mut input_multiasset = MultiAsset::new();
        input_multiasset.insert(policy_id, &{
            let mut assets = Assets::new();
            assets.insert(&name, &to_bignum(100));
            assets
        });
        input_amount.set_multiasset(&input_multiasset);

        tx_builder.add_input(
            &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address(),
            &TransactionInput::new(
                &genesis_id(),
                0
            ),
            &input_amount
        );

        // add an input that contains an asset & ADA
        let mut output_amount = Value::new(&to_bignum(2_000_000));
        let mut output_multiasset = MultiAsset::new();
        output_multiasset.insert(policy_id, &{
            let mut assets = Assets::new();
            assets.insert(&name, &to_bignum(100));
            assets
        });
        output_amount.set_multiasset(&output_multiasset);

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap();
        tx_builder.add_output(&TransactionOutput::new(
            &output_addr.to_address(),
            &output_amount
        )).unwrap();

        tx_builder.set_ttl(1);

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr.to_address()
        );
        assert!(added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 2);
        let final_tx = tx_builder.build().unwrap();
        let change_output = final_tx.outputs().get(1);
        let change_assets = change_output.amount().multiasset();

        // since all tokens got sent in the output
        // the change should be only ADA and not have any multiasset struct in it
        assert!(change_assets.is_none());
    }

    fn create_multiasset() -> (MultiAsset, [ScriptHash; 3], [AssetName; 3]) {
        let policy_ids = [
            PolicyID::from([0u8; 28]),
            PolicyID::from([1u8; 28]),
            PolicyID::from([2u8; 28]),
        ];
        let names = [
            AssetName::new(vec![99u8; 32]).unwrap(),
            AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
            AssetName::new(vec![4u8, 5, 6, 7, 8, 9]).unwrap(),
        ];
        let multiasset = policy_ids
            .iter()
            .zip(names.iter())
            .fold(MultiAsset::new(), |mut acc, (policy_id, name)| {
                acc.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(500));
                    assets
                });
                acc
            });
        return (multiasset, policy_ids, names);
    }

    #[test]
    fn build_tx_add_change_split_nfts() {
        let max_value_size = 100; // super low max output size to test with fewer assets
        let mut tx_builder = create_tx_builder_with_fee_and_val_size(
            &create_linear_fee(0, 1),
            max_value_size,
        );

        let (multiasset, policy_ids, names) = create_multiasset();

        let mut input_value = Value::new(&to_bignum(1000));
        input_value.set_multiasset(&multiasset);

        tx_builder.add_input(
            &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address(),
            &TransactionInput::new(
                &genesis_id(),
                0
            ),
            &input_value
        );

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap().to_address();
        let output_amount = Value::new(&to_bignum(100));

        tx_builder
            .add_output(&TransactionOutput::new(&output_addr, &output_amount))
            .unwrap();

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert_eq!(added_change, true);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 3);
        for (policy_id, asset_name) in policy_ids.iter().zip(names.iter()) {
            assert!(final_tx
                .outputs
                .0
                .iter()
                .find(|output| output.amount.multiasset.as_ref().map_or_else(
                    || false,
                    |ma| ma.0.iter().find(|(pid, a)| *pid == policy_id
                        && a.0.iter().find(|(name, _)| *name == asset_name).is_some()).is_some()
                )).is_some()
            );
        }
        for output in final_tx.outputs.0.iter() {
            assert!(output.amount.to_bytes().len() <= max_value_size as usize);
        }
    }

    #[test]
    fn build_tx_too_big_output() {
        let mut tx_builder = create_tx_builder_with_fee_and_val_size(
            &create_linear_fee(0, 1),
            10,
        );

        tx_builder.add_input(
            &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address(),
            &TransactionInput::new(
                &genesis_id(),
                0
            ),
            &Value::new(&to_bignum(500))
        );

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap().to_address();
        let mut output_amount = Value::new(&to_bignum(50));
        output_amount.set_multiasset(&create_multiasset().0);

        assert!(tx_builder.add_output(&TransactionOutput::new(&output_addr, &output_amount)).is_err());
    }

    #[test]
    fn build_tx_add_change_nfts_not_enough_ada() {
        let mut tx_builder = create_tx_builder_with_fee_and_val_size(
            &create_linear_fee(0, 1),
            150,  // super low max output size to test with fewer assets
        );

        let policy_ids = [
            PolicyID::from([0u8; 28]),
            PolicyID::from([1u8; 28]),
            PolicyID::from([2u8; 28]),
        ];
        let names = [
            AssetName::new(vec![99u8; 32]).unwrap(),
            AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
            AssetName::new(vec![4u8, 5, 6, 7, 8, 9]).unwrap(),
        ];

        let multiasset = policy_ids
            .iter()
            .zip(names.iter())
            .fold(MultiAsset::new(), |mut acc, (policy_id, name)| {
                acc.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(500));
                    assets
                });
                acc
            });

        let mut input_value = Value::new(&to_bignum(58));
        input_value.set_multiasset(&multiasset);

        tx_builder.add_input(
            &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address(),
            &TransactionInput::new(
                &genesis_id(),
                0
            ),
            &input_value
        );

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap().to_address();
        let output_amount = Value::new(&to_bignum(59));

        tx_builder
            .add_output(&TransactionOutput::new(&output_addr, &output_amount))
            .unwrap();

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();

        assert!(tx_builder.add_change_if_needed(&change_addr).is_err())
    }

    fn make_input(input_hash_byte: u8, value: Value) -> TransactionUnspentOutput {
        TransactionUnspentOutput::new(
            &TransactionInput::new(&TransactionHash::from([input_hash_byte; 32]), 0),
            &TransactionOutput::new(&Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap(), &value)
        )
    }

    #[test]
    fn tx_builder_cip2_largest_first_increasing_fees() {
        // we have a = 1 to test increasing fees when more inputs are added
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 0));
        tx_builder.add_output(&TransactionOutput::new(
            &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap(),
            &Value::new(&to_bignum(1000))
        )).unwrap();
        let mut available_inputs = TransactionUnspentOutputs::new();
        available_inputs.add(&make_input(0u8, Value::new(&to_bignum(150))));
        available_inputs.add(&make_input(1u8, Value::new(&to_bignum(200))));
        available_inputs.add(&make_input(2u8, Value::new(&to_bignum(800))));
        available_inputs.add(&make_input(3u8, Value::new(&to_bignum(400))));
        available_inputs.add(&make_input(4u8, Value::new(&to_bignum(100))));
        tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::LargestFirst).unwrap();
        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();
        let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(change_added);
        let tx = tx_builder.build().unwrap();
        // change needed
        assert_eq!(2, tx.outputs().len());
        assert_eq!(3, tx.inputs().len());
        // confirm order of only what is necessary
        assert_eq!(2u8, tx.inputs().get(0).transaction_id().0[0]);
        assert_eq!(3u8, tx.inputs().get(1).transaction_id().0[0]);
        assert_eq!(1u8, tx.inputs().get(2).transaction_id().0[0]);
    }


    #[test]
    fn tx_builder_cip2_largest_first_static_fees() {
        // we have a = 0 so we know adding inputs/outputs doesn't change the fee so we can analyze more
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
        tx_builder.add_output(&TransactionOutput::new(
            &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap(),
            &Value::new(&to_bignum(1200))
        )).unwrap();
        let mut available_inputs = TransactionUnspentOutputs::new();
        available_inputs.add(&make_input(0u8, Value::new(&to_bignum(150))));
        available_inputs.add(&make_input(1u8, Value::new(&to_bignum(200))));
        available_inputs.add(&make_input(2u8, Value::new(&to_bignum(800))));
        available_inputs.add(&make_input(3u8, Value::new(&to_bignum(400))));
        available_inputs.add(&make_input(4u8, Value::new(&to_bignum(100))));
        tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::LargestFirst).unwrap();
        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();
        let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(!change_added);
        let tx = tx_builder.build().unwrap();
        // change not needed - should be exact
        assert_eq!(1, tx.outputs().len());
        assert_eq!(2, tx.inputs().len());
        // confirm order of only what is necessary
        assert_eq!(2u8, tx.inputs().get(0).transaction_id().0[0]);
        assert_eq!(3u8, tx.inputs().get(1).transaction_id().0[0]);
    }

    #[test]
    fn tx_builder_cip2_random_improve() {
        // we have a = 1 to test increasing fees when more inputs are added
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 0));
        const COST: u64 = 10000;
        tx_builder.add_output(&TransactionOutput::new(
            &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap(),
            &Value::new(&to_bignum(COST))
        )).unwrap();
        let mut available_inputs = TransactionUnspentOutputs::new();
        available_inputs.add(&make_input(0u8, Value::new(&to_bignum(1500))));
        available_inputs.add(&make_input(1u8, Value::new(&to_bignum(2000))));
        available_inputs.add(&make_input(2u8, Value::new(&to_bignum(8000))));
        available_inputs.add(&make_input(3u8, Value::new(&to_bignum(4000))));
        available_inputs.add(&make_input(4u8, Value::new(&to_bignum(1000))));
        available_inputs.add(&make_input(5u8, Value::new(&to_bignum(2000))));
        available_inputs.add(&make_input(6u8, Value::new(&to_bignum(1500))));
        let add_inputs_res =
            tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::RandomImprove);
        assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();
        let add_change_res = tx_builder.add_change_if_needed(&change_addr);
        assert!(add_change_res.is_ok(), "{:?}", add_change_res.err());
        let tx_build_res = tx_builder.build();
        assert!(tx_build_res.is_ok(), "{:?}", tx_build_res.err());
        let tx = tx_build_res.unwrap();
        // we need to look up the values to ensure there's enough
        let mut input_values = BTreeMap::new();
        for utxo in available_inputs.0.iter() {
            input_values.insert(utxo.input.transaction_id(), utxo.output.amount.clone());
        }
        let mut encountered = std::collections::HashSet::new();
        let mut input_total = Value::new(&Coin::zero());
        for input in tx.inputs.0.iter() {
            let txid = input.transaction_id();
            if !encountered.insert(txid.clone()) {
                panic!("Input {:?} duplicated", txid);
            }
            let value = input_values.get(&txid).unwrap();
            input_total = input_total.checked_add(value).unwrap();
        }
        assert!(input_total >= Value::new(&tx_builder.min_fee().unwrap().checked_add(&to_bignum(COST)).unwrap()));
    }

    #[test]
    fn tx_builder_cip2_random_improve_when_using_all_available_inputs() {
        // we have a = 1 to test increasing fees when more inputs are added
        let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(0));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(0))
            .key_deposit(&to_bignum(0))
            .max_value_size(9999)
            .max_tx_size(9999)
            .coins_per_utxo_word(&Coin::zero())
            .build()
            .unwrap();
        let mut tx_builder = TransactionBuilder::new(&cfg);
        const COST: u64 = 1000;
        tx_builder.add_output(&TransactionOutput::new(
            &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap(),
            &Value::new(&to_bignum(COST))
        )).unwrap();
        let mut available_inputs = TransactionUnspentOutputs::new();
        available_inputs.add(&make_input(1u8, Value::new(&to_bignum(800))));
        available_inputs.add(&make_input(2u8, Value::new(&to_bignum(800))));
        let add_inputs_res =
            tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::RandomImprove);
        assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
    }

    #[test]
    fn tx_builder_cip2_random_improve_adds_enough_for_fees() {
        // we have a = 1 to test increasing fees when more inputs are added
        let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(0));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(0))
            .key_deposit(&to_bignum(0))
            .max_value_size(9999)
            .max_tx_size(9999)
            .coins_per_utxo_word(&Coin::zero())
            .build()
            .unwrap();
        let mut tx_builder = TransactionBuilder::new(&cfg);
        const COST: u64 = 100;
        tx_builder.add_output(&TransactionOutput::new(
            &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap(),
            &Value::new(&to_bignum(COST))
        )).unwrap();
        assert_eq!(tx_builder.min_fee().unwrap(), to_bignum(53));
        let mut available_inputs = TransactionUnspentOutputs::new();
        available_inputs.add(&make_input(1u8, Value::new(&to_bignum(150))));
        available_inputs.add(&make_input(2u8, Value::new(&to_bignum(150))));
        available_inputs.add(&make_input(3u8, Value::new(&to_bignum(150))));
        let add_inputs_res =
            tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::RandomImprove);
        assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
        assert_eq!(tx_builder.min_fee().unwrap(), to_bignum(264));
        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();
        let add_change_res = tx_builder.add_change_if_needed(&change_addr);
        assert!(add_change_res.is_ok(), "{:?}", add_change_res.err());
    }

    #[test]
    fn build_tx_pay_to_multisig() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(10, 2));
        let spend = root_key_15()
            .derive(harden(1854))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
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

        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred, &stake_cred).to_address();

        tx_builder.add_key_input(
            &spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000))
        );
        tx_builder.add_output(&TransactionOutput::new(
            &addr_net_0,
            &Value::new(&to_bignum(999_000 ))
        )).unwrap();
        tx_builder.set_ttl(1000);
        tx_builder.set_fee(&to_bignum(1_000));

        assert_eq!(tx_builder.outputs.len(),1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );


        let  _final_tx = tx_builder.build().unwrap();
        let _deser_t = TransactionBody::from_bytes(_final_tx.to_bytes()).unwrap();

        assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
    }

    fn build_full_tx(body: &TransactionBody,
        witness_set: &TransactionWitnessSet,
        auxiliary_data: Option<AuxiliaryData>) -> Transaction {
            return Transaction::new(
                body,
                witness_set,
                auxiliary_data
            );
        }

    #[test]
    fn build_tx_multisig_spend_1on1_unsigned() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(10, 2));

        let spend = root_key_15()//multisig
            .derive(harden(1854))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_15()//multisig
            .derive(harden(1854))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();

        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();

        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let addr_multisig = BaseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred, &stake_cred).to_address();
        let addr_output = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();

        tx_builder.add_input(
            &addr_multisig,
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000))
        );

        tx_builder.add_output(&TransactionOutput::new(
            &addr_output,
            &Value::new(&to_bignum(999_000 ))
        )).unwrap();
        tx_builder.set_ttl(1000);
        tx_builder.set_fee(&to_bignum(1_000));

        let mut auxiliary_data = AuxiliaryData::new();
        let mut pubkey_native_scripts = NativeScripts::new();
        let mut oneof_native_scripts = NativeScripts::new();

        let spending_hash = spend.to_raw_key().hash();
        pubkey_native_scripts.add(&NativeScript::new_script_pubkey(&ScriptPubkey::new(&spending_hash)));
        oneof_native_scripts.add(&NativeScript::new_script_n_of_k(&ScriptNOfK::new(1, &pubkey_native_scripts)));
        auxiliary_data.set_native_scripts(&oneof_native_scripts);
        tx_builder.set_auxiliary_data(&auxiliary_data);


        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );


        let  _final_tx = tx_builder.build().unwrap();
        let _deser_t = TransactionBody::from_bytes(_final_tx.to_bytes()).unwrap();

        assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
        assert_eq!(_deser_t.auxiliary_data_hash.unwrap(), utils::hash_auxiliary_data(&auxiliary_data));
    }

    #[test]
    fn build_tx_multisig_1on1_signed() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(10, 2));
        let spend = root_key_15()
            .derive(harden(1854))//multisig
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_15()
            .derive(harden(1854))//multisig
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();

        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred, &stake_cred).to_address();
        tx_builder.add_key_input(
            &spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000))
        );
        tx_builder.add_output(&TransactionOutput::new(
            &addr_net_0,
            &Value::new(&to_bignum(999_000 ))
        )).unwrap();
        tx_builder.set_ttl(1000);
        tx_builder.set_fee(&to_bignum(1_000));

        let mut auxiliary_data = AuxiliaryData::new();
        let mut pubkey_native_scripts = NativeScripts::new();
        let mut oneof_native_scripts = NativeScripts::new();

        let spending_hash = spend.to_raw_key().hash();
        pubkey_native_scripts.add(&NativeScript::new_script_pubkey(&ScriptPubkey::new(&spending_hash)));
        oneof_native_scripts.add(&NativeScript::new_script_n_of_k(&ScriptNOfK::new(1, &pubkey_native_scripts)));
        auxiliary_data.set_native_scripts(&oneof_native_scripts);
        tx_builder.set_auxiliary_data(&auxiliary_data);


        let body = tx_builder.build().unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );

        let mut witness_set = TransactionWitnessSet::new();
        let mut vkw = Vkeywitnesses::new();
        vkw.add(&make_vkey_witness(
            &hash_transaction(&body),
            &PrivateKey::from_normal_bytes(
                &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a").unwrap()
            ).unwrap()
        ));
        witness_set.set_vkeys(&vkw);

        let _final_tx = build_full_tx(&body, &witness_set, None);
        let _deser_t = Transaction::from_bytes(_final_tx.to_bytes()).unwrap();
        assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
        assert_eq!(_deser_t.body().auxiliary_data_hash.unwrap(), utils::hash_auxiliary_data(&auxiliary_data));
    }

    #[test]
    fn add_change_splits_change_into_multiple_outputs_when_nfts_overflow_output_size() {
        let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(1));
        let max_value_size = 100; // super low max output size to test with fewer assets
        let mut tx_builder = TransactionBuilder::new(
            &TransactionBuilderConfigBuilder::new()
                .fee_algo(&linear_fee)
                .pool_deposit(&to_bignum(0))
                .key_deposit(&to_bignum(0))
                .max_value_size(max_value_size)
                .max_tx_size(MAX_TX_SIZE)
                .coins_per_utxo_word(&to_bignum(1))
                .prefer_pure_change(true)
                .build()
                .unwrap()
        );

        let policy_id = PolicyID::from([0u8; 28]);
        let names = [
            AssetName::new(vec![99u8; 32]).unwrap(),
            AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
            AssetName::new(vec![4u8, 5, 6, 7]).unwrap(),
            AssetName::new(vec![5u8, 5, 6, 7]).unwrap(),
            AssetName::new(vec![6u8, 5, 6, 7]).unwrap(),
        ];
        let assets = names
            .iter()
            .fold(Assets::new(), |mut a, name| {
                a.insert(&name, &to_bignum(500));
                a
            });
        let mut multiasset = MultiAsset::new();
        multiasset.insert(&policy_id, &assets);

        let mut input_value = Value::new(&to_bignum(300));
        input_value.set_multiasset(&multiasset);

        tx_builder.add_input(
            &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address(),
            &TransactionInput::new(
                &genesis_id(),
                0
            ),
            &input_value
        );

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap().to_address();
        let output_amount = Value::new(&to_bignum(50));

        tx_builder
            .add_output(&TransactionOutput::new(&output_addr, &output_amount))
            .unwrap();

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();

        let add_change_result = tx_builder.add_change_if_needed(&change_addr);
        assert!(add_change_result.is_ok());
        assert_eq!(tx_builder.outputs.len(), 4);

        let change1 = tx_builder.outputs.get(1);
        let change2 = tx_builder.outputs.get(2);
        let change3 = tx_builder.outputs.get(3);

        assert_eq!(change1.address, change_addr);
        assert_eq!(change1.address, change2.address);
        assert_eq!(change1.address, change3.address);

        assert_eq!(change1.amount.coin, to_bignum(45));
        assert_eq!(change2.amount.coin, to_bignum(42));
        assert_eq!(change3.amount.coin, to_bignum(162));

        assert!(change1.amount.multiasset.is_some());
        assert!(change2.amount.multiasset.is_some());
        assert!(change3.amount.multiasset.is_none()); // purified

        let masset1 = change1.amount.multiasset.unwrap();
        let masset2 = change2.amount.multiasset.unwrap();

        assert_eq!(masset1.keys().len(), 1);
        assert_eq!(masset1.keys(), masset2.keys());

        let asset1 = masset1.get(&policy_id).unwrap();
        let asset2 = masset2.get(&policy_id).unwrap();
        assert_eq!(asset1.len(), 4);
        assert_eq!(asset2.len(), 1);

        names.iter().for_each(|name| {
            let v1 = asset1.get(name);
            let v2 = asset2.get(name);
            assert_ne!(v1.is_some(), v2.is_some());
            assert_eq!(v1.or(v2).unwrap(), to_bignum(500));
        });
    }

    fn create_json_metadatum_string() -> String {
        String::from("{ \"qwe\": 123 }")
    }

    fn create_json_metadatum() -> TransactionMetadatum {
        encode_json_str_to_metadatum(
            create_json_metadatum_string(),
            MetadataJsonSchema::NoConversions,
        ).unwrap()
    }

    fn create_aux_with_metadata(metadatum_key: &TransactionMetadatumLabel) -> AuxiliaryData {
        let mut metadata = GeneralTransactionMetadata::new();
        metadata.insert(metadatum_key, &create_json_metadatum());

        let mut aux = AuxiliaryData::new();
        aux.set_metadata(&metadata);

        let mut nats = NativeScripts::new();
        nats.add(
            &NativeScript::new_timelock_start(
                &TimelockStart::new(123),
            ),
        );
        aux.set_native_scripts(&nats);

        return aux;
    }

    fn assert_json_metadatum(dat: &TransactionMetadatum) {
        let map = dat.as_map().unwrap();
        assert_eq!(map.len(), 1);
        let key = TransactionMetadatum::new_text(String::from("qwe")).unwrap();
        let val = map.get(&key).unwrap();
        assert_eq!(val.as_int().unwrap(), Int::new_i32(123));
    }

    #[test]
    fn set_metadata_with_empty_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num = to_bignum(42);
        tx_builder.set_metadata(&create_aux_with_metadata(&num).metadata().unwrap());

        assert!(tx_builder.auxiliary_data.is_some());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();

        assert_eq!(met.len(), 1);
        assert_json_metadatum(&met.get(&num).unwrap());
    }

    #[test]
    fn set_metadata_with_existing_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num1 = to_bignum(42);
        tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

        let num2 = to_bignum(84);
        tx_builder.set_metadata(&create_aux_with_metadata(&num2).metadata().unwrap());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_some());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.len(), 1);
        assert!(met.get(&num1).is_none());
        assert_json_metadatum(&met.get(&num2).unwrap());
    }

    #[test]
    fn add_metadatum_with_empty_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num = to_bignum(42);
        tx_builder.add_metadatum(&num, &create_json_metadatum());

        assert!(tx_builder.auxiliary_data.is_some());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();

        assert_eq!(met.len(), 1);
        assert_json_metadatum(&met.get(&num).unwrap());
    }

    #[test]
    fn add_metadatum_with_existing_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num1 = to_bignum(42);
        tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

        let num2 = to_bignum(84);
        tx_builder.add_metadatum(&num2, &create_json_metadatum());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_some());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.len(), 2);
        assert_json_metadatum(&met.get(&num1).unwrap());
        assert_json_metadatum(&met.get(&num2).unwrap());
    }

    #[test]
    fn add_json_metadatum_with_empty_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num = to_bignum(42);
        tx_builder.add_json_metadatum(&num, create_json_metadatum_string()).unwrap();

        assert!(tx_builder.auxiliary_data.is_some());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();

        assert_eq!(met.len(), 1);
        assert_json_metadatum(&met.get(&num).unwrap());
    }

    #[test]
    fn add_json_metadatum_with_existing_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num1 = to_bignum(42);
        tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

        let num2 = to_bignum(84);
        tx_builder.add_json_metadatum(&num2, create_json_metadatum_string()).unwrap();

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_some());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.len(), 2);
        assert_json_metadatum(&met.get(&num1).unwrap());
        assert_json_metadatum(&met.get(&num2).unwrap());
    }

    fn create_asset_name() -> AssetName {
        AssetName::new(vec![0u8, 1, 2, 3]).unwrap()
    }

    fn create_mint_asset() -> MintAssets {
        MintAssets::new_from_entry(&create_asset_name(), Int::new_i32(1234))
    }

    fn create_assets() -> Assets {
        let mut assets = Assets::new();
        assets.insert(&create_asset_name(), &to_bignum(1234));
        return assets;
    }

    fn create_mint_with_one_asset(policy_id: &PolicyID) -> Mint {
        Mint::new_from_entry(policy_id, &create_mint_asset())
    }

    fn create_multiasset_one_asset(policy_id: &PolicyID) -> MultiAsset {
        let mut mint = MultiAsset::new();
        mint.insert(policy_id, &create_assets());
        return mint;
    }

    fn assert_mint_asset(mint: &Mint, policy_id: &PolicyID) {
        assert!(mint.get(&policy_id).is_some());
        let result_asset = mint.get(&policy_id).unwrap();
        assert_eq!(result_asset.len(), 1);
        assert_eq!(result_asset.get(&create_asset_name()).unwrap(), Int::new_i32(1234));
    }

    fn mint_script_and_policy(x: u8) -> (NativeScript, PolicyID) {
        let mint_script = NativeScript::new_script_pubkey(
            &ScriptPubkey::new(&fake_key_hash(x))
        );
        let policy_id = mint_script.hash(ScriptHashNamespace::NativeScript);
        (mint_script, policy_id)
    }

    #[test]
    fn set_mint_asset_with_empty_mint() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script, policy_id) = mint_script_and_policy(0);
        tx_builder.set_mint_asset(&mint_script, &create_mint_asset());

        assert!(tx_builder.mint.is_some());
        assert!(tx_builder.mint_scripts.is_some());

        let mint = tx_builder.mint.unwrap();
        let mint_scripts = tx_builder.mint_scripts.unwrap();

        assert_eq!(mint.len(), 1);
        assert_mint_asset(&mint, &policy_id);

        assert_eq!(mint_scripts.len(), 1);
        assert_eq!(mint_scripts.get(0), mint_script);
    }

    #[test]
    fn set_mint_asset_with_existing_mint() {
        let mut tx_builder = create_default_tx_builder();

        let (_, policy_id1) = mint_script_and_policy(0);
        let (mint_script2, policy_id2) = mint_script_and_policy(1);

        tx_builder.set_mint(&create_mint_with_one_asset(&policy_id1));

        tx_builder.set_mint_asset(&mint_script2, &create_mint_asset());

        assert!(tx_builder.mint.is_some());
        assert!(tx_builder.mint_scripts.is_some());

        let mint = tx_builder.mint.unwrap();
        let mint_scripts = tx_builder.mint_scripts.unwrap();

        assert_eq!(mint.len(), 2);
        assert_mint_asset(&mint, &policy_id1);
        assert_mint_asset(&mint, &policy_id2);

        // Only second script is present in the scripts
        assert_eq!(mint_scripts.len(), 1);
        assert_eq!(mint_scripts.get(0), mint_script2);
    }

    #[test]
    fn add_mint_asset_with_empty_mint() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script, policy_id) = mint_script_and_policy(0);

        tx_builder.add_mint_asset(&mint_script, &create_asset_name(), Int::new_i32(1234));

        assert!(tx_builder.mint.is_some());
        assert!(tx_builder.mint_scripts.is_some());

        let mint = tx_builder.mint.unwrap();
        let mint_scripts = tx_builder.mint_scripts.unwrap();

        assert_eq!(mint.len(), 1);
        assert_mint_asset(&mint, &policy_id);

        assert_eq!(mint_scripts.len(), 1);
        assert_eq!(mint_scripts.get(0), mint_script);
    }

    #[test]
    fn add_mint_asset_with_existing_mint() {
        let mut tx_builder = create_default_tx_builder();

        let (_, policy_id1) = mint_script_and_policy(0);
        let (mint_script2, policy_id2) = mint_script_and_policy(1);

        tx_builder.set_mint(&create_mint_with_one_asset(&policy_id1));
        tx_builder.add_mint_asset(&mint_script2, &create_asset_name(), Int::new_i32(1234));

        assert!(tx_builder.mint.is_some());
        assert!(tx_builder.mint_scripts.is_some());

        let mint = tx_builder.mint.unwrap();
        let mint_scripts = tx_builder.mint_scripts.unwrap();

        assert_eq!(mint.len(), 2);
        assert_mint_asset(&mint, &policy_id1);
        assert_mint_asset(&mint, &policy_id2);

        // Only second script is present in the scripts
        assert_eq!(mint_scripts.len(), 1);
        assert_eq!(mint_scripts.get(0), mint_script2);
    }

    #[test]
    fn add_output_amount() {
        let mut tx_builder = create_default_tx_builder();

        let policy_id1 = PolicyID::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);
        let mut value = Value::new(&to_bignum(42));
        value.set_multiasset(&multiasset);

        let address = byron_address();
        tx_builder.add_output_amount(&address, &value).unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount, value);
    }

    #[test]
    fn add_output_coin() {
        let mut tx_builder = create_default_tx_builder();

        let address = byron_address();
        let coin = to_bignum(43);
        tx_builder.add_output_coin(&address, &coin).unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.coin, coin);
        assert!(out.amount.multiasset.is_none());
    }

    #[test]
    fn add_output_coin_and_multiasset() {
        let mut tx_builder = create_default_tx_builder();

        let policy_id1 = PolicyID::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);

        let address = byron_address();
        let coin = to_bignum(42);

        tx_builder.add_output_coin_and_asset(&address, &coin, &multiasset).unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.coin, coin);
        assert_eq!(out.amount.multiasset.unwrap(), multiasset);
    }

    #[test]
    fn add_output_asset_and_min_required_coin() {
        let mut tx_builder = create_reallistic_tx_builder();

        let policy_id1 = PolicyID::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);

        let address = byron_address();
        tx_builder.add_output_asset_and_min_required_coin(&address, &multiasset).unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.multiasset.unwrap(), multiasset);
        assert_eq!(out.amount.coin, to_bignum(1344798));
    }

    #[test]
    fn add_mint_asset_and_output() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script0, policy_id0) = mint_script_and_policy(0);
        let (mint_script1, policy_id1) = mint_script_and_policy(1);

        let name = create_asset_name();
        let amount = Int::new_i32(1234);

        let address = byron_address();
        let coin = to_bignum(100);

        // Add unrelated mint first to check it is NOT added to output later
        tx_builder.add_mint_asset(&mint_script0, &name, amount.clone());

        tx_builder.add_mint_asset_and_output(
            &mint_script1,
            &name,
            amount.clone(),
            &address,
            &coin,
        ).unwrap();

        assert!(tx_builder.mint.is_some());
        assert!(tx_builder.mint_scripts.is_some());

        let mint = tx_builder.mint.as_ref().unwrap();
        let mint_scripts = tx_builder.mint_scripts.as_ref().unwrap();

        // Mint contains two entries
        assert_eq!(mint.len(), 2);
        assert_mint_asset(mint, &policy_id0);
        assert_mint_asset(mint, &policy_id1);

        assert_eq!(mint_scripts.len(), 2);
        assert_eq!(mint_scripts.get(0), mint_script0);
        assert_eq!(mint_scripts.get(1), mint_script1);

        // One new output is created
        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.coin, coin);

        let multiasset = out.amount.multiasset.unwrap();

        // Only second mint entry was added to the output
        assert_eq!(multiasset.len(), 1);
        assert!(multiasset.get(&policy_id0).is_none());
        assert!(multiasset.get(&policy_id1).is_some());

        let asset = multiasset.get(&policy_id1).unwrap();
        assert_eq!(asset.len(), 1);
        assert_eq!(asset.get(&name).unwrap(), to_bignum(1234));
    }

    #[test]
    fn add_mint_asset_and_min_required_coin() {
        let mut tx_builder = create_reallistic_tx_builder();

        let (mint_script0, policy_id0) = mint_script_and_policy(0);
        let (mint_script1, policy_id1) = mint_script_and_policy(1);

        let name = create_asset_name();
        let amount = Int::new_i32(1234);

        let address = byron_address();

        // Add unrelated mint first to check it is NOT added to output later
        tx_builder.add_mint_asset(&mint_script0, &name, amount.clone());

        tx_builder.add_mint_asset_and_output_min_required_coin(
            &mint_script1,
            &name,
            amount.clone(),
            &address,
        ).unwrap();

        assert!(tx_builder.mint.is_some());
        assert!(tx_builder.mint_scripts.is_some());

        let mint = tx_builder.mint.as_ref().unwrap();
        let mint_scripts = tx_builder.mint_scripts.as_ref().unwrap();

        // Mint contains two entries
        assert_eq!(mint.len(), 2);
        assert_mint_asset(mint, &policy_id0);
        assert_mint_asset(mint, &policy_id1);


        assert_eq!(mint_scripts.len(), 2);
        assert_eq!(mint_scripts.get(0), mint_script0);
        assert_eq!(mint_scripts.get(1), mint_script1);

        // One new output is created
        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.coin, to_bignum(1344798));

        let multiasset = out.amount.multiasset.unwrap();

        // Only second mint entry was added to the output
        assert_eq!(multiasset.len(), 1);
        assert!(multiasset.get(&policy_id0).is_none());
        assert!(multiasset.get(&policy_id1).is_some());

        let asset = multiasset.get(&policy_id1).unwrap();
        assert_eq!(asset.len(), 1);
        assert_eq!(asset.get(&name).unwrap(), to_bignum(1234));
    }


    // #[test]
    // fn add_mint_includes_witnesses_into_fee_estimation() {
    //     let mut tx_builder = create_reallistic_tx_builder();
    //
    //     let original_tx_fee = tx_builder.min_fee().unwrap();
    //     assert_eq!(original_tx_fee, to_bignum(156217));
    //
    //     let policy_id1 = PolicyID::from([0u8; 28]);
    //     let policy_id2 = PolicyID::from([1u8; 28]);
    //     let policy_id3 = PolicyID::from([2u8; 28]);
    //     let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
    //     let name2 = AssetName::new(vec![1u8, 1, 2, 3]).unwrap();
    //     let name3 = AssetName::new(vec![2u8, 1, 2, 3]).unwrap();
    //     let name4 = AssetName::new(vec![3u8, 1, 2, 3]).unwrap();
    //     let amount = Int::new_i32(1234);
    //
    //     let mut mint = Mint::new();
    //     mint.insert(
    //         &policy_id1,
    //         &MintAssets::new_from_entry(&name1, amount.clone()),
    //     );
    //     mint.insert(
    //         &policy_id2,
    //         &MintAssets::new_from_entry(&name2, amount.clone()),
    //     );
    //     // Third policy with two asset names
    //     let mut mass = MintAssets::new_from_entry(&name3, amount.clone());
    //     mass.insert(&name4, amount.clone());
    //     mint.insert(&policy_id3, &mass);
    //
    //     let mint_len = mint.to_bytes().len();
    //     let fee_coefficient = tx_builder.config.fee_algo.coefficient();
    //
    //     let raw_mint_fee = fee_coefficient
    //         .checked_mul(&to_bignum(mint_len as u64))
    //         .unwrap();
    //
    //     assert_eq!(raw_mint_fee, to_bignum(5544));
    //
    //     tx_builder.set_mint(&mint);
    //
    //     let new_tx_fee = tx_builder.min_fee().unwrap();
    //
    //     let fee_diff_from_adding_mint =
    //         new_tx_fee.checked_sub(&original_tx_fee)
    //         .unwrap();
    //
    //     let witness_fee_increase =
    //         fee_diff_from_adding_mint.checked_sub(&raw_mint_fee)
    //         .unwrap();
    //
    //     assert_eq!(witness_fee_increase, to_bignum(4356));
    //
    //     let fee_increase_bytes = from_bignum(&witness_fee_increase)
    //         .checked_div(from_bignum(&fee_coefficient))
    //         .unwrap();
    //
    //     // Three policy IDs of 32 bytes each + 3 byte overhead
    //     assert_eq!(fee_increase_bytes, 99);
    // }

    #[test]
    fn total_input_with_mint_and_burn() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
        let spend = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();

        let (mint_script1, policy_id1) = mint_script_and_policy(0);
        let (mint_script2, policy_id2) = mint_script_and_policy(1);

        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let ma_input1 = 100;
        let ma_input2 = 200;
        let ma_output1 = 60;

        let multiassets = [ma_input1, ma_input2, ma_output1]
            .iter()
            .map(|input| {
                let mut multiasset = MultiAsset::new();
                multiasset.insert(&policy_id1, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(*input));
                    assets
                });
                multiasset.insert(&policy_id2, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(*input));
                    assets
                });
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets
            .iter()
            .zip([100u64, 100, 100].iter().cloned().map(to_bignum))
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            tx_builder.add_key_input(
                &&spend.to_raw_key().hash(),
                &TransactionInput::new(&genesis_id(), 0),
                &input_amount,
            );
        }

        let total_input_before_mint = tx_builder.get_total_input().unwrap();

        assert_eq!(total_input_before_mint.coin, to_bignum(300));
        let ma1 = total_input_before_mint.multiasset.unwrap();
        assert_eq!(ma1.get(&policy_id1).unwrap().get(&name).unwrap(), to_bignum(360));
        assert_eq!(ma1.get(&policy_id2).unwrap().get(&name).unwrap(), to_bignum(360));

        tx_builder.add_mint_asset(&mint_script1, &name, Int::new_i32(40));
        tx_builder.add_mint_asset(&mint_script2, &name, Int::new_i32(-40));

        let total_input_after_mint = tx_builder.get_total_input().unwrap();

        assert_eq!(total_input_after_mint.coin, to_bignum(300));
        let ma2 = total_input_after_mint.multiasset.unwrap();
        assert_eq!(ma2.get(&policy_id1).unwrap().get(&name).unwrap(), to_bignum(400));
        assert_eq!(ma2.get(&policy_id2).unwrap().get(&name).unwrap(), to_bignum(320));
    }

}

