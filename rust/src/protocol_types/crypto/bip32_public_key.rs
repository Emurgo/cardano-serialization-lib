use crate::{JsError, PublicKey, wasm_bindgen};
use crate::chain_crypto::bech32::Bech32;

#[wasm_bindgen]
pub struct Bip32PublicKey(pub(crate) crate::chain_crypto::PublicKey<crate::chain_crypto::Ed25519Bip32>);

#[wasm_bindgen]
impl Bip32PublicKey {
    /// derive this public key with the given index.
    ///
    /// # Errors
    ///
    /// If the index is not a soft derivation index (< 0x80000000) then
    /// calling this method will fail.
    ///
    /// # Security considerations
    ///
    /// * hard derivation index cannot be soft derived with the public key
    ///
    /// # Hard derivation vs Soft derivation
    ///
    /// If you pass an index below 0x80000000 then it is a soft derivation.
    /// The advantage of soft derivation is that it is possible to derive the
    /// public key too. I.e. derivation the private key with a soft derivation
    /// index and then retrieving the associated public key is equivalent to
    /// deriving the public key associated to the parent private key.
    ///
    /// Hard derivation index does not allow public key derivation.
    ///
    /// This is why deriving the private key should not fail while deriving
    /// the public key may fail (if the derivation index is invalid).
    ///
    pub fn derive(&self, index: u32) -> Result<Bip32PublicKey, JsError> {
        crate::chain_crypto::derive::derive_pk_ed25519(&self.0, index)
            .map(Bip32PublicKey)
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
    }

    pub fn to_raw_key(&self) -> PublicKey {
        PublicKey(crate::chain_crypto::derive::to_raw_pk(&self.0))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Bip32PublicKey, JsError> {
        crate::chain_crypto::PublicKey::<crate::chain_crypto::Ed25519Bip32>::from_binary(bytes)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
            .map(Bip32PublicKey)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PublicKey, JsError> {
        crate::chain_crypto::PublicKey::try_from_bech32_str(&bech32_str)
            .map(Bip32PublicKey)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn chaincode(&self) -> Vec<u8> {
        const ED25519_PUBLIC_KEY_LENGTH: usize = 32;
        const XPUB_SIZE: usize = 64;
        self.0.as_ref()[ED25519_PUBLIC_KEY_LENGTH..XPUB_SIZE].to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }

    pub fn from_hex(hex_str: &str) -> Result<Bip32PublicKey, JsError> {
        match hex::decode(hex_str) {
            Ok(data) => Ok(Self::from_bytes(data.as_ref())?),
            Err(e) => Err(JsError::from_str(&e.to_string())),
        }
    }
}
