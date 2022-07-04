use std::collections::HashMap;
use js_sys::Set;
use super::*;

pub struct TransactionBatchList(Vec<TransactionBatch>);

pub struct TransactionBatch {
    transactions: Vec<Transaction>
}

struct UtxoIndex(usize);
struct PlaneAssetId(PolicyID, AssetName);
struct AssetGroups(HashMap<PlaneAsset, HashSet<UtxoIndex>>);

struct UtxosStat {
    assets_in_policy: HashMap<PolicyID, usize>,
    coins_in_assets: HashMap<PlaneMultiAssetId, BigNum>,
    ada_coins: BigNum,
}


struct TxOutputProposal {
    used_assests: HashMap<PlaneAssetId, BigNum>,
    used_policies: HashSet<PolicyID>,
    free_space: usize,
    used_space: usize
}

impl TxOutputProposal {

}

struct TxProposal {
    tx_output_proposals: Vec<TxOutputProposal>,
    used_utoxs: Vec<UtxoIndex>,

}

struct TxBatchBuilder {
    utxos: TransactionUnspentOutputs,
    asset_groups: AssetGroups,
    pure_utxos: Vec<UtxoIndex>,
    used_utoxs: HashSet<UtxoIndex>
}

struct PlaneAsset {
    asset_id: PlaneAssetId,
    amount: Coin,
    len_full: usize,
    len_without_policy: usize
}

fn create_send_all(utxos: &TransactionUnspentOutputs, config: &TransactionBuilderConfig) -> TransactionBatchList {
    unimplemented!()
}

