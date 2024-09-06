use crate::error::JsError;
use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedTransaction {
    pub(crate) body: TransactionBody,
    pub(crate) body_bytes: Vec<u8>,
    pub(crate) tx_hash: TransactionHash,

    pub(crate) witness_set: FixedTxWitnessesSet,

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
        let witness_set = FixedTxWitnessesSet::from_bytes(raw_witness_set.to_vec())?;
        let tx_hash = TransactionHash::from(blake2b256(raw_body));

        Ok(FixedTransaction {
            body,
            body_bytes: raw_body.to_vec(),
            tx_hash,
            witness_set,
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
        let witness_set = FixedTxWitnessesSet::from_bytes(raw_witness_set.to_vec())?;
        let tx_hash = TransactionHash::from(blake2b256(raw_body));
        let auxiliary_data = Some(AuxiliaryData::from_bytes(raw_auxiliary_data.to_vec())?);

        Ok(FixedTransaction {
            body,
            body_bytes: raw_body.to_vec(),
            tx_hash,
            witness_set,
            is_valid,
            auxiliary_data,
            auxiliary_bytes: Some(raw_auxiliary_data.to_vec()),
        })
    }

    pub(crate) fn new_with_original_bytes(
        tx_body: TransactionBody,
        raw_body: Vec<u8>,
        tx_witnesses_set: FixedTxWitnessesSet,
        is_valid: bool,
        auxiliary_data: Option<AuxiliaryData>,
        raw_auxiliary_data: Option<Vec<u8>>,
    ) -> FixedTransaction {
        let tx_hash = TransactionHash::from(blake2b256(&raw_body));

        FixedTransaction {
            body: tx_body,
            body_bytes: raw_body,
            tx_hash,
            witness_set: tx_witnesses_set,
            is_valid,
            auxiliary_data,
            auxiliary_bytes: raw_auxiliary_data,
        }
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

    /// We do not recommend using this function, since it might lead to script integrity hash.
    /// The only purpose of this struct is to sign the transaction from third-party sources.
    /// Use `.sign_and_add_vkey_signature` or `.sign_and_add_icarus_bootstrap_signature` or `.sign_and_add_daedalus_bootstrap_signature` instead.
    #[deprecated(since = "12.1.0", note = "Use `.sign_and_add_vkey_signature` or `.sign_and_add_icarus_bootstrap_signature` or `.sign_and_add_daedalus_bootstrap_signature` instead.")]
    pub fn set_witness_set(&mut self, raw_witness_set: &[u8]) -> Result<(), JsError> {
        let witness_set = FixedTxWitnessesSet::from_bytes(raw_witness_set.to_vec())?;
        self.witness_set = witness_set;
        Ok(())
    }

    pub fn witness_set(&self) -> TransactionWitnessSet {
        self.witness_set.tx_witnesses_set()
    }

    pub fn raw_witness_set(&self) -> Vec<u8> {
        self.witness_set.to_bytes()
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

    pub fn transaction_hash(&self) -> TransactionHash {
        self.tx_hash.clone()
    }

    pub fn add_vkey_witness(&mut self, vkey_witness: &Vkeywitness) {
        self.witness_set.add_vkey_witness(vkey_witness.clone());
    }

    pub fn add_bootstrap_witness(&mut self, bootstrap_witness: &BootstrapWitness) {
        self.witness_set.add_bootstrap_witness(bootstrap_witness.clone());
    }

    pub fn sign_and_add_vkey_signature(&mut self, private_key: &PrivateKey) -> Result<(), JsError> {
        let vkey_witness = make_vkey_witness(&self.tx_hash, private_key);
        self.witness_set.add_vkey_witness(vkey_witness);
        Ok(())
    }

    pub fn sign_and_add_icarus_bootstrap_signature(&mut self, addr: &ByronAddress, private_key: &Bip32PrivateKey) -> Result<(), JsError> {
        let bootstrap_witness = make_icarus_bootstrap_witness(&self.tx_hash, addr, private_key);
        self.witness_set.add_bootstrap_witness(bootstrap_witness);
        Ok(())
    }

    pub fn sign_and_add_daedalus_bootstrap_signature(&mut self, addr: &ByronAddress, private_key: &LegacyDaedalusPrivateKey) -> Result<(), JsError> {
        let bootstrap_witness = make_daedalus_bootstrap_witness(&self.tx_hash, addr, private_key);
        self.witness_set.add_bootstrap_witness(bootstrap_witness);
        Ok(())
    }
}
