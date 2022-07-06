use std::collections::HashMap;
use js_sys::{add, Set};
use crate::tx_builder::batch_tools::assets_groups::{AssetGroups, PlaneAssetId, UtxoIndex};
use super::*;

pub struct TransactionBatchList(Vec<TransactionBatch>);

pub struct TransactionBatch {
    transactions: Vec<Transaction>
}



struct TxOutputProposal {
    used_assets: HashMap<PlaneAssetId, BigNum>,
    used_policies: HashSet<PolicyID>,
    current_value: Value,
    address: Address,
    free_asset_space: usize,
    used_asset_space: usize,
    total_space: usize
}

impl TxOutputProposal {

    fn new(address: &Address, max_ada: &Coin, max_value_size: usize) -> Result<Self, JsError> {
        let value = Value::new(max_ada);
        let output = TransactionOutput::new(address, &value);
        let total_space = output.to_bytes().len();
        let value_len = value.to_bytes().len();
        let free_len = max_value_size.checked_sub(value_len)?;
        Ok(Self {
            used_assets: HashMap::new(),
            used_policies: HashSet::new(),
            current_value: value,
            address: address.clone(),
            free_asset_space: free_len,
            used_asset_space: value_len,
            total_space,
        })
    }

    fn try_take_next_asset(&mut self, &mut assets: AssetGroups) -> Option<UtxoIndex> {
        unimplemented!()
    }

    fn create_output(&self) -> TransactionOutput {
        return TransactionOutput::new(&self.address, &self.current_value);
    }

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

