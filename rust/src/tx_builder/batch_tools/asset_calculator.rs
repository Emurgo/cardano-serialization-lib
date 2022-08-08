use std::collections::HashMap;
use crate::tx_builder::batch_tools::assets_groups::{AssetIndex, PolicyIndex, UtxoIndex};
use super::super::*;

pub(super) struct UtxosStat {
    total_policies: usize,
    assets_in_policy: HashMap<PolicyIndex, usize>,
    coins_in_assets: HashMap<AssetIndex, Coin>,
    ada_coins: Coin,
}

impl UtxosStat {
    pub(super) fn new(total_ada: &Coin, policy_to_asset: &HashMap<PolicyIndex, HashSet<AssetIndex>>,
                      amounts: &HashMap<(AssetIndex, UtxoIndex), Coin>) -> Self {
        let mut utxos_stat = UtxosStat {
            total_policies: 0,
            assets_in_policy: HashMap::new(),
            coins_in_assets: HashMap::new(),
            ada_coins: Coin::zero(),
        };
        for (policy_index, assets) in policy_to_asset {
            utxos_stat.assets_in_policy.insert(policy_index.clone(), assets.len());
        }

        for ((asset_index, utxo_index), amount) in amounts {
            if let Some(coins) = utxos_stat.coins_in_assets.get(asset_index) {
                utxos_stat.coins_in_assets.insert(asset_index.clone(), coins.clone() + amount);
            } else {
                utxos_stat.coins_in_assets.insert(asset_index.clone(), amount.clone());
            }
        }

        utxos_stat.total_policies = policy_to_asset.len();
        utxos_stat.ada_coins = total_ada.clone();

        utxos_stat
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub(super) struct AssetCalculator {
    assets_name_sizes: Vec<usize>,
    policies_sizes: Vec<usize>,
    utxo_stat: UtxosStat,
    bare_output_size: usize
}

impl AssetCalculator {

    pub(super) fn new(utxo_stat: UtxosStat, assets_name_sizes: Vec<usize>,
                      policies_sizes: Vec<usize>, address: &Address) -> Self {
        let bare_output_size =
            Self::prepare_output_size_without_assets(&utxo_stat.ada_coins, address);
        Self {
            assets_name_sizes,
            policies_sizes,
            utxo_stat,
            bare_output_size
        }
    }

    // According to the CBOR spec, the maximum size of a inlined CBOR value is 23 bytes.
    // Otherwise, the value is encoded as pair of type and value.
    fn get_struct_size(items_count: u64) -> usize {
        if items_count <= MAX_INLINE_ENCODING {
            return 1;
        } else if items_count < 0x1_00 {
            return 2;
        } else if items_count < 0x1_00_00 {
            return 3;
        } else if items_count < 0x1_00_00_00_00 {
            return 5;
        } else {
            return 9;
        }
    }

    fn get_coin_size(coin: &Coin)-> usize {
        Self::get_struct_size(coin.into())
    }

    pub(super) fn calc_aprox_value_size(&self, assets: &HashSet<AssetIndex>) -> usize {
        let mut size = 0;
        let mut policy_groups = self.get_grouped_assets(assets);
        size += Self::get_struct_size(policy_groups.len().into());
        for (policy_index, assets_in_policy) in policy_groups {
            size += self.policies_sizes.get(policy_index).unwrap().clone();
            size += Self::get_struct_size(assets_in_policy.len().into());
            for asset_in_policy in &assets_in_policy {
                size += self.assets_name_sizes.get(asset_in_policy).unwrap().clone();
                let asset_coins = self.utxo_stat.coins_in_assets.get(asset_in_policy).unwrap();
                size += Self::get_coin_size(asset_coins);
            }
        }
        size
    }

    pub(super) fn calc_value_size(&self, assets: &HashSet<AssetIndex>, utxos: &HashSet<UtxoIndex>,
                       assets_amounts: &HashMap<(AssetIndex, UtxoIndex), Coin>) -> usize {
        let mut size = 0;
        let mut policy_groups = self.get_grouped_assets(assets);
        size += Self::get_struct_size(policy_groups.len().into());
        for (policy_index, assets_in_policy) in policy_groups {
            size += self.policies_sizes.get(policy_index).unwrap().clone();
            size += Self::get_struct_size(assets_in_policy.len().into());
            for asset_in_policy in &assets_in_policy {
                size += self.assets_name_sizes.get(asset_in_policy).unwrap().clone();
                let mut asset_coins = Coin::zero();
                for uxto in utxos {
                    if let Some(coin) = assets_amounts.get(&(asset_in_policy.clone(), uxto.clone())) {
                        asset_coins += coin.clone();
                    }
                }
                size += Self::get_coin_size(&asset_coins);
            }
        }
        size
    }

    fn get_output_size_without_assets(&self) -> usize {
        self.bare_output_size
    }

    fn prepare_output_size_without_assets(max_coins: &Coin, address: &Address) -> usize {
        let value = Value::new(max_coins);
        let output = TransactionOutput::new(address, &value);
        //output size without assets + overhead for storing coins and assents in the same time
        //see cardano cddl for more info
        output.to_bytes().len() + AssetSize::get_struct_size(2)
    }
}