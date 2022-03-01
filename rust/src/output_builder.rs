use super::*;

/// We introduce a builder-pattern format for creating transaction outputs
/// This is because:
/// 1. Some fields (i.e. data hash) are optional, and we can't easily expose Option<> in WASM
/// 2. Some fields like amounts have many ways it could be set (some depending on other field values being known)
/// 3. Easier to adapt as the output format gets more complicated in future Cardano releases

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionOutputBuilder {
    address: Option<Address>,
    data_hash: Option<DataHash>,
}

#[wasm_bindgen]
impl TransactionOutputBuilder {
    pub fn new() -> Self {
        Self {
            address: None,
            data_hash: None,
        }
    }

    pub fn with_address(&self, address: &Address) -> Self {
        let mut cfg = self.clone();
        cfg.address = Some(address.clone());
        cfg
    }

    pub fn with_data_hash(&self, data_hash: &DataHash) -> Self {
        let mut cfg = self.clone();
        cfg.data_hash = Some(data_hash.clone());
        cfg
    }

    pub fn next(&self) -> Result<TransactionOutputAmountBuilder, JsError> {
        Ok(TransactionOutputAmountBuilder {
            address: self.address.clone().ok_or(JsError::from_str("TransactionOutputBaseBuilder: Address missing"))?,
            amount: None,
            data_hash: self.data_hash.clone(),
        })
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionOutputAmountBuilder {
    address: Address,
    amount: Option<Value>,
    data_hash: Option<DataHash>,
}

#[wasm_bindgen]
impl TransactionOutputAmountBuilder {

    pub fn with_value(&self, amount: &Value) -> Self {
        let mut cfg = self.clone();
        cfg.amount = Some(amount.clone());
        cfg
    }

    pub fn with_coin(&self, coin: &Coin) -> Self {
        let mut cfg = self.clone();

        cfg.amount = Some(Value::new(coin));
        cfg
    }

    pub fn with_coin_and_asset(&self, coin: &Coin, multiasset: &MultiAsset) -> Self {
        let mut cfg = self.clone();

        let mut val = Value::new(coin);
        val.set_multiasset(multiasset);
        cfg.amount = Some(val.clone());
        cfg
    }

    pub fn with_asset_and_min_required_coin(&self, multiasset: &MultiAsset, coins_per_utxo_word: &Coin) -> Result<TransactionOutputAmountBuilder, JsError> {
        let min_possible_coin = min_pure_ada(&coins_per_utxo_word, self.data_hash.is_some())?;
        let mut value = Value::new(&min_possible_coin);
        value.set_multiasset(multiasset);
        let required_coin = min_ada_required(
            &value,
            self.data_hash.is_some(),
            &coins_per_utxo_word,
        )?;

        Ok(self.with_coin_and_asset(&required_coin, &multiasset))
    }

    pub fn build(&self) -> Result<TransactionOutput, JsError> {
        Ok(TransactionOutput {
            address: self.address.clone(),
            amount: self.amount.clone().ok_or(JsError::from_str("TransactionOutputAmountBuilder: amount missing"))?,
            data_hash: self.data_hash.clone(),
        })
    }
}