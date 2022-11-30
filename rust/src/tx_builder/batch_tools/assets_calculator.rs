use std::collections::HashMap;
use super::utxo_stat::UtxosStat;
use super::cbor_calculator::CborCalculator;
use super::indexes::{UtxoIndex, AssetIndex, PolicyIndex};
use crate::utils::*;
use super::super::*;

#[derive(Clone)]
struct IntermediatePolicyState {
    assets: HashSet<AssetIndex>,
    total_size: usize,
}

impl IntermediatePolicyState {
    fn new() -> Self {
        IntermediatePolicyState {
            assets: HashSet::new(),
            total_size: 0,
        }
    }

    pub(super) fn add_asset(&mut self, asset_index: &AssetIndex, size: usize, coin_size: usize) -> usize {
        if !self.assets.contains(asset_index) {
            let mut new_size = self.total_size;
            if self.assets.len() > 0 {
                new_size -= CborCalculator::get_struct_size(self.assets.len() as u64);
            }
            self.assets.insert(asset_index.clone());
            new_size += CborCalculator::get_struct_size(self.assets.len() as u64);
            self.total_size = new_size + size + coin_size;
        }

        self.total_size
    }
}

#[derive(Clone)]
pub(super) struct IntermediateOutputValue {
    multi_asset: HashMap<PolicyIndex, IntermediatePolicyState>,
    total_size: usize,
}

impl IntermediateOutputValue {
    pub(super) fn new() -> Self {
        IntermediateOutputValue {
            multi_asset: HashMap::new(),
            total_size: 0,
        }
    }

    pub(super) fn set_coin(&mut self, coin: &Coin) -> usize{
        self.total_size += CborCalculator::get_coin_size(coin);
        self.total_size
    }

    pub(super) fn add_asset(&mut self, policy_index: &PolicyIndex, asset_index: &AssetIndex,
                            policy_size: usize, asset_size: usize, coin_size: usize) -> usize {
        if self.is_empty() {
            //value with assets and ada is array of 2 elements
            self.total_size += CborCalculator::get_struct_size(2);
        }
        if let Some(assets) = self.multi_asset.get_mut(policy_index) {
            let old_size = self.total_size - assets.total_size;
            self.total_size = old_size + assets.add_asset(asset_index, asset_size, coin_size);
        } else {
            let mut new_size = self.total_size;
            if self.multi_asset.len() > 0 {
                new_size -= CborCalculator::get_struct_size(self.multi_asset.len() as u64);
            }

            let mut policy_state = IntermediatePolicyState::new();
            new_size += policy_state.add_asset(asset_index, asset_size, coin_size);
            self.multi_asset.insert(policy_index.clone(), policy_state);
            new_size += CborCalculator::get_struct_size(self.multi_asset.len() as u64);
            self.total_size = new_size + policy_size;
        }

        self.total_size
    }

    pub(super) fn is_empty(&self) -> bool {
        self.multi_asset.iter()
            .map(|(_, policy)| policy.assets.len()).sum::<usize>() <= 0
    }
}



#[derive(Clone)]
pub(super) struct AssetsCalculator {
    assets_name_sizes: Vec<usize>,
    policy_size: usize,
    utxo_stat: UtxosStat,
}

impl AssetsCalculator {

    pub(super) fn new(utxo_stat: UtxosStat, assets_name_sizes: Vec<usize>) -> Self {
        //28 is the size of a policy id in bytes
        let policy_size= 28 + CborCalculator::get_struct_size(28u64);

        Self {
            assets_name_sizes,
            policy_size,
            utxo_stat,
        }
    }

    pub(super) fn calc_value_size(&self,
                                  coin: &Coin,
                                  grouped_assets: &HashMap<PolicyIndex, HashSet<AssetIndex>>,
                                  utxos: &HashSet<UtxoIndex>,
                                  assets_amounts: &Vec<HashMap<UtxoIndex, Coin>>) -> Result<usize, JsError> {
        let mut size = 0;

        size += CborCalculator::get_coin_size(coin);

        if grouped_assets.len() > 0 {
            size += CborCalculator::get_struct_size(grouped_assets.len() as u64);
        }

        for (_, assets_in_policy) in grouped_assets {
            size += self.policy_size;
            size += CborCalculator::get_struct_size(assets_in_policy.len() as u64);
            for asset_in_policy in assets_in_policy {
                size += self.assets_name_sizes[asset_in_policy.0];
                let mut asset_coins = Coin::zero();
                for (utxo, coins) in &assets_amounts[asset_in_policy.0] {
                    if utxos.contains(utxo) {
                       asset_coins = asset_coins.checked_add(coins)?;
                    }
                }
                size += CborCalculator::get_coin_size(&asset_coins);
            }
        }
        Ok(size)
    }

    pub(super) fn add_asset_to_intermediate_value(&self, intermediate_value: &mut IntermediateOutputValue, asset_index: &AssetIndex,
                                                  policy_index: &PolicyIndex) -> usize {
        intermediate_value.add_asset(policy_index, asset_index,
                                     self.policy_size,
                                     self.assets_name_sizes[asset_index.0],
                                     CborCalculator::get_coin_size(&self.utxo_stat.coins_in_assets[asset_index]))
    }

    pub(super) fn build_intermediate_value(&self, assets_ids: &HashSet<AssetIndex>,
                                           asset_to_policy: &HashMap<AssetIndex, PolicyIndex>) -> IntermediateOutputValue {
        let mut intermediate_data = IntermediateOutputValue::new();
        for asset_index in assets_ids {
            let asset_coin_size = CborCalculator::get_coin_size(&self.utxo_stat.coins_in_assets[asset_index]);
            let policy_index = &asset_to_policy[asset_index];
            intermediate_data.add_asset(policy_index, asset_index,
                                        self.policy_size,
                                         self.assets_name_sizes[asset_index.0],
                                         asset_coin_size);
        }
        intermediate_data.set_coin(&self.utxo_stat.ada_coins);
        intermediate_data
    }

    pub(super) fn build_empty_intermediate_value(&self) -> IntermediateOutputValue {
        let mut value = IntermediateOutputValue::new();
        value.set_coin(&self.utxo_stat.ada_coins);
        value
    }
}