use super::*;

pub struct TransactionBatchList(Vec<TransactionBatch>);

pub struct TransactionBatch {
    transactions: Vec<Transaction>
}

struct PlaneMultiAssetId(PolicyID, AssetName);
struct AssetGroup(Vec(MultiAsset));

fn create_send_all(utxos: &TransactionUnspentOutputs, config: &TransactionBuilderConfig) -> TransactionBatchList {
    unimplemented!()
}