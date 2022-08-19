use std::collections::HashMap;
use crate::tx_builder::batch_tools::proposals::TxProposal;
use super::assets_calculator::AssetsCalculator;
use super::cbor_calculator::CborCalculator;
use super::utxo_stat::UtxosStat;
use super::indexes::{UtxoIndex, AssetIndex, PolicyIndex, PlaneAssetId};
use super::super::*;

pub struct TxProposalChanges {
    tx_proposal: TxProposal,
    makes_new_outputs: bool,
    utxo: UtxoIndex,
}

pub struct AssetGroups {
    assets: Vec<PlaneAssetId>,
    policies: Vec<PolicyID>,
    assets_calculator: AssetsCalculator,
    assets_amounts: HashMap<(AssetIndex, UtxoIndex), Coin>,
    assets_counts: Vec<(AssetIndex, usize)>,

    //assets and utoxs that can be used
    free_utxo_to_assets: HashMap<UtxoIndex, HashSet<AssetIndex>>,
    free_asset_to_utxos: HashMap<AssetIndex, HashSet<UtxoIndex>>,

    asset_to_policy: HashMap<AssetIndex, PolicyIndex>,
    policy_to_asset: HashMap<PolicyIndex, HashSet<AssetIndex>>,
    inputs_sizes: Vec<usize>,

    free_ada_utxos : Vec<(UtxoIndex, Coin)>,
}

impl AssetGroups {
    pub(crate) fn new(utxos: &TransactionUnspentOutputs, address: &Address) -> Result<Self, JsError> {
        let mut assets: Vec<PlaneAssetId> = Vec::new();
        let mut policies: Vec<PolicyID> = Vec::new();
        let mut assets_name_sizes: Vec<usize> = Vec::new();
        let mut policies_sizes: Vec<usize> = Vec::new();
        let mut assets_amounts: HashMap<(AssetIndex, UtxoIndex), Coin> = HashMap::new();
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

        for utxo in &utxos.0 {
            total_ada = total_ada.checked_add(&utxo.output.amount.coin)?;

            let current_utxo_index = UtxoIndex(current_utxo_num.clone());
            if let Some(assests) = &utxo.output.amount.multiasset {
                for policy in &assests.0 {
                    let mut current_policy_index = PolicyIndex(policy_count.clone());
                    if let Some(policy_index) = policy_ids.get(policy.0) {
                        current_policy_index = policy_index.clone()
                    } else {
                        let policy_id_size = CborCalculator::get_struct_size(policy.0.0.len() as u64);
                        policies.push(policy.0.clone());
                        policies_sizes.push(policy_id_size);
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
                            let asset_name_size = CborCalculator::get_struct_size(asset.0.0.len() as u64);
                            assets.push(plane_id.clone());
                            assets_name_sizes.push(asset_name_size);
                            asset_ids.insert(plane_id, current_asset_index.clone());
                            assets_counts.push((current_asset_index.clone(), 0));
                            asset_count += 1;
                        }

                        asset_to_policy.insert(current_asset_index.clone(), current_policy_index.clone());
                        if let Some(mut assets_set) = policy_to_asset.get(&current_policy_index) {
                            assets_set.insert(current_asset_index.clone());
                        } else {
                            let mut assets_set = HashSet::new();
                            assets_set.insert(current_asset_index.clone());
                            policy_to_asset.insert(current_policy_index.clone(), assets_set);
                        }

                        if let Some(mut utxo_set) = free_asset_to_utxos.get(&current_asset_index) {
                            utxo_set.insert(current_utxo_index.clone());
                        } else {
                            let mut utxo_set = HashSet::new();
                            utxo_set.insert(current_utxo_index.clone());
                            free_asset_to_utxos.insert(current_asset_index.clone(), utxo_set);
                        }

                        if let Some(mut assets_set) = free_utxo_to_assets.get(&current_utxo_index) {
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
        let assets_calculator = AssetsCalculator::new(utxos_stat, assets_name_sizes, policies_sizes, address);
        let inputs_sizes = Self::get_inputs_sizes(&utxos);

        assets_counts.sort_by(|a, b| b.1.cmp(&a.1));
        free_ada_utxos.sort_by(|a, b| a.1.cmp(&b.1));

        Ok(Self {
            assets,
            policies,
            assets_calculator,
            assets_amounts,
            assets_counts,
            free_utxo_to_assets,
            free_asset_to_utxos,
            asset_to_policy,
            policy_to_asset,
            inputs_sizes,
            free_ada_utxos,
        })
    }

    pub(crate) fn build_value(assets: &hashset<assetindex>, utxos: &vec<utxoindex>) -> value {
        unimplemented!()
    }

    pub(crate) fn try_append_next_utxos(&mut self, tx_proposal: &TxProposal) -> Option<TxProposal> {
        let mut tx_utxo: Option<TxProposal> = None;
        let asset_intersections = self.get_asset_intersections(tx_proposal.used_assets);
        let (output_utxo, tx_utxo_tmp) =
            self.make_candidate(&asset_intersections, tx_proposal);
        if let Some(res_utxo) = &output_utxo {
            Some(res_utxo.clone())
        }
        tx_utxo = tx_utxo_tmp;

        let policy_intersections = self.get_policy_intersections(tx_proposal.used_assets);
        let (output_utxo, tx_utxo_tmp) =
            self.make_candidate(&policy_intersections, tx_proposal);
        if let Some(res_utxo) = &output_utxo {
            Some(res_utxo.clone())
        }
        if tx_utxo.is_none() {
            tx_utxo = tx_utxo_tmp.clone();
        }
        //TODO: add dedup
        let (output_utxo, tx_utxo_tmp) =
            self.make_candidate(&self.assets_counts, tx_proposal);

        if let Some(res_utxo) = &output_utxo {
            Some(res_utxo.clone())
        }
        if tx_utxo.is_none() {
            return tx_utxo_tmp.clone();
        }

        tx_utxo
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
                        utxo: &UtxoIndex) -> Option<TxProposalChanges> {
        let utxo_assets = self.free_utxo_to_assets.get(utxo);
        let used_assets_in_output = match tx_proposal.tx_output_proposals.last() {
            Some(output) => &output.used_assets,
            None => &HashSet::new(),
        };

        let used_assets = &tx_proposal.used_assets;
        if let Some(utxo_assets) = utxo_assets {
            let output_intersection = used_assets_in_output & utxo_assets;
            let rest_assets = utxo_assets - &output_intersection;
            let asset_for_old_ouputs = &rest_assets & utxo_assets;
            let asset_for_add = &(utxo_assets - &output_intersection) - &asset_for_old_ouputs;

            let mut old_value_state =
                self.assets_calculator.build_intermediate_data(
                    used_assets_in_output,
                    &self.asset_to_policy);

            let mut new_value_state = old_value_state.clone();

            let mut asset_to_output = HashSet::new();
            let mut asset_to_new_output = HashSet::new();

            for asset in &asset_for_add {
                let new_size = self.assets_calculator.add_asset(
                    &new_value_state,
                    asset,
                    &self.asset_to_policy[asset]);
                if new_size <= *value_capacity {
                    asset_to_output.insert(asset.clone());
                    old_value_state = new_value_state.clone();
                } else {
                    new_value_state = old_value_state.clone();
                    asset_to_new_output.insert(asset.clone());
                }
            }

            return Some(AssetsForAppendPrototype {
                assets_for_current_output: asset_for_add,
                assets_for_new_output: asset_to_new_output,
            });
        }
        None
    }

    fn get_next_pure_ada_utxo(&mut self) -> Option<(UtxoIndex, Coin)> {
        self.free_ada_utxos.pop()
    }

    fn get_next_pure_ada_utxo_by_amount(&mut self, need_ada: &Coin) -> Result<Vec<(UtxoIndex, Coin)>, JsError> {
        let mut ada_left = need_ada.clone();
        let mut utxos = Vec::new();
        while let Some((utxo, utxo_ada)) = self.free_ada_utxos.pop() {
            if utxo_ada >= ada_left {
                ada_left = Coin::zero();
            } else {
                ada_left = ada_left.checked_sub(&utxo_ada)?;
            }
            utxos.push((utxo, utxo_ada));
        }
        if ada_left.is_zero() {
            Ok(utxos)
        } else {
            for utxo_for_resotre in utxos.into_iter().rev() {
                self.free_ada_utxos.push(utxo_for_resotre);
            }
            Err(JsError::from_str("Not enough funds"))
        }
    }

    fn get_grouped_assets(&self, assets: &HashSet<AssetIndex>) -> HashMap<PolicyIndex, HashSet<AssetIndex>> {
        let mut grouped_assets = HashMap::new();
        for asset in assets {
            let policy_index = self.asset_to_policy.get(asset).unwrap();
            let assets_set = grouped_assets.entry(policy_index.clone()).or_insert(HashSet::new());
            assets_set.insert(asset.clone());
        }
        grouped_assets
    }

    fn make_candidate(&self, assets: &Vec<(AssetIndex, usize)>, tx_propoasl: &TxProposal)
            -> (Option<TxProposal>, Option<TxProposal>) {
        let mut txp_with_new_output: Option<TxProposalChanges> = None;
        let mut txp: Option<TxProposalChanges> = None;
        for (index, asset_count) in assets.iter() {
            let utxos_set = self.free_asset_to_utxos.get(index);
            if let Some(utxos) = utxos_set {
                for utxo in utxos {
                    if let Some(new_txp) = self.prototype_append(tx_propoasl, utxo) {
                        if new_txp.makes_new_outputs {
                            txp_with_new_output = Some(new_txp);
                        } else {
                            txp = Some(new_txp);
                        }
                    }
                }
            }
        }

        (txp, txp_with_new_output)
    }

    fn remove_utxo(&mut self, utxo: &UtxoIndex) {
        if let Some(mut assets) = self.free_utxo_to_assets.get(utxo) {
            for asset in assets {
                if let Some(mut utxos) = self.free_asset_to_utxos.get_mut(asset) {
                    utxos.remove(utxo);
                    if utxos.is_empty() {
                        self.free_asset_to_utxos.remove(asset);
                    }
                }
            }
            self.free_utxo_to_assets.remove(utxo);
        }
    }

    fn get_assets_indexes(&self, utxo_index: &UtxoIndex) -> HashSet<AssetIndex> {
        match &self.free_utxo_to_assets.get(utxo_index) {
            Some(&set) => set.clone(),
            None => HashSet::new()
        }
    }
}

