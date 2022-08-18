use std::collections::HashMap;
use super::indexes::{AssetGroups, AssetIndex, PlaneAssetId, UtxoIndex};
use super::*;

pub struct TransactionBatchList(Vec<TransactionBatch>);

pub struct TransactionBatch {
    transactions: Vec<TransactionBuilder>
}

struct TxBatchBuilder {
    asset_groups: AssetGroups,
    config: TransactionBuilderConfig,
    tx_proposals: Vec<TxProposal>,
}

impl TxBatchBuilder {
    pub fn new(utxos: &TransactionUnspentOutputs, address: &Address, config: TransactionBuilderConfig) -> Result<Self, JsError> {
        let asset_groups = AssetGroups::new(utxos, address)?;
        Ok(Self {
            asset_groups,
            config,
            tx_proposals: Vec::new(),
        })
    }

    pub fn build(&self) -> Result<TransactionBatchList, JsError> {
        let mut current_tx_proposal = TxProposal::new(&self.config)?;
    }
}

fn create_send_all(utxos: &TransactionUnspentOutputs, config: &TransactionBuilderConfig) -> TransactionBatchList {
    unimplemented!()
}

