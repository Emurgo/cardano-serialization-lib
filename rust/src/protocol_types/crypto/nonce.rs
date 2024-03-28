use crate::*;

// Evolving nonce type (used for Update's crypto)
#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct Nonce {
    pub(crate) hash: Option<[u8; 32]>,
}

impl_to_from!(Nonce);

// can't export consts via wasm_bindgen
impl Nonce {
    pub const HASH_LEN: usize = 32;
}

#[wasm_bindgen]
impl Nonce {
    pub fn new_identity() -> Nonce {
        Self { hash: None }
    }

    pub fn new_from_hash(hash: Vec<u8>) -> Result<Nonce, JsError> {
        use std::convert::TryInto;
        match hash[..Self::HASH_LEN].try_into() {
            Ok(bytes_correct_size) => Ok(Self {
                hash: Some(bytes_correct_size),
            }),
            Err(e) => Err(JsError::from_str(&e.to_string())),
        }
    }

    pub fn get_hash(&self) -> Option<Vec<u8>> {
        Some(self.hash?.to_vec())
    }
}