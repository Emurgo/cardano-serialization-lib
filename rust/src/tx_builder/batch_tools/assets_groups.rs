use std::collections::HashMap;
use super::super::*;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct UtxoIndex(usize);

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct AssetIndex(usize);

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PlaneAssetId(PolicyID, AssetName);

#[derive(PartialEq, Eq, Hash, Clone)]
struct AssetSizeCost {
    full_size: usize,
    without_policy_size: usize
}

struct UtxoSizeCost {
    input_size: usize,
}

pub struct AssetGroups {
    assets: Vec<PlaneAssetId>,
    assets_sizes: Vec<AssetSizeCost>,
    utxos: TransactionUnspentOutputs,
    assets_amounts: HashMap<(AssetIndex, UtxoIndex), Coin>,
    assets_counts: Vec<(AssetIndex, usize)>,
    utxo_to_assets: HashMap<UtxoIndex, HashSet<AssetIndex>>,
    asset_to_utxos: HashMap<AssetIndex, HashSet<UtxoIndex>>,
    used_utxo: HashSet<UtxoIndex>,
    used_assets: HashSet<AssetIndex>
}

impl AssetGroups {
    fn new(utxos: &TransactionUnspentOutputs) -> Self {
        unimplemented!()
    }

    fn get_next_utxo_index(&self, used_assets: &HashSet<AssetIndex>,
                           value_free_space: usize, tx_free_space: usize) -> UtxoIndex {
        unimplemented!()
    }

    fn get_assets_indexes(&self, utxo_index: &UtxoIndex) -> HashSet<AssetIndex>{
        unimplemented!()
    }

    fn get_asset_size(&self, asset_index: &AssetIndex) -> Result<AssetSizeCost, JsError> {
        unimplemented!()
    }

    fn get_output_size_without_assets() -> usize {
        //TODO: create static calculation
        unimplemented!()
    }
}

struct UtxosStat {
    assets_in_policy: HashMap<PolicyID, usize>,
    coins_in_assets: HashMap<PlaneAssetId, Coin>,
    ada_coins: Coin,
}