use crate::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DatumSourceEnum {
    Datum(PlutusData),
    RefInput(TransactionInput),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DatumSource(pub(crate) DatumSourceEnum);

#[wasm_bindgen]
impl DatumSource {
    pub fn new(datum: &PlutusData) -> Self {
        Self(DatumSourceEnum::Datum(datum.clone()))
    }

    pub fn new_ref_input(input: &TransactionInput) -> Self {
        Self(DatumSourceEnum::RefInput(input.clone()))
    }
}