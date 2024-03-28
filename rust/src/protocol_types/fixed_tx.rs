use crate::error::JsError;
use crate::*;

#[wasm_bindgen]
pub struct FixedTransaction {
    pub(crate) body: TransactionBody,
    pub(crate) body_bytes: Vec<u8>,

    pub(crate) witness_set: TransactionWitnessSet,
    pub(crate) witness_bytes: Vec<u8>,

    pub(crate) is_valid: bool,

    pub(crate) auxiliary_data: Option<AuxiliaryData>,
    pub(crate) auxiliary_bytes: Option<Vec<u8>>,
}

to_from_bytes!(FixedTransaction);

#[wasm_bindgen]
impl FixedTransaction {
    pub fn new(
        raw_body: &[u8],
        raw_witness_set: &[u8],
        is_valid: bool,
    ) -> Result<FixedTransaction, JsError> {
        let body = TransactionBody::from_bytes(raw_body.to_vec())?;
        let witness_set = TransactionWitnessSet::from_bytes(raw_witness_set.to_vec())?;

        Ok(FixedTransaction {
            body,
            body_bytes: raw_body.to_vec(),
            witness_set,
            witness_bytes: raw_witness_set.to_vec(),
            is_valid,
            auxiliary_data: None,
            auxiliary_bytes: None,
        })
    }

    pub fn new_with_auxiliary(
        raw_body: &[u8],
        raw_witness_set: &[u8],
        raw_auxiliary_data: &[u8],
        is_valid: bool,
    ) -> Result<FixedTransaction, JsError> {
        let body = TransactionBody::from_bytes(raw_body.to_vec())?;
        let witness_set = TransactionWitnessSet::from_bytes(raw_witness_set.to_vec())?;
        let auxiliary_data = Some(AuxiliaryData::from_bytes(raw_auxiliary_data.to_vec())?);

        Ok(FixedTransaction {
            body,
            body_bytes: raw_body.to_vec(),
            witness_set,
            witness_bytes: raw_witness_set.to_vec(),
            is_valid,
            auxiliary_data,
            auxiliary_bytes: Some(raw_auxiliary_data.to_vec()),
        })
    }

    pub fn body(&self) -> TransactionBody {
        self.body.clone()
    }

    pub fn raw_body(&self) -> Vec<u8> {
        self.body_bytes.clone()
    }

    pub fn set_body(&mut self, raw_body: &[u8]) -> Result<(), JsError> {
        let body = TransactionBody::from_bytes(raw_body.to_vec())?;
        self.body = body;
        self.body_bytes = raw_body.to_vec();
        Ok(())
    }

    pub fn set_witness_set(&mut self, raw_witness_set: &[u8]) -> Result<(), JsError> {
        let witness_set = TransactionWitnessSet::from_bytes(raw_witness_set.to_vec())?;
        self.witness_set = witness_set;
        self.witness_bytes = raw_witness_set.to_vec();
        Ok(())
    }

    pub fn witness_set(&self) -> TransactionWitnessSet {
        self.witness_set.clone()
    }

    pub fn raw_witness_set(&self) -> Vec<u8> {
        self.witness_bytes.clone()
    }

    pub fn set_is_valid(&mut self, valid: bool) {
        self.is_valid = valid
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid.clone()
    }

    pub fn set_auxiliary_data(&mut self, raw_auxiliary_data: &[u8]) -> Result<(), JsError> {
        let auxiliary_data = AuxiliaryData::from_bytes(raw_auxiliary_data.to_vec())?;
        self.auxiliary_data = Some(auxiliary_data);
        self.auxiliary_bytes = Some(raw_auxiliary_data.to_vec());
        Ok(())
    }

    pub fn auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.auxiliary_data.clone()
    }

    pub fn raw_auxiliary_data(&self) -> Option<Vec<u8>> {
        self.auxiliary_bytes.clone()
    }
}
