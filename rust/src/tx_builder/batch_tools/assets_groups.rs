use std::collections::HashMap;
use super::super::*;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct UtxoIndex(usize);

// impl From<UtxoIndex> for usize {
//     fn from(value: UtxoIndex) -> Self {
//         value.0
//     }
// }

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct AssetIndex(usize);

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PolicyIndex(usize);

#[derive(PartialEq, Eq, Clone)]
pub struct PlaneAssetId(PolicyIndex, AssetName);

#[derive(PartialEq, Eq, Hash, Clone)]
struct AssetSizeCost {
    full_size: usize,
    without_policy_size: usize
}

struct UtxoSizeCost {
    input_size: usize,
}

struct UtxosStat {
    assets_in_policy: HashMap<PolicyID, usize>,
    coins_in_assets: HashMap<PlaneAssetId, Coin>,
    ada_coins: Coin,
}

pub struct AssetGroups {
    assets: Vec<PlaneAssetId>,
    policies: Vec<PolicyID>,
    assets_sizes: Vec<AssetSizeCost>,
    utxos: TransactionUnspentOutputs,
    assets_amounts: HashMap<(AssetIndex, UtxoIndex), Coin>,
    assets_counts: Vec<(AssetIndex, usize)>,
    free_utxo_to_assets: HashMap<UtxoIndex, HashSet<AssetIndex>>,
    free_asset_to_utxos: HashMap<AssetIndex, HashSet<UtxoIndex>>,
    asset_to_policy: HashMap<AssetIndex, PolicyIndex>,
    policy_to_asset: HashMap<PolicyIndex, HashSet<AssetIndex>>,
    inputs: HashMap<UtxoIndex, (TransactionInput, usize)>
}

impl AssetGroups {
    fn new(utxos: &TransactionUnspentOutputs) -> Self {

    }

    fn get_asset_intersections(&self,
                               used_assets: &HashSet<AssetIndex>,
                               used_assets_in_output: &HashSet<AssetIndex>) -> Vec<(AssetIndex, usize)> {
        let mut intersections = Vec::new();
        for (index, asset_count) in &self.assets_counts {
            if used_assets.contains(index) && self.free_asset_to_utxos.contains_key(index){
                intersections.push((index.clone(), asset_count.clone()));
            }
        }
        intersections
    }

    fn get_policy_intersections(&self, used_assets: &HashSet<AssetIndex>) -> Vec<(AssetIndex, usize)> {
        let mut intersections = Vec::new();
        let used_policies= used_assets.iter()
            .filter_map(|x| self.asset_to_policy.get(x));
        let available_assets: HashSet<AssetIndex> = used_policies
            .filter_map(|x| self.policy_to_asset.get(x))
            .flatten()
            .cloned()
            .collect();
        for (index, asset_count) in &self.assets_counts {
            if available_assets.contains(index) && self.free_asset_to_utxos.contains_key(index){
                intersections.push((index.clone(), asset_count.clone()));
            }
        }
        intersections
    }

    fn if_add_to_output_possible(&self,
                                 used_assets: &HashSet<AssetIndex>,
                                 used_assets_in_output: &HashSet<AssetIndex>,
                                 utxo: &UtxoIndex,
                                 value_free_space: &usize, tx_free_space: &usize) -> bool {
        true
    }

    fn if_add_to_tx_possible(&self,
                             used_assets: &HashSet<AssetIndex>,
                             utxo: &UtxoIndex, tx_free_space: &usize) -> bool {
        true
    }

    fn choose_candidate(&self, assets: &Vec<(AssetIndex, usize)>, value_free_space: &usize, tx_free_space: &usize) -> (Option<UtxoIndex>, Option<UtxoIndex>) {
        let mut output_utxo: Option<UtxoIndex> = None;
        let mut tx_utxo:  Option<UtxoIndex> = None;
        for (index, asset_count) in assets.iter() {
            let utxos_set = self.free_asset_to_utxos.get(index);
            if let Some(utxos) = utxos_set {
                for utxo in utxos {
                    if self.if_add_to_output_possible(utxo, value_free_space, tx_free_space) {
                        output_utxo = Some(utxo.clone());
                    } else if self.if_add_to_tx_possible(utxo, tx_free_space) {
                        tx_utxo = Some(utxo.clone());
                    }
                }
            }

        }

        (output_utxo, tx_utxo)
    }

    fn get_next_utxo_index(&mut self, used_assets: &HashSet<AssetIndex>,
                           used_assets_in_output: &HashSet<AssetIndex>,
                           value_free_space: &usize, tx_free_space: &usize) -> Option<UtxoIndex> {
        let mut tx_utxo: Option<UtxoIndex> = None;
        let asset_intersections = self.get_asset_intersections(used_assets);
        let (output_utxo, tx_utxo_tmp) =
            self.choose_candidate(&asset_intersections, value_free_space, tx_free_space);
        if let Some(res_utxo) = &output_utxo {
            Some(res_utxo.clone())
        }
        tx_utxo = tx_utxo_tmp;

        let policy_intersections = self.get_policy_intersections(used_assets);
        let (output_utxo, tx_utxo_tmp) =
            self.choose_candidate(&policy_intersections, value_free_space, tx_free_space);
        if let Some(res_utxo) = &output_utxo {
            Some(res_utxo.clone())
        }
        if tx_utxo.is_none() {
            tx_utxo = tx_utxo_tmp.clone();
        }

        let (output_utxo, tx_utxo_tmp) =
            self.choose_candidate(&self.assets_counts, value_free_space, tx_free_space);

        if let Some(res_utxo) = &output_utxo {
            Some(res_utxo.clone())
        }
        if tx_utxo.is_none() {
            return  tx_utxo_tmp.clone();
        }

        tx_utxo
    }

    fn get_assets_indexes(&self, utxo_index: &UtxoIndex) -> HashSet<AssetIndex>{
        match &self.free_utxo_to_assets.get(utxo_index) {
            Some(&set) => set.clone(),
            None => HashSet::new()
        }
    }

    fn get_asset_size(&self, asset_index: &AssetIndex) -> Result<AssetSizeCost, JsError> {
        match &self.assets_sizes.get(asset_index.0) {
            Some(&size) => Ok(size.clone()),
            None => Err(JsError::from_str(&"Wrong index for asset sizes. Invalid AssetGroups state."))
        }
    }

    fn get_output_size_without_assets(max_coins: &Coin, address: &Address) -> usize {
        //TODO: create static calculation
        let value = Value::new(max_coins);
        //TODO: add asset subtraction
        let fake_asset = MultiAsset::new();
        let output = TransactionOutput::new(address, &value);

        output.to_bytes().len()
    }
}

