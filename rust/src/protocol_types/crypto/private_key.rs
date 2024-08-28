use crate::{Ed25519Signature, JsError, PublicKey, wasm_bindgen};
use crate::impl_mockchain::key;
use rand_os::OsRng;
use crate::chain_crypto::bech32::Bech32;

#[wasm_bindgen]
pub struct PrivateKey(pub(crate) key::EitherEd25519SecretKey);

impl From<key::EitherEd25519SecretKey> for PrivateKey {
    fn from(secret_key: key::EitherEd25519SecretKey) -> PrivateKey {
        PrivateKey(secret_key)
    }
}

#[wasm_bindgen]
impl PrivateKey {
    pub fn to_public(&self) -> PublicKey {
        self.0.to_public().into()
    }

    pub fn generate_ed25519() -> Result<PrivateKey, JsError> {
        OsRng::new()
            .map(crate::chain_crypto::SecretKey::<crate::chain_crypto::Ed25519>::generate)
            .map(key::EitherEd25519SecretKey::Normal)
            .map(PrivateKey)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
    }

    pub fn generate_ed25519extended() -> Result<PrivateKey, JsError> {
        OsRng::new()
            .map(crate::chain_crypto::SecretKey::<crate::chain_crypto::Ed25519Extended>::generate)
            .map(key::EitherEd25519SecretKey::Extended)
            .map(PrivateKey)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
    }

    /// Get private key from its bech32 representation
    /// ```javascript
    /// PrivateKey.from_bech32(&#39;ed25519_sk1ahfetf02qwwg4dkq7mgp4a25lx5vh9920cr5wnxmpzz9906qvm8qwvlts0&#39;);
    /// ```
    /// For an extended 25519 key
    /// ```javascript
    /// PrivateKey.from_bech32(&#39;ed25519e_sk1gqwl4szuwwh6d0yk3nsqcc6xxc3fpvjlevgwvt60df59v8zd8f8prazt8ln3lmz096ux3xvhhvm3ca9wj2yctdh3pnw0szrma07rt5gl748fp&#39;);
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PrivateKey, JsError> {
        crate::chain_crypto::SecretKey::try_from_bech32_str(&bech32_str)
            .map(key::EitherEd25519SecretKey::Extended)
            .or_else(|_| {
                crate::chain_crypto::SecretKey::try_from_bech32_str(&bech32_str)
                    .map(key::EitherEd25519SecretKey::Normal)
            })
            .map(PrivateKey)
            .map_err(|_| JsError::from_str("Invalid secret key"))
    }

    pub fn to_bech32(&self) -> String {
        match self.0 {
            key::EitherEd25519SecretKey::Normal(ref secret) => secret.to_bech32_str(),
            key::EitherEd25519SecretKey::Extended(ref secret) => secret.to_bech32_str(),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self.0 {
            key::EitherEd25519SecretKey::Normal(ref secret) => secret.as_ref().to_vec(),
            key::EitherEd25519SecretKey::Extended(ref secret) => secret.as_ref().to_vec(),
        }
    }

    pub fn from_extended_bytes(bytes: &[u8]) -> Result<PrivateKey, JsError> {
        crate::chain_crypto::SecretKey::from_binary(bytes)
            .map(key::EitherEd25519SecretKey::Extended)
            .map(PrivateKey)
            .map_err(|_| JsError::from_str("Invalid extended secret key"))
    }

    pub fn from_normal_bytes(bytes: &[u8]) -> Result<PrivateKey, JsError> {
        crate::chain_crypto::SecretKey::from_binary(bytes)
            .map(key::EitherEd25519SecretKey::Normal)
            .map(PrivateKey)
            .map_err(|_| JsError::from_str("Invalid normal secret key"))
    }

    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        Ed25519Signature(self.0.sign(&message.to_vec()))
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }

    pub fn from_hex(hex_str: &str) -> Result<PrivateKey, JsError> {
        let data: Vec<u8> = match hex::decode(hex_str) {
            Ok(d) => d,
            Err(e) => return Err(JsError::from_str(&e.to_string())),
        };
        let data_slice: &[u8] = data.as_slice();
        crate::chain_crypto::SecretKey::from_binary(data_slice)
            .map(key::EitherEd25519SecretKey::Normal)
            .or_else(|_| {
                crate::chain_crypto::SecretKey::from_binary(data_slice)
                    .map(key::EitherEd25519SecretKey::Extended)
            })
            .map(PrivateKey)
            .map_err(|_| JsError::from_str("Invalid secret key"))
    }
}