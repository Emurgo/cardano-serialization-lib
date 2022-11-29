use batch_tools::proposals::{TxProposal};
use batch_tools::asset_categorizer::{AssetCategorizer};
use super::*;

#[wasm_bindgen]
pub struct TransactionBatchList(Vec<TransactionBatch>);

#[wasm_bindgen]
impl TransactionBatchList {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionBatch {
        self.0[index].clone()
    }
}

impl<'a> IntoIterator for &'a TransactionBatchList {
    type Item = &'a TransactionBatch;
    type IntoIter =  std::slice::Iter<'a, TransactionBatch>;

    fn into_iter(self) -> std::slice::Iter<'a, TransactionBatch> {
        self.0.iter()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TransactionBatch {
    transactions: Vec<Transaction>,
}

#[wasm_bindgen]
impl TransactionBatch {
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    pub fn get(&self, index: usize) -> Transaction {
        self.transactions[index].clone()
    }
}

impl<'a> IntoIterator for &'a TransactionBatch {
    type Item = &'a Transaction;
    type IntoIter = std::slice::Iter<'a, Transaction>;

    fn into_iter(self) -> std::slice::Iter<'a, Transaction> {
        self.transactions.iter()
    }
}

impl TransactionBatch {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }
}

struct TxBatchBuilder {
    asset_groups: AssetCategorizer,
    tx_proposals: Vec<TxProposal>,
}

impl TxBatchBuilder {
    pub fn new(utxos: &TransactionUnspentOutputs, address: &Address, config: &TransactionBuilderConfig) -> Result<Self, JsError> {
        let asset_groups = AssetCategorizer::new(config, utxos, address)?;
        Ok(Self {
            asset_groups,
            tx_proposals: Vec::new(),
        })
    }

    pub fn build(&mut self, utxos: &TransactionUnspentOutputs) -> Result<TransactionBatch, JsError> {
        while self.asset_groups.has_assets() || self.asset_groups.has_ada() {
            let mut current_tx_proposal = TxProposal::new();
            while let Some(tx_proposal) = self.asset_groups.try_append_next_utxos(&current_tx_proposal)? {
                current_tx_proposal = tx_proposal;
            }

            if current_tx_proposal.is_empty() && (self.asset_groups.has_assets() || self.asset_groups.has_ada()) {
                return Err(JsError::from_str("Unable to build transaction batch"));
            }

            current_tx_proposal.add_last_ada_to_last_output()?;
            self.asset_groups.set_min_ada_for_tx(&mut current_tx_proposal)?;
            self.tx_proposals.push(current_tx_proposal);
        }

        let mut batch = TransactionBatch::new();
        for tx_proposal in self.tx_proposals.iter_mut() {
            batch.add_transaction(tx_proposal.create_tx(&self.asset_groups, utxos)?);
        }

        Ok(batch)
    }
}

#[wasm_bindgen]
pub fn create_send_all(address: &Address, utxos: &TransactionUnspentOutputs, config: &TransactionBuilderConfig)
    -> Result<TransactionBatchList, JsError> {
    let mut tx_batch_builder = TxBatchBuilder::new(utxos, address, config)?;
    let batch = tx_batch_builder.build(utxos)?;
    Ok(TransactionBatchList(vec![batch]))
}

