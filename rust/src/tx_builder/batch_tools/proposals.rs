use std::collections::HashMap;
use crate::serialization_tools::map_names::TxBodyNames;
use super::super::*;
use super::indexes::{UtxoIndex, AssetIndex, PolicyIndex};
use super::asset_categorizer::AssetCategorizer;
use super::witnesses_calculator::WitnessesCalculator;

#[derive(Clone)]
pub(crate) struct TxOutputProposal {
    pub(super) used_assets: HashSet<AssetIndex>,
    pub(super) grouped_assets: HashMap<PolicyIndex, HashSet<AssetIndex>>,
    pub(super) address: Address,
    pub(super) min_ada: Coin,
    pub(super) total_ada: Coin,
    pub(super) size: usize,
}

impl TxOutputProposal {
    pub(super) fn new(address: &Address) -> Self {
        TxOutputProposal {
            used_assets: HashSet::new(),
            grouped_assets: HashMap::new(),
            address: address.clone(),
            min_ada: Coin::zero(),
            total_ada: Coin::zero(),
            size: 0,
        }
    }

    pub(super) fn add_ada(&mut self, ada_coins: &Coin) -> Result<(), JsError> {
        self.total_ada = self.total_ada.checked_add(ada_coins)?;
        Ok(())
    }

    pub(super) fn add_asset(&mut self, asset: &AssetIndex, policy_index: &PolicyIndex) {
        self.used_assets.insert(asset.clone());
        let policy = self.grouped_assets.entry(policy_index.clone())
            .or_insert(HashSet::new());
        policy.insert(asset.clone());
    }

    pub(super) fn contains_only_ada(&self) -> bool {
        self.used_assets.is_empty()
    }

    pub(super) fn get_used_assets(&self) -> &HashSet<AssetIndex> {
        &self.used_assets
    }

    pub(super) fn get_total_ada(&self) -> Coin {
        self.total_ada
    }

    pub(super) fn set_total_ada(&mut self, ada_coins: &Coin) {
        self.total_ada = ada_coins.clone();
    }

    pub(super) fn get_min_ada(&self) -> Coin {
        self.min_ada
    }

    pub(super) fn set_min_ada(&mut self, min_ada: &Coin) {
        self.min_ada = min_ada.clone();
    }

    pub(super) fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    fn create_output(&self, asset_groups: &AssetCategorizer, used_utxos: &HashSet<UtxoIndex>)
                     -> Result<TransactionOutput, JsError> {
        Ok(TransactionOutput::new(&self.address, &asset_groups.build_value(used_utxos, self)?))
    }
}

#[derive(Clone)]
pub(crate) struct TxProposal {
    pub(super) used_body_fields: HashSet<TxBodyNames>,
    pub(super) tx_output_proposals: Vec<TxOutputProposal>,
    pub(super) used_utoxs: HashSet<UtxoIndex>,
    pub(super) used_assets: HashSet<AssetIndex>,
    pub(super) total_ada: Coin,
    pub(super) fee: Coin,
    pub(super) witnesses_calculator: WitnessesCalculator,
}

impl TxProposal {
    pub(crate) fn new() -> Self {
        let mut body_fields = HashSet::new();
        body_fields.insert(TxBodyNames::Inputs);
        body_fields.insert(TxBodyNames::Outputs);
        body_fields.insert(TxBodyNames::Fee);

        Self {
            used_body_fields: body_fields,
            tx_output_proposals: Vec::new(),
            used_utoxs: HashSet::new(),
            used_assets: HashSet::new(),
            total_ada: Coin::zero(),
            fee: Coin::zero(),
            witnesses_calculator: WitnessesCalculator::new(),
        }
    }

    pub(super) fn add_new_output(&mut self, address: &Address) {
        self.tx_output_proposals.push(TxOutputProposal::new(address));
    }

    pub(super) fn add_asset(&mut self, asset: &AssetIndex, policy_index: &PolicyIndex) {
        self.used_assets.insert(asset.clone());
        if let Some(output) = self.tx_output_proposals.last_mut() {
            output.add_asset(asset, policy_index);
        }
    }

    pub(super) fn add_utxo(&mut self, utxo: &UtxoIndex, ada_coins: &Coin, address: &Address) -> Result<(), JsError> {
        if self.used_utoxs.contains(utxo) {
            return Err(JsError::from_str("UTxO already used"));
        }
        self.used_utoxs.insert(utxo.clone());
        self.total_ada = self.total_ada.checked_add(ada_coins)?;
        self.witnesses_calculator.add_address(address)?;
        Ok(())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.used_utoxs.is_empty()
    }

    pub(super) fn get_used_assets(&self) -> &HashSet<AssetIndex> {
        &self.used_assets
    }

    pub(super) fn get_outputs(&self) -> &Vec<TxOutputProposal> {
        &self.tx_output_proposals
    }

    pub(super) fn get_fee(&self) -> &Coin {
        &self.fee
    }

    pub(super) fn set_fee(&mut self, fee: &Coin) {
        self.fee = fee.clone();
    }


    pub(super) fn get_total_ada_for_ouputs(&self) -> Result<Coin, JsError> {
        self.tx_output_proposals.iter()
            .map(|output| output.get_total_ada())
            .try_fold(Coin::zero(), |acc, ada| acc.checked_add(&ada))
    }

    pub(super) fn get_need_ada(&self) -> Result<Coin, JsError> {
        let need_ada = self.get_total_ada_for_ouputs()?
            .checked_add(&self.fee)?;
         Ok(need_ada.checked_sub(&self.total_ada).unwrap_or(Coin::zero()))
    }

    pub(super) fn get_unused_ada(&self) -> Result<Coin, JsError> {
        let need_ada = self.get_total_ada_for_ouputs()?
            .checked_add(&self.fee)?;
        return Ok(self.total_ada.checked_sub(&need_ada).unwrap_or(Coin::zero()));
    }

    pub(crate) fn add_last_ada_to_last_output(&mut self) -> Result<(), JsError> {
        let unused_ada = self.get_unused_ada()?;
        if let Some(output) = self.tx_output_proposals.last_mut() {
            output.add_ada(&unused_ada)?;
        }

        Ok(())
    }

    pub(crate) fn create_tx(&self, asset_groups: &AssetCategorizer, utxos: &TransactionUnspentOutputs)
                            -> Result<Transaction, JsError> {
        let mut outputs = Vec::new();
        for proposal in &self.tx_output_proposals {
            outputs.push(proposal.create_output(asset_groups, &self.used_utoxs)?);
        }

        let mut inputs = Vec::new();
        for utxo in &self.used_utoxs {
            inputs.push(utxos.0[utxo.0].input.clone());
        }

        let body = TransactionBody::new(
            &TransactionInputs(inputs),
            &TransactionOutputs(outputs),
            &self.fee,
            None,
        );
        let tx = Transaction::new(
            &body,
            &self.witnesses_calculator.create_mock_witnesses_set(),
            None,
        );

        Ok(tx)
    }
}