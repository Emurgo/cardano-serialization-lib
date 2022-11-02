use std::collections::HashMap;
use crate::tx_builder::batch_tools::proposals::{TxOutputProposal, TxProposal};
use super::assets_calculator::AssetsCalculator;
use super::cbor_calculator::CborCalculator;
use super::utxo_stat::UtxosStat;
use super::indexes::{UtxoIndex, AssetIndex, PolicyIndex, PlaneAssetId};
use super::super::*;

#[derive(Clone)]
pub(crate) struct TxProposalChanges {
    tx_proposal: TxProposal,
    makes_new_outputs: bool,
    asset_utxo: Vec<UtxoIndex>,
    ada_utxos: Vec<UtxoIndex>,
}

impl TxProposalChanges {
    pub(crate) fn new(tx_proposal: TxProposal, makes_new_outputs: bool) -> Self {
        TxProposalChanges {
            tx_proposal,
            makes_new_outputs,
            asset_utxo: Vec::new(),
            ada_utxos: Vec::new(),
        }
    }
}

pub struct AssetCategorizer {
    address: Address,
    config: TransactionBuilderConfig,
    assets: Vec<PlaneAssetId>,
    policies: Vec<PolicyID>,
    assets_calculator: AssetsCalculator,
    assets_amounts: Vec<HashMap<UtxoIndex, Coin>>,
    assets_counts: Vec<(AssetIndex, usize)>,
    utxos_ada: Vec<Coin>,
    addresses: Vec<Address>,

    //assets and utoxs that can be used
    free_utxo_to_assets: HashMap<UtxoIndex, HashSet<AssetIndex>>,
    free_asset_to_utxos: HashMap<AssetIndex, HashSet<UtxoIndex>>,

    asset_to_policy: HashMap<AssetIndex, PolicyIndex>,
    policy_to_asset: HashMap<PolicyIndex, HashSet<AssetIndex>>,
    inputs_sizes: Vec<usize>,

    free_ada_utxos: Vec<(UtxoIndex, Coin)>,
    //utxos_with_ada_overhead: Vec<(UtxoIndex, Coin)>,

    output_size: usize,
}

impl AssetCategorizer {
    pub(crate) fn new(config: &TransactionBuilderConfig, utxos: &TransactionUnspentOutputs, address: &Address) -> Result<Self, JsError> {
        let mut assets: Vec<PlaneAssetId> = Vec::new();
        let mut utxos_ada: Vec<Coin> = Vec::new();
        let mut policies: Vec<PolicyID> = Vec::new();
        let mut assets_name_sizes: Vec<usize> = Vec::new();
        let mut assets_amounts: Vec<HashMap<UtxoIndex, Coin>> = Vec::new();
        //let mut assets_counts: Vec<(AssetIndex, usize)> = Vec::new();
        let mut free_utxo_to_assets: HashMap<UtxoIndex, HashSet<AssetIndex>> = HashMap::new();
        let mut free_asset_to_utxos: HashMap<AssetIndex, HashSet<UtxoIndex>> = HashMap::new();
        let mut asset_to_policy: HashMap<AssetIndex, PolicyIndex> = HashMap::new();
        let mut policy_to_asset: HashMap<PolicyIndex, HashSet<AssetIndex>> = HashMap::new();

        let mut asset_ids: HashMap<PlaneAssetId, AssetIndex> = HashMap::new();
        let mut policy_ids: HashMap<PolicyID, PolicyIndex> = HashMap::new();
        let mut assets_counts: Vec<(AssetIndex, usize)> = Vec::new();

        let mut current_utxo_num = 0usize;
        let mut asset_count = 0usize;
        let mut policy_count = 0usize;
        let mut total_ada = Coin::zero();

        let mut free_ada_utxos = Vec::new();
        let mut addresses = Vec::new();

        let mut utxos_with_ada_overhead = Vec::new();

        for utxo in &utxos.0 {
            total_ada = total_ada.checked_add(&utxo.output.amount.coin)?;

            let current_utxo_index = UtxoIndex(current_utxo_num.clone());
            utxos_ada.push(utxo.output.amount.coin.clone());
            addresses.push(utxo.output.address.clone());

            let ada_overhead = Self::calc_utxo_output_overhead(address, &utxo.output.amount, config)?;
            if ada_overhead > Coin::zero() {
                utxos_with_ada_overhead.push((current_utxo_index.clone(), ada_overhead));
            }

            if let Some(assests) = &utxo.output.amount.multiasset {
                for policy in &assests.0 {
                    let mut current_policy_index = PolicyIndex(policy_count.clone());
                    if let Some(policy_index) = policy_ids.get(policy.0) {
                        current_policy_index = policy_index.clone()
                    } else {
                        policies.push(policy.0.clone());
                        policy_ids.insert(policy.0.clone(), current_policy_index.clone());
                        policy_count += 1;
                    }

                    for asset in &policy.1.0 {
                        let mut current_asset_index = AssetIndex(asset_count.clone());
                        let plane_id = PlaneAssetId(current_policy_index.clone(), asset.0.clone());

                        if let Some(asset_index) = asset_ids.get(&plane_id) {
                            current_asset_index = asset_index.clone();
                            assets_counts[current_asset_index.0].1 += 1;
                        } else {
                            let mut asset_name_size = CborCalculator::get_struct_size(asset.0.0.len() as u64);
                            asset_name_size += asset.0.0.len();
                            assets.push(plane_id.clone());
                            assets_name_sizes.push(asset_name_size);
                            asset_ids.insert(plane_id, current_asset_index.clone());
                            assets_counts.push((current_asset_index.clone(), 0));
                            assets_amounts.push(HashMap::new());
                            asset_count += 1;
                        }

                        let asset_utxo_amounts = &mut assets_amounts[current_asset_index.0];
                        asset_utxo_amounts.insert(current_utxo_index.clone(), asset.1.clone());

                        asset_to_policy.insert(current_asset_index.clone(), current_policy_index.clone());
                        if let Some(assets_set) = policy_to_asset.get_mut(&current_policy_index) {
                            assets_set.insert(current_asset_index.clone());
                        } else {
                            let mut assets_set = HashSet::new();
                            assets_set.insert(current_asset_index.clone());
                            policy_to_asset.insert(current_policy_index.clone(), assets_set);
                        }

                        if let Some(utxo_set) = free_asset_to_utxos.get_mut(&current_asset_index) {
                            utxo_set.insert(current_utxo_index.clone());
                        } else {
                            let mut utxo_set = HashSet::new();
                            utxo_set.insert(current_utxo_index.clone());
                            free_asset_to_utxos.insert(current_asset_index.clone(), utxo_set);
                        }

                        if let Some(assets_set) = free_utxo_to_assets.get_mut(&current_utxo_index) {
                            assets_set.insert(current_asset_index.clone());
                        } else {
                            let mut assets_set = HashSet::new();
                            assets_set.insert(current_asset_index.clone());
                            free_utxo_to_assets.insert(current_utxo_index.clone(), assets_set);
                        }
                    }
                }
            } else {
                free_ada_utxos.push((current_utxo_index.clone(), utxo.output.amount.coin.clone()));
            }
            current_utxo_num += 1;
        }

        let utxos_stat = UtxosStat::new(&total_ada, &policy_to_asset, &assets_amounts)?;
        let assets_calculator = AssetsCalculator::new(utxos_stat, assets_name_sizes);
        let inputs_sizes = Self::get_inputs_sizes(&utxos);

        assets_counts.sort_by(|a, b| b.1.cmp(&a.1));
        free_ada_utxos.sort_by(|a, b| a.1.cmp(&b.1));
        utxos_with_ada_overhead.sort_by(|a, b| a.1.cmp(&b.1));
        let output_size = CborCalculator::get_output_size(address);

        Ok(Self {
            address: address.clone(),
            config: config.clone(),
            addresses,
            assets,
            policies,
            assets_calculator,
            assets_amounts,
            assets_counts,
            utxos_ada,
            free_utxo_to_assets,
            free_asset_to_utxos,
            asset_to_policy,
            policy_to_asset,
            inputs_sizes,
            free_ada_utxos,
            //utxos_with_ada_overhead,
            output_size,
        })
    }

    pub(crate) fn has_assets(&self) -> bool {
        !self.free_asset_to_utxos.is_empty()
    }

    pub(crate) fn has_ada(&self) -> bool {
        !self.free_ada_utxos.is_empty()
    }

    pub(crate) fn build_value(&self, used_utxos: &HashSet<UtxoIndex>, tx_output_proposal: &TxOutputProposal)
                              -> Result<Value, JsError> {
        let mut value = Value::new(&tx_output_proposal.total_ada);
        if tx_output_proposal.used_assets.is_empty() {
            return Ok(value);
        }
        let mut multiasset = MultiAsset::new();
        for (policy_index, assets) in &tx_output_proposal.grouped_assets {
            for asset_index in assets {
                let mut asset_coins = Coin::zero();
                for utxo in used_utxos {
                    if let Some(coins) = self.assets_amounts[asset_index.0].get(utxo) {
                        asset_coins = asset_coins.checked_add(coins)?;
                    }
                }
                multiasset.set_asset(&self.policies[policy_index.0], &self.assets[asset_index.0].1, asset_coins);
            }
        }

        value.set_multiasset(&multiasset);
        Ok(value)
    }

    pub(crate) fn try_append_next_utxos(&mut self, tx_proposal: &TxProposal) -> Result<Option<TxProposal>, JsError> {
        let mut proposal_changes = None;
        if self.has_assets() {
            proposal_changes = self.try_append_next_asset_utxos(tx_proposal)?;
        } else if self.has_ada() {
            proposal_changes = self.try_append_pure_ada_utxo(tx_proposal)?;
        }

        if let Some(proposal_changes) = proposal_changes {

            for utxo in proposal_changes.asset_utxo {
                self.remove_assets_utxo(&utxo);
            }

            for utxo in proposal_changes.ada_utxos {
                self.remove_pure_ada_utxo(&utxo);
            }

            Ok(Some(proposal_changes.tx_proposal))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn try_append_next_asset_utxos(&self, tx_proposal: &TxProposal) -> Result<Option<TxProposalChanges>, JsError> {
        let asset_intersections = self.get_asset_intersections(&tx_proposal.used_assets);
        let asset_intersected_utxo = self.make_candidate(&asset_intersections, tx_proposal, false)?;
        if let Some(res_utxo) = asset_intersected_utxo {
            return Ok(Some(res_utxo));
        }

        let policy_intersections = self.get_policy_intersections(&tx_proposal.used_assets);
        let policy_intersected_utxo = self.make_candidate(&policy_intersections, tx_proposal, false)?;
        if let Some(res_utxo) = policy_intersected_utxo {
            return Ok(Some(res_utxo));
        }

        self.make_candidate(&self.assets_counts, tx_proposal, true)
    }

    fn try_append_pure_ada_utxo(&self, tx_proposal: &TxProposal) -> Result<Option<TxProposalChanges>, JsError> {
        let mut new_proposal: TxProposal = tx_proposal.clone();
        let mut used_utxos = HashSet::new();
        if new_proposal.get_need_ada()? == Coin::zero() {
            if let Some((utxo, coin)) = &self.get_next_pure_ada_utxo() {
                if new_proposal.get_outputs().is_empty() {
                    new_proposal.add_new_output(&self.address);
                }
                new_proposal.add_utxo(utxo, coin, &self.addresses[utxo.0])?;
                used_utxos.insert(utxo.clone());
            } else {
                return Ok(None);
            }
        }

        let mut new_size = self.set_min_ada_for_tx(&mut new_proposal)?;

        if new_proposal.get_need_ada()? > Coin::zero() {
            let next_utxos = self.get_next_pure_ada_utxo_by_amount(
                &new_proposal.get_need_ada()?,
                &used_utxos)?;

            for (utxo, coin) in &next_utxos {
                new_proposal.add_utxo(utxo, coin, &self.addresses[utxo.0])?;
                used_utxos.insert(utxo.clone());
            }

            new_size = self.set_min_ada_for_tx(&mut new_proposal)?;
            if new_size > (self.config.max_tx_size as usize) && tx_proposal.used_utoxs.is_empty() {
                return Err(JsError::from_str("Utxo can not be places into tx, utxo value is too big."));
            }
        }

        if new_size > (self.config.max_tx_size as usize) {
            return Ok(None);
        }

        let mut changes = TxProposalChanges::new(new_proposal, false);
        changes.ada_utxos = used_utxos.into_iter().collect();

        Ok(Some(changes))
    }

    fn get_inputs_sizes(utoxs: &TransactionUnspentOutputs) -> Vec<usize> {
        let mut sizes = Vec::with_capacity(utoxs.0.len());
        for utxo in &utoxs.0 {
            let len = utxo.input.to_bytes().len();
            sizes.push(len);
        }
        sizes
    }

    fn get_asset_intersections(&self, used_assets: &HashSet<AssetIndex>) -> Vec<(AssetIndex, usize)> {
        let mut intersections = Vec::new();
        for (index, asset_count) in &self.assets_counts {
            if used_assets.contains(index) && self.free_asset_to_utxos.contains_key(index) {
                intersections.push((index.clone(), asset_count.clone()));
            }
        }
        intersections
    }

    fn get_policy_intersections(&self, used_assets: &HashSet<AssetIndex>) -> Vec<(AssetIndex, usize)> {
        let mut intersections = Vec::new();
        let used_policies = used_assets.iter()
            .filter_map(|x| self.asset_to_policy.get(x));
        let available_assets: HashSet<AssetIndex> = used_policies
            .filter_map(|x| self.policy_to_asset.get(x))
            .flatten()
            .cloned()
            .collect();
        for (index, asset_count) in &self.assets_counts {
            if available_assets.contains(index) && self.free_asset_to_utxos.contains_key(index) {
                intersections.push((index.clone(), asset_count.clone()));
            }
        }
        intersections
    }

    fn prototype_append(&self,
                        tx_proposal: &TxProposal,
                        utxo: &UtxoIndex) -> Result<Option<TxProposalChanges>, JsError> {
        let utxo_assets = self.free_utxo_to_assets.get(utxo);
        let mut new_proposal = tx_proposal.clone();
        let used_assets_in_output = match new_proposal.tx_output_proposals.last() {
            Some(output) => output.used_assets.clone(),
            None => HashSet::new(),
        };

        let used_assets = new_proposal.get_used_assets();
        if let Some(utxo_assets) = utxo_assets {
            let output_intersection = &used_assets_in_output & utxo_assets;
            let rest_assets = utxo_assets - &output_intersection;
            let asset_for_old_ouputs = &rest_assets & used_assets;
            let asset_for_add = &(utxo_assets - &output_intersection) - &asset_for_old_ouputs;

            if new_proposal.get_outputs().is_empty() {
                new_proposal.add_new_output(&self.address);
            }

            let mut assets_for_next_output = asset_for_add;
            let mut create_new_output = false;
            while let Some(next_assets) = self.add_assets_to_proposal_output(
                create_new_output, &mut new_proposal, &assets_for_next_output)? {
                assets_for_next_output = next_assets;
                create_new_output = true;
            }

            new_proposal.add_utxo(utxo, &self.utxos_ada[utxo.0], &self.addresses[utxo.0])?;
            let new_size = self.set_min_ada_for_tx(&mut new_proposal)?;

            if new_size > (self.config.max_tx_size as usize) {
                if tx_proposal.used_utoxs.is_empty() {
                    return Err(JsError::from_str(
                        &format!("Utxo can not be places into tx, utxo value is too big. Utxo index {}", utxo.0)));
                }

                //that means that we above limit of tx size and we cannot create that tx
                return Ok(None);
            }

            if new_proposal.get_need_ada()? > Coin::zero() {
                if let Some(mut proposal_with_ada) = self.try_append_pure_ada_utxo(&new_proposal)? {
                    proposal_with_ada.makes_new_outputs = create_new_output;
                    proposal_with_ada.asset_utxo.push(utxo.clone());
                    return Ok(Some(proposal_with_ada));
                } else {
                    return Ok(None);
                }
            }

            let mut changes = TxProposalChanges::new(new_proposal, create_new_output);
            changes.asset_utxo.push(utxo.clone());
            return Ok(Some(changes));
        }

        Ok(None)
    }

    fn add_assets_to_proposal_output(&self, create_new: bool, tx_proposal: &mut TxProposal, assets: &HashSet<AssetIndex>)
                                     -> Result<Option<HashSet<AssetIndex>>, JsError> {
        let last_output = tx_proposal.get_outputs().last();
        let mut old_value_state = if create_new || last_output.is_none() {
            self.assets_calculator.build_empty_intermediate_value()
        } else {
            self.assets_calculator.build_intermediate_value(
                last_output.unwrap().get_used_assets(),
                &self.asset_to_policy)
        };

        let mut new_value_state = old_value_state.clone();

        let mut asset_to_output = HashSet::new();
        let mut asset_to_new_output = HashSet::new();

        for asset in assets {
            let new_size = self.assets_calculator.add_asset_to_intermediate_value(
                &mut new_value_state,
                asset,
                &self.asset_to_policy[asset]);
            if new_size <= self.config.max_value_size as usize {
                asset_to_output.insert(asset.clone());
                old_value_state = new_value_state.clone();
            } else {
                if old_value_state.is_empty() {
                    return Err(JsError::from_str(
                        &format!("Asset can not be places into tx, asset size is too big. Asset index {}", asset.0)));
                }
                new_value_state = old_value_state.clone();
                asset_to_new_output.insert(asset.clone());
            }
        }

        if create_new {
            tx_proposal.add_new_output(&self.address);
        }

        for asset in &asset_to_output {
            tx_proposal.add_asset(asset, &self.asset_to_policy[asset]);
        }

        if asset_to_new_output.is_empty() {
            Ok(None)
        } else {
            Ok(Some(asset_to_new_output))
        }
    }

    pub(crate) fn set_min_ada_for_tx(&self, tx_proposal: &mut TxProposal) -> Result<usize, JsError> {
        self.recalculate_outputs(tx_proposal)?;
        let (tx_fee, tx_size) = self.estimate_fee(tx_proposal)?;
        tx_proposal.set_fee(&tx_fee);

        Ok(tx_size)
    }

    fn recalculate_outputs(&self, tx_proposal: &mut TxProposal) -> Result<(), JsError> {
        let used_utxos = &tx_proposal.used_utoxs;
        for output in tx_proposal.tx_output_proposals.iter_mut() {
            let (min_output_ada, output_size) = self.estimate_output_cost(&used_utxos, output)?;
            output.set_min_ada(&min_output_ada);
            output.set_size(output_size);
            if output.get_total_ada() < min_output_ada {
                output.set_total_ada(&min_output_ada);
            }
        }
        Ok(())
    }

    pub(super) fn get_tx_proposal_size(&self, tx_proposal: &TxProposal, with_fee: bool) -> usize {
        let mut size = CborCalculator::get_bare_tx_size(false);
        size += CborCalculator::get_bare_tx_body_size(&tx_proposal.used_body_fields);
        size += tx_proposal.witnesses_calculator.get_full_size();
        if !tx_proposal.get_outputs().is_empty() {
            size += CborCalculator::get_struct_size(tx_proposal.get_outputs().len() as u64);
            for output in tx_proposal.get_outputs() {
                size += output.size;
            }
        }

        if with_fee {
            size += CborCalculator::get_coin_size(tx_proposal.get_fee());
        }

        //input list size
        size += CborCalculator::get_struct_size(tx_proposal.used_utoxs.len() as u64);
        for utxo in &tx_proposal.used_utoxs {
            size += self.inputs_sizes[utxo.0];
        }
        size
    }

    fn get_next_pure_ada_utxo(&self) -> Option<&(UtxoIndex, Coin)> {
        self.free_ada_utxos.last()
    }

    fn get_next_pure_ada_utxo_by_amount(&self, need_ada: &Coin, ignore_list: &HashSet<UtxoIndex>)
                                        -> Result<Vec<(UtxoIndex, Coin)>, JsError> {
        //TODO: add algo with minimal count of utxos
        let mut ada_left = need_ada.clone();
        let mut utxos = Vec::new();
        for (utxo, utxo_ada) in self.free_ada_utxos.iter().rev() {
            if ignore_list.contains(&utxo) {
                continue;
            }
            ada_left = ada_left.checked_sub(utxo_ada).unwrap_or(Coin::zero());
            utxos.push((utxo.clone(), utxo_ada.clone()));

            if ada_left.is_zero() {
                break;
            }
        }

        if ada_left.is_zero() {
            Ok(utxos)
        } else {
            Err(JsError::from_str("Not enough funds"))
        }
    }

    fn make_candidate(&self, assets: &Vec<(AssetIndex, usize)>, tx_propoasl: &TxProposal, choose_first: bool)
                      -> Result<Option<TxProposalChanges>, JsError> {
        let mut txp_with_new_output: Option<TxProposalChanges> = None;
        for (index, _) in assets.iter() {
            let utxos_set = self.free_asset_to_utxos.get(index);
            if let Some(utxos) = utxos_set {
                for utxo in utxos {
                    if let Some(new_txp) = self.prototype_append(tx_propoasl, utxo)? {
                        if new_txp.makes_new_outputs {
                            if choose_first {
                                return Ok(Some(new_txp));
                            } else {
                                txp_with_new_output = Some(new_txp);
                            }
                        } else {
                            return Ok(Some(new_txp));
                        }
                    }
                }
            }
        }

        Ok(txp_with_new_output)
    }

    fn estimate_output_cost(&self, used_utoxs: &HashSet<UtxoIndex>, output_proposal: &TxOutputProposal) -> Result<(Coin, usize), JsError> {
        let assets_size = self.assets_calculator.calc_value_size(
            &output_proposal.total_ada,
            &output_proposal.grouped_assets,
            used_utoxs,
            &self.assets_amounts)?;
        let mut output_size = self.output_size + assets_size;
        output_size += CborCalculator::get_value_struct_size(output_proposal.contains_only_ada());
        CborCalculator::estimate_output_cost(
            &output_proposal.get_total_ada(),
            output_size,
            &self.config.data_cost)
    }

    pub(crate) fn estimate_fee(&self, tx_proposal: &TxProposal) -> Result<(Coin, usize), JsError> {
        let mut tx_len = self.get_tx_proposal_size(tx_proposal, false);
        let mut dependable_value = None;
        let mut min_value = None;
        if let Some(last_output) = tx_proposal.get_outputs().last() {
            dependable_value = Some(tx_proposal.get_unused_ada()?
                .checked_add(&last_output.get_total_ada())?);
            min_value = Some(last_output.get_min_ada());
            tx_len -= CborCalculator::get_coin_size(&last_output.get_total_ada());
        }
        CborCalculator::estimate_fee(
            tx_len,
            min_value,
            dependable_value,
            &self.config.fee_algo)
    }

    fn remove_assets_utxo(&mut self, utxo: &UtxoIndex) {
        if let Some(assets) = self.free_utxo_to_assets.get(utxo) {
            for asset in assets {
                if let Some(utxos) = self.free_asset_to_utxos.get_mut(asset) {
                    utxos.remove(utxo);
                    if utxos.is_empty() {
                        self.free_asset_to_utxos.remove(asset);
                    }
                }
            }
            self.free_utxo_to_assets.remove(utxo);
        }
    }

    fn remove_pure_ada_utxo(&mut self, utxo: &UtxoIndex) {
        let index = self.free_ada_utxos.iter().rev().position(|x| x.0 == *utxo);
        if let Some(mut index) = index {
            index = self.free_ada_utxos.len() - index - 1;
            self.free_ada_utxos.remove(index);
        }
    }

    fn calc_utxo_output_overhead(address: &Address, value: &Value, cfg: &TransactionBuilderConfig)
        -> Result<Coin, JsError> {
        let ada = value.coin;
        let output = TransactionOutput::new(address, value);
        let req_coin = MinOutputAdaCalculator::calc_required_coin(&output, &cfg.data_cost)?;
        Ok(ada.checked_sub(&req_coin).unwrap_or(Coin::zero()))
    }
}

