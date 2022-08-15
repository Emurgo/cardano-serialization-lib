use std::collections::HashMap;
use crate::tx_builder::batch_tools::assets_groups::{AssetGroups, AssetIndex, PlaneAssetId, UtxoIndex};
use super::*;

pub struct TransactionBatchList(Vec<TransactionBatch>);

pub struct TransactionBatch {
    transactions: Vec<TransactionBuilder>
}

struct TxOutputProposal {
    used_assets: HashSet<AssetIndex>,
    address: Address,
    min_ada: Coin,
}

impl TxOutputProposal {

    fn new(address: &Address) -> Self {
        TxOutputProposal {
            used_assets: HashSet::new(),
            address: address.clone(),
            min_ada: Coin::zero()
        }
    }

    fn add_assets(&mut self, assets: &HashSet<AssetIndex>) {
        self.used_assets.extend(assets);
    }

    fn create_output(&self, asset_groups: &AssetGroups) -> TransactionOutput {
        return TransactionOutput::new(&self.address, &self.current_value);
    }

}

struct TxProposal {
    tx_output_proposals: Vec<TxOutputProposal>,
    used_utoxs: Vec<UtxoIndex>,
    used_assets: HashSet<AssetIndex>,
    total_ada: Coin,
    need_ada: Coin,
}

impl TxProposal {
    fn new() -> Self {
        Self {
            tx_output_proposals: Vec::new(),
            used_utoxs: Vec::new(),
            used_assets: HashSet::new()
        }
    }

    fn prepare_builder(config: &TransactionBuilderConfig) -> TransactionBuilder {
        let mut tx_builder = TransactionBuilder::new(config);
        tx_builder
    }
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

