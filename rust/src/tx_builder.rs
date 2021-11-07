use super::*;
use super::fees;
use super::utils;
use std::collections::{BTreeMap, BTreeSet};

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
    let script_keys = match tx_builder.input_types.scripts.len() {
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
    let witness_set = TransactionWitnessSet {
        vkeys: vkeys,
        native_scripts: script_keys,
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
    let full_tx = fake_full_tx(tx_builder, tx_builder.build()?)?;
    fees::min_fee(&full_tx, &tx_builder.fee_algo)
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
pub struct TransactionBuilder {
    coins_per_utxo_word: BigNum,
    pool_deposit: BigNum,
    key_deposit: BigNum,
    max_value_size: u32,
    max_tx_size: u32,
    fee_algo: fees::LinearFee,
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
    inputs_auto_added: bool,
    prefer_pure_change: bool,
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
        let mut input_total = self
            .get_explicit_input()?
            .checked_add(&self.get_implicit_input()?)?;
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
                        let random_index = rng.gen_range(0..available_inputs.len());
                        let input = available_inputs.swap_remove(random_index);
                        // differing from CIP2, we include the needed fees in the targets instead of just output values
                        let input_fee = self.fee_for_input(&input.output.address, &input.input, &input.output.amount)?;
                        self.add_input(&input.output.address, &input.input, &input.output.amount);
                        input_total = input_total.checked_add(&input.output.amount)?;
                        output_total = output_total.checked_add(&Value::new(&input_fee))?;
                        needed = needed.checked_add(&Value::new(&input_fee))?;
                        added = added.checked_add(&input.output.amount)?;
                        associated_inputs.entry(output.clone()).or_default().push(input);
                    }
                }
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

    pub fn add_output(&mut self, output: &TransactionOutput) -> Result<(), JsError> {
        let value_size = output.amount.to_bytes().len();
        if value_size > self.max_value_size as usize {
            return Err(JsError::from_str(&format!(
                "Maximum value size of {} exceeded. Found: {}",
                self.max_value_size,
                value_size
            )));
        }
        let min_ada = min_ada_required(&output.amount(), false, &self.coins_per_utxo_word)?;
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

    pub fn set_auxiliary_data(&mut self, auxiliary_data: &AuxiliaryData) {
        self.auxiliary_data = Some(auxiliary_data.clone())
    }

    pub fn set_prefer_pure_change(&mut self, prefer_pure_change: bool) {
        self.prefer_pure_change = prefer_pure_change;
    }

    pub fn new(
        linear_fee: &fees::LinearFee,
        pool_deposit: &BigNum, // protocol parameter
        key_deposit: &BigNum,  // protocol parameter
        max_value_size: u32, // protocol parameter
        max_tx_size: u32, // protocol parameter
        coins_per_utxo_word: &Coin, // protocol parameter
    ) -> Self {
        Self {
            coins_per_utxo_word: coins_per_utxo_word.clone(),
            key_deposit: key_deposit.clone(),
            pool_deposit: pool_deposit.clone(),
            max_value_size,
            max_tx_size,
            fee_algo: linear_fee.clone(),
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
            inputs_auto_added: false,
            prefer_pure_change: false,
        }
    }

    /// does not include refunds or withdrawals
    pub fn get_explicit_input(&self) -> Result<Value, JsError> {
        self.inputs
            .iter()
            .try_fold(Value::new(&to_bignum(0)), |acc, ref tx_builder_input| {
                acc.checked_add(&tx_builder_input.amount)
            })
    }
    /// withdrawals and refunds
    pub fn get_implicit_input(&self) -> Result<Value, JsError> {
        internal_get_implicit_input(
            &self.withdrawals,
            &self.certs,
            &self.pool_deposit,
            &self.key_deposit,
        )
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
            &self.pool_deposit,
            &self.key_deposit,
        )
    }

    pub fn get_fee_if_set(&self) -> Option<Coin> {
        self.fee.clone()
    }

    /// Warning: this function will mutate the /fee/ field
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

        let input_total = self
            .get_explicit_input()?
            .checked_add(&self.get_implicit_input()?)?;

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
                    fn pack_nfts_for_change(max_value_size: u32, change_address: &Address, change_estimator: &Value) -> Result<MultiAsset, JsError> {
                        // we insert the entire available ADA temporarily here since that could potentially impact the size
                        // as it could be 1, 2 3 or 4 bytes for Coin.
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
                            let old_amount = output.amount.clone();
                            let mut val = Value::new(&Coin::zero());
                            let mut next_nft = MultiAsset::new();
                            next_nft.insert(policy, assets);
                            val.set_multiasset(&next_nft);
                            output.amount = output.amount.checked_add(&val)?;
                            if output.amount.to_bytes().len() > max_value_size as usize {
                                output.amount = old_amount;
                                break;
                            }
                        }
                        Ok(output.amount.multiasset().unwrap())
                    }
                    let mut change_left = input_total.checked_sub(&output_total)?;
                    let mut new_fee = fee.clone();
                    // we might need multiple change outputs for cases where the change has many asset types
                    // which surpass the max UTXO size limit
                    let minimum_utxo_val = min_pure_ada(&self.coins_per_utxo_word)?;
                    while let Some(Ordering::Greater) = change_left.multiasset.as_ref().map_or_else(|| None, |ma| ma.partial_cmp(&MultiAsset::new())) {
                        let nft_change = pack_nfts_for_change(self.max_value_size, address, &change_left)?;
                        if nft_change.len() == 0 {
                            // this likely should never happen
                            return Err(JsError::from_str("NFTs too large for change output"));
                        }
                        // we only add the minimum needed (for now) to cover this output
                        let mut change_value = Value::new(&Coin::zero());
                        change_value.set_multiasset(&nft_change);
                        let min_ada = min_ada_required(&change_value, false, &self.coins_per_utxo_word)?;
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
                    change_left = change_left.checked_sub(&Value::new(&new_fee))?;
                    // add potentially a separate pure ADA change output
                    let left_above_minimum = change_left.coin.compare(&minimum_utxo_val) > 0;
                    if self.prefer_pure_change && left_above_minimum {
                        let pure_output = TransactionOutput::new(address, &change_left);
                        let additional_fee = self.fee_for_output(&pure_output)?;
                        let potential_pure_value = change_left.checked_sub(&Value::new(&additional_fee))?;
                        let potential_pure_above_minimum = potential_pure_value.coin.compare(&minimum_utxo_val) > 0;
                        if potential_pure_above_minimum {
                            new_fee = new_fee.checked_add(&additional_fee)?;
                            change_left = Value::zero();
                            self.add_output(&TransactionOutput::new(address, &potential_pure_value));
                        }
                    }
                    self.set_fee(&new_fee);
                    // add in the rest of the ADA
                    if !change_left.is_zero() {
                        self.outputs.0.last_mut().unwrap().amount = self.outputs.0.last().unwrap().amount.checked_add(&change_left)?;
                    }
                    Ok(true)
                } else {
                    let min_ada = min_ada_required(&change_estimator, false, &self.coins_per_utxo_word)?;
                    // no-asset case so we have no problem burning the rest if there is no other option
                    fn burn_extra(builder: &mut TransactionBuilder, burn_amount: &BigNum) -> Result<bool, JsError> {
                        // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
                        builder.set_fee(burn_amount);
                        Ok(false) // not enough input to covert the extra fee from adding an output so we just burn whatever is left
                    };
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

    pub fn build(&self) -> Result<TransactionBody, JsError> {
        let (body, full_tx_size) = self.build_and_size()?;
        if full_tx_size > self.max_tx_size as usize {
            Err(JsError::from_str(&format!(
                "Maximum transaction size of {} exceeded. Found: {}",
                self.max_tx_size,
                full_tx_size
            )))
        } else {
            Ok(body)
        }
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

    #[test]
    fn build_tx_with_change() {
        let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(1),
            &to_bignum(1),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &to_bignum(1),
        );
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
        let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(1),
            &to_bignum(1),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &to_bignum(1),
        );
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
        let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(1),
            &to_bignum(1_000_000),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &to_bignum(1),
        );
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
        let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(0));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(0),
            &to_bignum(0),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &to_bignum(1),
        );
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
        let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(0));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(0),
            &to_bignum(0),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &to_bignum(1)
        );
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
        let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(0));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(0),
            &to_bignum(5),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &to_bignum(1)
        );
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
        let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(1),
            &to_bignum(1),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &to_bignum(1)
        );
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
    fn build_tx_with_native_assets_change() {
        let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(1));
        let coins_per_utxo_word = to_bignum(1);
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(0),
            &to_bignum(0),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &coins_per_utxo_word,
        );
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
        let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(1));
        let minimum_utxo_value = to_bignum(1);
        let coin_per_utxo_word = to_bignum(1);
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(0),
            &to_bignum(0),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &coin_per_utxo_word,
        );
        // Prefer pure change!
        tx_builder.set_prefer_pure_change(true);
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
        let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(1));
        let minimum_utxo_value = to_bignum(10);
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(0),
            &to_bignum(0),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &to_bignum(1),
        );
        // Prefer pure change!
        tx_builder.set_prefer_pure_change(true);
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
        let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(1),
            &to_bignum(1),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &to_bignum(1)
        );
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
        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(500000000),
            &to_bignum(2000000),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            // with this mainnet value we should end up with a final min_ada_required of just under 1_000_000
            &to_bignum(COINS_PER_UTXO_WORD),
        );

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
        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(500000000),
            &to_bignum(2000000),
            MAX_VALUE_SIZE,
            MAX_TX_SIZE,
            &to_bignum(COINS_PER_UTXO_WORD)
        );

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
        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let mut tx_builder =
            TransactionBuilder::new(
                &linear_fee,
                &to_bignum(500000000),
                &to_bignum(2000000),
                MAX_VALUE_SIZE,
                MAX_TX_SIZE,
                &to_bignum(COINS_PER_UTXO_WORD)
            );

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
        let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(1));
        let max_value_size = 100; // super low max output size to test with fewer assets
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(0),
            &to_bignum(0),
            max_value_size,
            MAX_TX_SIZE,
            &to_bignum(1)
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
        let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(1));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(0),
            &to_bignum(0),
            10, // super low max output size to test,
            MAX_TX_SIZE,
            &to_bignum(1)
        );

        tx_builder.add_input(
            &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address(),
            &TransactionInput::new(
                &genesis_id(),
                0
            ),
            &Value::new(&to_bignum(10))
        );

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap().to_address();
        let mut output_amount = Value::new(&to_bignum(1));
        output_amount.set_multiasset(&create_multiasset().0);

        assert!(tx_builder.add_output(&TransactionOutput::new(&output_addr, &output_amount)).is_err());
    }

    #[test]
    fn build_tx_add_change_nfts_not_enough_ada() {
        let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(1));
        let max_value_size = 150; // super low max output size to test with fewer assets
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &to_bignum(0),
            &to_bignum(0),
            max_value_size,
            MAX_TX_SIZE,
            &to_bignum(1)
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
        let output_amount = Value::new(&to_bignum(29));

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
        let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(0));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &Coin::zero(),
            &to_bignum(0),
            9999,
            9999,
            &to_bignum(0),
        );
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
        let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(0));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &Coin::zero(),
            &to_bignum(0),
            9999,
            9999,
            &to_bignum(0),
        );
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
        let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(0));
        let mut tx_builder = TransactionBuilder::new(
            &linear_fee,
            &Coin::zero(),
            &to_bignum(0),
            9999,
            9999,
            &to_bignum(0),
        );
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

    fn build_tx_pay_to_multisig() {
        let linear_fee = LinearFee::new(&to_bignum(10), &to_bignum(2));
        let mut tx_builder =
            TransactionBuilder::new(&linear_fee, &to_bignum(1), &to_bignum(1), MAX_VALUE_SIZE, MAX_TX_SIZE, &to_bignum(1));
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
        let linear_fee = LinearFee::new(&to_bignum(10), &to_bignum(2));
        let mut tx_builder =
            TransactionBuilder::new(&linear_fee, &to_bignum(1), &to_bignum(1), MAX_VALUE_SIZE, MAX_TX_SIZE, &to_bignum(1));

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


        let body = tx_builder.build().unwrap();

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
        let linear_fee = LinearFee::new(&to_bignum(10), &to_bignum(2));
        let mut tx_builder =
            TransactionBuilder::new(&linear_fee, &to_bignum(1), &to_bignum(1), MAX_VALUE_SIZE, MAX_TX_SIZE, &to_bignum(1));
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
}
