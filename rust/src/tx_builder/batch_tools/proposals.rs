use super::super::*;
use super::indexes::{UtxoIndex, AssetIndex};
use super::assets_groups::AssetGroups;
use super::assets_calculator::{IntermediateValueState};

pub(super) struct TxOutputProposal {
    used_assets: HashSet<AssetIndex>,
    address: Address,
    min_ada: Coin,
}

impl TxOutputProposal {

    pub(super) fn new(address: &Address) -> Self {
        TxOutputProposal {
            used_assets: HashSet::new(),
            address: address.clone(),
            min_ada: Coin::zero()
        }
    }

    pub(super) fn add_assets(self, assets: &HashSet<AssetIndex>) {
        self.used_assets.extend(assets);
    }

    fn create_output(&self, asset_groups: &AssetGroups) -> TransactionOutput {
        return TransactionOutput::new(&self.address, &self.current_value);
    }

}

pub(super) struct TxProposal {
    pub(super)  tx_output_proposals: Vec<TxOutputProposal>,
    pub(super)  used_utoxs: Vec<UtxoIndex>,
    pub(super)  used_assets: HashSet<AssetIndex>,
    pub(super)  total_ada: Coin,
    pub(super)  min_outputs_ada: Coin,
    pub(super)  fee: Coin,
}

impl TxProposal {
    pub(super) fn new() -> Self {
        Self {
            tx_output_proposals: Vec::new(),
            used_utoxs: Vec::new(),
            used_assets: HashSet::new(),
            total_ada: Coin::zero(),
            min_outputs_ada: Coin::zero(),
            fee: Coin::zero(),
        }
    }

    fn prepare_builder(config: &TransactionBuilderConfig) -> TransactionBuilder {
        let mut tx_builder = TransactionBuilder::new(config);
        tx_builder
    }
}