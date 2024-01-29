#![allow(deprecated)]

use crate::*;

use super::*;
use crate::fees;
use crate::utils;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use crate::map_names::TxBodyNames::RequiredSigners;

pub(crate) fn fake_private_key() -> Bip32PrivateKey {
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

pub(crate) fn fake_raw_key_sig() -> Ed25519Signature {
    Ed25519Signature::from_bytes(vec![
        36, 248, 153, 211, 155, 23, 253, 93, 102, 193, 146, 196, 181, 13, 52, 62, 66, 247, 35, 91,
        48, 80, 76, 138, 231, 97, 159, 147, 200, 40, 220, 109, 206, 69, 104, 221, 105, 23, 124, 85,
        24, 40, 73, 45, 119, 122, 103, 39, 253, 102, 194, 251, 204, 189, 168, 194, 174, 237, 146,
        3, 44, 153, 121, 10,
    ])
    .unwrap()
}

pub(crate) fn fake_raw_key_public() -> PublicKey {
    PublicKey::from_bytes(&[
        207, 118, 57, 154, 33, 13, 232, 114, 14, 159, 168, 148, 228, 94, 65, 226, 154, 181, 37,
        227, 11, 196, 2, 128, 28, 7, 98, 80, 209, 88, 91, 205,
    ])
    .unwrap()
}

fn count_needed_vkeys(tx_builder: &TransactionBuilder) -> usize {
    let mut input_hashes: Ed25519KeyHashesSet = Ed25519KeyHashesSet::from(&tx_builder.inputs);
    input_hashes.extend(Ed25519KeyHashesSet::from(&tx_builder.collateral));
    input_hashes.extend(tx_builder.required_signers.clone());
    if let Some(mint_builder) = &tx_builder.mint {
        input_hashes.extend(Ed25519KeyHashesSet::from(&mint_builder.get_native_scripts()));
    }
    if let Some(withdrawals_builder) = &tx_builder.withdrawals {
        input_hashes.extend(withdrawals_builder.get_required_signers());
    }
    if let Some(certs_builder) = &tx_builder.certs {
        input_hashes.extend(certs_builder.get_required_signers());
    }
    if let Some(voting_builder) = &tx_builder.voting_procedures {
        input_hashes.extend(voting_builder.get_required_signers());
    }
    input_hashes.len()
}

// tx_body must be the result of building from tx_builder
// constructs the rest of the Transaction using fake witness data of the correct length
// for use in calculating the size of the final Transaction
pub(crate) fn fake_full_tx(
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
            (Some(s), d, Some(r))
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
    pub(crate) fee_algo: fees::LinearFee,
    pub(crate) pool_deposit: Coin,            // protocol parameter
    pub(crate) key_deposit: Coin,             // protocol parameter
    pub(crate) max_value_size: u32,           // protocol parameter
    pub(crate) max_tx_size: u32,              // protocol parameter
    pub(crate) data_cost: DataCost,           // protocol parameter
    pub(crate) ex_unit_prices: Option<ExUnitPrices>, // protocol parameter
    pub(crate) prefer_pure_change: bool,
}

impl TransactionBuilderConfig {
    pub(crate) fn utxo_cost(&self) -> DataCost {
        self.data_cost.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilderConfigBuilder {
    fee_algo: Option<fees::LinearFee>,
    pool_deposit: Option<Coin>,            // protocol parameter
    key_deposit: Option<Coin>,             // protocol parameter
    max_value_size: Option<u32>,           // protocol parameter
    max_tx_size: Option<u32>,              // protocol parameter
    data_cost: Option<DataCost>,           // protocol parameter
    ex_unit_prices: Option<ExUnitPrices>,  // protocol parameter
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
    pub(crate) config: TransactionBuilderConfig,
    pub(crate) inputs: TxInputsBuilder,
    pub(crate) collateral: TxInputsBuilder,
    pub(crate) outputs: TransactionOutputs,
    pub(crate) fee: Option<Coin>,
    pub(crate) ttl: Option<SlotBigNum>, // absolute slot number
    pub(crate) certs: Option<CertificatesBuilder>,
    pub(crate) withdrawals: Option<WithdrawalsBuilder>,
    pub(crate) auxiliary_data: Option<AuxiliaryData>,
    pub(crate) validity_start_interval: Option<SlotBigNum>,
    pub(crate) mint: Option<MintBuilder>,
    pub(crate) script_data_hash: Option<ScriptDataHash>,
    pub(crate) required_signers: Ed25519KeyHashesSet,
    pub(crate) collateral_return: Option<TransactionOutput>,
    pub(crate) total_collateral: Option<Coin>,
    pub(crate) reference_inputs: HashSet<TransactionInput>,
    pub(crate) extra_datums: Option<PlutusList>,
    pub(crate) voting_procedures: Option<VotingBuilder>,
    pub(crate) voting_proposals: Option<VotingProposalBuilder>,
    pub(crate) current_treasury_value: Option<Coin>,
    pub(crate) donation: Option<Coin>,
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
                    true,
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
                    self.add_regular_input(&input.output.address, &input.input, &input.output.amount);
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
                                false,
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
                    false,
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
                    self.add_regular_input(&input.output.address, &input.input, &input.output.amount);
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
            self.add_regular_input(&input.output.address, &input.input, &input.output.amount);
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
        pure_ada: bool,
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
        let mut available_coins = by(input_total).unwrap_or(BigNum::zero());
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
            let mut added = available_coins.clone();
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
            available_coins = added.checked_sub(&needed)?;
        }
        if !relevant_indices.is_empty() && pure_ada {
            // Phase 2: Improvement
            for output in outputs.iter_mut() {
                let associated = associated_indices.get_mut(output).unwrap();
                for i in associated.iter_mut() {
                    let random_index = rng.gen_range(0..relevant_indices.len());
                    let j: &mut usize = relevant_indices.get_mut(random_index).unwrap();
                    let input = &available_inputs[*i];
                    let new_input = &available_inputs[*j];
                    let cur = from_bignum(&by(&input.output.amount).unwrap_or(BigNum::zero()));
                    let new = from_bignum(&by(&new_input.output.amount).unwrap_or(BigNum::zero()));
                    let min = from_bignum(&by(&output.amount).unwrap_or(BigNum::zero()));
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
            if let Some(associated) = associated_indices.get(output) {
                for i in associated.iter() {
                    let input = &available_inputs[*i];
                    let input_fee = self.fee_for_input(
                        &input.output.address,
                        &input.input,
                        &input.output.amount,
                    )?;
                    self.add_regular_input(&input.output.address, &input.input, &input.output.amount);
                    *input_total = input_total.checked_add(&input.output.amount)?;
                    *output_total = output_total.checked_add(&Value::new(&input_fee))?;
                }
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

    /// This function is replace for previous one add_input.
    /// The functions adds a non script input, if it is a script input it returns an error.
    /// To add script input you need to use add_native_script_input or add_plutus_script_input.
    /// Also we recommend to use TxInputsBuilder and .set_inputs, because all add_*_input functions might be removed from transaction builder.
    #[deprecated(since = "12.0.0", note = "Use `.set_inputs`")]
    pub fn add_regular_input(&mut self, address: &Address, input: &TransactionInput, amount: &Value) -> Result<(), JsError>{
        self.inputs.add_regular_input(address, input, amount)
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

        self_copy.add_regular_input(&address, &input, &amount);
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

    /// !!! DEPRECATED !!!
    /// Can emit error if add a cert with script credential.
    /// Use set_certs_builder instead.
    #[deprecated(
        since = "11.4.1",
        note = "Can emit an error if you add a cert with script credential. Use set_certs_builder instead."
    )]
    pub fn set_certs(&mut self, certs: &Certificates) -> Result<(), JsError> {
        let mut builder = CertificatesBuilder::new();
        for cert in &certs.0 {
            builder.add(cert)?;
        }

        self.certs = Some(builder);

        Ok(())
    }

    pub fn set_certs_builder(&mut self, certs: &CertificatesBuilder) {
        self.certs = Some(certs.clone());
    }

    /// !!! DEPRECATED !!!
    /// Can emit error if add a withdrawal with script credential.
    /// Use set_withdrawals_builder instead.
    #[deprecated(
        since = "11.4.1",
        note = "Can emit an error if you add a withdrawal with script credential. Use set_withdrawals_builder instead."
    )]
    pub fn set_withdrawals(&mut self, withdrawals: &Withdrawals) -> Result<(), JsError> {
        let mut withdrawals_builder = WithdrawalsBuilder::new();
        for (withdrawal, coin) in &withdrawals.0 {
            withdrawals_builder.add(&withdrawal, &coin)?;
        }

        self.withdrawals = Some(withdrawals_builder);

        Ok(())
    }

    pub fn set_withdrawals_builder(&mut self, withdrawals: &WithdrawalsBuilder) {
        self.withdrawals = Some(withdrawals.clone());
    }

    pub fn set_voting_builder(&mut self, voting_builder: &VotingBuilder) {
        self.voting_procedures = Some(voting_builder.clone());
    }

    pub fn set_voting_proposal_builder(&mut self, voting_proposal_builder: &VotingProposalBuilder) {
        self.voting_proposals = Some(voting_proposal_builder.clone());
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

    pub fn set_mint_builder(&mut self, mint_builder: &MintBuilder) {
        self.mint = Some(mint_builder.clone());
    }

    pub fn get_mint_builder(&self) -> Option<MintBuilder> {
        self.mint.clone()
    }

    /// !!! DEPRECATED !!!
    /// Mints are defining by MintBuilder now.
    /// Use `.set_mint_builder()` and `MintBuilder` instead.
    #[deprecated(
        since = "11.2.0",
        note = "Mints are defining by MintBuilder now. Use `.set_mint_builder()` and `MintBuilder` instead."
    )]
    /// Set explicit Mint object and the required witnesses to this builder
    /// it will replace any previously existing mint and mint scripts
    /// NOTE! Error will be returned in case a mint policy does not have a matching script
    pub fn set_mint(&mut self, mint: &Mint, mint_scripts: &NativeScripts) -> Result<(), JsError> {
        assert_required_mint_scripts(mint, Some(mint_scripts))?;
        let mut scripts_policies = HashMap::new();
        for scipt in &mint_scripts.0 {
            scripts_policies.insert(scipt.hash(), scipt.clone());
        }

        let mut mint_builder = MintBuilder::new();

        for (policy_id, asset_map) in &mint.0 {
            for (asset_name, amount) in &asset_map.0 {
                if let Some(script) = scripts_policies.get(policy_id) {
                    let mint_witness = MintWitness::new_native_script(script);
                    mint_builder.set_asset(&mint_witness, asset_name, amount);
                } else {
                    return Err(JsError::from_str(
                        "Mint policy does not have a matching script",
                    ));
                }
            }
        }
        self.mint = Some(mint_builder);
        Ok(())
    }

    /// !!! DEPRECATED !!!
    /// Mints are defining by MintBuilder now.
    /// Use `.get_mint_builder()` and `.build()` instead.
    #[deprecated(
        since = "11.2.0",
        note = "Mints are defining by MintBuilder now. Use `.get_mint_builder()` and `.build()` instead."
    )]
    /// Returns a copy of the current mint state in the builder
    pub fn get_mint(&self) -> Option<Mint> {
        match &self.mint {
            Some(mint) => Some(mint.build()),
            None => None,
        }
    }

    /// Returns a copy of the current mint witness scripts in the builder
    pub fn get_mint_scripts(&self) -> Option<NativeScripts> {
        match &self.mint {
            Some(mint) => Some(mint.get_native_scripts()),
            None => None,
        }
    }

    /// !!! DEPRECATED !!!
    /// Mints are defining by MintBuilder now.
    /// Use `.set_mint_builder()` and `MintBuilder` instead.
    #[deprecated(
        since = "11.2.0",
        note = "Mints are defining by MintBuilder now. Use `.set_mint_builder()` and `MintBuilder` instead."
    )]
    /// Add a mint entry to this builder using a PolicyID and MintAssets object
    /// It will be securely added to existing or new Mint in this builder
    /// It will replace any existing mint assets with the same PolicyID
    pub fn set_mint_asset(&mut self, policy_script: &NativeScript, mint_assets: &MintAssets) {
        let mint_witness = MintWitness::new_native_script(policy_script);
        if let Some(mint) = &mut self.mint {
            for (asset, amount) in mint_assets.0.iter() {
                mint.set_asset(&mint_witness, asset, amount);
            }
        } else {
            let mut mint = MintBuilder::new();
            for (asset, amount) in mint_assets.0.iter() {
                mint.set_asset(&mint_witness, asset, amount);
            }
            self.mint = Some(mint);
        }
    }

    /// !!! DEPRECATED !!!
    /// Mints are defining by MintBuilder now.
    /// Use `.set_mint_builder()` and `MintBuilder` instead.
    #[deprecated(
        since = "11.2.0",
        note = "Mints are defining by MintBuilder now. Use `.set_mint_builder()` and `MintBuilder` instead."
    )]
    /// Add a mint entry to this builder using a PolicyID, AssetName, and Int object for amount
    /// It will be securely added to existing or new Mint in this builder
    /// It will replace any previous existing amount same PolicyID and AssetName
    pub fn add_mint_asset(
        &mut self,
        policy_script: &NativeScript,
        asset_name: &AssetName,
        amount: &Int,
    ) {
        let mint_witness = MintWitness::new_native_script(policy_script);
        if let Some(mint) = &mut self.mint {
            mint.add_asset(&mint_witness, asset_name, &amount);
        } else {
            let mut mint = MintBuilder::new();
            mint.add_asset(&mint_witness, asset_name, &amount);
            self.mint = Some(mint);
        }
    }

    /// Add a mint entry together with an output to this builder
    /// Using a PolicyID, AssetName, Int for amount, Address, and Coin (BigNum) objects
    /// The asset will be securely added to existing or new Mint in this builder
    /// A new output will be added with the specified Address, the Coin value, and the minted asset
    pub fn add_mint_asset_and_output(
        &mut self,
        policy_script: &NativeScript,
        asset_name: &AssetName,
        amount: &Int,
        output_builder: &TransactionOutputAmountBuilder,
        output_coin: &Coin,
    ) -> Result<(), JsError> {
        if !amount.is_positive() {
            return Err(JsError::from_str("Output value must be positive!"));
        }
        let policy_id: PolicyID = policy_script.hash();
        self.add_mint_asset(policy_script, asset_name, amount);
        let multiasset = Mint::new_from_entry(
            &policy_id,
            &MintAssets::new_from_entry(asset_name, amount)?,
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
        amount: &Int,
        output_builder: &TransactionOutputAmountBuilder,
    ) -> Result<(), JsError> {
        if !amount.is_positive() {
            return Err(JsError::from_str("Output value must be positive!"));
        }
        let policy_id: PolicyID = policy_script.hash();
        self.add_mint_asset(policy_script, asset_name, amount);
        let multiasset = Mint::new_from_entry(
            &policy_id,
            &MintAssets::new_from_entry(asset_name, amount)?,
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

    pub fn add_extra_witness_datum(&mut self, datum: &PlutusData) {
        if let Some(extra_datums) = &mut self.extra_datums {
            extra_datums.add(datum);
        } else {
            let mut extra_datums = PlutusList::new();
            extra_datums.add(datum);
            self.extra_datums = Some(extra_datums);
        }
    }

    pub fn get_extra_witness_datums(&self) -> Option<PlutusList> {
        self.extra_datums.clone()
    }

    pub fn set_donation(&mut self, donation: &Coin) {
        self.donation = Some(donation.clone());
    }

    pub fn get_donation(&self) -> Option<Coin> {
        self.donation.clone()
    }

    pub fn set_current_treasury_value(&mut self, current_treasury_value: &Coin) -> Result<(), JsError> {
        if current_treasury_value == &Coin::zero() {
            return Err(JsError::from_str("Current treasury value cannot be zero!"));
        }
        self.current_treasury_value = Some(current_treasury_value.clone());
        Ok(())
    }

    pub fn get_current_treasury_value(&self) -> Option<Coin> {
        self.current_treasury_value.clone()
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
            script_data_hash: None,
            required_signers: Ed25519KeyHashesSet::new(),
            collateral_return: None,
            total_collateral: None,
            reference_inputs: HashSet::new(),
            extra_datums: None,
            voting_procedures: None,
            voting_proposals: None,
            donation: None,
            current_treasury_value: None,
        }
    }

    pub fn get_reference_inputs(&self) -> TransactionInputs {
        let mut inputs = self.reference_inputs.clone();
        for input in self.inputs.get_ref_inputs().0 {
            inputs.insert(input);
        }

        if let Some(mint) = &self.mint {
            for input in mint.get_ref_inputs().0 {
                inputs.insert(input);
            }
        }

        if let Some(withdrawals) = &self.withdrawals {
            for input in withdrawals.get_ref_inputs().0 {
                inputs.insert(input);
            }
        }

        if let Some(certs) = &self.certs {
            for input in certs.get_ref_inputs().0 {
                inputs.insert(input);
            }
        }

        if let Some(voting_procedures) = &self.voting_procedures {
            for input in voting_procedures.get_ref_inputs().0 {
                inputs.insert(input);
            }
        }

        if let Some(voting_proposals) = &self.voting_proposals {
            for input in voting_proposals.get_ref_inputs().0 {
                inputs.insert(input);
            }
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
        let mut implicit_input = Value::zero();
        if let Some(withdrawals) = &self.withdrawals {
            implicit_input = implicit_input.checked_add(&withdrawals.get_total_withdrawals()?)?;
        }
        if let Some(refunds) = &self.certs {
            implicit_input = implicit_input.checked_add(
                &refunds
                    .get_certificates_refund(&self.config.pool_deposit, &self.config.key_deposit)?,
            )?;
        }

        Ok(implicit_input)
    }

    /// Returns mint as tuple of (mint_value, burn_value) or two zero values
    fn get_mint_as_values(&self) -> (Value, Value) {
        self.mint
            .as_ref()
            .map(|m| {
                (
                    Value::new_from_assets(&m.build().as_positive_multiasset()),
                    Value::new_from_assets(&m.build().as_negative_multiasset()),
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
        let mut total = self.get_explicit_output()?
            .checked_add(&Value::new(&self.get_deposit()?))?
            .checked_add(&burn_value)?;
        if let Some(donation) = &self.donation {
            total = total.checked_add(&Value::new(donation))?;
        }
        Ok(total)
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
        let mut total_deposit = Coin::zero();
        if let Some(certs) = &self.certs {
            total_deposit =
                total_deposit.checked_add(&certs.get_certificates_deposit(
                    &self.config.pool_deposit,
                    &self.config.key_deposit,
                )?)?;
        }

        if let Some(voting_proposal_builder) = &self.voting_proposals {
            total_deposit = total_deposit.checked_add(
                &voting_proposal_builder.get_total_deposit()?,
            )?;
        }

        Ok(total_deposit)
    }

    pub fn get_fee_if_set(&self) -> Option<Coin> {
        self.fee.clone()
    }

    /// Warning: this function will mutate the /fee/ field
    /// Make sure to call this function last after setting all other tx-body properties
    /// Editing inputs, outputs, mint, etc. after change been calculated
    /// might cause a mismatch in calculated fee versus the required fee
    pub fn add_change_if_needed(&mut self, address: &Address) -> Result<bool, JsError> {
        self.add_change_if_needed_with_optional_script_and_datum(address, None, None)
    }

    pub fn add_change_if_needed_with_datum(
        &mut self,
        address: &Address,
        plutus_data: &OutputDatum,
    ) -> Result<bool, JsError> {
        self.add_change_if_needed_with_optional_script_and_datum(
            address,
            Some(plutus_data.0.clone()),
            None,
        )
    }

    fn add_change_if_needed_with_optional_script_and_datum(
        &mut self,
        address: &Address,
        plutus_data: Option<DataOption>,
        script_ref: Option<ScriptRef>,
    ) -> Result<bool, JsError> {
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
        let output_total = self.get_total_output()?;

        let shortage = get_input_shortage(&input_total, &output_total, &fee)?;
        if let Some(shortage) = shortage {
            return Err(JsError::from_str(&format!(
                "Insufficient input in transaction. {}",
                shortage
            )));
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
                            serialization_format: None,
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
                                        serialization_format: None,
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
                                serialization_format: None,
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
                            serialization_format: None,
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
                                serialization_format: None,
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
                                serialization_format: None,
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
                                        serialization_format: None,
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
        let mut used_langs = BTreeSet::new();
        let mut retained_cost_models = Costmdls::new();
        let mut plutus_witnesses = PlutusWitnesses::new();
        if let Some(mut inputs_plutus) = self.inputs.get_plutus_input_scripts() {
            used_langs.append(&mut self.inputs.get_used_plutus_lang_versions());
            plutus_witnesses.0.append(&mut inputs_plutus.0)
        }
        if let Some(mut collateral_plutus) = self.collateral.get_plutus_input_scripts() {
            used_langs.append(&mut self.collateral.get_used_plutus_lang_versions());
            plutus_witnesses.0.append(&mut collateral_plutus.0)
        }
        if let Some(mint_builder) = &self.mint {
            used_langs.append(&mut mint_builder.get_used_plutus_lang_versions());
            plutus_witnesses
                .0
                .append(&mut mint_builder.get_plutus_witnesses().0)
        }
        if let Some(certs_builder) = &self.certs {
            used_langs.append(&mut certs_builder.get_used_plutus_lang_versions());
            plutus_witnesses
                .0
                .append(&mut certs_builder.get_plutus_witnesses().0)
        }
        if let Some(withdrawals_builder) = &self.withdrawals {
            used_langs.append(&mut withdrawals_builder.get_used_plutus_lang_versions());
            plutus_witnesses
                .0
                .append(&mut withdrawals_builder.get_plutus_witnesses().0)
        }
        if let Some(voting_builder) = &self.voting_procedures {
            used_langs.append(&mut voting_builder.get_used_plutus_lang_versions());
            plutus_witnesses
                .0
                .append(&mut voting_builder.get_plutus_witnesses().0)
        }

        if let Some(voting_proposal_builder) = &self.voting_proposals {
            used_langs.append(&mut voting_proposal_builder.get_used_plutus_lang_versions());
            plutus_witnesses
                .0
                .append(&mut voting_proposal_builder.get_plutus_witnesses().0)
        }

        let (_scripts, mut datums, redeemers) = plutus_witnesses.collect();
        for lang in used_langs {
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

        if let Some(extra_datum) = &self.extra_datums {
            if datums.is_none() {
                datums = Some(PlutusList::new());
            }

            for datum in extra_datum {
                if let Some(datums) = &mut datums {
                    datums.add(datum);
                }
            }
        }

        if datums.is_some() || redeemers.len() > 0 || retained_cost_models.len() > 0 {
            self.script_data_hash =
                Some(hash_script_data(&redeemers, &retained_cost_models, datums));
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
            certs: self.certs.as_ref().map(|x| x.build()),
            withdrawals: self.withdrawals.as_ref().map(|x| x.build()),
            update: None,
            auxiliary_data_hash: self
                .auxiliary_data
                .as_ref()
                .map(|x| utils::hash_auxiliary_data(x)),
            validity_start_interval: self.validity_start_interval,
            mint: self.mint.as_ref().map(|x| x.build()),
            script_data_hash: self.script_data_hash.clone(),
            collateral: self.collateral.inputs_option(),
            required_signers: self.required_signers.to_option(),
            network_id: None,
            collateral_return: self.collateral_return.clone(),
            total_collateral: self.total_collateral.clone(),
            reference_inputs: self.get_reference_inputs().to_option(),
            voting_procedures: self.voting_procedures.as_ref().map(|x| x.build()),
            voting_proposals: self.voting_proposals.as_ref().map(|x| x.build()),
            donation: self.donation.clone(),
            current_treasury_value: self.current_treasury_value.clone(),
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
        if let Some(mint_builder) = &self.mint {
            mint_builder.get_native_scripts().0.iter().for_each(|s| {
                ns.add(s);
            });
        }
        if let Some(certificates_builder) = &self.certs {
            certificates_builder
                .get_native_scripts()
                .0
                .iter()
                .for_each(|s| {
                    ns.add(s);
                });
        }
        if let Some(withdrawals_builder) = &self.withdrawals {
            withdrawals_builder
                .get_native_scripts()
                .0
                .iter()
                .for_each(|s| {
                    ns.add(s);
                });
        }
        if let Some(voting_builder) = &self.voting_procedures {
            voting_builder.get_native_scripts().0.iter().for_each(|s| {
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
        if let Some(mint_builder) = &self.mint {
            mint_builder.get_plutus_witnesses().0.iter().for_each(|s| {
                res.add(s);
            })
        }
        if let Some(certificates_builder) = &self.certs {
            certificates_builder
                .get_plutus_witnesses()
                .0
                .iter()
                .for_each(|s| {
                    res.add(s);
                })
        }
        if let Some(withdrawals_builder) = &self.withdrawals {
            withdrawals_builder
                .get_plutus_witnesses()
                .0
                .iter()
                .for_each(|s| {
                    res.add(s);
                })
        }
        if let Some(voting_builder) = &self.voting_procedures {
            voting_builder
                .get_plutus_witnesses()
                .0
                .iter()
                .for_each(|s| {
                    res.add(s);
                })
        }
        if let Some(voting_proposal_builder) = &self.voting_proposals {
            voting_proposal_builder
                .get_plutus_witnesses()
                .0
                .iter()
                .for_each(|s| {
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
    pub(crate) fn get_witness_set(&self) -> TransactionWitnessSet {
        let mut wit = TransactionWitnessSet::new();
        if let Some(scripts) = self.get_combined_native_scripts() {
            wit.set_native_scripts(&scripts);
        }
        let mut all_datums = None;
        if let Some(pw) = self.get_combined_plutus_scripts() {
            let (scripts, datums, redeemers) = pw.collect();
            wit.set_plutus_scripts(&scripts);
            all_datums = datums;
            wit.set_redeemers(&redeemers);
        }

        if let Some(extra_datum) = &self.extra_datums {
            if all_datums.is_none() {
                all_datums = Some(PlutusList::new());
            }

            for datum in extra_datum {
                if let Some(datums) = &mut all_datums {
                    datums.add(datum);
                }
            }
        }

        if let Some(datums) = &all_datums {
            wit.set_plutus_data(datums);
        }

        wit
    }

    fn has_plutus_inputs(&self) -> bool {
        if self.inputs.has_plutus_scripts() {
            return true;
        }
        if self.mint.as_ref().map_or(false, |m| m.has_plutus_scripts()) {
            return true;
        }
        if self
            .certs
            .as_ref()
            .map_or(false, |c| c.has_plutus_scripts())
        {
            return true;
        }
        if self
            .withdrawals
            .as_ref()
            .map_or(false, |w| w.has_plutus_scripts())
        {
            return true;
        }
        if self
            .voting_procedures
            .as_ref()
            .map_or(false, |w| w.has_plutus_scripts())
        {
            return true;
        }
        if self
            .voting_proposals
            .as_ref()
            .map_or(false, |w| w.has_plutus_scripts())
        {
            return true;
        }

        return false;
    }

    /// Returns full Transaction object with the body and the auxiliary data
    /// NOTE: witness_set will contain all mint_scripts if any been added or set
    /// NOTE: is_valid set to true
    /// NOTE: Will fail in case there are any script inputs added with no corresponding witness
    pub fn build_tx(&self) -> Result<Transaction, JsError> {
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
