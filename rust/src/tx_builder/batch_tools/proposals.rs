use super::super::*;
use super::indexes::{UtxoIndex, AssetIndex};
use super::assets_groups::AssetGroups;
use super::assets_calculator::{IntermediateValueState};

pub(crate) struct TxOutputProposal {
    pub(super) used_assets: HashSet<AssetIndex>,
    pub(super) address: Address,
    pub(super) min_ada: Coin,
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

    fn create_output(&self, asset_groups: &AssetGroups, used_utxos: &Vec<UtxoIndex>) -> TransactionOutput {
        return TransactionOutput::new(&self.address, asset_groups.build_value(self.used_assets, used_utxos));
    }

}

pub(crate) struct TxProposal {
    pub(super)  tx_output_proposals: Vec<TxOutputProposal>,
    pub(super)  used_utoxs: Vec<UtxoIndex>,
    pub(super)  used_assets: HashSet<AssetIndex>,
    pub(super)  total_ada: Coin,
    pub(super)  min_outputs_ada: Coin,
    pub(super)  fee: Coin,
}

impl TxProposal {
    pub(crate) fn new() -> Self {
        Self {
            tx_output_proposals: Vec::new(),
            used_utoxs: Vec::new(),
            used_assets: HashSet::new(),
            total_ada: Coin::zero(),
            min_outputs_ada: Coin::zero(),
            fee: Coin::zero(),
        }
    }

    pub(crate) fn create_tx(&self, asset_groups: &AssetGroups, utxos: &TransactionUnspentOutputs) -> Transaction {
        let witnesses = TransactionWitnessSet::new();
        let mut outputs = Vec::new();
        for proposal in &self.tx_output_proposals {
            outputs.push(proposal.create_output(asset_groups, self.used_utoxs));
        }

        let mut inputs = Vec::new();
        for utxo in &self.used_utoxs {
            inputs.push(utxos.0[utxo.0].input.clone());
        }

        let body = TransactionBody::new(
            TransactionInputs(inputs),
            TransactionOutputs(outputs),
            self.fee,
            None,
        );
        let mut tx = Transaction::new(
            body,
            witnesses,
            None
        );

        return tx;
    }
}