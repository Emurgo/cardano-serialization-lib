use std::collections::HashMap;
use batch_tools::proposals::{TxProposal, TxOutputProposal};
use batch_tools::indexes::{UtxoIndex, AssetIndex};
use batch_tools::assets_groups::{AssetGroups};
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
        let mut current_tx_proposal = TxProposal::new();

    }
}

fn create_send_all(utxos: &TransactionUnspentOutputs, config: &TransactionBuilderConfig) -> TransactionBatchList {
    unimplemented!()
}

