use std::collections::HashMap;
use js_sys::new;
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
    assets_in_policy: HashMap<PolicyIndex, usize>,
    coins_in_assets: HashMap<AssetIndex, Coin>,
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
    inputs_sizes: Vec<usize>,
    utxos_stat: UtxosStat
}

impl AssetGroups {
   pub fn new(utxos: &TransactionUnspentOutputs) -> Self {
        let mut assets: Vec<PlaneAssetId> = Vec::new();
        let mut policies: Vec<PolicyID> = Vec::new();
        let mut assets_sizes: Vec<AssetSizeCost> = Vec::new();
        let mut assets_amounts: HashMap<(AssetIndex, UtxoIndex), Coin> = HashMap::new();
        //let mut assets_counts: Vec<(AssetIndex, usize)> = Vec::new();
        let mut free_utxo_to_assets: HashMap<UtxoIndex, HashSet<AssetIndex>> = HashMap::new();
        let mut free_asset_to_utxos: HashMap<AssetIndex, HashSet<UtxoIndex>> = HashMap::new();
        let mut asset_to_policy: HashMap<AssetIndex, PolicyIndex> = HashMap::new();
        let mut policy_to_asset: HashMap<PolicyIndex, HashSet<AssetIndex>> = HashMap::new();

        let mut asset_ids: HashMap<PlaneAssetId, AssetIndex> = HashMap::new();
        let mut policy_ids: HashMap<PolicyID, PolicyIndex> = HashMap::new();
        let mut assets_counts: HashMap<AssetIndex, usize> = HashMap::new();

        let mut utxo_stat = UtxosStat {
            assets_in_policy: HashMap::new(),
            coins_in_assets: HashMap::new(),
            ada_coins: Coin::zero()
        };

        let mut current_utxo_num = 0usize;
        let mut asset_count = 0usize;
        let mut policy_count = 0usize;

       let input_sizes = Self::get_inputs_sizes(&utxos);

        for utxo in &utxos.0 {
            //TODO add input calc
            let current_utxo_index= UtxoIndex(current_utxo_num.clone());
                if let Some(assests) = &utxo.output.amount.multiasset {
                for policy in &assests.0 {
                    let mut current_policy_index = PolicyIndex(policy_count.clone());
                    if Some(policy_index) = policy_ids.get(policy.0) {
                        current_policy_index = policy_index.clone()
                    } else {
                        policy_ids.insert(policy.0.clone(), current_policy_index.clone());
                        policy_count += 1;
                    }

                    for asset in &policy.1.0 {
                        let mut current_asset_index = AssetIndex(asset_count.clone());
                        let plane_id = PlaneAssetId(current_policy_index.clone(), asset.0.clone());
                        if Some(asset_index) = asset_ids.get(&plane_id) {
                            current_asset_index = asset_index.clone();
                        } else {
                            asset_ids.insert(plane_id, current_asset_index);
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
                        //TODO: add size calc
                    }
                }
            }
            current_utxo_num += 1;
        }

        AssetGroups(
            assets,
            policies,
            assets_sizes,
            utxos.clone(),
            assets_amounts,
            assets_counts,
            free_utxo_to_assets,
            free_asset_to_utxos,
            asset_to_policy,
            policy_to_asset,
            input_sizes)
    }

    fn get_inputs_sizes(utoxs: &TransactionUnspentOutputs) -> Vec<usize> {
        let mut sizes = Vec::with_capacity(utoxs.0.len());
        for utxo in &utoxs.0 {
            let len = utxo.input.to_bytes().len();
            sizes.push(len);
        }
        sizes
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
        //TODO: add dedup
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
