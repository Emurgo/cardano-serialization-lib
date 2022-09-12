#![allow(deprecated)]

pub mod tx_inputs_builder;

use super::fees;
use super::output_builder::TransactionOutputAmountBuilder;
use super::utils;
use super::*;
use crate::tx_builder::tx_inputs_builder::{get_bootstraps, TxInputsBuilder};
use linked_hash_map::LinkedHashMap;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use tx_inputs_builder::{PlutusWitness, PlutusWitnesses};

// comes from witsVKeyNeeded in the Ledger spec
fn witness_keys_for_cert(cert_enum: &Certificate) -> RequiredSigners {
    let mut set = RequiredSigners::new();
    match &cert_enum.0 {
        // stake key registrations do not require a witness
        CertificateEnum::StakeRegistration(_cert) => {}
        CertificateEnum::StakeDeregistration(cert) => {
            set.add(&cert.stake_credential().to_keyhash().unwrap());
        }
        CertificateEnum::StakeDelegation(cert) => {
            set.add(&cert.stake_credential().to_keyhash().unwrap());
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

fn fake_private_key() -> Bip32PrivateKey {
    Bip32PrivateKey::from_bytes(&[
        0xb8, 0xf2, 0xbe, 0xce, 0x9b, 0xdf, 0xe2, 0xb0, 0x28, 0x2f, 0x5b, 0xad, 0x70, 0x55, 0x62,
        0xac, 0x99, 0x6e, 0xfb, 0x6a, 0xf9, 0x6b, 0x64, 0x8f, 0x44, 0x45, 0xec, 0x44, 0xf4, 0x7a,
        0xd9, 0x5c, 0x10, 0xe3, 0xd7, 0x2f, 0x26, 0xed, 0x07, 0x54, 0x22, 0xa3, 0x6e, 0xd8, 0x58,
        0x5c, 0x74, 0x5a, 0x0e, 0x11, 0x50, 0xbc, 0xce, 0xba, 0x23, 0x57, 0xd0, 0x58, 0x63, 0x69,
        0x91, 0xf3, 0x8a, 0x37, 0x91, 0xe2, 0x48, 0xde, 0x50, 0x9c, 0x07, 0x0d, 0x81, 0x2a, 0xb2,
        0xfd, 0xa5, 0x78, 0x60, 0xac, 0x87, 0x6b, 0xc4, 0x89, 0x19, 0x2c, 0x1e, 0xf4, 0xce, 0x25,
        0x3c, 0x19, 0x7e, 0xe2, 0x19, 0xa4,
    ])
    .unwrap()
}

fn fake_raw_key_sig() -> Ed25519Signature {
    Ed25519Signature::from_bytes(vec![
        36, 248, 153, 211, 155, 23, 253, 93, 102, 193, 146, 196, 181, 13, 52, 62, 66, 247, 35, 91,
        48, 80, 76, 138, 231, 97, 159, 147, 200, 40, 220, 109, 206, 69, 104, 221, 105, 23, 124, 85,
        24, 40, 73, 45, 119, 122, 103, 39, 253, 102, 194, 251, 204, 189, 168, 194, 174, 237, 146,
        3, 44, 153, 121, 10,
    ])
    .unwrap()
}

fn fake_raw_key_public() -> PublicKey {
    PublicKey::from_bytes(&[
        207, 118, 57, 154, 33, 13, 232, 114, 14, 159, 168, 148, 228, 94, 65, 226, 154, 181, 37,
        227, 11, 196, 2, 128, 28, 7, 98, 80, 209, 88, 91, 205,
    ])
    .unwrap()
}

fn count_needed_vkeys(tx_builder: &TransactionBuilder) -> usize {
    let mut input_hashes: RequiredSignersSet = RequiredSignersSet::from(&tx_builder.inputs);
    input_hashes.extend(RequiredSignersSet::from(&tx_builder.collateral));
    input_hashes.extend(RequiredSignersSet::from(&tx_builder.required_signers));
    if let Some(scripts) = &tx_builder.mint_scripts {
        input_hashes.extend(RequiredSignersSet::from(scripts));
    }
    input_hashes.len()
}

// tx_body must be the result of building from tx_builder
// constructs the rest of the Transaction using fake witness data of the correct length
// for use in calculating the size of the final Transaction
fn fake_full_tx(
    tx_builder: &TransactionBuilder,
    body: TransactionBody,
) -> Result<Transaction, JsError> {
    let fake_key_root = fake_private_key();
    let raw_key_public = fake_raw_key_public();
    let fake_sig = fake_raw_key_sig();

    // recall: this includes keys for input, certs and withdrawals
    let vkeys = match count_needed_vkeys(tx_builder) {
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
    let bootstraps = get_bootstraps(&tx_builder.inputs);
    let bootstrap_keys = match bootstraps.len() {
        0 => None,
        _x => {
            let mut result = BootstrapWitnesses::new();
            for addr in bootstraps {
                // picking icarus over daedalus for fake witness generation shouldn't matter
                result.add(&make_icarus_bootstrap_witness(
                    &TransactionHash::from([0u8; TransactionHash::BYTE_COUNT]),
                    &ByronAddress::from_bytes(addr.clone()).unwrap(),
                    &fake_key_root,
                ));
            }
            Some(result)
        }
    };
    let (plutus_scripts, plutus_data, redeemers) = {
        if let Some(s) = tx_builder.get_combined_plutus_scripts() {
            let (s, d, r) = s.collect();
            (Some(s), Some(d), Some(r))
        } else {
            (None, None, None)
        }
    };
    let witness_set = TransactionWitnessSet {
        vkeys,
        native_scripts: tx_builder.get_combined_native_scripts(),
        bootstraps: bootstrap_keys,
        plutus_scripts,
        plutus_data,
        redeemers,
    };
    Ok(Transaction {
        body,
        witness_set,
        is_valid: true,
        auxiliary_data: tx_builder.auxiliary_data.clone(),
    })
}

fn assert_required_mint_scripts(
    mint: &Mint,
    maybe_mint_scripts: Option<&NativeScripts>,
) -> Result<(), JsError> {
    if maybe_mint_scripts.is_none_or_empty() {
        return Err(JsError::from_str(
            "Mint is present in the builder, but witness scripts are not provided!",
        ));
    }
    let mint_scripts = maybe_mint_scripts.unwrap();
    let witness_hashes: HashSet<ScriptHash> =
        mint_scripts.0.iter().map(|script| script.hash()).collect();
    for mint_hash in mint.keys().0.iter() {
        if !witness_hashes.contains(mint_hash) {
            return Err(JsError::from_str(&format!(
                "No witness script is found for mint policy '{:?}'! Script is required!",
                hex::encode(mint_hash.to_bytes()),
            )));
        }
    }
    Ok(())
}

fn min_fee(tx_builder: &TransactionBuilder) -> Result<Coin, JsError> {
    // Commented out for performance, `min_fee` is a critical function
    // This was mostly added here as a paranoid step anyways
    // If someone is using `set_mint` and `add_mint*` API function, everything is expected to be intact
    // TODO: figure out if assert is needed here and a better way to do it maybe only once if mint doesn't change
    // if let Some(mint) = tx_builder.mint.as_ref() {
    //     assert_required_mint_scripts(mint, tx_builder.mint_scripts.as_ref())?;
    // }
    let full_tx = fake_full_tx(tx_builder, tx_builder.build()?)?;
    let fee: Coin = fees::min_fee(&full_tx, &tx_builder.config.fee_algo)?;
    if let Some(ex_unit_prices) = &tx_builder.config.ex_unit_prices {
        let script_fee: Coin = fees::min_script_fee(&full_tx, &ex_unit_prices)?;
        return fee.checked_add(&script_fee);
    }
    if tx_builder.has_plutus_inputs() {
        return Err(JsError::from_str(
            "Plutus inputs are present but ex unit prices are missing in the config!",
        ));
    }
    Ok(fee)
}

#[wasm_bindgen]
pub enum CoinSelectionStrategyCIP2 {
    /// Performs CIP2's Largest First ada-only selection. Will error if outputs contain non-ADA assets.
    LargestFirst,
    /// Performs CIP2's Random Improve ada-only selection. Will error if outputs contain non-ADA assets.
    RandomImprove,
    /// Same as LargestFirst, but before adding ADA, will insert by largest-first for each asset type.
    LargestFirstMultiAsset,
    /// Same as RandomImprove, but before adding ADA, will insert by random-improve for each asset type.
    RandomImproveMultiAsset,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilderConfig {
    fee_algo: fees::LinearFee,
    pool_deposit: Coin,                   // protocol parameter
    key_deposit: Coin,                    // protocol parameter
    max_value_size: u32,                  // protocol parameter
    max_tx_size: u32,                     // protocol parameter
    data_cost: DataCost,                  // protocol parameter
    ex_unit_prices: Option<ExUnitPrices>, // protocol parameter
    prefer_pure_change: bool,
}

impl TransactionBuilderConfig {
    fn utxo_cost(&self) -> DataCost {
        self.data_cost.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilderConfigBuilder {
    fee_algo: Option<fees::LinearFee>,
    pool_deposit: Option<Coin>,           // protocol parameter
    key_deposit: Option<Coin>,            // protocol parameter
    max_value_size: Option<u32>,          // protocol parameter
    max_tx_size: Option<u32>,             // protocol parameter
    data_cost: Option<DataCost>,          // protocol parameter
    ex_unit_prices: Option<ExUnitPrices>, // protocol parameter
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
            data_cost: None,
            ex_unit_prices: None,
            prefer_pure_change: false,
        }
    }

    pub fn fee_algo(&self, fee_algo: &fees::LinearFee) -> Self {
        let mut cfg = self.clone();
        cfg.fee_algo = Some(fee_algo.clone());
        cfg
    }

    /// !!! DEPRECATED !!!
    /// Since babbage era cardano nodes use coins per byte. Use '.coins_per_utxo_byte' instead.
    #[deprecated(
        since = "11.0.0",
        note = "Since babbage era cardano nodes use coins per byte. Use '.coins_per_utxo_byte' instead."
    )]
    pub fn coins_per_utxo_word(&self, coins_per_utxo_word: &Coin) -> Self {
        let mut cfg = self.clone();
        cfg.data_cost = Some(DataCost::new_coins_per_word(coins_per_utxo_word));
        cfg
    }

    pub fn coins_per_utxo_byte(&self, coins_per_utxo_byte: &Coin) -> Self {
        let mut cfg = self.clone();
        cfg.data_cost = Some(DataCost::new_coins_per_byte(coins_per_utxo_byte));
        cfg
    }

    pub fn ex_unit_prices(&self, ex_unit_prices: &ExUnitPrices) -> Self {
        let mut cfg = self.clone();
        cfg.ex_unit_prices = Some(ex_unit_prices.clone());
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
        let cfg: Self = self.clone();
        Ok(TransactionBuilderConfig {
            fee_algo: cfg
                .fee_algo
                .ok_or(JsError::from_str("uninitialized field: fee_algo"))?,
            pool_deposit: cfg
                .pool_deposit
                .ok_or(JsError::from_str("uninitialized field: pool_deposit"))?,
            key_deposit: cfg
                .key_deposit
                .ok_or(JsError::from_str("uninitialized field: key_deposit"))?,
            max_value_size: cfg
                .max_value_size
                .ok_or(JsError::from_str("uninitialized field: max_value_size"))?,
            max_tx_size: cfg
                .max_tx_size
                .ok_or(JsError::from_str("uninitialized field: max_tx_size"))?,
            data_cost: cfg.data_cost.ok_or(JsError::from_str(
                "uninitialized field: coins_per_utxo_byte or coins_per_utxo_word",
            ))?,
            ex_unit_prices: cfg.ex_unit_prices,
            prefer_pure_change: cfg.prefer_pure_change,
        })
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilder {
    config: TransactionBuilderConfig,
    inputs: TxInputsBuilder,
    collateral: TxInputsBuilder,
    outputs: TransactionOutputs,
    fee: Option<Coin>,
    ttl: Option<SlotBigNum>, // absolute slot number
    certs: Option<Certificates>,
    withdrawals: Option<Withdrawals>,
    auxiliary_data: Option<AuxiliaryData>,
    validity_start_interval: Option<SlotBigNum>,
    mint: Option<Mint>,
    mint_scripts: Option<NativeScripts>,
    script_data_hash: Option<ScriptDataHash>,
    required_signers: Ed25519KeyHashes,
    collateral_return: Option<TransactionOutput>,
    total_collateral: Option<Coin>,
    reference_inputs: HashSet<TransactionInput>,
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
    pub fn add_inputs_from(
        &mut self,
        inputs: &TransactionUnspentOutputs,
        strategy: CoinSelectionStrategyCIP2,
    ) -> Result<(), JsError> {
        let available_inputs = &inputs.0.clone();
        let mut input_total = self.get_total_input()?;
        let mut output_total = self
            .get_total_output()?
            .checked_add(&Value::new(&self.min_fee()?))?;
        match strategy {
            CoinSelectionStrategyCIP2::LargestFirst => {
                if self
                    .outputs
                    .0
                    .iter()
                    .any(|output| output.amount.multiasset.is_some())
                {
                    return Err(JsError::from_str("Multiasset values not supported by LargestFirst. Please use LargestFirstMultiAsset"));
                }
                self.cip2_largest_first_by(
                    available_inputs,
                    &mut (0..available_inputs.len()).collect(),
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin),
                )?;
            }
            CoinSelectionStrategyCIP2::RandomImprove => {
                if self
                    .outputs
                    .0
                    .iter()
                    .any(|output| output.amount.multiasset.is_some())
                {
                    return Err(JsError::from_str("Multiasset values not supported by RandomImprove. Please use RandomImproveMultiAsset"));
                }
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let mut available_indices =
                    (0..available_inputs.len()).collect::<BTreeSet<usize>>();
                self.cip2_random_improve_by(
                    available_inputs,
                    &mut available_indices,
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin),
                    &mut rng,
                )?;
                // Phase 3: add extra inputs needed for fees (not covered by CIP-2)
                // We do this at the end because this new inputs won't be associated with
                // a specific output, so the improvement algorithm we do above does not apply here.
                while input_total.coin < output_total.coin {
                    if available_indices.is_empty() {
                        return Err(JsError::from_str("UTxO Balance Insufficient[x]"));
                    }
                    let i = *available_indices
                        .iter()
                        .nth(rng.gen_range(0..available_indices.len()))
                        .unwrap();
                    available_indices.remove(&i);
                    let input = &available_inputs[i];
                    let input_fee = self.fee_for_input(
                        &input.output.address,
                        &input.input,
                        &input.output.amount,
                    )?;
                    self.add_input(&input.output.address, &input.input, &input.output.amount);
                    input_total = input_total.checked_add(&input.output.amount)?;
                    output_total = output_total.checked_add(&Value::new(&input_fee))?;
                }
            }
            CoinSelectionStrategyCIP2::LargestFirstMultiAsset => {
                // indices into {available_inputs} for inputs that contain {policy_id}:{asset_name}
                let mut available_indices = (0..available_inputs.len()).collect::<Vec<usize>>();
                // run largest-fist by each asset type
                if let Some(ma) = output_total.multiasset.clone() {
                    for (policy_id, assets) in ma.0.iter() {
                        for (asset_name, _) in assets.0.iter() {
                            self.cip2_largest_first_by(
                                available_inputs,
                                &mut available_indices,
                                &mut input_total,
                                &mut output_total,
                                |value| value.multiasset.as_ref()?.get(policy_id)?.get(asset_name),
                            )?;
                        }
                    }
                }
                // add in remaining ADA
                self.cip2_largest_first_by(
                    available_inputs,
                    &mut available_indices,
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin),
                )?;
            }
            CoinSelectionStrategyCIP2::RandomImproveMultiAsset => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let mut available_indices =
                    (0..available_inputs.len()).collect::<BTreeSet<usize>>();
                // run random-improve by each asset type
                if let Some(ma) = output_total.multiasset.clone() {
                    for (policy_id, assets) in ma.0.iter() {
                        for (asset_name, _) in assets.0.iter() {
                            self.cip2_random_improve_by(
                                available_inputs,
                                &mut available_indices,
                                &mut input_total,
                                &mut output_total,
                                |value| value.multiasset.as_ref()?.get(policy_id)?.get(asset_name),
                                &mut rng,
                            )?;
                        }
                    }
                }
                // add in remaining ADA
                self.cip2_random_improve_by(
                    available_inputs,
                    &mut available_indices,
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin),
                    &mut rng,
                )?;
                // Phase 3: add extra inputs needed for fees (not covered by CIP-2)
                // We do this at the end because this new inputs won't be associated with
                // a specific output, so the improvement algorithm we do above does not apply here.
                while input_total.coin < output_total.coin {
                    if available_indices.is_empty() {
                        return Err(JsError::from_str("UTxO Balance Insufficient[x]"));
                    }
                    let i = *available_indices
                        .iter()
                        .nth(rng.gen_range(0..available_indices.len()))
                        .unwrap();
                    available_indices.remove(&i);
                    let input = &available_inputs[i];
                    let input_fee = self.fee_for_input(
                        &input.output.address,
                        &input.input,
                        &input.output.amount,
                    )?;
                    self.add_input(&input.output.address, &input.input, &input.output.amount);
                    input_total = input_total.checked_add(&input.output.amount)?;
                    output_total = output_total.checked_add(&Value::new(&input_fee))?;
                }
            }
        }

        Ok(())
    }

    fn cip2_largest_first_by<F>(
        &mut self,
        available_inputs: &Vec<TransactionUnspentOutput>,
        available_indices: &mut Vec<usize>,
        input_total: &mut Value,
        output_total: &mut Value,
        by: F,
    ) -> Result<(), JsError>
    where
        F: Fn(&Value) -> Option<BigNum>,
    {
        let mut relevant_indices = available_indices.clone();
        relevant_indices.retain(|i| by(&available_inputs[*i].output.amount).is_some());
        // ordered in ascending order by predicate {by}
        relevant_indices
            .sort_by_key(|i| by(&available_inputs[*i].output.amount).expect("filtered above"));

        // iterate in decreasing order for predicate {by}
        for i in relevant_indices.iter().rev() {
            if by(input_total).unwrap_or(BigNum::zero())
                >= by(output_total).expect("do not call on asset types that aren't in the output")
            {
                break;
            }
            let input = &available_inputs[*i];
            // differing from CIP2, we include the needed fees in the targets instead of just output values
            let input_fee =
                self.fee_for_input(&input.output.address, &input.input, &input.output.amount)?;
            self.add_input(&input.output.address, &input.input, &input.output.amount);
            *input_total = input_total.checked_add(&input.output.amount)?;
            *output_total = output_total.checked_add(&Value::new(&input_fee))?;
            available_indices.swap_remove(available_indices.iter().position(|j| i == j).unwrap());
        }

        if by(input_total).unwrap_or(BigNum::zero())
            < by(output_total).expect("do not call on asset types that aren't in the output")
        {
            return Err(JsError::from_str("UTxO Balance Insufficient"));
        }

        Ok(())
    }

    fn cip2_random_improve_by<F>(
        &mut self,
        available_inputs: &Vec<TransactionUnspentOutput>,
        available_indices: &mut BTreeSet<usize>,
        input_total: &mut Value,
        output_total: &mut Value,
        by: F,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<(), JsError>
    where
        F: Fn(&Value) -> Option<BigNum>,
    {
        use rand::Rng;
        // Phase 1: Random Selection
        let mut relevant_indices = available_indices
            .iter()
            .filter(|i| by(&available_inputs[**i].output.amount).is_some())
            .cloned()
            .collect::<Vec<usize>>();
        let mut associated_indices: BTreeMap<TransactionOutput, Vec<usize>> = BTreeMap::new();
        let mut outputs = self
            .outputs
            .0
            .iter()
            .filter(|output| by(&output.amount).is_some())
            .cloned()
            .collect::<Vec<TransactionOutput>>();
        outputs.sort_by_key(|output| by(&output.amount).expect("filtered above"));
        for output in outputs.iter().rev() {
            // TODO: how should we adapt this to inputs being associated when running for other assets?
            // if we do these two phases for each asset and don't take into account the other runs for other assets
            // then we over-add (and potentially fail if we don't have plenty of inputs)
            // On the other hand, the improvement phase it difficult to determine if a change is an improvement
            // if we're trying to improve for multiple assets at a time without knowing how important each input is
            // e.g. maybe we have lots of asset A but not much of B
            // For now I will just have this be entirely separarte per-asset but we might want to in a later commit
            // consider the improvements separately and have it take some kind of dot product / distance for assets
            // during the improvement phase and have the improvement phase target multiple asset types at once.
            // One issue with that is how to scale in between differnet assets. We could maybe normalize them by
            // dividing each asset type by the sum of the required asset type in all outputs.
            // Another possibility for adapting this to multiasstes is when associating an input x for asset type a
            // we try and subtract all other assets b != a from the outputs we're trying to cover.
            // It might make sense to diverge further and not consider it per-output and to instead just match against
            // the sum of all outputs as one single value.
            let mut added = BigNum::zero();
            let needed = by(&output.amount).unwrap();
            while added < needed {
                if relevant_indices.is_empty() {
                    return Err(JsError::from_str("UTxO Balance Insufficient"));
                }
                let random_index = rng.gen_range(0..relevant_indices.len());
                let i = relevant_indices.swap_remove(random_index);
                available_indices.remove(&i);
                let input = &available_inputs[i];
                added = added.checked_add(
                    &by(&input.output.amount)
                        .expect("do not call on asset types that aren't in the output"),
                )?;
                associated_indices
                    .entry(output.clone())
                    .or_default()
                    .push(i);
            }
        }
        if !relevant_indices.is_empty() {
            // Phase 2: Improvement
            for output in outputs.iter_mut() {
                let associated = associated_indices.get_mut(output).unwrap();
                for i in associated.iter_mut() {
                    let random_index = rng.gen_range(0..relevant_indices.len());
                    let j: &mut usize = relevant_indices.get_mut(random_index).unwrap();
                    let input = &available_inputs[*i];
                    let new_input = &available_inputs[*j];
                    let cur = from_bignum(&input.output.amount.coin);
                    let new = from_bignum(&new_input.output.amount.coin);
                    let min = from_bignum(&output.amount.coin);
                    let ideal = 2 * min;
                    let max = 3 * min;
                    let move_closer =
                        (ideal as i128 - new as i128).abs() < (ideal as i128 - cur as i128).abs();
                    let not_exceed_max = new < max;
                    if move_closer && not_exceed_max {
                        std::mem::swap(i, j);
                        available_indices.insert(*i);
                        available_indices.remove(j);
                    }
                }
            }
        }

        // after finalizing the improvement we need to actually add these results to the builder
        for output in outputs.iter() {
            for i in associated_indices.get(output).unwrap().iter() {
                let input = &available_inputs[*i];
                let input_fee =
                    self.fee_for_input(&input.output.address, &input.input, &input.output.amount)?;
                self.add_input(&input.output.address, &input.input, &input.output.amount);
                *input_total = input_total.checked_add(&input.output.amount)?;
                *output_total = output_total.checked_add(&Value::new(&input_fee))?;
            }
        }

        Ok(())
    }

    pub fn set_inputs(&mut self, inputs: &TxInputsBuilder) {
        self.inputs = inputs.clone();
    }

    pub fn set_collateral(&mut self, collateral: &TxInputsBuilder) {
        self.collateral = collateral.clone();
    }

    pub fn set_collateral_return(&mut self, collateral_return: &TransactionOutput) {
        self.collateral_return = Some(collateral_return.clone());
    }

    /// This function will set the collateral-return value and then auto-calculate and assign
    /// the total collateral coin value. Will raise an error in case no collateral inputs are set
    /// or in case the total collateral value will have any assets in it except coin.
    pub fn set_collateral_return_and_total(
        &mut self,
        collateral_return: &TransactionOutput,
    ) -> Result<(), JsError> {
        let collateral = &self.collateral;
        if collateral.len() == 0 {
            return Err(JsError::from_str(
                "Cannot calculate total collateral value when collateral inputs are missing",
            ));
        }
        let col_input_value: Value = collateral.total_value()?;
        let total_col: Value = col_input_value.checked_sub(&collateral_return.amount())?;
        if total_col.multiasset.is_some() {
            return Err(JsError::from_str(
                "Total collateral value cannot contain assets!",
            ));
        }

        let min_ada = min_ada_for_output(&collateral_return, &self.config.utxo_cost())?;
        if min_ada > collateral_return.amount.coin {
            return Err(JsError::from_str(&format!(
                "Not enough coin to make return on the collateral value!\
                 Increase amount of return coins. \
                 Min ada for return {}, but was {}",
                min_ada, collateral_return.amount.coin
            )));
        }

        self.set_collateral_return(collateral_return);
        self.total_collateral = Some(total_col.coin);
        Ok(())
    }

    pub fn set_total_collateral(&mut self, total_collateral: &Coin) {
        self.total_collateral = Some(total_collateral.clone());
    }

    /// This function will set the total-collateral coin and then auto-calculate and assign
    /// the collateral return value. Will raise an error in case no collateral inputs are set.
    /// The specified address will be the received of the collateral return
    pub fn set_total_collateral_and_return(
        &mut self,
        total_collateral: &Coin,
        return_address: &Address,
    ) -> Result<(), JsError> {
        let collateral = &self.collateral;
        if collateral.len() == 0 {
            return Err(JsError::from_str(
                "Cannot calculate collateral return when collateral inputs are missing",
            ));
        }
        let col_input_value: Value = collateral.total_value()?;
        let col_return: Value = col_input_value.checked_sub(&Value::new(&total_collateral))?;
        if col_return.multiasset.is_some() || col_return.coin > BigNum::zero() {
            let return_output = TransactionOutput::new(return_address, &col_return);
            let min_ada = min_ada_for_output(&return_output, &self.config.utxo_cost())?;
            if min_ada > col_return.coin {
                return Err(JsError::from_str(&format!(
                    "Not enough coin to make return on the collateral value!\
                 Decrease the total collateral value or add more collateral inputs. \
                 Min ada for return {}, but was {}",
                    min_ada, col_return.coin
                )));
            }
            self.collateral_return = Some(return_output);
        }
        self.set_total_collateral(total_collateral);

        Ok(())
    }

    pub fn add_reference_input(&mut self, reference_input: &TransactionInput) {
        self.reference_inputs.insert(reference_input.clone());
    }

    /// We have to know what kind of inputs these are to know what kind of mock witnesses to create since
    /// 1) mock witnesses have different lengths depending on the type which changes the expecting fee
    /// 2) Witnesses are a set so we need to get rid of duplicates to avoid over-estimating the fee
    #[deprecated(since = "10.2.0", note = "Use `.set_inputs`")]
    pub fn add_key_input(
        &mut self,
        hash: &Ed25519KeyHash,
        input: &TransactionInput,
        amount: &Value,
    ) {
        self.inputs.add_key_input(hash, input, amount);
    }

    /// This method adds the input to the builder BUT leaves a missing spot for the witness native script
    ///
    /// After adding the input with this method, use `.add_required_native_input_scripts`
    /// and `.add_required_plutus_input_scripts` to add the witness scripts
    ///
    /// Or instead use `.add_native_script_input` and `.add_plutus_script_input`
    /// to add inputs right along with the script, instead of the script hash
    #[deprecated(since = "10.2.0", note = "Use `.set_inputs`")]
    pub fn add_script_input(
        &mut self,
        hash: &ScriptHash,
        input: &TransactionInput,
        amount: &Value,
    ) {
        self.inputs.add_script_input(hash, input, amount);
    }

    /// This method will add the input to the builder and also register the required native script witness
    #[deprecated(since = "10.2.0", note = "Use `.set_inputs`")]
    pub fn add_native_script_input(
        &mut self,
        script: &NativeScript,
        input: &TransactionInput,
        amount: &Value,
    ) {
        self.inputs.add_native_script_input(script, input, amount);
    }

    /// This method will add the input to the builder and also register the required plutus witness
    #[deprecated(since = "10.2.0", note = "Use `.set_inputs`")]
    pub fn add_plutus_script_input(
        &mut self,
        witness: &PlutusWitness,
        input: &TransactionInput,
        amount: &Value,
    ) {
        self.inputs.add_plutus_script_input(witness, input, amount);
    }

    #[deprecated(since = "10.2.0", note = "Use `.set_inputs`")]
    pub fn add_bootstrap_input(
        &mut self,
        hash: &ByronAddress,
        input: &TransactionInput,
        amount: &Value,
    ) {
        self.inputs.add_bootstrap_input(hash, input, amount);
    }

    /// Note that for script inputs this method will use underlying generic `.add_script_input`
    /// which leaves a required empty spot for the script witness (or witnesses in case of Plutus).
    /// You can use `.add_native_script_input` or `.add_plutus_script_input` directly to register the input along with the witness.
    #[deprecated(since = "10.2.0", note = "Use `.set_inputs`")]
    pub fn add_input(&mut self, address: &Address, input: &TransactionInput, amount: &Value) {
        self.inputs.add_input(address, input, amount);
    }

    /// Returns the number of still missing input scripts (either native or plutus)
    /// Use `.add_required_native_input_scripts` or `.add_required_plutus_input_scripts` to add the missing scripts
    #[deprecated(since = "10.2.0", note = "Use `.set_inputs`")]
    pub fn count_missing_input_scripts(&self) -> usize {
        self.inputs.count_missing_input_scripts()
    }

    /// Try adding the specified scripts as witnesses for ALREADY ADDED script inputs
    /// Any scripts that don't match any of the previously added inputs will be ignored
    /// Returns the number of remaining required missing witness scripts
    /// Use `.count_missing_input_scripts` to find the number of still missing scripts
    #[deprecated(since = "10.2.0", note = "Use `.set_inputs`")]
    pub fn add_required_native_input_scripts(&mut self, scripts: &NativeScripts) -> usize {
        self.inputs.add_required_native_input_scripts(scripts)
    }

    /// Try adding the specified scripts as witnesses for ALREADY ADDED script inputs
    /// Any scripts that don't match any of the previously added inputs will be ignored
    /// Returns the number of remaining required missing witness scripts
    /// Use `.count_missing_input_scripts` to find the number of still missing scripts
    #[deprecated(since = "10.2.0", note = "Use `.set_inputs`")]
    pub fn add_required_plutus_input_scripts(&mut self, scripts: &PlutusWitnesses) -> usize {
        self.inputs.add_required_plutus_input_scripts(scripts)
    }

    /// Returns a copy of the current script input witness scripts in the builder
    #[deprecated(since = "10.2.0", note = "Use `.set_inputs`")]
    pub fn get_native_input_scripts(&self) -> Option<NativeScripts> {
        self.inputs.get_native_input_scripts()
    }

    /// Returns a copy of the current plutus input witness scripts in the builder.
    /// NOTE: each plutus witness will be cloned with a specific corresponding input index
    #[deprecated(since = "10.2.0", note = "Use `.set_inputs`")]
    pub fn get_plutus_input_scripts(&self) -> Option<PlutusWitnesses> {
        self.inputs.get_plutus_input_scripts()
    }

    /// calculates how much the fee would increase if you added a given output
    pub fn fee_for_input(
        &self,
        address: &Address,
        input: &TransactionInput,
        amount: &Value,
    ) -> Result<Coin, JsError> {
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

    /// Add explicit output via a TransactionOutput object
    pub fn add_output(&mut self, output: &TransactionOutput) -> Result<(), JsError> {
        let value_size = output.amount.to_bytes().len();
        if value_size > self.config.max_value_size as usize {
            return Err(JsError::from_str(&format!(
                "Maximum value size of {} exceeded. Found: {}",
                self.config.max_value_size, value_size
            )));
        }
        let min_ada = min_ada_for_output(&output, &self.config.utxo_cost())?;
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

    /// !!! DEPRECATED !!!
    /// Set ttl value.
    #[deprecated(
        since = "10.1.0",
        note = "Underlying value capacity of ttl (BigNum u64) bigger then Slot32. Use set_ttl_bignum instead."
    )]
    pub fn set_ttl(&mut self, ttl: Slot32) {
        self.ttl = Some(ttl.into())
    }

    pub fn set_ttl_bignum(&mut self, ttl: &SlotBigNum) {
        self.ttl = Some(ttl.clone())
    }

    /// !!! DEPRECATED !!!
    /// Uses outdated slot number format.
    #[deprecated(
        since = "10.1.0",
        note = "Underlying value capacity of validity_start_interval (BigNum u64) bigger then Slot32. Use set_validity_start_interval_bignum instead."
    )]
    pub fn set_validity_start_interval(&mut self, validity_start_interval: Slot32) {
        self.validity_start_interval = Some(validity_start_interval.into())
    }

    pub fn set_validity_start_interval_bignum(&mut self, validity_start_interval: SlotBigNum) {
        self.validity_start_interval = Some(validity_start_interval.clone())
    }

    pub fn set_certs(&mut self, certs: &Certificates) {
        self.certs = Some(certs.clone());
        for cert in &certs.0 {
            self.inputs
                .add_required_signers(&witness_keys_for_cert(cert))
        }
    }

    pub fn set_withdrawals(&mut self, withdrawals: &Withdrawals) {
        self.withdrawals = Some(withdrawals.clone());
        for (withdrawal, _coin) in &withdrawals.0 {
            self.inputs
                .add_required_signer(&withdrawal.payment_cred().to_keyhash().unwrap())
        }
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
        let mut aux = self
            .auxiliary_data
            .as_ref()
            .cloned()
            .unwrap_or(AuxiliaryData::new());
        aux.set_metadata(metadata);
        self.set_auxiliary_data(&aux);
    }

    /// Add a single metadatum using TransactionMetadatumLabel and TransactionMetadatum objects
    /// It will be securely added to existing or new metadata in this builder
    pub fn add_metadatum(&mut self, key: &TransactionMetadatumLabel, val: &TransactionMetadatum) {
        let mut metadata = self
            .auxiliary_data
            .as_ref()
            .map(|aux| aux.metadata().as_ref().cloned())
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

    /// Set explicit Mint object and the required witnesses to this builder
    /// it will replace any previously existing mint and mint scripts
    /// NOTE! Error will be returned in case a mint policy does not have a matching script
    pub fn set_mint(&mut self, mint: &Mint, mint_scripts: &NativeScripts) -> Result<(), JsError> {
        assert_required_mint_scripts(mint, Some(mint_scripts))?;
        self.mint = Some(mint.clone());
        self.mint_scripts = Some(mint_scripts.clone());
        Ok(())
    }

    /// Returns a copy of the current mint state in the builder
    pub fn get_mint(&self) -> Option<Mint> {
        self.mint.clone()
    }

    /// Returns a copy of the current mint witness scripts in the builder
    pub fn get_mint_scripts(&self) -> Option<NativeScripts> {
        self.mint_scripts.clone()
    }

    fn _set_mint_asset(
        &mut self,
        policy_id: &PolicyID,
        policy_script: &NativeScript,
        mint_assets: &MintAssets,
    ) {
        let mut mint = self.mint.as_ref().cloned().unwrap_or(Mint::new());
        let is_new_policy = mint.insert(&policy_id, mint_assets).is_none();
        let mint_scripts = {
            let mut witness_scripts = self
                .mint_scripts
                .as_ref()
                .cloned()
                .unwrap_or(NativeScripts::new());
            if is_new_policy {
                // If policy has not been encountered before - insert the script into witnesses
                witness_scripts.add(&policy_script.clone());
            }
            witness_scripts
        };
        self.mint = Some(mint);
        self.mint_scripts = Some(mint_scripts.clone());
    }

    /// Add a mint entry to this builder using a PolicyID and MintAssets object
    /// It will be securely added to existing or new Mint in this builder
    /// It will replace any existing mint assets with the same PolicyID
    pub fn set_mint_asset(&mut self, policy_script: &NativeScript, mint_assets: &MintAssets) {
        let policy_id: PolicyID = policy_script.hash();
        self._set_mint_asset(&policy_id, policy_script, mint_assets);
    }

    fn _add_mint_asset(
        &mut self,
        policy_id: &PolicyID,
        policy_script: &NativeScript,
        asset_name: &AssetName,
        amount: Int,
    ) {
        let mut asset = self
            .mint
            .as_ref()
            .map(|m| m.get(&policy_id).as_ref().cloned())
            .unwrap_or(None)
            .unwrap_or(MintAssets::new());
        if let Some(mint_amount) = asset.get(asset_name) {
            let new_amount = mint_amount.0 + amount.0;
            asset.insert(asset_name, Int(new_amount));
        } else {
            asset.insert(asset_name, amount);
        }
        self._set_mint_asset(&policy_id, policy_script, &asset);
    }

    /// Add a mint entry to this builder using a PolicyID, AssetName, and Int object for amount
    /// It will be securely added to existing or new Mint in this builder
    /// It will replace any previous existing amount same PolicyID and AssetName
    pub fn add_mint_asset(
        &mut self,
        policy_script: &NativeScript,
        asset_name: &AssetName,
        amount: Int,
    ) {
        let policy_id: PolicyID = policy_script.hash();
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
        output_builder: &TransactionOutputAmountBuilder,
        output_coin: &Coin,
    ) -> Result<(), JsError> {
        if !amount.is_positive() {
            return Err(JsError::from_str("Output value must be positive!"));
        }
        let policy_id: PolicyID = policy_script.hash();
        self._add_mint_asset(&policy_id, policy_script, asset_name, amount.clone());
        let multiasset = Mint::new_from_entry(
            &policy_id,
            &MintAssets::new_from_entry(asset_name, amount.clone()),
        )
        .as_positive_multiasset();

        self.add_output(
            &output_builder
                .with_coin_and_asset(&output_coin, &multiasset)
                .build()?,
        )
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
        output_builder: &TransactionOutputAmountBuilder,
    ) -> Result<(), JsError> {
        if !amount.is_positive() {
            return Err(JsError::from_str("Output value must be positive!"));
        }
        let policy_id: PolicyID = policy_script.hash();
        self._add_mint_asset(&policy_id, policy_script, asset_name, amount.clone());
        let multiasset = Mint::new_from_entry(
            &policy_id,
            &MintAssets::new_from_entry(asset_name, amount.clone()),
        )
        .as_positive_multiasset();

        self.add_output(
            &output_builder
                .with_asset_and_min_required_coin_by_utxo_cost(
                    &multiasset,
                    &self.config.utxo_cost(),
                )?
                .build()?,
        )
    }

    pub fn new(cfg: &TransactionBuilderConfig) -> Self {
        Self {
            config: cfg.clone(),
            inputs: TxInputsBuilder::new(),
            collateral: TxInputsBuilder::new(),
            outputs: TransactionOutputs::new(),
            fee: None,
            ttl: None,
            certs: None,
            withdrawals: None,
            auxiliary_data: None,
            validity_start_interval: None,
            mint: None,
            mint_scripts: None,
            script_data_hash: None,
            required_signers: Ed25519KeyHashes::new(),
            collateral_return: None,
            total_collateral: None,
            reference_inputs: HashSet::new(),
        }
    }

    pub fn get_reference_inputs(&self) -> TransactionInputs {
        let mut inputs = self.reference_inputs.clone();
        for input in self.inputs.get_ref_inputs().0 {
            inputs.insert(input);
        }

        let vec_inputs = inputs.into_iter().collect();
        TransactionInputs(vec_inputs)
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
        self.mint
            .as_ref()
            .map(|m| {
                (
                    Value::new_from_assets(&m.as_positive_multiasset()),
                    Value::new_from_assets(&m.as_negative_multiasset()),
                )
            })
            .unwrap_or((Value::zero(), Value::zero()))
    }

    /// Return explicit input plus implicit input plus mint
    pub fn get_total_input(&self) -> Result<Value, JsError> {
        let (mint_value, _) = self.get_mint_as_values();
        self.get_explicit_input()?
            .checked_add(&self.get_implicit_input()?)?
            .checked_add(&mint_value)
    }

    /// Return explicit output plus deposit plus burn
    pub fn get_total_output(&self) -> Result<Value, JsError> {
        let (_, burn_value) = self.get_mint_as_values();
        self.get_explicit_output()?
            .checked_add(&Value::new(&self.get_deposit()?))?
            .checked_add(&burn_value)
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

        // note: can't add plutus data or data hash and script to change
        // because we don't know how many change outputs will need to be created
        let plutus_data: Option<DataOption> = None;
        let script_ref: Option<ScriptRef> = None;

        let input_total = self.get_total_input()?;
        let output_total = self.get_total_output()?;

        let shortage = get_input_shortage(&input_total, &output_total, &fee)?;
        if let Some(shortage) = shortage {
            return Err(JsError::from_str(&format!("Insufficient input in transaction. {}", shortage)));
        }

        use std::cmp::Ordering;
        match &input_total.partial_cmp(&output_total.checked_add(&Value::new(&fee))?) {
            Some(Ordering::Equal) => {
                // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
                self.set_fee(&input_total.checked_sub(&output_total)?.coin());
                Ok(false)
            }
            Some(Ordering::Less) => Err(JsError::from_str("Insufficient input in transaction")),
            Some(Ordering::Greater) => {
                fn has_assets(ma: Option<MultiAsset>) -> bool {
                    ma.map(|assets| assets.len() > 0).unwrap_or(false)
                }
                let change_estimator = input_total.checked_sub(&output_total)?;
                if has_assets(change_estimator.multiasset()) {
                    fn will_adding_asset_make_output_overflow(
                        output: &TransactionOutput,
                        current_assets: &Assets,
                        asset_to_add: (PolicyID, AssetName, BigNum),
                        max_value_size: u32,
                        data_cost: &DataCost,
                    ) -> Result<bool, JsError> {
                        let (policy, asset_name, value) = asset_to_add;
                        let mut current_assets_clone = current_assets.clone();
                        current_assets_clone.insert(&asset_name, &value);
                        let mut amount_clone = output.amount.clone();
                        let mut val = Value::new(&Coin::zero());
                        let mut ma = MultiAsset::new();

                        ma.insert(&policy, &current_assets_clone);
                        val.set_multiasset(&ma);
                        amount_clone = amount_clone.checked_add(&val)?;

                        // calculate minADA for more precise max value size
                        let mut calc = MinOutputAdaCalculator::new_empty(data_cost)?;
                        calc.set_amount(&val);
                        let min_ada = calc.calculate_ada()?;
                        amount_clone.set_coin(&min_ada);

                        Ok(amount_clone.to_bytes().len() > max_value_size as usize)
                    }
                    fn pack_nfts_for_change(
                        max_value_size: u32,
                        data_cost: &DataCost,
                        change_address: &Address,
                        change_estimator: &Value,
                        plutus_data: &Option<DataOption>,
                        script_ref: &Option<ScriptRef>,
                    ) -> Result<Vec<MultiAsset>, JsError> {
                        // we insert the entire available ADA temporarily here since that could potentially impact the size
                        // as it could be 1, 2 3 or 4 bytes for Coin.
                        let mut change_assets: Vec<MultiAsset> = Vec::new();

                        let mut base_coin = Value::new(&change_estimator.coin());
                        base_coin.set_multiasset(&MultiAsset::new());
                        let mut output = TransactionOutput {
                            address: change_address.clone(),
                            amount: base_coin.clone(),
                            plutus_data: plutus_data.clone(),
                            script_ref: script_ref.clone(),
                        };
                        // If this becomes slow on large TXs we can optimize it like the following
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

                                if will_adding_asset_make_output_overflow(
                                    &output,
                                    &rebuilt_assets,
                                    (policy.clone(), asset_name.clone(), value),
                                    max_value_size,
                                    data_cost,
                                )? {
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
                                    output = TransactionOutput {
                                        address: change_address.clone(),
                                        amount: base_coin.clone(),
                                        plutus_data: plutus_data.clone(),
                                        script_ref: script_ref.clone(),
                                    };

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

                            // calculate minADA for more precise max value size
                            let mut amount_clone = output.amount.clone();
                            let mut calc = MinOutputAdaCalculator::new_empty(data_cost)?;
                            calc.set_amount(&val);
                            let min_ada = calc.calculate_ada()?;
                            amount_clone.set_coin(&min_ada);

                            if amount_clone.to_bytes().len() > max_value_size as usize {
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
                    let utxo_cost = self.config.utxo_cost();
                    let mut calc = MinOutputAdaCalculator::new_empty(&utxo_cost)?;
                    if let Some(data) = &plutus_data {
                        match data {
                            DataOption::DataHash(data_hash) => calc.set_data_hash(data_hash),
                            DataOption::Data(datum) => calc.set_plutus_data(datum),
                        };
                    }
                    if let Some(script_ref) = &script_ref {
                        calc.set_script_ref(script_ref);
                    }
                    let minimum_utxo_val = calc.calculate_ada()?;
                    while let Some(Ordering::Greater) = change_left
                        .multiasset
                        .as_ref()
                        .map_or_else(|| None, |ma| ma.partial_cmp(&MultiAsset::new()))
                    {
                        let nft_changes = pack_nfts_for_change(
                            self.config.max_value_size,
                            &utxo_cost,
                            address,
                            &change_left,
                            &plutus_data.clone(),
                            &script_ref.clone(),
                        )?;
                        if nft_changes.len() == 0 {
                            // this likely should never happen
                            return Err(JsError::from_str("NFTs too large for change output"));
                        }
                        // we only add the minimum needed (for now) to cover this output
                        let mut change_value = Value::new(&Coin::zero());
                        for nft_change in nft_changes.iter() {
                            change_value.set_multiasset(&nft_change);
                            let mut calc = MinOutputAdaCalculator::new_empty(&utxo_cost)?;
                            //TODO add precise calculation
                            let mut fake_change = change_value.clone();
                            fake_change.set_coin(&change_left.coin);
                            calc.set_amount(&fake_change);
                            if let Some(data) = &plutus_data {
                                match data {
                                    DataOption::DataHash(data_hash) => {
                                        calc.set_data_hash(data_hash)
                                    }
                                    DataOption::Data(datum) => calc.set_plutus_data(datum),
                                };
                            }
                            if let Some(script_ref) = &script_ref {
                                calc.set_script_ref(script_ref);
                            }
                            let min_ada = calc.calculate_ada()?;
                            change_value.set_coin(&min_ada);
                            let change_output = TransactionOutput {
                                address: address.clone(),
                                amount: change_value.clone(),
                                plutus_data: plutus_data.clone(),
                                script_ref: script_ref.clone(),
                            };

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
                        let pure_output = TransactionOutput {
                            address: address.clone(),
                            amount: change_left.clone(),
                            plutus_data: plutus_data.clone(),
                            script_ref: script_ref.clone(),
                        };
                        let additional_fee = self.fee_for_output(&pure_output)?;
                        let potential_pure_value =
                            change_left.checked_sub(&Value::new(&additional_fee))?;
                        let potential_pure_above_minimum =
                            potential_pure_value.coin.compare(&minimum_utxo_val) > 0;
                        if potential_pure_above_minimum {
                            new_fee = new_fee.checked_add(&additional_fee)?;
                            change_left = Value::zero();
                            self.add_output(&TransactionOutput {
                                address: address.clone(),
                                amount: potential_pure_value.clone(),
                                plutus_data: plutus_data.clone(),
                                script_ref: script_ref.clone(),
                            })?;
                        }
                    }
                    self.set_fee(&new_fee);
                    // add in the rest of the ADA
                    if !change_left.is_zero() {
                        self.outputs.0.last_mut().unwrap().amount = self
                            .outputs
                            .0
                            .last()
                            .unwrap()
                            .amount
                            .checked_add(&change_left)?;
                    }
                    Ok(true)
                } else {
                    let mut calc = MinOutputAdaCalculator::new_empty(&self.config.utxo_cost())?;
                    calc.set_amount(&change_estimator);
                    if let Some(data) = &plutus_data {
                        match data {
                            DataOption::DataHash(data_hash) => calc.set_data_hash(data_hash),
                            DataOption::Data(datum) => calc.set_plutus_data(datum),
                        };
                    }
                    if let Some(script_ref) = &script_ref {
                        calc.set_script_ref(script_ref);
                    }
                    let min_ada = calc.calculate_ada()?;

                    // no-asset case so we have no problem burning the rest if there is no other option
                    fn burn_extra(
                        builder: &mut TransactionBuilder,
                        burn_amount: &BigNum,
                    ) -> Result<bool, JsError> {
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
                                plutus_data: plutus_data.clone(),
                                script_ref: script_ref.clone(),
                            })?;

                            let new_fee = fee.checked_add(&fee_for_change)?;
                            match change_estimator.coin()
                                >= min_ada.checked_add(&Value::new(&new_fee).coin())?
                            {
                                false => burn_extra(self, &change_estimator.coin()),
                                true => {
                                    // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
                                    self.set_fee(&new_fee);

                                    self.add_output(&TransactionOutput {
                                        address: address.clone(),
                                        amount: change_estimator
                                            .checked_sub(&Value::new(&new_fee.clone()))?,
                                        plutus_data: plutus_data.clone(),
                                        script_ref: script_ref.clone(),
                                    })?;

                                    Ok(true)
                                }
                            }
                        }
                    }
                }
            }
            None => Err(JsError::from_str(
                "missing input or output for some native asset",
            )),
        }
    }

    /// This method will calculate the script hash data
    /// using the plutus datums and redeemers already present in the builder
    /// along with the provided cost model, and will register the calculated value
    /// in the builder to be used when building the tx body.
    /// In case there are no plutus input witnesses present - nothing will change
    /// You can set specific hash value using `.set_script_data_hash`
    /// NOTE: this function will check which language versions are used in the present scripts
    /// and will assert and require for a corresponding cost-model to be present in the passed map.
    /// Only the cost-models for the present language versions will be used in the hash calculation.
    pub fn calc_script_data_hash(&mut self, cost_models: &Costmdls) -> Result<(), JsError> {
        let mut retained_cost_models = Costmdls::new();
        if let Some(pw) = self.inputs.get_plutus_input_scripts() {
            let (scripts, datums, redeemers) = pw.collect();
            for lang in Languages::list().0 {
                if scripts.has_version(&lang) {
                    match cost_models.get(&lang) {
                        Some(cost) => {
                            retained_cost_models.insert(&lang, &cost);
                        }
                        _ => {
                            return Err(JsError::from_str(&format!(
                                "Missing cost model for language version: {:?}",
                                lang
                            )))
                        }
                    }
                }
            }
            self.script_data_hash = Some(hash_script_data(
                &redeemers,
                &retained_cost_models,
                Some(datums),
            ));
        }
        Ok(())
    }

    /// Sets the specified hash value.
    /// Alternatively you can use `.calc_script_data_hash` to calculate the hash automatically.
    /// Or use `.remove_script_data_hash` to delete the previously set value
    pub fn set_script_data_hash(&mut self, hash: &ScriptDataHash) {
        self.script_data_hash = Some(hash.clone());
    }

    /// Deletes any previously set plutus data hash value.
    /// Use `.set_script_data_hash` or `.calc_script_data_hash` to set it.
    pub fn remove_script_data_hash(&mut self) {
        self.script_data_hash = None;
    }

    pub fn add_required_signer(&mut self, key: &Ed25519KeyHash) {
        self.required_signers.add(key);
    }

    fn build_and_size(&self) -> Result<(TransactionBody, usize), JsError> {
        let fee = self
            .fee
            .ok_or_else(|| JsError::from_str("Fee not specified"))?;
        let built = TransactionBody {
            inputs: self.inputs.inputs(),
            outputs: self.outputs.clone(),
            fee,
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
            script_data_hash: self.script_data_hash.clone(),
            collateral: self.collateral.inputs_option(),
            required_signers: self.required_signers.to_option(),
            network_id: None,
            collateral_return: self.collateral_return.clone(),
            total_collateral: self.total_collateral.clone(),
            reference_inputs: self.get_reference_inputs().to_option(),
        };
        // we must build a tx with fake data (of correct size) to check the final Transaction size
        let full_tx = fake_full_tx(self, built)?;
        let full_tx_size = full_tx.to_bytes().len();
        return Ok((full_tx.body, full_tx_size));
    }

    pub fn full_size(&self) -> Result<usize, JsError> {
        return self.build_and_size().map(|r| r.1);
    }

    pub fn output_sizes(&self) -> Vec<usize> {
        return self.outputs.0.iter().map(|o| o.to_bytes().len()).collect();
    }

    /// Returns object the body of the new transaction
    /// Auxiliary data itself is not included
    /// You can use `get_auxiliary_data` or `build_tx`
    pub fn build(&self) -> Result<TransactionBody, JsError> {
        let (body, full_tx_size) = self.build_and_size()?;
        if full_tx_size > self.config.max_tx_size as usize {
            Err(JsError::from_str(&format!(
                "Maximum transaction size of {} exceeded. Found: {}",
                self.config.max_tx_size, full_tx_size
            )))
        } else {
            Ok(body)
        }
    }

    fn get_combined_native_scripts(&self) -> Option<NativeScripts> {
        let mut ns = NativeScripts::new();
        if let Some(input_scripts) = self.inputs.get_native_input_scripts() {
            input_scripts.0.iter().for_each(|s| {
                ns.add(s);
            });
        }
        if let Some(input_scripts) = self.collateral.get_native_input_scripts() {
            input_scripts.0.iter().for_each(|s| {
                ns.add(s);
            });
        }
        if let Some(mint_scripts) = &self.mint_scripts {
            mint_scripts.0.iter().for_each(|s| {
                ns.add(s);
            });
        }
        if ns.len() > 0 {
            Some(ns)
        } else {
            None
        }
    }

    fn get_combined_plutus_scripts(&self) -> Option<PlutusWitnesses> {
        let mut res = PlutusWitnesses::new();
        if let Some(scripts) = self.inputs.get_plutus_input_scripts() {
            scripts.0.iter().for_each(|s| {
                res.add(s);
            })
        }
        if let Some(scripts) = self.collateral.get_plutus_input_scripts() {
            scripts.0.iter().for_each(|s| {
                res.add(s);
            })
        }
        if res.len() > 0 {
            Some(res)
        } else {
            None
        }
    }

    // This function should be producing the total witness-set
    // that is created by the tx-builder itself,
    // before the transaction is getting signed by the actual wallet.
    // E.g. scripts or something else that has been used during the tx preparation
    fn get_witness_set(&self) -> TransactionWitnessSet {
        let mut wit = TransactionWitnessSet::new();
        if let Some(scripts) = self.get_combined_native_scripts() {
            wit.set_native_scripts(&scripts);
        }
        if let Some(pw) = self.get_combined_plutus_scripts() {
            let (scripts, datums, redeemers) = pw.collect();
            wit.set_plutus_scripts(&scripts);
            wit.set_plutus_data(&datums);
            wit.set_redeemers(&redeemers);
        }
        wit
    }

    fn has_plutus_inputs(&self) -> bool {
        self.inputs.get_plutus_input_scripts().is_some()
    }

    /// Returns full Transaction object with the body and the auxiliary data
    /// NOTE: witness_set will contain all mint_scripts if any been added or set
    /// NOTE: is_valid set to true
    /// NOTE: Will fail in case there are any script inputs added with no corresponding witness
    pub fn build_tx(&self) -> Result<Transaction, JsError> {
        if self.count_missing_input_scripts() > 0 {
            return Err(JsError::from_str(
                "There are some script inputs added that don't have the corresponding script provided as a witness!",
            ));
        }
        if self.has_plutus_inputs() {
            if self.script_data_hash.is_none() {
                return Err(JsError::from_str(
                    "Plutus inputs are present, but script data hash is not specified",
                ));
            }
            if self.collateral.len() == 0 {
                return Err(JsError::from_str(
                    "Plutus inputs are present, but no collateral inputs are added",
                ));
            }
        }
        self.build_tx_unsafe()
    }

    /// Similar to `.build_tx()` but will NOT fail in case there are missing script witnesses
    pub fn build_tx_unsafe(&self) -> Result<Transaction, JsError> {
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
    use super::output_builder::TransactionOutputBuilder;
    use super::*;
    use crate::fakes::{fake_base_address, fake_bytes_32, fake_key_hash, fake_policy_id, fake_tx_hash, fake_tx_input, fake_tx_input2, fake_value, fake_value2};
    use crate::tx_builder_constants::TxBuilderConstants;
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
        let entropy = [
            0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22,
            0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12,
        ];
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
        ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address()
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
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
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
        create_tx_builder_full(
            linear_fee,
            pool_deposit,
            key_deposit,
            MAX_VALUE_SIZE,
            coins_per_utxo_word,
        )
    }

    fn create_reallistic_tx_builder() -> TransactionBuilder {
        create_tx_builder(
            &create_linear_fee(44, 155381),
            COINS_PER_UTXO_WORD,
            500000000,
            2000000,
        )
    }

    fn create_tx_builder_with_fee_and_val_size(
        linear_fee: &LinearFee,
        max_val_size: u32,
    ) -> TransactionBuilder {
        create_tx_builder_full(linear_fee, 1, 1, max_val_size, 8)
    }

    fn create_tx_builder_with_fee(linear_fee: &LinearFee) -> TransactionBuilder {
        create_tx_builder(linear_fee, 8, 1, 1)
    }

    fn create_tx_builder_with_fee_and_pure_change(linear_fee: &LinearFee) -> TransactionBuilder {
        TransactionBuilder::new(
            &TransactionBuilderConfigBuilder::new()
                .fee_algo(linear_fee)
                .pool_deposit(&to_bignum(1))
                .key_deposit(&to_bignum(1))
                .max_value_size(MAX_VALUE_SIZE)
                .max_tx_size(MAX_TX_SIZE)
                .coins_per_utxo_word(&to_bignum(8))
                .prefer_pure_change(true)
                .build()
                .unwrap(),
        )
    }

    fn create_tx_builder_with_key_deposit(deposit: u64) -> TransactionBuilder {
        create_tx_builder(&create_default_linear_fee(), 8, 1, deposit)
    }

    fn create_default_tx_builder() -> TransactionBuilder {
        create_tx_builder_with_fee(&create_default_linear_fee())
    }

    #[ignore]
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
        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();
        tx_builder.add_key_input(
            &spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(222))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();
        let added_change = tx_builder.add_change_if_needed(&change_addr);
        assert!(added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 2);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
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
        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();
        tx_builder.add_key_input(
            &spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(880_000))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();
        let added_change = tx_builder.add_change_if_needed(&change_addr);
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
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
            &Value::new(&to_bignum(5_000_000)),
        );
        tx_builder.set_ttl(1000);

        let mut certs = Certificates::new();
        certs.add(&Certificate::new_stake_registration(
            &StakeRegistration::new(&stake_cred),
        ));
        certs.add(&Certificate::new_stake_delegation(&StakeDelegation::new(
            &stake_cred,
            &stake.to_raw_key().hash(), // in reality, this should be the pool owner's key, not ours
        )));
        tx_builder.set_certs(&certs);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();
        tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert_eq!(tx_builder.min_fee().unwrap().to_str(), "214002");
        assert_eq!(tx_builder.get_fee_if_set().unwrap().to_str(), "214002");
        assert_eq!(tx_builder.get_deposit().unwrap().to_str(), "1000000");
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
                .checked_add(&Value::new(&tx_builder.get_deposit().unwrap()))
                .unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[ignore]
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
            &Value::new(&to_bignum(222)),
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
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(222))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(0);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();
        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert_eq!(added_change, false);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 1);
    }

    #[ignore]
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
            &Value::new(&to_bignum(700)),
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
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(222))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(0);

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
        assert_eq!(final_tx.outputs().get(1).amount().coin().to_str(), "478");
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
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(5))
                    .build()
                    .unwrap(),
            )
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
            assert_eq!(
                tx_builder
                    .fee_for_input(
                        &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred)
                            .to_address(),
                        &TransactionInput::new(&genesis_id(), 0),
                        &Value::new(&to_bignum(1_000_000))
                    )
                    .unwrap()
                    .to_str(),
                "69500"
            );
            tx_builder.add_input(
                &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred)
                    .to_address(),
                &TransactionInput::new(&genesis_id(), 0),
                &Value::new(&to_bignum(1_000_000)),
            );
        }
        tx_builder.add_input(
            &BaseAddress::new(
                NetworkInfo::testnet().network_id(),
                &spend_cred,
                &stake_cred,
            )
            .to_address(),
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new(&to_bignum(1_000_000)),
        );
        tx_builder.add_input(
            &PointerAddress::new(
                NetworkInfo::testnet().network_id(),
                &spend_cred,
                &Pointer::new_pointer(&to_bignum(0), &to_bignum(0), &to_bignum(0)),
            )
            .to_address(),
            &TransactionInput::new(&genesis_id(), 2),
            &Value::new(&to_bignum(1_000_000)),
        );
        tx_builder.add_input(
            &ByronAddress::icarus_from_key(&spend, NetworkInfo::testnet().protocol_magic())
                .to_address(),
            &TransactionInput::new(&genesis_id(), 3),
            &Value::new(&to_bignum(1_000_000)),
        );

        assert_eq!(tx_builder.inputs.len(), 4);
    }

    #[test]
    fn add_ref_inputs_to_builder() {
        let mut tx_builder = create_default_tx_builder();

        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 1));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 2));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 3));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 4));

        assert_eq!(tx_builder.reference_inputs.len(), 4);
    }

    #[test]
    fn build_tx_with_script_ref() {
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

        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 1));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 2));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 3));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 4));

        tx_builder.add_input(
            &PointerAddress::new(
                NetworkInfo::testnet().network_id(),
                &spend_cred,
                &Pointer::new_pointer(&to_bignum(0), &to_bignum(0), &to_bignum(0)),
            )
            .to_address(),
            &TransactionInput::new(&genesis_id(), 2),
            &Value::new(&to_bignum(1_000_000)),
        );

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

        let output_amount = Value::new(&to_bignum(500));

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
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
        assert_eq!(final_tx.reference_inputs().unwrap().len(), 4);
        assert_eq!(final_tx.outputs().get(1).amount().coin(), to_bignum(999499));
    }

    #[test]
    fn serialization_tx_body_with_script_ref() {
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

        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 1));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 2));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 3));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 4));

        tx_builder.add_input(
            &PointerAddress::new(
                NetworkInfo::testnet().network_id(),
                &spend_cred,
                &Pointer::new_pointer(&to_bignum(0), &to_bignum(0), &to_bignum(0)),
            )
            .to_address(),
            &TransactionInput::new(&genesis_id(), 2),
            &Value::new(&to_bignum(1_000_000)),
        );

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

        let output_amount = Value::new(&to_bignum(500));

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        tx_builder.add_change_if_needed(&change_addr).unwrap();
        let final_tx = tx_builder.build().unwrap();

        let deser_t = TransactionBody::from_bytes(final_tx.to_bytes()).unwrap();

        assert_eq!(deser_t.to_bytes(), final_tx.to_bytes());
    }

    #[test]
    fn json_serialization_tx_body_with_script_ref() {
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

        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 1));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 2));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 3));
        tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 4));

        tx_builder.add_input(
            &PointerAddress::new(
                NetworkInfo::testnet().network_id(),
                &spend_cred,
                &Pointer::new_pointer(&to_bignum(0), &to_bignum(0), &to_bignum(0)),
            )
            .to_address(),
            &TransactionInput::new(&genesis_id(), 2),
            &Value::new(&to_bignum(1_000_000)),
        );

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

        let output_amount = Value::new(&to_bignum(500));

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        tx_builder.add_change_if_needed(&change_addr).unwrap();
        let final_tx = tx_builder.build().unwrap();

        let json_tx_body = final_tx.to_json().unwrap();
        let deser_t = TransactionBody::from_json(json_tx_body.as_str()).unwrap();

        assert_eq!(deser_t.to_bytes(), final_tx.to_bytes());
        assert_eq!(deser_t.to_json().unwrap(), final_tx.to_json().unwrap());
    }

    #[ignore]
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
            &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(500)),
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
        let mut output_amount = Value::new(&to_bignum(264));
        output_amount.set_multiasset(&mass);

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
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
        assert_eq!(change.coin(), to_bignum(235));
        assert!(change.multiasset().is_none());
    }

    #[ignore]
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

        // Input with 600 coins
        tx_builder.add_input(
            &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(600)),
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
        let mut output_amount = Value::new(&to_bignum(300));
        output_amount.set_multiasset(&mass);

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
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
        assert_eq!(change.coin(), to_bignum(299));
        assert!(change.multiasset().is_some());

        let change_assets = change.multiasset().unwrap();
        let change_asset = change_assets.get(&policy_id).unwrap();
        assert_eq!(
            change_asset.get(&name).unwrap(),
            amount_minted.checked_sub(&amount_sent).unwrap(),
        );
    }

    #[test]
    fn change_with_input_and_mint_not_enough_ada() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 1));
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

        let (min_script, policy_id) = mint_script_and_policy(0);
        let asset_name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let amount_minted = to_bignum(1000);
        let amount_sent = to_bignum(500);
        let amount_input_amount = to_bignum(600);

        let mut asset_input = Assets::new();
        asset_input.insert(&asset_name, &amount_input_amount);
        let mut mass_input = MultiAsset::new();
        mass_input.insert(&policy_id, &asset_input);

        // Input with 600 coins
        tx_builder.add_input(
            &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(600)),
        );

        tx_builder.add_input(
            &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new_with_assets(&to_bignum(1), &mass_input),
        );

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        ).to_address();

        // Adding mint of the asset - which should work as an input
        tx_builder.add_mint_asset(&min_script, &asset_name, Int::new(&amount_minted));

        let mut asset = Assets::new();
        asset.insert(&asset_name, &amount_sent);
        let mut mass = MultiAsset::new();
        mass.insert(&policy_id, &asset);

        // One coin and the minted asset goes into the output
        let mut output_amount = Value::new(&to_bignum(400));
        output_amount.set_multiasset(&mass);

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
            .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr);
        assert!(added_change.is_err());
    }

    #[test]
    fn change_with_input_and_mint_not_enough_assets() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 1));
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

        let (min_script, policy_id) = mint_script_and_policy(0);
        let asset_name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let amount_minted = to_bignum(1000);
        let amount_sent = to_bignum(100000);
        let amount_input_amount = to_bignum(600);

        let mut asset_input = Assets::new();
        asset_input.insert(&asset_name, &amount_input_amount);
        let mut mass_input = MultiAsset::new();
        mass_input.insert(&policy_id, &asset_input);

        // Input with 600 coins
        tx_builder.add_input(
            &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(100000)),
        );

        tx_builder.add_input(
            &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new_with_assets(&to_bignum(1), &mass_input),
        );

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        ).to_address();

        // Adding mint of the asset - which should work as an input
        tx_builder.add_mint_asset(&min_script, &asset_name, Int::new(&amount_minted));

        let mut asset = Assets::new();
        asset.insert(&asset_name, &amount_sent);
        let mut mass = MultiAsset::new();
        mass.insert(&policy_id, &asset);

        // One coin and the minted asset goes into the output
        let mut output_amount = Value::new(&to_bignum(400));
        output_amount.set_multiasset(&mass);

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
            .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr);
        assert!(added_change.is_err());
    }

    #[ignore]
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

        for (i, (multiasset, ada)) in multiassets
            .iter()
            .zip([100u64, 1000].iter().cloned().map(to_bignum))
            .enumerate()
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            tx_builder.add_key_input(
                &&spend.to_raw_key().hash(),
                &TransactionInput::new(&genesis_id(), i as u32),
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

        let mut output_amount = Value::new(&to_bignum(500));
        output_amount.set_multiasset(&multiassets[2]);

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
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
        assert_eq!(final_tx.outputs().get(1).amount().coin(), to_bignum(599));
    }

    #[ignore]
    #[test]
    fn build_tx_with_native_assets_change_and_purification() {
        let coin_per_utxo_word = to_bignum(8);
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

        for (i, (multiasset, ada)) in multiassets
            .iter()
            .zip([100u64, 1000].iter().cloned().map(to_bignum))
            .enumerate()
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            tx_builder.add_key_input(
                &&spend.to_raw_key().hash(),
                &TransactionInput::new(&genesis_id(), i as u32),
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

        let mut output_amount = Value::new(&to_bignum(600));
        output_amount.set_multiasset(&multiassets[2]);

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
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
        assert_eq!(final_tx.outputs().get(0).amount().coin(), to_bignum(600));
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
        )
        .unwrap();
        assert_eq!(
            final_tx.outputs().get(1).amount().coin(),
            min_coin_for_dirty_change
        );
        assert_eq!(final_tx.outputs().get(2).amount().coin(), to_bignum(236));
        assert_eq!(final_tx.outputs().get(2).amount().multiasset(), None);
    }

    #[ignore]
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

        for (i, (multiasset, ada)) in multiassets
            .iter()
            .zip([300u64, 900].iter().cloned().map(to_bignum))
            .enumerate()
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            tx_builder.add_key_input(
                &&spend.to_raw_key().hash(),
                &TransactionInput::new(&genesis_id(), i as u32),
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

        let mut output_amount = Value::new(&to_bignum(300));
        output_amount.set_multiasset(&multiassets[2]);

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
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
        assert_eq!(final_tx.outputs().get(0).amount().coin(), to_bignum(300));
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
        assert_eq!(final_tx.outputs().get(1).amount().coin(), to_bignum(499));
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
        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

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
            &input_amount,
        );

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(880_000))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();
        let added_change = tx_builder.add_change_if_needed(&change_addr);
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_burn_less_than_min_ada() {
        // with this mainnet value we should end up with a final min_ada_required of just under 1_000_000
        let mut tx_builder = create_reallistic_tx_builder();

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap();
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&output_addr.to_address())
                    .next()
                    .unwrap()
                    .with_value(&Value::new(&to_bignum(2_000_000)))
                    .build()
                    .unwrap(),
            )
            .unwrap();

        tx_builder.add_input(
            &ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(2_400_000)),
        );

        tx_builder.set_ttl(1);

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap();
        let added_change = tx_builder.add_change_if_needed(&change_addr.to_address());
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_burn_empty_assets() {
        let mut tx_builder = create_reallistic_tx_builder();

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap();
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&output_addr.to_address())
                    .next()
                    .unwrap()
                    .with_value(&Value::new(&to_bignum(2_000_000)))
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let mut input_value = Value::new(&to_bignum(2_400_000));
        input_value.set_multiasset(&MultiAsset::new());
        tx_builder.add_input(
            &ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &input_value,
        );

        tx_builder.set_ttl(1);

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap();
        let added_change = tx_builder.add_change_if_needed(&change_addr.to_address());
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap()
                .coin(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
                .coin()
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
            &ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &input_amount,
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

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap();
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&output_addr.to_address())
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        tx_builder.set_ttl(1);

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap();
        let added_change = tx_builder.add_change_if_needed(&change_addr.to_address());
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
            fake_policy_id(0),
            fake_policy_id(1),
            fake_policy_id(2),
        ];
        let names = [
            AssetName::new(vec![99u8; 32]).unwrap(),
            AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
            AssetName::new(vec![4u8, 5, 6, 7, 8, 9]).unwrap(),
        ];
        let multiasset = policy_ids.iter().zip(names.iter()).fold(
            MultiAsset::new(),
            |mut acc, (policy_id, name)| {
                acc.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(500));
                    assets
                });
                acc
            },
        );
        return (multiasset, policy_ids, names);
    }

    #[ignore]
    #[test]
    fn build_tx_add_change_split_nfts() {
        let max_value_size = 100; // super low max output size to test with fewer assets
        let mut tx_builder =
            create_tx_builder_with_fee_and_val_size(&create_linear_fee(0, 1), max_value_size);

        let (multiasset, policy_ids, names) = create_multiasset();

        let mut input_value = Value::new(&to_bignum(1000));
        input_value.set_multiasset(&multiasset);

        tx_builder.add_input(
            &ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &input_value,
        );

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap()
        .to_address();
        let output_amount = Value::new(&to_bignum(208));

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&output_addr)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();

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
                    |ma| ma
                        .0
                        .iter()
                        .find(|(pid, a)| *pid == policy_id
                            && a.0.iter().find(|(name, _)| *name == asset_name).is_some())
                        .is_some()
                ))
                .is_some());
        }
        for output in final_tx.outputs.0.iter() {
            assert!(output.amount.to_bytes().len() <= max_value_size as usize);
        }
    }

    #[test]
    fn build_tx_too_big_output() {
        let mut tx_builder = create_tx_builder_with_fee_and_val_size(&create_linear_fee(0, 1), 10);

        tx_builder.add_input(
            &ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(500)),
        );

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap()
        .to_address();
        let mut output_amount = Value::new(&to_bignum(50));
        output_amount.set_multiasset(&create_multiasset().0);

        assert!(tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&output_addr)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap()
            )
            .is_err());
    }

    #[ignore]
    #[test]
    fn build_tx_add_change_nfts_not_enough_ada() {
        let mut tx_builder = create_tx_builder_with_fee_and_val_size(
            &create_linear_fee(0, 1),
            150, // super low max output size to test with fewer assets
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

        let multiasset = policy_ids.iter().zip(names.iter()).fold(
            MultiAsset::new(),
            |mut acc, (policy_id, name)| {
                acc.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(500));
                    assets
                });
                acc
            },
        );

        let mut input_value = Value::new(&to_bignum(58));
        input_value.set_multiasset(&multiasset);

        tx_builder.add_input(
            &ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &input_value,
        );

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap()
        .to_address();
        let output_amount = Value::new(&to_bignum(208));

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&output_addr)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();

        assert!(tx_builder.add_change_if_needed(&change_addr).is_err())
    }

    fn make_input(input_hash_byte: u8, value: Value) -> TransactionUnspentOutput {
        TransactionUnspentOutput::new(
            &fake_tx_input(input_hash_byte),
            &TransactionOutputBuilder::new()
                .with_address(
                    &Address::from_bech32(
                        "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                    )
                    .unwrap(),
                )
                .next()
                .unwrap()
                .with_value(&value)
                .build()
                .unwrap(),
        )
    }

    #[test]
    fn tx_builder_cip2_largest_first_increasing_fees() {
        // we have a = 1 to test increasing fees when more inputs are added
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 0));
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(
                        &Address::from_bech32(
                            "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                        )
                        .unwrap(),
                    )
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(9000))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        let mut available_inputs = TransactionUnspentOutputs::new();
        available_inputs.add(&make_input(0u8, Value::new(&to_bignum(1200))));
        available_inputs.add(&make_input(1u8, Value::new(&to_bignum(1600))));
        available_inputs.add(&make_input(2u8, Value::new(&to_bignum(6400))));
        available_inputs.add(&make_input(3u8, Value::new(&to_bignum(2400))));
        available_inputs.add(&make_input(4u8, Value::new(&to_bignum(800))));
        tx_builder
            .add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::LargestFirst)
            .unwrap();
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
        let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(change_added);
        let tx = tx_builder.build().unwrap();
        // change needed
        assert_eq!(2, tx.outputs().len());
        assert_eq!(3, tx.inputs().len());
        // confirm order of only what is necessary
        assert_eq!(1u8, tx.inputs().get(0).transaction_id().0[0]);
        assert_eq!(2u8, tx.inputs().get(1).transaction_id().0[0]);
        assert_eq!(3u8, tx.inputs().get(2).transaction_id().0[0]);
    }

    #[test]
    fn tx_builder_cip2_largest_first_static_fees() {
        // we have a = 0 so we know adding inputs/outputs doesn't change the fee so we can analyze more
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(
                        &Address::from_bech32(
                            "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                        )
                        .unwrap(),
                    )
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(1200))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        let mut available_inputs = TransactionUnspentOutputs::new();
        available_inputs.add(&make_input(0u8, Value::new(&to_bignum(150))));
        available_inputs.add(&make_input(1u8, Value::new(&to_bignum(200))));
        available_inputs.add(&make_input(2u8, Value::new(&to_bignum(800))));
        available_inputs.add(&make_input(3u8, Value::new(&to_bignum(400))));
        available_inputs.add(&make_input(4u8, Value::new(&to_bignum(100))));
        tx_builder
            .add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::LargestFirst)
            .unwrap();
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
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

    #[ignore]
    #[test]
    fn tx_builder_cip2_largest_first_multiasset() {
        // we have a = 0 so we know adding inputs/outputs doesn't change the fee so we can analyze more
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
        let pid1 = PolicyID::from([1u8; 28]);
        let pid2 = PolicyID::from([2u8; 28]);
        let asset_name1 = AssetName::new(vec![1u8; 8]).unwrap();
        let asset_name2 = AssetName::new(vec![2u8; 11]).unwrap();
        let asset_name3 = AssetName::new(vec![3u8; 9]).unwrap();

        let mut output_value = Value::new(&to_bignum(415));
        let mut output_ma = MultiAsset::new();
        output_ma.set_asset(&pid1, &asset_name1, to_bignum(5));
        output_ma.set_asset(&pid1, &asset_name2, to_bignum(1));
        output_ma.set_asset(&pid2, &asset_name2, to_bignum(2));
        output_ma.set_asset(&pid2, &asset_name3, to_bignum(4));
        output_value.set_multiasset(&output_ma);
        tx_builder
            .add_output(&TransactionOutput::new(
                &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z")
                    .unwrap(),
                &output_value,
            ))
            .unwrap();

        let mut available_inputs = TransactionUnspentOutputs::new();
        // should not be taken
        available_inputs.add(&make_input(0u8, Value::new(&to_bignum(150))));

        // should not be taken
        let mut input1 = make_input(1u8, Value::new(&to_bignum(200)));
        let mut ma1 = MultiAsset::new();
        ma1.set_asset(&pid1, &asset_name1, to_bignum(10));
        ma1.set_asset(&pid1, &asset_name2, to_bignum(1));
        ma1.set_asset(&pid2, &asset_name2, to_bignum(2));
        input1.output.amount.set_multiasset(&ma1);
        available_inputs.add(&input1);

        // taken first to satisfy pid1:asset_name1 (but also satisfies pid2:asset_name3)
        let mut input2 = make_input(2u8, Value::new(&to_bignum(10)));
        let mut ma2 = MultiAsset::new();
        ma2.set_asset(&pid1, &asset_name1, to_bignum(20));
        ma2.set_asset(&pid2, &asset_name3, to_bignum(4));
        input2.output.amount.set_multiasset(&ma2);
        available_inputs.add(&input2);

        // taken second to satisfy pid1:asset_name2 (but also satisfies pid2:asset_name1)
        let mut input3 = make_input(3u8, Value::new(&to_bignum(50)));
        let mut ma3 = MultiAsset::new();
        ma3.set_asset(&pid2, &asset_name1, to_bignum(5));
        ma3.set_asset(&pid1, &asset_name2, to_bignum(15));
        input3.output.amount.multiasset = Some(ma3);
        available_inputs.add(&input3);

        // should not be taken either
        let mut input4 = make_input(4u8, Value::new(&to_bignum(10)));
        let mut ma4 = MultiAsset::new();
        ma4.set_asset(&pid1, &asset_name1, to_bignum(10));
        ma4.set_asset(&pid1, &asset_name2, to_bignum(10));
        input4.output.amount.multiasset = Some(ma4);
        available_inputs.add(&input4);

        // taken third to satisfy pid2:asset_name_2
        let mut input5 = make_input(5u8, Value::new(&to_bignum(10)));
        let mut ma5 = MultiAsset::new();
        ma5.set_asset(&pid1, &asset_name2, to_bignum(10));
        ma5.set_asset(&pid2, &asset_name2, to_bignum(3));
        input5.output.amount.multiasset = Some(ma5);
        available_inputs.add(&input5);

        // should be taken to get enough ADA
        let input6 = make_input(6u8, Value::new(&to_bignum(900)));
        available_inputs.add(&input6);

        // should not be taken
        available_inputs.add(&make_input(7u8, Value::new(&to_bignum(100))));
        tx_builder
            .add_inputs_from(
                &available_inputs,
                CoinSelectionStrategyCIP2::LargestFirstMultiAsset,
            )
            .unwrap();
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
        let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(change_added);
        let tx = tx_builder.build().unwrap();

        assert_eq!(2, tx.outputs().len());
        assert_eq!(4, tx.inputs().len());
        // check order expected per-asset
        assert_eq!(2u8, tx.inputs().get(0).transaction_id().0[0]);
        assert_eq!(3u8, tx.inputs().get(1).transaction_id().0[0]);
        assert_eq!(5u8, tx.inputs().get(2).transaction_id().0[0]);
        assert_eq!(6u8, tx.inputs().get(3).transaction_id().0[0]);

        let change = tx.outputs().get(1).amount;
        assert_eq!(from_bignum(&change.coin), 555);
        let change_ma = change.multiasset().unwrap();
        assert_eq!(15, from_bignum(&change_ma.get_asset(&pid1, &asset_name1)));
        assert_eq!(24, from_bignum(&change_ma.get_asset(&pid1, &asset_name2)));
        assert_eq!(1, from_bignum(&change_ma.get_asset(&pid2, &asset_name2)));
        assert_eq!(0, from_bignum(&change_ma.get_asset(&pid2, &asset_name3)));
        let expected_input = input2
            .output
            .amount
            .checked_add(&input3.output.amount)
            .unwrap()
            .checked_add(&input5.output.amount)
            .unwrap()
            .checked_add(&input6.output.amount)
            .unwrap();
        let expected_change = expected_input.checked_sub(&output_value).unwrap();
        assert_eq!(expected_change, change);
    }

    #[ignore]
    #[test]
    fn tx_builder_cip2_random_improve_multiasset() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
        let pid1 = PolicyID::from([1u8; 28]);
        let pid2 = PolicyID::from([2u8; 28]);
        let asset_name1 = AssetName::new(vec![1u8; 8]).unwrap();
        let asset_name2 = AssetName::new(vec![2u8; 11]).unwrap();
        let asset_name3 = AssetName::new(vec![3u8; 9]).unwrap();

        let mut output_value = Value::new(&to_bignum(415));
        let mut output_ma = MultiAsset::new();
        output_ma.set_asset(&pid1, &asset_name1, to_bignum(5));
        output_ma.set_asset(&pid1, &asset_name2, to_bignum(1));
        output_ma.set_asset(&pid2, &asset_name2, to_bignum(2));
        output_ma.set_asset(&pid2, &asset_name3, to_bignum(4));
        output_value.set_multiasset(&output_ma);
        tx_builder
            .add_output(&TransactionOutput::new(
                &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z")
                    .unwrap(),
                &output_value,
            ))
            .unwrap();

        let mut available_inputs = TransactionUnspentOutputs::new();
        available_inputs.add(&make_input(0u8, Value::new(&to_bignum(150))));

        let mut input1 = make_input(1u8, Value::new(&to_bignum(200)));
        let mut ma1 = MultiAsset::new();
        ma1.set_asset(&pid1, &asset_name1, to_bignum(10));
        ma1.set_asset(&pid1, &asset_name2, to_bignum(1));
        ma1.set_asset(&pid2, &asset_name2, to_bignum(2));
        input1.output.amount.set_multiasset(&ma1);
        available_inputs.add(&input1);

        let mut input2 = make_input(2u8, Value::new(&to_bignum(10)));
        let mut ma2 = MultiAsset::new();
        ma2.set_asset(&pid1, &asset_name1, to_bignum(20));
        ma2.set_asset(&pid2, &asset_name3, to_bignum(4));
        input2.output.amount.set_multiasset(&ma2);
        available_inputs.add(&input2);

        let mut input3 = make_input(3u8, Value::new(&to_bignum(50)));
        let mut ma3 = MultiAsset::new();
        ma3.set_asset(&pid2, &asset_name1, to_bignum(5));
        ma3.set_asset(&pid1, &asset_name2, to_bignum(15));
        input3.output.amount.multiasset = Some(ma3);
        available_inputs.add(&input3);

        let mut input4 = make_input(4u8, Value::new(&to_bignum(10)));
        let mut ma4 = MultiAsset::new();
        ma4.set_asset(&pid1, &asset_name1, to_bignum(10));
        ma4.set_asset(&pid1, &asset_name2, to_bignum(10));
        input4.output.amount.multiasset = Some(ma4);
        available_inputs.add(&input4);

        let mut input5 = make_input(5u8, Value::new(&to_bignum(10)));
        let mut ma5 = MultiAsset::new();
        ma5.set_asset(&pid1, &asset_name2, to_bignum(10));
        ma5.set_asset(&pid2, &asset_name2, to_bignum(3));
        input5.output.amount.multiasset = Some(ma5);
        available_inputs.add(&input5);

        let input6 = make_input(6u8, Value::new(&to_bignum(1000)));
        available_inputs.add(&input6);
        available_inputs.add(&make_input(7u8, Value::new(&to_bignum(100))));

        let mut input8 = make_input(8u8, Value::new(&to_bignum(10)));
        let mut ma8 = MultiAsset::new();
        ma8.set_asset(&pid2, &asset_name2, to_bignum(10));
        input8.output.amount.multiasset = Some(ma8);
        available_inputs.add(&input8);

        let mut input9 = make_input(9u8, Value::new(&to_bignum(10)));
        let mut ma9 = MultiAsset::new();
        ma9.set_asset(&pid2, &asset_name3, to_bignum(10));
        input9.output.amount.multiasset = Some(ma9);
        available_inputs.add(&input9);

        tx_builder
            .add_inputs_from(
                &available_inputs,
                CoinSelectionStrategyCIP2::RandomImproveMultiAsset,
            )
            .unwrap();
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
        let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(change_added);
        let tx = tx_builder.build().unwrap();

        assert_eq!(2, tx.outputs().len());

        let input_total = tx_builder.get_explicit_input().unwrap();
        assert!(input_total >= output_value);
    }

    #[test]
    fn tx_builder_cip2_random_improve() {
        // we have a = 1 to test increasing fees when more inputs are added
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 0));
        const COST: u64 = 10000;
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(
                        &Address::from_bech32(
                            "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                        )
                        .unwrap(),
                    )
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(COST))
                    .build()
                    .unwrap(),
            )
            .unwrap();
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
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
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
        assert!(
            input_total
                >= Value::new(
                    &tx_builder
                        .min_fee()
                        .unwrap()
                        .checked_add(&to_bignum(COST))
                        .unwrap()
                )
        );
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
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(
                        &Address::from_bech32(
                            "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                        )
                        .unwrap(),
                    )
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(COST))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        let mut available_inputs = TransactionUnspentOutputs::new();
        available_inputs.add(&make_input(1u8, Value::new(&to_bignum(800))));
        available_inputs.add(&make_input(2u8, Value::new(&to_bignum(800))));
        let add_inputs_res =
            tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::RandomImprove);
        assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
    }

    #[ignore]
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
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(
                        &Address::from_bech32(
                            "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                        )
                        .unwrap(),
                    )
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(COST))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(tx_builder.min_fee().unwrap(), to_bignum(53));
        let mut available_inputs = TransactionUnspentOutputs::new();
        available_inputs.add(&make_input(1u8, Value::new(&to_bignum(150))));
        available_inputs.add(&make_input(2u8, Value::new(&to_bignum(150))));
        available_inputs.add(&make_input(3u8, Value::new(&to_bignum(150))));
        let add_inputs_res =
            tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::RandomImprove);
        assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
        assert_eq!(tx_builder.min_fee().unwrap(), to_bignum(264));
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
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

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

        tx_builder.add_key_input(
            &spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(999_000))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);
        tx_builder.set_fee(&to_bignum(1_000));

        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );

        let _final_tx = tx_builder.build().unwrap();
        let _deser_t = TransactionBody::from_bytes(_final_tx.to_bytes()).unwrap();

        assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
    }

    fn build_full_tx(
        body: &TransactionBody,
        witness_set: &TransactionWitnessSet,
        auxiliary_data: Option<AuxiliaryData>,
    ) -> Transaction {
        return Transaction::new(body, witness_set, auxiliary_data);
    }

    #[test]
    fn build_tx_multisig_spend_1on1_unsigned() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(10, 2));

        let spend = root_key_15() //multisig
            .derive(harden(1854))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_15() //multisig
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
        let addr_multisig = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();
        let addr_output = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        tx_builder.add_input(
            &addr_multisig,
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_output)
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(999_000))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);
        tx_builder.set_fee(&to_bignum(1_000));

        let mut auxiliary_data = AuxiliaryData::new();
        let mut pubkey_native_scripts = NativeScripts::new();
        let mut oneof_native_scripts = NativeScripts::new();

        let spending_hash = spend.to_raw_key().hash();
        pubkey_native_scripts.add(&NativeScript::new_script_pubkey(&ScriptPubkey::new(
            &spending_hash,
        )));
        oneof_native_scripts.add(&NativeScript::new_script_n_of_k(&ScriptNOfK::new(
            1,
            &pubkey_native_scripts,
        )));
        auxiliary_data.set_native_scripts(&oneof_native_scripts);
        tx_builder.set_auxiliary_data(&auxiliary_data);

        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );

        let _final_tx = tx_builder.build().unwrap();
        let _deser_t = TransactionBody::from_bytes(_final_tx.to_bytes()).unwrap();

        assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
        assert_eq!(
            _deser_t.auxiliary_data_hash.unwrap(),
            utils::hash_auxiliary_data(&auxiliary_data)
        );
    }

    #[test]
    fn build_tx_multisig_1on1_signed() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(10, 2));
        let spend = root_key_15()
            .derive(harden(1854)) //multisig
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_15()
            .derive(harden(1854)) //multisig
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();

        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();
        tx_builder.add_key_input(
            &spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&addr_net_0)
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(999_000))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);
        tx_builder.set_fee(&to_bignum(1_000));

        let mut auxiliary_data = AuxiliaryData::new();
        let mut pubkey_native_scripts = NativeScripts::new();
        let mut oneof_native_scripts = NativeScripts::new();

        let spending_hash = spend.to_raw_key().hash();
        pubkey_native_scripts.add(&NativeScript::new_script_pubkey(&ScriptPubkey::new(
            &spending_hash,
        )));
        oneof_native_scripts.add(&NativeScript::new_script_n_of_k(&ScriptNOfK::new(
            1,
            &pubkey_native_scripts,
        )));
        auxiliary_data.set_native_scripts(&oneof_native_scripts);
        tx_builder.set_auxiliary_data(&auxiliary_data);

        let body = tx_builder.build().unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );

        let mut witness_set = TransactionWitnessSet::new();
        let mut vkw = Vkeywitnesses::new();
        vkw.add(&make_vkey_witness(
            &hash_transaction(&body),
            &PrivateKey::from_normal_bytes(
                &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
                    .unwrap(),
            )
            .unwrap(),
        ));
        witness_set.set_vkeys(&vkw);

        let _final_tx = build_full_tx(&body, &witness_set, None);
        let _deser_t = Transaction::from_bytes(_final_tx.to_bytes()).unwrap();
        assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
        assert_eq!(
            _deser_t.body().auxiliary_data_hash.unwrap(),
            utils::hash_auxiliary_data(&auxiliary_data)
        );
    }

    #[ignore]
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
                .coins_per_utxo_word(&to_bignum(8))
                .prefer_pure_change(true)
                .build()
                .unwrap(),
        );

        let policy_id = PolicyID::from([0u8; 28]);
        let names = [
            AssetName::new(vec![99u8; 32]).unwrap(),
            AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
            AssetName::new(vec![4u8, 5, 6, 7]).unwrap(),
            AssetName::new(vec![5u8, 5, 6, 7]).unwrap(),
            AssetName::new(vec![6u8, 5, 6, 7]).unwrap(),
        ];
        let assets = names.iter().fold(Assets::new(), |mut a, name| {
            a.insert(&name, &to_bignum(500));
            a
        });
        let mut multiasset = MultiAsset::new();
        multiasset.insert(&policy_id, &assets);

        let mut input_value = Value::new(&to_bignum(1200));
        input_value.set_multiasset(&multiasset);

        tx_builder.add_input(
            &ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &input_value,
        );

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap()
        .to_address();
        let output_amount = Value::new(&to_bignum(208));

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&output_addr)
                    .next()
                    .unwrap()
                    .with_value(&output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();

        let add_change_result = tx_builder.add_change_if_needed(&change_addr);
        assert!(add_change_result.is_ok());
        assert_eq!(tx_builder.outputs.len(), 4);

        let change1 = tx_builder.outputs.get(1);
        let change2 = tx_builder.outputs.get(2);
        let change3 = tx_builder.outputs.get(3);

        assert_eq!(change1.address, change_addr);
        assert_eq!(change1.address, change2.address);
        assert_eq!(change1.address, change3.address);

        assert_eq!(change1.amount.coin, to_bignum(288));
        assert_eq!(change2.amount.coin, to_bignum(293));
        assert_eq!(change3.amount.coin, to_bignum(410));

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
        )
        .unwrap()
    }

    fn create_aux_with_metadata(metadatum_key: &TransactionMetadatumLabel) -> AuxiliaryData {
        let mut metadata = GeneralTransactionMetadata::new();
        metadata.insert(metadatum_key, &create_json_metadatum());

        let mut aux = AuxiliaryData::new();
        aux.set_metadata(&metadata);

        let mut nats = NativeScripts::new();
        nats.add(&NativeScript::new_timelock_start(&TimelockStart::new(123)));
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
        tx_builder
            .add_json_metadatum(&num, create_json_metadatum_string())
            .unwrap();

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
        tx_builder
            .add_json_metadatum(&num2, create_json_metadatum_string())
            .unwrap();

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
        assert_eq!(
            result_asset.get(&create_asset_name()).unwrap(),
            Int::new_i32(1234)
        );
    }

    fn mint_script_and_policy_and_hash(x: u8) -> (NativeScript, PolicyID, Ed25519KeyHash) {
        let hash = fake_key_hash(x);
        let mint_script = NativeScript::new_script_pubkey(&ScriptPubkey::new(&hash));
        let policy_id = mint_script.hash();
        (mint_script, policy_id, hash)
    }

    fn mint_script_and_policy(x: u8) -> (NativeScript, ScriptHash) {
        let (m, p, _) = mint_script_and_policy_and_hash(x);
        (m, p)
    }

    fn plutus_script_and_hash(x: u8) -> (PlutusScript, ScriptHash) {
        let s = PlutusScript::new(fake_bytes_32(x));
        (s.clone(), s.hash())
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

        let (mint_script1, policy_id1) = mint_script_and_policy(0);
        let (mint_script2, policy_id2) = mint_script_and_policy(1);

        tx_builder
            .set_mint(
                &create_mint_with_one_asset(&policy_id1),
                &NativeScripts::from(vec![mint_script1.clone()]),
            )
            .unwrap();

        tx_builder.set_mint_asset(&mint_script2, &create_mint_asset());

        assert!(tx_builder.mint.is_some());
        assert!(tx_builder.mint_scripts.is_some());

        let mint = tx_builder.mint.unwrap();
        let mint_scripts = tx_builder.mint_scripts.unwrap();

        assert_eq!(mint.len(), 2);
        assert_mint_asset(&mint, &policy_id1);
        assert_mint_asset(&mint, &policy_id2);

        // Only second script is present in the scripts
        assert_eq!(mint_scripts.len(), 2);
        assert_eq!(mint_scripts.get(0), mint_script1);
        assert_eq!(mint_scripts.get(1), mint_script2);
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

        let (mint_script1, policy_id1) = mint_script_and_policy(0);
        let (mint_script2, policy_id2) = mint_script_and_policy(1);

        tx_builder
            .set_mint(
                &create_mint_with_one_asset(&policy_id1),
                &NativeScripts::from(vec![mint_script1.clone()]),
            )
            .unwrap();
        tx_builder.add_mint_asset(&mint_script2, &create_asset_name(), Int::new_i32(1234));

        assert!(tx_builder.mint.is_some());
        assert!(tx_builder.mint_scripts.is_some());

        let mint = tx_builder.mint.unwrap();
        let mint_scripts = tx_builder.mint_scripts.unwrap();

        assert_eq!(mint.len(), 2);
        assert_mint_asset(&mint, &policy_id1);
        assert_mint_asset(&mint, &policy_id2);

        assert_eq!(mint_scripts.len(), 2);
        assert_eq!(mint_scripts.get(0), mint_script1);
        assert_eq!(mint_scripts.get(1), mint_script2);
    }

    #[ignore]
    #[test]
    fn add_output_amount() {
        let mut tx_builder = create_default_tx_builder();

        let policy_id1 = PolicyID::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);
        let mut value = Value::new(&to_bignum(249));
        value.set_multiasset(&multiasset);

        let address = byron_address();
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&address)
                    .next()
                    .unwrap()
                    .with_value(&value)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount, value);
    }

    #[ignore]
    #[test]
    fn add_output_coin() {
        let mut tx_builder = create_default_tx_builder();

        let address = byron_address();
        let coin = to_bignum(208);
        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&address)
                    .next()
                    .unwrap()
                    .with_coin(&coin)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.coin, coin);
        assert!(out.amount.multiasset.is_none());
    }

    #[ignore]
    #[test]
    fn add_output_coin_and_multiasset() {
        let mut tx_builder = create_default_tx_builder();

        let policy_id1 = PolicyID::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);

        let address = byron_address();
        let coin = to_bignum(249);

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&address)
                    .next()
                    .unwrap()
                    .with_coin_and_asset(&coin, &multiasset)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.coin, coin);
        assert_eq!(out.amount.multiasset.unwrap(), multiasset);
    }

    #[ignore]
    #[test]
    fn add_output_asset_and_min_required_coin() {
        let mut tx_builder = create_reallistic_tx_builder();

        let policy_id1 = PolicyID::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);

        let address = byron_address();

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&address)
                    .next()
                    .unwrap()
                    .with_asset_and_min_required_coin_by_utxo_cost(
                        &multiasset,
                        &tx_builder.config.utxo_cost(),
                    )
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.multiasset.unwrap(), multiasset);
        assert_eq!(out.amount.coin, to_bignum(1146460));
    }

    #[ignore]
    #[test]
    fn add_mint_asset_and_output() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script0, policy_id0) = mint_script_and_policy(0);
        let (mint_script1, policy_id1) = mint_script_and_policy(1);

        let name = create_asset_name();
        let amount = Int::new_i32(1234);

        let address = byron_address();
        let coin = to_bignum(249);

        // Add unrelated mint first to check it is NOT added to output later
        tx_builder.add_mint_asset(&mint_script0, &name, amount.clone());

        tx_builder
            .add_mint_asset_and_output(
                &mint_script1,
                &name,
                amount.clone(),
                &TransactionOutputBuilder::new()
                    .with_address(&address)
                    .next()
                    .unwrap(),
                &coin,
            )
            .unwrap();

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

    #[ignore]
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

        tx_builder
            .add_mint_asset_and_output_min_required_coin(
                &mint_script1,
                &name,
                amount.clone(),
                &TransactionOutputBuilder::new()
                    .with_address(&address)
                    .next()
                    .unwrap(),
            )
            .unwrap();

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
        assert_eq!(out.amount.coin, to_bignum(1146460));

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
    fn add_mint_includes_witnesses_into_fee_estimation() {
        let mut tx_builder = create_reallistic_tx_builder();

        let hash0 = fake_key_hash(0);

        let (mint_script1, _, hash1) = mint_script_and_policy_and_hash(1);
        let (mint_script2, _, _) = mint_script_and_policy_and_hash(2);
        let (mint_script3, _, _) = mint_script_and_policy_and_hash(3);

        let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let name2 = AssetName::new(vec![1u8, 1, 2, 3]).unwrap();
        let name3 = AssetName::new(vec![2u8, 1, 2, 3]).unwrap();
        let name4 = AssetName::new(vec![3u8, 1, 2, 3]).unwrap();
        let amount = Int::new_i32(1234);

        // One input from unrelated address
        tx_builder.add_key_input(
            &hash0,
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(10_000_000)),
        );

        // One input from same address as mint
        tx_builder.add_key_input(
            &hash1,
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new(&to_bignum(10_000_000)),
        );

        // Original tx fee now assumes two VKey signatures for two inputs
        let original_tx_fee = tx_builder.min_fee().unwrap();
        assert_eq!(original_tx_fee, to_bignum(168361));

        // Add minting four assets from three different policies
        tx_builder.add_mint_asset(&mint_script1, &name1, amount.clone());
        tx_builder.add_mint_asset(&mint_script2, &name2, amount.clone());
        tx_builder.add_mint_asset(&mint_script3, &name3, amount.clone());
        tx_builder.add_mint_asset(&mint_script3, &name4, amount.clone());

        let mint = tx_builder.get_mint().unwrap();
        let mint_len = mint.to_bytes().len();

        let mint_scripts = tx_builder.get_witness_set();
        let mint_scripts_len =
            mint_scripts.to_bytes().len() - TransactionWitnessSet::new().to_bytes().len();

        let fee_coefficient = tx_builder.config.fee_algo.coefficient();

        let raw_mint_fee = fee_coefficient
            .checked_mul(&to_bignum(mint_len as u64))
            .unwrap();

        let raw_mint_script_fee = fee_coefficient
            .checked_mul(&to_bignum(mint_scripts_len as u64))
            .unwrap();

        assert_eq!(raw_mint_fee, to_bignum(5544));
        assert_eq!(raw_mint_script_fee, to_bignum(4312));

        let new_tx_fee = tx_builder.min_fee().unwrap();

        let fee_diff_from_adding_mint = new_tx_fee.checked_sub(&original_tx_fee).unwrap();

        let witness_fee_increase = fee_diff_from_adding_mint
            .checked_sub(&raw_mint_fee)
            .unwrap()
            .checked_sub(&raw_mint_script_fee)
            .unwrap();

        assert_eq!(witness_fee_increase, to_bignum(8932));

        let fee_increase_bytes = from_bignum(&witness_fee_increase)
            .checked_div(from_bignum(&fee_coefficient))
            .unwrap();

        // Two vkey witnesses 96 bytes each (32 byte pubkey + 64 byte signature)
        // Plus 11 bytes overhead for CBOR wrappers
        // This is happening because we have three different minting policies
        // but the same key-hash from one of them is already also used in inputs
        // so no suplicate witness signature is require for that one
        assert_eq!(fee_increase_bytes, 203);
    }

    #[test]
    fn fee_estimation_fails_on_missing_mint_scripts() {
        let mut tx_builder = create_reallistic_tx_builder();

        // No error estimating fee without mint
        assert!(tx_builder.min_fee().is_ok());

        let (mint_script1, policy_id1) = mint_script_and_policy(0);
        let (mint_script2, _) = mint_script_and_policy(1);

        let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let amount = Int::new_i32(1234);

        let mut mint = Mint::new();
        mint.insert(
            &policy_id1,
            &MintAssets::new_from_entry(&name1, amount.clone()),
        );

        tx_builder
            .set_mint(&mint, &NativeScripts::from(vec![mint_script1]))
            .unwrap();

        let est1 = tx_builder.min_fee();
        assert!(est1.is_ok());

        tx_builder.add_mint_asset(&mint_script2, &name1, amount.clone());

        let est2 = tx_builder.min_fee();
        assert!(est2.is_ok());

        // Native script assertion has been commented out in `.min_fee`
        // Until implemented in a more performant manner
        // TODO: these test parts might be returned back when it's done

        // // Remove one mint script
        // tx_builder.mint_scripts =
        //     Some(NativeScripts::from(vec![tx_builder.mint_scripts.unwrap().get(1)]));
        //
        // // Now two different policies are minted but only one witness script is present
        // let est3 = tx_builder.min_fee();
        // assert!(est3.is_err());
        // assert!(est3.err().unwrap().to_string().contains(&format!("{:?}", hex::encode(policy_id1.to_bytes()))));
        //
        // // Remove all mint scripts
        // tx_builder.mint_scripts = Some(NativeScripts::new());
        //
        // // Mint exists but no witness scripts at all present
        // let est4 = tx_builder.min_fee();
        // assert!(est4.is_err());
        // assert!(est4.err().unwrap().to_string().contains("witness scripts are not provided"));
        //
        // // Remove all mint scripts
        // tx_builder.mint_scripts = None;
        //
        // // Mint exists but no witness scripts at all present
        // let est5 = tx_builder.min_fee();
        // assert!(est5.is_err());
        // assert!(est5.err().unwrap().to_string().contains("witness scripts are not provided"));
    }

    #[ignore]
    #[test]
    fn total_input_output_with_mint_and_burn() {
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

        for (i, (multiasset, ada)) in multiassets
            .iter()
            .zip([100u64, 100, 100].iter().cloned().map(to_bignum))
            .enumerate()
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            tx_builder.add_key_input(
                &&spend.to_raw_key().hash(),
                &TransactionInput::new(&genesis_id(), i as u32),
                &input_amount,
            );
        }

        tx_builder
            .add_output(
                &TransactionOutputBuilder::new()
                    .with_address(&byron_address())
                    .next()
                    .unwrap()
                    .with_coin(&to_bignum(208))
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let total_input_before_mint = tx_builder.get_total_input().unwrap();
        let total_output_before_mint = tx_builder.get_total_output().unwrap();

        assert_eq!(total_input_before_mint.coin, to_bignum(300));
        assert_eq!(total_output_before_mint.coin, to_bignum(208));
        let ma1_input = total_input_before_mint.multiasset.unwrap();
        let ma1_output = total_output_before_mint.multiasset;
        assert_eq!(
            ma1_input.get(&policy_id1).unwrap().get(&name).unwrap(),
            to_bignum(360)
        );
        assert_eq!(
            ma1_input.get(&policy_id2).unwrap().get(&name).unwrap(),
            to_bignum(360)
        );
        assert!(ma1_output.is_none());

        // Adding mint
        tx_builder.add_mint_asset(&mint_script1, &name, Int::new_i32(40));

        // Adding burn
        tx_builder.add_mint_asset(&mint_script2, &name, Int::new_i32(-40));

        let total_input_after_mint = tx_builder.get_total_input().unwrap();
        let total_output_after_mint = tx_builder.get_total_output().unwrap();

        assert_eq!(total_input_after_mint.coin, to_bignum(300));
        assert_eq!(total_output_before_mint.coin, to_bignum(208));
        let ma2_input = total_input_after_mint.multiasset.unwrap();
        let ma2_output = total_output_after_mint.multiasset.unwrap();
        assert_eq!(
            ma2_input.get(&policy_id1).unwrap().get(&name).unwrap(),
            to_bignum(400)
        );
        assert_eq!(
            ma2_input.get(&policy_id2).unwrap().get(&name).unwrap(),
            to_bignum(360)
        );
        assert_eq!(
            ma2_output.get(&policy_id2).unwrap().get(&name).unwrap(),
            to_bignum(40)
        );
    }

    fn create_base_address_from_script_hash(sh: &ScriptHash) -> Address {
        BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &StakeCredential::from_scripthash(sh),
            &StakeCredential::from_keyhash(&fake_key_hash(0)),
        )
        .to_address()
    }

    #[test]
    fn test_set_input_scripts() {
        let mut tx_builder = create_reallistic_tx_builder();
        let (script1, hash1) = mint_script_and_policy(0);
        let (script2, hash2) = mint_script_and_policy(1);
        let (script3, _hash3) = mint_script_and_policy(2);
        // Trying to set native scripts to the builder
        let rem0 = tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![
            script1.clone(),
            script2.clone(),
            script3.clone(),
        ]));
        assert_eq!(rem0, 0);
        let missing0 = tx_builder.count_missing_input_scripts();
        assert_eq!(missing0, 0);
        // Adding two script inputs using script1 and script2 hashes
        tx_builder.add_input(
            &create_base_address_from_script_hash(&hash1),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        tx_builder.add_input(
            &create_base_address_from_script_hash(&hash2),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        // Setting a non-matching script will not change anything
        let rem1 = tx_builder
            .add_required_native_input_scripts(&NativeScripts::from(vec![script3.clone()]));
        assert_eq!(rem1, 2);
        let missing1 = tx_builder.count_missing_input_scripts();
        assert_eq!(missing1, 2);
        // Setting one of the required scripts leaves one to be required
        let rem2 = tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![
            script1.clone(),
            script3.clone(),
        ]));
        assert_eq!(rem2, 1);
        let missing2 = tx_builder.count_missing_input_scripts();
        assert_eq!(missing2, 1);
        // Setting one non-required script again does not change anything
        // But shows the state has changed
        let rem3 = tx_builder
            .add_required_native_input_scripts(&NativeScripts::from(vec![script3.clone()]));
        assert_eq!(rem3, 1);
        let missing3 = tx_builder.count_missing_input_scripts();
        assert_eq!(missing3, 1);
        // Setting two required scripts will show both of them added
        // And the remainder required is zero
        let rem4 = tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![
            script1.clone(),
            script2.clone(),
        ]));
        assert_eq!(rem4, 0);
        let missing4 = tx_builder.count_missing_input_scripts();
        assert_eq!(missing4, 0);
        // Setting empty scripts does not change anything
        // But shows the state has changed
        let rem5 = tx_builder.add_required_native_input_scripts(&NativeScripts::new());
        assert_eq!(rem5, 0);
    }

    #[test]
    fn test_add_native_script_input() {
        let mut tx_builder = create_reallistic_tx_builder();
        let (script1, _hash1) = mint_script_and_policy(0);
        let (script2, _hash2) = mint_script_and_policy(1);
        let (script3, hash3) = mint_script_and_policy(2);
        // Adding two script inputs directly with their witness
        tx_builder.add_native_script_input(
            &script1,
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        tx_builder.add_native_script_input(
            &script2,
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        // Adding one script input indirectly via hash3 address
        tx_builder.add_input(
            &create_base_address_from_script_hash(&hash3),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        // Checking missing input scripts shows one
        // Because first two inputs already have their witness
        let missing1 = tx_builder.count_missing_input_scripts();
        assert_eq!(missing1, 1);
        // Setting the required script leaves none to be required`
        let rem1 = tx_builder
            .add_required_native_input_scripts(&NativeScripts::from(vec![script3.clone()]));
        assert_eq!(rem1, 0);
        let missing2 = tx_builder.count_missing_input_scripts();
        assert_eq!(missing2, 0);
    }

    fn unsafe_tx_len(b: &TransactionBuilder) -> usize {
        b.build_tx_unsafe().unwrap().to_bytes().len()
    }

    #[test]
    fn test_native_input_scripts_are_added_to_the_witnesses() {
        let mut tx_builder = create_reallistic_tx_builder();
        let (script1, _hash1) = mint_script_and_policy(0);
        let (script2, hash2) = mint_script_and_policy(1);
        tx_builder.set_fee(&to_bignum(42));
        tx_builder.add_native_script_input(
            &script1,
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        let tx_len_before_new_script_input = unsafe_tx_len(&tx_builder);
        tx_builder.add_input(
            &create_base_address_from_script_hash(&hash2),
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new(&to_bignum(1_000_000)),
        );
        let tx_len_after_new_script_input = unsafe_tx_len(&tx_builder);
        // Tx size increased cuz input is added even without the witness
        assert!(tx_len_after_new_script_input > tx_len_before_new_script_input);
        tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![script2.clone()]));
        let tx_len_after_adding_script_witness = unsafe_tx_len(&tx_builder);
        // Tx size increased cuz the witness is added to the witnesses
        assert!(tx_len_after_adding_script_witness > tx_len_after_new_script_input);
        tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![
            script1.clone(),
            script2.clone(),
        ]));
        let tx_len_after_adding_script_witness_again = unsafe_tx_len(&tx_builder);
        // Tx size did not change because calling to add same witnesses again doesn't change anything
        assert!(tx_len_after_adding_script_witness == tx_len_after_adding_script_witness_again);
        let tx: Transaction = tx_builder.build_tx_unsafe().unwrap();
        assert!(tx.witness_set.native_scripts.is_some());
        let native_scripts = tx.witness_set.native_scripts.unwrap();
        assert_eq!(native_scripts.len(), 2);
        assert_eq!(native_scripts.get(0), script1);
        assert_eq!(native_scripts.get(1), script2);
    }

    #[test]
    fn test_building_with_missing_witness_script_fails() {
        let mut tx_builder = create_reallistic_tx_builder();
        let (script1, _hash1) = mint_script_and_policy(0);
        let (script2, hash2) = mint_script_and_policy(1);
        tx_builder.set_fee(&to_bignum(42));
        // Ok to build before any inputs
        assert!(tx_builder.build_tx().is_ok());
        // Adding native script input which adds the witness right away
        tx_builder.add_native_script_input(
            &script1,
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        // Ok to build when witness is added along with the input
        assert!(tx_builder.build_tx().is_ok());
        // Adding script input without the witness
        tx_builder.add_input(
            &create_base_address_from_script_hash(&hash2),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        // Not ok to build when missing a witness
        assert!(tx_builder.build_tx().is_err());
        // Can force to build using unsafe
        assert!(tx_builder.build_tx_unsafe().is_ok());
        // Adding the missing witness script
        tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![script2.clone()]));
        // Ok to build when all witnesses are added
        assert!(tx_builder.build_tx().is_ok());
    }

    #[test]
    fn test_adding_plutus_script_input() {
        let mut tx_builder = create_reallistic_tx_builder();
        let (script1, _) = plutus_script_and_hash(0);
        let datum = PlutusData::new_bytes(fake_bytes_32(1));
        let redeemer_datum = PlutusData::new_bytes(fake_bytes_32(2));
        let redeemer = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &redeemer_datum,
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );
        tx_builder.add_plutus_script_input(
            &PlutusWitness::new(&script1, &datum, &redeemer),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        tx_builder.set_fee(&to_bignum(42));
        // There are no missing script witnesses
        assert_eq!(tx_builder.count_missing_input_scripts(), 0);
        let tx: Transaction = tx_builder.build_tx_unsafe().unwrap();
        assert!(tx.witness_set.plutus_scripts.is_some());
        assert_eq!(tx.witness_set.plutus_scripts.unwrap().get(0), script1);
        assert!(tx.witness_set.plutus_data.is_some());
        assert_eq!(tx.witness_set.plutus_data.unwrap().get(0), datum);
        assert!(tx.witness_set.redeemers.is_some());
        assert_eq!(tx.witness_set.redeemers.unwrap().get(0), redeemer);
    }

    #[test]
    fn test_adding_plutus_script_witnesses() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        let (script1, hash1) = plutus_script_and_hash(0);
        let (script2, hash2) = plutus_script_and_hash(1);
        let (script3, _hash3) = plutus_script_and_hash(3);
        let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
        let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
        let redeemer1 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &PlutusData::new_bytes(fake_bytes_32(20)),
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );
        let redeemer2 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(1),
            &PlutusData::new_bytes(fake_bytes_32(21)),
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );
        tx_builder.add_input(
            &create_base_address_from_script_hash(&hash1),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        tx_builder.add_input(
            &create_base_address_from_script_hash(&hash2),
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new(&to_bignum(1_000_000)),
        );
        // There are TWO missing script witnesses
        assert_eq!(tx_builder.count_missing_input_scripts(), 2);
        // Calling to add two plutus witnesses, one of which is irrelevant
        tx_builder.add_required_plutus_input_scripts(&PlutusWitnesses::from(vec![
            PlutusWitness::new(&script1, &datum1, &redeemer1),
            PlutusWitness::new(&script3, &datum2, &redeemer2),
        ]));
        // There is now ONE missing script witnesses
        assert_eq!(tx_builder.count_missing_input_scripts(), 1);
        // Calling to add the one remaining relevant plutus witness now
        tx_builder.add_required_plutus_input_scripts(&PlutusWitnesses::from(vec![
            PlutusWitness::new(&script2, &datum2, &redeemer2),
        ]));
        // There is now no missing script witnesses
        assert_eq!(tx_builder.count_missing_input_scripts(), 0);
        let tx: Transaction = tx_builder.build_tx_unsafe().unwrap();
        // Check there are two correct scripts
        assert!(tx.witness_set.plutus_scripts.is_some());
        let pscripts = tx.witness_set.plutus_scripts.unwrap();
        assert_eq!(pscripts.len(), 2);
        assert_eq!(pscripts.get(0), script1);
        assert_eq!(pscripts.get(1), script2);
        // Check there are two correct datums
        assert!(tx.witness_set.plutus_data.is_some());
        let datums = tx.witness_set.plutus_data.unwrap();
        assert_eq!(datums.len(), 2);
        assert_eq!(datums.get(0), datum1);
        assert_eq!(datums.get(1), datum2);
        // Check there are two correct redeemers
        assert!(tx.witness_set.redeemers.is_some());
        let redeems = tx.witness_set.redeemers.unwrap();
        assert_eq!(redeems.len(), 2);
        assert_eq!(redeems.get(0), redeemer1);
        assert_eq!(redeems.get(1), redeemer2);
    }

    fn create_collateral() -> TxInputsBuilder {
        let mut collateral_builder = TxInputsBuilder::new();
        collateral_builder.add_input(
            &byron_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        collateral_builder
    }

    #[test]
    fn test_existing_plutus_scripts_require_data_hash() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        tx_builder.set_collateral(&create_collateral());
        let (script1, _) = plutus_script_and_hash(0);
        let datum = PlutusData::new_bytes(fake_bytes_32(1));
        let redeemer_datum = PlutusData::new_bytes(fake_bytes_32(2));
        let redeemer = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &redeemer_datum,
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );
        tx_builder.add_plutus_script_input(
            &PlutusWitness::new(&script1, &datum, &redeemer),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );

        // Using SAFE `.build_tx`
        let res = tx_builder.build_tx();
        assert!(res.is_err());
        if let Err(e) = res {
            assert!(e.as_string().unwrap().contains("script data hash"));
        }

        // Setting script data hash removes the error
        tx_builder.set_script_data_hash(&ScriptDataHash::from_bytes(fake_bytes_32(42)).unwrap());
        // Using SAFE `.build_tx`
        let res2 = tx_builder.build_tx();
        assert!(res2.is_ok());

        // Removing script data hash will cause error again
        tx_builder.remove_script_data_hash();
        // Using SAFE `.build_tx`
        let res3 = tx_builder.build_tx();
        assert!(res3.is_err());
    }

    #[test]
    fn test_calc_script_hash_data() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        tx_builder.set_collateral(&create_collateral());

        let (script1, _) = plutus_script_and_hash(0);
        let datum = PlutusData::new_bytes(fake_bytes_32(1));
        let redeemer_datum = PlutusData::new_bytes(fake_bytes_32(2));
        let redeemer = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &redeemer_datum,
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );
        tx_builder.add_plutus_script_input(
            &PlutusWitness::new(&script1, &datum, &redeemer),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );

        // Setting script data hash removes the error
        tx_builder
            .calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models())
            .unwrap();

        // Using SAFE `.build_tx`
        let res2 = tx_builder.build_tx();
        assert!(res2.is_ok());

        let data_hash = hash_script_data(
            &Redeemers::from(vec![redeemer.clone()]),
            &TxBuilderConstants::plutus_default_cost_models(),
            Some(PlutusList::from(vec![datum])),
        );
        assert_eq!(tx_builder.script_data_hash.unwrap(), data_hash);
    }

    #[test]
    fn test_plutus_witness_redeemer_index_auto_changing() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        tx_builder.set_collateral(&create_collateral());
        let (script1, _) = plutus_script_and_hash(0);
        let (script2, _) = plutus_script_and_hash(1);
        let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
        let datum2 = PlutusData::new_bytes(fake_bytes_32(11));

        // Creating redeemers with indexes ZERO
        let redeemer1 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &PlutusData::new_bytes(fake_bytes_32(20)),
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );
        let redeemer2 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &PlutusData::new_bytes(fake_bytes_32(21)),
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );

        // Add a regular NON-script input first
        tx_builder.add_input(
            &byron_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );

        // Adding two plutus inputs then
        // both have redeemers with index ZERO
        tx_builder.add_plutus_script_input(
            &PlutusWitness::new(&script1, &datum1, &redeemer1),
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new(&to_bignum(1_000_000)),
        );
        tx_builder.add_plutus_script_input(
            &PlutusWitness::new(&script2, &datum2, &redeemer2),
            &TransactionInput::new(&genesis_id(), 2),
            &Value::new(&to_bignum(1_000_000)),
        );

        // Calc the script data hash
        tx_builder
            .calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models())
            .unwrap();

        let tx: Transaction = tx_builder.build_tx().unwrap();
        assert!(tx.witness_set.redeemers.is_some());
        let redeems = tx.witness_set.redeemers.unwrap();
        assert_eq!(redeems.len(), 2);

        fn compare_redeems(r1: Redeemer, r2: Redeemer) {
            assert_eq!(r1.tag(), r2.tag());
            assert_eq!(r1.data(), r2.data());
            assert_eq!(r1.ex_units(), r2.ex_units());
        }

        compare_redeems(redeems.get(0), redeemer1);
        compare_redeems(redeems.get(1), redeemer2);

        // Note the redeemers from the result transaction are equal with source redeemers
        // In everything EXCEPT the index field, the indexes have changed to 1 and 2
        // To match the position of their corresponding input
        assert_eq!(redeems.get(0).index(), to_bignum(1));
        assert_eq!(redeems.get(1).index(), to_bignum(2));
    }

    #[test]
    fn test_native_and_plutus_scripts_together() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        tx_builder.set_collateral(&create_collateral());
        let (pscript1, _) = plutus_script_and_hash(0);
        let (pscript2, phash2) = plutus_script_and_hash(1);
        let (nscript1, _) = mint_script_and_policy(0);
        let (nscript2, nhash2) = mint_script_and_policy(1);
        let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
        let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
        // Creating redeemers with indexes ZERO
        let redeemer1 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &PlutusData::new_bytes(fake_bytes_32(20)),
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );
        let redeemer2 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &PlutusData::new_bytes(fake_bytes_32(21)),
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );

        // Add one plutus input directly with witness
        tx_builder.add_plutus_script_input(
            &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        // Add one native input directly with witness
        tx_builder.add_native_script_input(
            &nscript1,
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new(&to_bignum(1_000_000)),
        );
        // Add one plutus input generically without witness
        tx_builder.add_input(
            &create_base_address_from_script_hash(&phash2),
            &TransactionInput::new(&genesis_id(), 2),
            &Value::new(&to_bignum(1_000_000)),
        );
        // Add one native input generically without witness
        tx_builder.add_input(
            &create_base_address_from_script_hash(&nhash2),
            &TransactionInput::new(&genesis_id(), 3),
            &Value::new(&to_bignum(1_000_000)),
        );

        // There are two missing script witnesses
        assert_eq!(tx_builder.count_missing_input_scripts(), 2);

        let remaining1 =
            tx_builder.add_required_plutus_input_scripts(&PlutusWitnesses::from(vec![
                PlutusWitness::new(&pscript2, &datum2, &redeemer2),
            ]));

        // There is one missing script witness now
        assert_eq!(remaining1, 1);
        assert_eq!(tx_builder.count_missing_input_scripts(), 1);

        let remaining2 = tx_builder
            .add_required_native_input_scripts(&NativeScripts::from(vec![nscript2.clone()]));

        // There are no missing script witnesses now
        assert_eq!(remaining2, 0);
        assert_eq!(tx_builder.count_missing_input_scripts(), 0);

        tx_builder
            .calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models())
            .unwrap();

        let tx: Transaction = tx_builder.build_tx().unwrap();

        let wits = tx.witness_set;
        assert!(wits.native_scripts.is_some());
        assert!(wits.plutus_scripts.is_some());
        assert!(wits.plutus_data.is_some());
        assert!(wits.redeemers.is_some());

        let nscripts = wits.native_scripts.unwrap();
        assert_eq!(nscripts.len(), 2);
        assert_eq!(nscripts.get(0), nscript1);
        assert_eq!(nscripts.get(1), nscript2);

        let pscripts = wits.plutus_scripts.unwrap();
        assert_eq!(pscripts.len(), 2);
        assert_eq!(pscripts.get(0), pscript1);
        assert_eq!(pscripts.get(1), pscript2);

        let datums = wits.plutus_data.unwrap();
        assert_eq!(datums.len(), 2);
        assert_eq!(datums.get(0), datum1);
        assert_eq!(datums.get(1), datum2);

        let redeems = wits.redeemers.unwrap();
        assert_eq!(redeems.len(), 2);
        assert_eq!(redeems.get(0), redeemer1);

        // The second plutus input redeemer index has automatically changed to 2
        // because it was added on the third position
        assert_eq!(redeems.get(1), redeemer2.clone_with_index(&to_bignum(2)));
    }

    #[test]
    fn test_json_serialization_native_and_plutus_scripts_together() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        tx_builder.set_collateral(&create_collateral());
        let (pscript1, _) = plutus_script_and_hash(0);
        let (pscript2, phash2) = plutus_script_and_hash(1);
        let (nscript1, _) = mint_script_and_policy(0);
        let (nscript2, nhash2) = mint_script_and_policy(1);
        let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
        let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
        // Creating redeemers with indexes ZERO
        let redeemer1 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &PlutusData::new_bytes(fake_bytes_32(20)),
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );
        let redeemer2 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &PlutusData::new_bytes(fake_bytes_32(21)),
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );

        // Add one plutus input directly with witness
        tx_builder.add_plutus_script_input(
            &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        // Add one native input directly with witness
        tx_builder.add_native_script_input(
            &nscript1,
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new(&to_bignum(1_000_000)),
        );
        // Add one plutus input generically without witness
        tx_builder.add_input(
            &create_base_address_from_script_hash(&phash2),
            &TransactionInput::new(&genesis_id(), 2),
            &Value::new(&to_bignum(1_000_000)),
        );
        // Add one native input generically without witness
        tx_builder.add_input(
            &create_base_address_from_script_hash(&nhash2),
            &TransactionInput::new(&genesis_id(), 3),
            &Value::new(&to_bignum(1_000_000)),
        );

        // There are two missing script witnesses
        assert_eq!(tx_builder.count_missing_input_scripts(), 2);

        let remaining1 =
            tx_builder.add_required_plutus_input_scripts(&PlutusWitnesses::from(vec![
                PlutusWitness::new(&pscript2, &datum2, &redeemer2),
            ]));

        // There is one missing script witness now
        assert_eq!(remaining1, 1);
        assert_eq!(tx_builder.count_missing_input_scripts(), 1);

        let remaining2 = tx_builder
            .add_required_native_input_scripts(&NativeScripts::from(vec![nscript2.clone()]));

        // There are no missing script witnesses now
        assert_eq!(remaining2, 0);
        assert_eq!(tx_builder.count_missing_input_scripts(), 0);

        tx_builder.calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models());

        let tx: Transaction = tx_builder.build_tx().unwrap();

        let json_tx = tx.to_json().unwrap();
        let deser_tx = Transaction::from_json(json_tx.as_str()).unwrap();

        assert_eq!(deser_tx.to_bytes(), tx.to_bytes());
        assert_eq!(deser_tx.to_json().unwrap(), tx.to_json().unwrap());
    }

    #[test]
    fn test_regular_and_collateral_inputs_same_keyhash() {
        let mut input_builder = TxInputsBuilder::new();
        let mut collateral_builder = TxInputsBuilder::new();

        // Add a single input of both kinds with the SAME keyhash
        input_builder.add_input(
            &fake_base_address(0),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        collateral_builder.add_input(
            &fake_base_address(0),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );

        fn get_fake_vkeys_count(i: &TxInputsBuilder, c: &TxInputsBuilder) -> usize {
            let mut tx_builder = create_reallistic_tx_builder();
            tx_builder.set_fee(&to_bignum(42));
            tx_builder.set_inputs(i);
            tx_builder.set_collateral(c);
            let tx: Transaction = fake_full_tx(&tx_builder, tx_builder.build().unwrap()).unwrap();
            tx.witness_set.vkeys.unwrap().len()
        }

        // There's only one fake witness in the builder
        // because a regular and a collateral inputs both use the same keyhash
        assert_eq!(get_fake_vkeys_count(&input_builder, &collateral_builder), 1);

        // Add a new input of each kind with DIFFERENT keyhashes
        input_builder.add_input(
            &fake_base_address(1),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        collateral_builder.add_input(
            &fake_base_address(2),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );

        // There are now three fake witnesses in the builder
        // because all three unique keyhashes got combined
        assert_eq!(get_fake_vkeys_count(&input_builder, &collateral_builder), 3);
    }

    #[test]
    fn test_regular_and_collateral_inputs_together() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        let (pscript1, _) = plutus_script_and_hash(0);
        let (pscript2, _) = plutus_script_and_hash(1);
        let (nscript1, _) = mint_script_and_policy(0);
        let (nscript2, _) = mint_script_and_policy(1);
        let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
        let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
        // Creating redeemers with indexes ZERO
        let redeemer1 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &PlutusData::new_bytes(fake_bytes_32(20)),
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );
        let redeemer2 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &PlutusData::new_bytes(fake_bytes_32(21)),
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );

        let mut input_builder = TxInputsBuilder::new();
        let mut collateral_builder = TxInputsBuilder::new();

        input_builder.add_native_script_input(
            &nscript1,
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        collateral_builder.add_native_script_input(
            &nscript2,
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new(&to_bignum(1_000_000)),
        );

        input_builder.add_plutus_script_input(
            &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
            &TransactionInput::new(&genesis_id(), 2),
            &Value::new(&to_bignum(1_000_000)),
        );
        collateral_builder.add_plutus_script_input(
            &PlutusWitness::new(&pscript2, &datum2, &redeemer2),
            &TransactionInput::new(&genesis_id(), 3),
            &Value::new(&to_bignum(1_000_000)),
        );

        tx_builder.set_inputs(&input_builder);
        tx_builder.set_collateral(&collateral_builder);

        tx_builder
            .calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models())
            .unwrap();

        let w: &TransactionWitnessSet = &tx_builder.build_tx().unwrap().witness_set;

        assert!(w.native_scripts.is_some());
        let nscripts = w.native_scripts.as_ref().unwrap();
        assert_eq!(nscripts.len(), 2);
        assert_eq!(nscripts.get(0), nscript1);
        assert_eq!(nscripts.get(1), nscript2);

        assert!(w.plutus_scripts.is_some());
        let pscripts = w.plutus_scripts.as_ref().unwrap();
        assert_eq!(pscripts.len(), 2);
        assert_eq!(pscripts.get(0), pscript1);
        assert_eq!(pscripts.get(1), pscript2);

        assert!(w.plutus_data.is_some());
        let datums = w.plutus_data.as_ref().unwrap();
        assert_eq!(datums.len(), 2);
        assert_eq!(datums.get(0), datum1);
        assert_eq!(datums.get(1), datum2);

        assert!(w.redeemers.is_some());
        let redeemers = w.redeemers.as_ref().unwrap();
        assert_eq!(redeemers.len(), 2);
        assert_eq!(redeemers.get(0), redeemer1.clone_with_index(&to_bignum(1)));
        assert_eq!(redeemers.get(1), redeemer2.clone_with_index(&to_bignum(1)));
    }

    #[test]
    fn test_ex_unit_costs_are_added_to_the_fees() {
        fn calc_fee_with_ex_units(mem: u64, step: u64) -> Coin {
            let mut input_builder = TxInputsBuilder::new();
            let mut collateral_builder = TxInputsBuilder::new();

            // Add a single input of both kinds with the SAME keyhash
            input_builder.add_input(
                &fake_base_address(0),
                &TransactionInput::new(&genesis_id(), 0),
                &Value::new(&to_bignum(1_000_000)),
            );
            collateral_builder.add_input(
                &fake_base_address(0),
                &TransactionInput::new(&genesis_id(), 1),
                &Value::new(&to_bignum(1_000_000)),
            );

            let (pscript1, _) = plutus_script_and_hash(0);
            let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
            let redeemer1 = Redeemer::new(
                &RedeemerTag::new_spend(),
                &to_bignum(0),
                &PlutusData::new_bytes(fake_bytes_32(20)),
                &ExUnits::new(&to_bignum(mem), &to_bignum(step)),
            );
            input_builder.add_plutus_script_input(
                &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
                &TransactionInput::new(&genesis_id(), 2),
                &Value::new(&to_bignum(1_000_000)),
            );

            let mut tx_builder = create_reallistic_tx_builder();
            tx_builder.set_inputs(&input_builder);
            tx_builder.set_collateral(&collateral_builder);

            tx_builder
                .add_change_if_needed(&fake_base_address(42))
                .unwrap();

            tx_builder.get_fee_if_set().unwrap()
        }

        assert_eq!(calc_fee_with_ex_units(0, 0), to_bignum(173509));
        assert_eq!(calc_fee_with_ex_units(10000, 0), to_bignum(174174));
        assert_eq!(calc_fee_with_ex_units(0, 10000000), to_bignum(174406));
        assert_eq!(calc_fee_with_ex_units(10000, 10000000), to_bignum(175071));
    }

    #[test]
    fn test_script_inputs_ordering() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        let (nscript1, _) = mint_script_and_policy(0);
        let (pscript1, _) = plutus_script_and_hash(0);
        let (pscript2, _) = plutus_script_and_hash(1);
        let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
        let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
        // Creating redeemers with indexes ZERO
        let pdata1 = PlutusData::new_bytes(fake_bytes_32(20));
        let pdata2 = PlutusData::new_bytes(fake_bytes_32(21));
        let redeemer1 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &pdata1,
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );
        let redeemer2 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &pdata2,
            &ExUnits::new(&to_bignum(1), &to_bignum(2)),
        );

        tx_builder.add_plutus_script_input(
            &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
            &fake_tx_input2(2, 1),
            &fake_value(),
        );
        tx_builder.add_native_script_input(&nscript1, &fake_tx_input2(1, 0), &fake_value());
        tx_builder.add_plutus_script_input(
            &PlutusWitness::new(&pscript2, &datum2, &redeemer2),
            &fake_tx_input2(2, 0),
            &fake_value(),
        );

        let tx: Transaction = tx_builder.build_tx_unsafe().unwrap();

        let ins = tx.body.inputs;
        assert_eq!(ins.len(), 3);
        assert_eq!(ins.get(0).transaction_id.0[0], 1);
        assert_eq!(ins.get(1).transaction_id.0[0], 2);
        assert_eq!(ins.get(1).index, 0);
        assert_eq!(ins.get(2).transaction_id.0[0], 2);
        assert_eq!(ins.get(2).index, 1);

        let r: Redeemers = tx.witness_set.redeemers.unwrap();
        assert_eq!(r.len(), 2);

        // Redeemer1 now has the index 2 even tho the input was added first
        assert_eq!(r.get(0).data(), pdata1);
        assert_eq!(r.get(0).index(), to_bignum(2));

        // Redeemer1 now has the index 1 even tho the input was added last
        assert_eq!(r.get(1).data(), pdata2);
        assert_eq!(r.get(1).index(), to_bignum(1));
    }

    #[test]
    fn test_required_signers() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        let tx1: TransactionBody = tx_builder.build().unwrap();
        assert!(tx1.required_signers.is_none());

        let s1 = fake_key_hash(1);
        let s2 = fake_key_hash(22);
        let s3 = fake_key_hash(133);

        tx_builder.add_required_signer(&s1);
        tx_builder.add_required_signer(&s3);
        tx_builder.add_required_signer(&s2);

        let tx1: TransactionBody = tx_builder.build().unwrap();
        assert!(tx1.required_signers.is_some());

        let rs: RequiredSigners = tx1.required_signers.unwrap();
        assert_eq!(rs.len(), 3);
        assert_eq!(rs.get(0), s1);
        assert_eq!(rs.get(1), s3);
        assert_eq!(rs.get(2), s2);
    }

    #[test]
    fn test_required_signers_are_added_to_the_witness_estimate() {
        fn count_fake_witnesses_with_required_signers(keys: &Ed25519KeyHashes) -> usize {
            let mut tx_builder = create_reallistic_tx_builder();
            tx_builder.set_fee(&to_bignum(42));
            tx_builder.add_input(
                &fake_base_address(0),
                &TransactionInput::new(&fake_tx_hash(0), 0),
                &Value::new(&to_bignum(10_000_000)),
            );

            keys.0.iter().for_each(|k| {
                tx_builder.add_required_signer(k);
            });

            let tx: Transaction = fake_full_tx(&tx_builder, tx_builder.build().unwrap()).unwrap();
            tx.witness_set.vkeys.unwrap().len()
        }

        assert_eq!(
            count_fake_witnesses_with_required_signers(&Ed25519KeyHashes::new(),),
            1
        );

        assert_eq!(
            count_fake_witnesses_with_required_signers(&Ed25519KeyHashes(vec![fake_key_hash(1)]),),
            2
        );

        assert_eq!(
            count_fake_witnesses_with_required_signers(&Ed25519KeyHashes(vec![
                fake_key_hash(1),
                fake_key_hash(2)
            ]),),
            3
        );

        // This case still produces only 3 fake signatures, because the same key is already used in the input address
        assert_eq!(
            count_fake_witnesses_with_required_signers(&Ed25519KeyHashes(vec![
                fake_key_hash(1),
                fake_key_hash(2),
                fake_key_hash(0)
            ]),),
            3
        );

        // When a different key is used - 4 fake witnesses are produced
        assert_eq!(
            count_fake_witnesses_with_required_signers(&Ed25519KeyHashes(vec![
                fake_key_hash(1),
                fake_key_hash(2),
                fake_key_hash(3)
            ]),),
            4
        );
    }

    #[test]
    fn collateral_return_and_total_collateral_setters() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let mut inp = TxInputsBuilder::new();
        inp.add_input(&fake_base_address(0), &fake_tx_input(0), &fake_value());

        tx_builder.set_inputs(&inp);
        tx_builder.set_collateral(&inp);

        let col_return = TransactionOutput::new(&fake_base_address(1), &fake_value2(123123));
        let col_total = to_bignum(234234);

        tx_builder.set_collateral_return(&col_return);
        tx_builder.set_total_collateral(&col_total);

        let tx: Transaction = tx_builder.build_tx_unsafe().unwrap();
        assert!(tx.body.collateral_return.is_some());
        assert_eq!(tx.body.collateral_return.unwrap(), col_return);
        assert!(tx.body.total_collateral.is_some());
        assert_eq!(tx.body.total_collateral.unwrap(), col_total);
    }

    fn fake_multiasset(amount: u64) -> MultiAsset {
        let (_, policy_id) = mint_script_and_policy(234);
        let mut assets = Assets::new();
        assets.insert(
            &AssetName::new(fake_bytes_32(235)).unwrap(),
            &to_bignum(amount),
        );
        let mut masset = MultiAsset::new();
        masset.insert(&policy_id, &assets);
        masset
    }

    #[test]
    fn inputs_builder_total_value() {
        let mut b = TxInputsBuilder::new();
        assert_eq!(b.total_value().unwrap(), Value::zero());

        b.add_input(
            &fake_base_address(0),
            &fake_tx_input(0),
            &fake_value2(100_000),
        );
        assert_eq!(b.total_value().unwrap(), Value::new(&to_bignum(100_000)));

        b.add_input(
            &fake_base_address(1),
            &fake_tx_input(1),
            &fake_value2(200_000),
        );
        assert_eq!(b.total_value().unwrap(), Value::new(&to_bignum(300_000)));

        let masset = fake_multiasset(123);

        b.add_input(
            &fake_base_address(2),
            &fake_tx_input(2),
            &Value::new_with_assets(&to_bignum(300_000), &masset),
        );
        assert_eq!(
            b.total_value().unwrap(),
            Value::new_with_assets(&to_bignum(600_000), &masset)
        );
    }

    #[ignore]
    #[test]
    fn test_auto_calc_total_collateral() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let mut inp = TxInputsBuilder::new();
        let collateral_input_value = 2_000_000;
        inp.add_input(
            &fake_base_address(0),
            &fake_tx_input(0),
            &fake_value2(collateral_input_value.clone()),
        );

        tx_builder.set_collateral(&inp);

        let collateral_return_value = 1_234_567;
        let col_return = TransactionOutput::new(
            &fake_base_address(1),
            &fake_value2(collateral_return_value.clone()),
        );

        tx_builder
            .set_collateral_return_and_total(&col_return)
            .unwrap();

        assert!(tx_builder.collateral_return.is_some());
        assert_eq!(tx_builder.collateral_return.unwrap(), col_return,);

        assert!(tx_builder.total_collateral.is_some());
        assert_eq!(
            tx_builder.total_collateral.unwrap(),
            to_bignum(collateral_input_value - collateral_return_value),
        );
    }

    #[ignore]
    #[test]
    fn test_auto_calc_total_collateral_with_assets() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let masset = fake_multiasset(123);

        let mut inp = TxInputsBuilder::new();
        let collateral_input_value = 2_000_000;
        inp.add_input(
            &fake_base_address(0),
            &fake_tx_input(0),
            &Value::new_with_assets(&to_bignum(collateral_input_value.clone()), &masset),
        );

        tx_builder.set_collateral(&inp);

        let collateral_return_value = 1_345_678;
        let col_return = TransactionOutput::new(
            &fake_base_address(1),
            &Value::new_with_assets(&to_bignum(collateral_return_value.clone()), &masset),
        );

        tx_builder
            .set_collateral_return_and_total(&col_return)
            .unwrap();

        assert!(tx_builder.collateral_return.is_some());
        assert_eq!(tx_builder.collateral_return.unwrap(), col_return,);

        assert!(tx_builder.total_collateral.is_some());
        assert_eq!(
            tx_builder.total_collateral.unwrap(),
            to_bignum(collateral_input_value - collateral_return_value),
        );
    }

    #[test]
    fn test_auto_calc_total_collateral_fails_with_assets() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let masset = fake_multiasset(123);

        let mut inp = TxInputsBuilder::new();
        let collateral_input_value = 2_000_000;
        inp.add_input(
            &fake_base_address(0),
            &fake_tx_input(0),
            &Value::new_with_assets(&to_bignum(collateral_input_value.clone()), &masset),
        );

        tx_builder.set_collateral(&inp);

        // Collateral return does not handle ALL the assets from collateral input
        let collateral_return_value = 1_345_678;
        let col_return = TransactionOutput::new(
            &fake_base_address(1),
            &fake_value2(collateral_return_value.clone()),
        );

        let res = tx_builder.set_collateral_return_and_total(&col_return);

        // Function call returns an error
        assert!(res.is_err());

        // NEITHER total collateral nor collateral return are changed in the builder
        assert!(tx_builder.total_collateral.is_none());
        assert!(tx_builder.collateral_return.is_none());
    }

    #[test]
    fn test_auto_calc_total_collateral_fails_on_no_collateral() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let res = tx_builder.set_collateral_return_and_total(&TransactionOutput::new(
            &fake_base_address(1),
            &fake_value2(1_345_678),
        ));

        // Function call returns an error
        assert!(res.is_err());

        // NEITHER total collateral nor collateral return are changed in the builder
        assert!(tx_builder.total_collateral.is_none());
        assert!(tx_builder.collateral_return.is_none());
    }

    #[test]
    fn test_auto_calc_total_collateral_fails_on_no_ada() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let mut inp = TxInputsBuilder::new();
        let collateral_input_value = 2_000_000;
        inp.add_input(
            &fake_base_address(0),
            &fake_tx_input(0),
            &Value::new(&to_bignum(collateral_input_value.clone())),
        );

        tx_builder.set_collateral(&inp);

        let res = tx_builder.set_collateral_return_and_total(&TransactionOutput::new(
            &fake_base_address(1),
            &fake_value2(1),
        ));

        // Function call returns an error
        assert!(res.is_err());

        // NEITHER total collateral nor collateral return are changed in the builder
        assert!(tx_builder.total_collateral.is_none());
        assert!(tx_builder.collateral_return.is_none());
    }

    #[test]
    fn test_auto_calc_collateral_return() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let mut inp = TxInputsBuilder::new();
        let collateral_input_value = 2_000_000;
        inp.add_input(
            &fake_base_address(0),
            &fake_tx_input(0),
            &fake_value2(collateral_input_value.clone()),
        );

        tx_builder.set_collateral(&inp);

        let total_collateral_value = 234_567;
        let collateral_return_address = fake_base_address(1);

        tx_builder
            .set_total_collateral_and_return(
                &to_bignum(total_collateral_value.clone()),
                &collateral_return_address,
            )
            .unwrap();

        assert!(tx_builder.total_collateral.is_some());
        assert_eq!(
            tx_builder.total_collateral.unwrap(),
            to_bignum(total_collateral_value.clone()),
        );

        assert!(tx_builder.collateral_return.is_some());
        let col_return: TransactionOutput = tx_builder.collateral_return.unwrap();
        assert_eq!(col_return.address, collateral_return_address);
        assert_eq!(
            col_return.amount,
            Value::new(&to_bignum(collateral_input_value - total_collateral_value),)
        );
    }

    #[test]
    fn test_auto_calc_collateral_return_with_assets() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let masset = fake_multiasset(123);

        let mut inp = TxInputsBuilder::new();
        let collateral_input_value = 2_000_000;
        inp.add_input(
            &fake_base_address(0),
            &fake_tx_input(0),
            &Value::new_with_assets(&to_bignum(collateral_input_value.clone()), &masset),
        );

        tx_builder.set_collateral(&inp);

        let total_collateral_value = 345_678;
        let collateral_return_address = fake_base_address(1);

        tx_builder
            .set_total_collateral_and_return(
                &to_bignum(total_collateral_value.clone()),
                &collateral_return_address,
            )
            .unwrap();

        assert!(tx_builder.total_collateral.is_some());
        assert_eq!(
            tx_builder.total_collateral.unwrap(),
            to_bignum(total_collateral_value.clone()),
        );

        assert!(tx_builder.collateral_return.is_some());
        let col_return: TransactionOutput = tx_builder.collateral_return.unwrap();
        assert_eq!(col_return.address, collateral_return_address);
        assert_eq!(
            col_return.amount,
            Value::new_with_assets(
                &to_bignum(collateral_input_value - total_collateral_value),
                &masset,
            )
        );
    }

    #[test]
    fn test_add_collateral_return_succeed_with_border_amount() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let masset = fake_multiasset(123);

        let mut inp = TxInputsBuilder::new();
        let collateral_input_value = 2_000_000;
        inp.add_input(
            &fake_base_address(0),
            &fake_tx_input(0),
            &Value::new_with_assets(&to_bignum(collateral_input_value.clone()), &masset),
        );

        tx_builder.set_collateral(&inp);

        let collateral_return_address = fake_base_address(1);

        let possible_ret = Value::new_from_assets(&masset);
        let fake_out = TransactionOutput::new(&collateral_return_address, &possible_ret);
        let min_ada = min_ada_for_output(&fake_out, &tx_builder.config.utxo_cost()).unwrap();

        let total_collateral_value = to_bignum(collateral_input_value)
            .checked_sub(&min_ada)
            .unwrap();

        tx_builder
            .set_total_collateral_and_return(&total_collateral_value, &collateral_return_address)
            .unwrap();

        assert!(tx_builder.total_collateral.is_some());
        assert!(tx_builder.collateral_return.is_some());
        let col_return: TransactionOutput = tx_builder.collateral_return.unwrap();
        assert_eq!(col_return.address, collateral_return_address);
        assert_eq!(
            col_return.amount,
            Value::new_with_assets(&min_ada, &masset,)
        );
    }

    #[test]
    fn test_add_zero_collateral_return() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let mut inp = TxInputsBuilder::new();
        let collateral_input_value = 2_000_000;
        inp.add_input(
            &fake_base_address(0),
            &fake_tx_input(0),
            &Value::new(&to_bignum(collateral_input_value.clone())),
        );

        tx_builder.set_collateral(&inp);

        let collateral_return_address = fake_base_address(1);

        tx_builder
            .set_total_collateral_and_return(
                &to_bignum(collateral_input_value.clone()),
                &collateral_return_address,
            )
            .unwrap();

        assert!(tx_builder.total_collateral.is_some());
        assert!(tx_builder.collateral_return.is_none());
    }

    #[test]
    fn test_add_collateral_return_fails_no_enough_ada() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let masset = fake_multiasset(123);

        let mut inp = TxInputsBuilder::new();
        let collateral_input_value = 2_000_000;
        inp.add_input(
            &fake_base_address(0),
            &fake_tx_input(0),
            &Value::new_with_assets(&to_bignum(collateral_input_value.clone()), &masset),
        );

        tx_builder.set_collateral(&inp);

        let collateral_return_address = fake_base_address(1);

        let possible_ret = Value::new_from_assets(&masset);
        let fake_out = TransactionOutput::new(&collateral_return_address, &possible_ret);
        let min_ada = min_ada_for_output(&fake_out, &tx_builder.config.utxo_cost()).unwrap();
        let mut total_collateral_value = to_bignum(collateral_input_value)
            .checked_sub(&min_ada)
            .unwrap();
        //make total collateral value bigger for make collateral return less then min ada
        total_collateral_value = total_collateral_value.checked_add(&to_bignum(1)).unwrap();

        let coll_add_res = tx_builder
            .set_total_collateral_and_return(&total_collateral_value, &collateral_return_address);

        assert!(coll_add_res.is_err());
        assert!(tx_builder.total_collateral.is_none());
        assert!(tx_builder.collateral_return.is_none());
    }

    #[test]
    fn test_auto_calc_collateral_return_fails_on_no_collateral() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(123456));

        let res = tx_builder
            .set_total_collateral_and_return(&to_bignum(345_678.clone()), &fake_base_address(1));

        assert!(res.is_err());
        assert!(tx_builder.total_collateral.is_none());
        assert!(tx_builder.collateral_return.is_none());
    }

    #[test]
    fn test_costmodel_retaining_for_v1() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        tx_builder.set_collateral(&create_collateral());

        let (script1, _) = plutus_script_and_hash(0);
        let datum = PlutusData::new_integer(&BigInt::from_str("42").unwrap());
        let redeemer = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &datum,
            &ExUnits::new(&to_bignum(1700), &to_bignum(368100)),
        );
        tx_builder.add_plutus_script_input(
            &PlutusWitness::new(&script1, &datum, &redeemer),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );

        // Setting script data hash removes the error
        tx_builder
            .calc_script_data_hash(&TxBuilderConstants::plutus_vasil_cost_models())
            .unwrap();

        // Using SAFE `.build_tx`
        let res2 = tx_builder.build_tx();
        assert!(res2.is_ok());

        let v1 = Language::new_plutus_v1();
        let v1_costmodel = TxBuilderConstants::plutus_vasil_cost_models()
            .get(&v1)
            .unwrap();
        let mut retained_cost_models = Costmdls::new();
        retained_cost_models.insert(&v1, &v1_costmodel);

        let data_hash = hash_script_data(
            &Redeemers::from(vec![redeemer.clone()]),
            &retained_cost_models,
            Some(PlutusList::from(vec![datum])),
        );
        assert_eq!(tx_builder.script_data_hash.unwrap(), data_hash);
    }

    #[test]
    fn test_costmodel_retaining_fails_on_missing_costmodel() {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        tx_builder.set_collateral(&create_collateral());

        let (script1, _) = plutus_script_and_hash(0);
        let datum = PlutusData::new_integer(&BigInt::from_str("42").unwrap());
        let redeemer = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &datum,
            &ExUnits::new(&to_bignum(1700), &to_bignum(368100)),
        );
        tx_builder.add_plutus_script_input(
            &PlutusWitness::new(&script1, &datum, &redeemer),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );

        let v2 = Language::new_plutus_v2();
        let v2_costmodel = TxBuilderConstants::plutus_vasil_cost_models()
            .get(&v2)
            .unwrap();
        let mut retained_cost_models = Costmdls::new();
        retained_cost_models.insert(&v2, &v2_costmodel);

        // Setting script data hash removes the error
        let calc_result = tx_builder.calc_script_data_hash(&retained_cost_models);
        assert!(calc_result.is_err());
    }
}
