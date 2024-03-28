use crate::*;
use crate::chain_crypto as crypto;

#[wasm_bindgen]
pub struct LegacyDaedalusPrivateKey(pub(crate) crypto::SecretKey<crypto::LegacyDaedalus>);

#[wasm_bindgen]
impl LegacyDaedalusPrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<LegacyDaedalusPrivateKey, JsError> {
        crypto::SecretKey::<crypto::LegacyDaedalus>::from_binary(bytes)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
            .map(LegacyDaedalusPrivateKey)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn chaincode(&self) -> Vec<u8> {
        const ED25519_PRIVATE_KEY_LENGTH: usize = 64;
        const XPRV_SIZE: usize = 96;
        self.0.as_ref()[ED25519_PRIVATE_KEY_LENGTH..XPRV_SIZE].to_vec()
    }
}