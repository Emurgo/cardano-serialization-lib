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
    data: Option<DataOption>,
    script_ref: Option<ScriptRef>,
}

#[wasm_bindgen]
impl TransactionOutputBuilder {
    pub fn new() -> Self {
        Self {
            address: None,
            data: None,
            script_ref: None,
        }
    }

    pub fn with_address(&self, address: &Address) -> Self {
        let mut cfg = self.clone();
        cfg.address = Some(address.clone());
        cfg
    }

    pub fn with_data_hash(&self, data_hash: &DataHash) -> Self {
        let mut cfg = self.clone();
        cfg.data = Some(DataOption::DataHash(data_hash.clone()));
        cfg
    }

    pub fn with_plutus_data(&self, data: &PlutusData) -> Self {
        let mut cfg = self.clone();
        cfg.data = Some(DataOption::Data(data.clone()));
        cfg
    }

    pub fn with_script_ref(&self, script_ref: &ScriptRef) -> Self {
        let mut cfg = self.clone();
        cfg.script_ref = Some(script_ref.clone());
        cfg
    }

    pub fn next(&self) -> Result<TransactionOutputAmountBuilder, JsError> {
        Ok(TransactionOutputAmountBuilder {
            address: self.address.clone().ok_or(JsError::from_str(
                "TransactionOutputBaseBuilder: Address missing",
            ))?,
            amount: None,
            data: self.data.clone(),
            script_ref: self.script_ref.clone(),
        })
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionOutputAmountBuilder {
    address: Address,
    amount: Option<Value>,
    data: Option<DataOption>,
    script_ref: Option<ScriptRef>,
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

    /// !!! DEPRECATED !!!
    /// Since babbage era cardano nodes use coins per byte. Use '.with_asset_and_min_required_coin_by_utxo_cost' instead.
    #[deprecated(
        since = "11.0.0",
        note = "Since babbage era cardano nodes use coins per byte. Use '.with_asset_and_min_required_coin_by_utxo_cost' instead."
    )]
    pub fn with_asset_and_min_required_coin(
        &self,
        multiasset: &MultiAsset,
        coins_per_utxo_word: &Coin,
    ) -> Result<TransactionOutputAmountBuilder, JsError> {
        let data_cost = DataCost::new_coins_per_word(coins_per_utxo_word);
        self.with_asset_and_min_required_coin_by_utxo_cost(multiasset, &data_cost)
    }

    pub fn with_asset_and_min_required_coin_by_utxo_cost(
        &self,
        multiasset: &MultiAsset,
        data_cost: &DataCost,
    ) -> Result<TransactionOutputAmountBuilder, JsError> {
        // TODO: double ada calculation needs to check if it redundant
        let mut calc = MinOutputAdaCalculator::new_empty(data_cost)?;
        if let Some(data) = &self.data {
            match data {
                DataOption::DataHash(data_hash) => calc.set_data_hash(data_hash),
                DataOption::Data(datum) => calc.set_plutus_data(datum),
            };
        }
        if let Some(script_ref) = &self.script_ref {
            calc.set_script_ref(script_ref);
        }
        let min_possible_coin = calc.calculate_ada()?;
        let mut value = Value::new(&min_possible_coin);
        value.set_multiasset(multiasset);

        let mut calc = MinOutputAdaCalculator::new_empty(data_cost)?;
        calc.set_amount(&value);
        if let Some(data) = &self.data {
            match data {
                DataOption::DataHash(data_hash) => calc.set_data_hash(data_hash),
                DataOption::Data(datum) => calc.set_plutus_data(datum),
            };
        }
        if let Some(script_ref) = &self.script_ref {
            calc.set_script_ref(script_ref);
        }
        let required_coin = calc.calculate_ada()?;

        Ok(self.with_coin_and_asset(&required_coin, &multiasset))
    }

    pub fn build(&self) -> Result<TransactionOutput, JsError> {
        Ok(TransactionOutput {
            address: self.address.clone(),
            amount: self.amount.clone().ok_or(JsError::from_str(
                "TransactionOutputAmountBuilder: amount missing",
            ))?,
            plutus_data: self.data.clone(),
            script_ref: self.script_ref.clone(),
            serialization_format: None
        })
    }
}
