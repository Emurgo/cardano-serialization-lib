use std::collections::HashMap;
use super::*;

pub struct TransactionBatchList(Vec<TransactionBatch>);

pub struct TransactionBatch {
    transactions: Vec<Transaction>
}

struct PlaneMultiAssetId(PolicyID, AssetName);
struct AssetGroup(PlaneMultiAssetId, BigNum);
struct AssetGroups(HashMap<AssetGroup, TransactionUnspentOutput>);

struct UtxosStat {
    assets_in_policy: HashMap<PolicyID, usize>,
    coins_in_assets: HashMap<PlaneMultiAssetId, BigNum>,
    ada_coins: BigNum,
}

impl AssetGroup {

    fn len_without_policy_id(&self) -> usize {

    }

    fn len(&self) -> usize {
        MultiAsset::new()
    }
}

fn create_send_all(utxos: &TransactionUnspentOutputs, config: &TransactionBuilderConfig) -> TransactionBatchList {
    unimplemented!()
}

