use schemars::JsonSchema;
use crate::{Ed25519KeyHash, Ed25519Signature, JsError, wasm_bindgen};
use crate::chain_crypto::bech32::Bech32;
use crate::crypto::blake2b224;

/// ED25519 key used as public key
#[wasm_bindgen]
#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct PublicKey(pub(crate) crate::chain_crypto::PublicKey<crate::chain_crypto::Ed25519>);

impl From<crate::chain_crypto::PublicKey<crate::chain_crypto::Ed25519>> for PublicKey {
    fn from(key: crate::chain_crypto::PublicKey<crate::chain_crypto::Ed25519>) -> PublicKey {
        PublicKey(key)
    }
}

#[wasm_bindgen]
impl PublicKey {
    /// Get public key from its bech32 representation
    /// Example:
    /// ```javascript
    /// const pkey = PublicKey.from_bech32(&#39;ed25519_pk1dgaagyh470y66p899txcl3r0jaeaxu6yd7z2dxyk55qcycdml8gszkxze2&#39;);
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PublicKey, JsError> {
        crate::chain_crypto::PublicKey::try_from_bech32_str(&bech32_str)
            .map(PublicKey)
            .map_err(|_| JsError::from_str("Malformed public key"))
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, JsError> {
        crate::chain_crypto::PublicKey::from_binary(bytes)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
            .map(PublicKey)
    }

    pub fn verify(&self, data: &[u8], signature: &Ed25519Signature) -> bool {
        signature.0.verify_slice(&self.0, data) == crate::chain_crypto::Verification::Success
    }

    pub fn hash(&self) -> Ed25519KeyHash {
        Ed25519KeyHash::from(blake2b224(self.as_bytes().as_ref()))
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }

    pub fn from_hex(hex_str: &str) -> Result<PublicKey, JsError> {
        match hex::decode(hex_str) {
            Ok(data) => Ok(Self::from_bytes(data.as_ref())?),
            Err(e) => Err(JsError::from_str(&e.to_string())),
        }
    }
}

impl serde::Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_bech32())
    }
}

impl<'de> serde::de::Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        PublicKey::from_bech32(&s).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"bech32 public key string",
            )
        })
    }
}

impl JsonSchema for PublicKey {
    fn schema_name() -> String {
        String::from("PublicKey")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}