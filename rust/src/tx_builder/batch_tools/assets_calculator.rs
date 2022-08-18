use std::collections::HashMap;
use super::utxo_stat::UtxosStat;
use super::cbor_calculator::CborCalculator;
use super::indexes::{UtxoIndex, AssetIndex, PolicyIndex};
use crate::utils::*;
use super::super::*;

#[derive(Clone)]
struct IntermediatePolicyState {
    assets: HashMap<AssetIndex, usize>,
    total_size: usize,
}

impl IntermediatePolicyState {
    fn new() -> Self {
        IntermediatePolicyState {
            assets: HashMap::new(),
            total_size: 0,
        }
    }

    pub(super) fn add_asset(&mut self, asset_index: &AssetIndex, size: usize, coin_size: usize) -> usize {
        if !self.assets.contains_key(asset_index) {
            let old_map_size = CborCalculator::get_struct_size(self.assets.len() as u64);
            let mut new_size = self.total_size - old_map_size;
            self.assets.insert(asset_index.clone(), size);
            new_size += CborCalculator::get_struct_size(self.assets.len() as u64);
            self.total_size = new_size + size + coin_size;
        }

        self.total_size
    }
}

#[derive(Clone)]
pub(super) struct IntermediateValueState {
    multi_asset: HashMap<PolicyIndex, IntermediatePolicyState>,
    coin_size: usize,
    total_size: usize,
}

impl IntermediateValueState {
    pub(super) fn new() -> Self {
        IntermediateValueState {
            multi_asset: HashMap::new(),
            coin_size: 0,
            total_size: 0,
        }
    }

    pub(super) fn set_coin(&mut self, coin: &Coin) -> usize{
        self.total_size += CborCalculator::get_coin_size(coin);
        self.total_size
    }

    pub(super) fn add_asset(&mut self, policy_index: &PolicyIndex, asset_index: &AssetIndex,
                            policy_size: usize, asset_size: usize, coin_size: usize) -> usize {
        if let Some(mut assets) = self.multi_asset.get(policy_index) {
            let old_size = self.total_size - assets.total_size;
            self.total_size = old_size + assets.add_asset(asset_index, asset_size, coin_size);
        } else {
            let old_map_size = CborCalculator::get_struct_size(self.multi_asset.len() as u64);
            let mut new_size = self.total_size - old_map_size;
            let mut policy_state = IntermediatePolicyState::new();
            policy_state.add_asset(asset_index, asset_size, coin_size);
            new_size += policy_state.total_size;
            self.multi_asset.insert(policy_index.clone(), IntermediatePolicyState::new());
            new_size += CborCalculator::get_struct_size(self.multi_asset.len() as u64);
            self.total_size = new_size + policy_size;
        }

        self.total_size
    }
}



#[derive(Clone)]
pub(super) struct AssetsCalculator {
    assets_name_sizes: Vec<usize>,
    policies_sizes: Vec<usize>,
    utxo_stat: UtxosStat,
    bare_output_size: usize,
    min_name_size: usize,
}

impl AssetsCalculator {

    pub(super) fn new(utxo_stat: UtxosStat, assets_name_sizes: Vec<usize>,
                      policies_sizes: Vec<usize>, address: &Address) -> Self {
        let bare_output_size =
            CborCalculator::output_size_without_assets(&utxo_stat.ada_coins, address);
        let min_name_size = match assets_name_sizes.iter().min() {
            Some(min) => *min,
            None => 0,
        };

        Self {
            assets_name_sizes,
            policies_sizes,
            utxo_stat,
            bare_output_size,
            min_name_size
        }
    }

    pub(super) fn calc_aprox_value_size(&self, grouped_assets: &HashMap<PolicyIndex, HashSet<AssetIndex>>)
        -> Result<usize, JsError> {
        let mut size = 0;
        size += CborCalculator::get_struct_size(grouped_assets.len() as u64);
        for (policy_index, assets_in_policy) in grouped_assets {
            size += self.policies_sizes[policy_index.0];
            size += CborCalculator::get_struct_size(assets_in_policy.len() as u64);
            for asset_in_policy in assets_in_policy {
                size += self.assets_name_sizes[asset_in_policy.0];
                let asset_coins = self.utxo_stat.coins_in_assets[asset_in_policy];
                size += CborCalculator::get_coin_size(&asset_coins);
            }
        }
        Ok(size)
    }

    pub(super) fn calc_value_size(&self, grouped_assets: &HashMap<PolicyIndex, HashSet<AssetIndex>>, utxos: &HashSet<UtxoIndex>,
                       assets_amounts: &HashMap<(AssetIndex, UtxoIndex), Coin>) -> Result<usize, JsError> {
        let mut size = 0;
        size += CborCalculator::get_struct_size(grouped_assets.len() as u64);
        for (policy_index, assets_in_policy) in grouped_assets {
            size += self.policies_sizes[policy_index.0];
            size += CborCalculator::get_struct_size(assets_in_policy.len() as u64);
            for asset_in_policy in assets_in_policy {
                size += self.assets_name_sizes[asset_in_policy.0];
                let mut asset_coins = Coin::zero();
                for uxto in utxos {
                    if let Some(coin) = assets_amounts.get(&(asset_in_policy.clone(), uxto.clone())) {
                        asset_coins = asset_coins.checked_add(coin)?;
                    }
                }
                size += CborCalculator::get_coin_size(&asset_coins);
            }
        }
        Ok(size)
    }

    pub(super) fn add_asset(&self, mut intermediate_value: &IntermediateValueState, asset_index: &AssetIndex,
                 policy_index: &PolicyIndex) -> usize {
        intermediate_value.add_asset(policy_index, asset_index,
                                     self.policies_sizes[policy_index.0],
                                     self.assets_name_sizes[asset_index.0],
                                     CborCalculator::get_coin_size(&self.utxo_stat.coins_in_assets[asset_index]))
    }

    pub(super) fn build_intermediate_data(&self, assets_ids: &HashSet<AssetIndex>,
                               asset_to_policy: &HashMap<AssetIndex, PolicyIndex>) -> IntermediateValueState {
        let mut intermediate_data = IntermediateValueState::new();
        for asset_index in assets_ids {
            let asset_coin_size = CborCalculator::get_coin_size(&self.utxo_stat.coins_in_assets[asset_index]);
            let policy_index = &asset_to_policy[asset_index];
            intermediate_data.add_asset(policy_index, asset_index,
                                         self.policies_sizes[policy_index.0],
                                         self.assets_name_sizes[asset_index.0],
                                         asset_coin_size);
        }
        intermediate_data
    }
}