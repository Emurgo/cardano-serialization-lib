// taken directly from js-chain-libs just to test things:

use chain_impl_mockchain as chain;
use chain_crypto as crypto;
use chain::{key};
use crypto::bech32::Bech32 as _;
use rand_os::OsRng;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PrivateKey(key::EitherEd25519SecretKey);

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

    pub fn generate_ed25519() -> Result<PrivateKey, JsValue> {
        OsRng::new()
            .map(crypto::SecretKey::<crypto::Ed25519>::generate)
            .map(key::EitherEd25519SecretKey::Normal)
            .map(PrivateKey)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    pub fn generate_ed25519extended() -> Result<PrivateKey, JsValue> {
        OsRng::new()
            .map(crypto::SecretKey::<crypto::Ed25519Extended>::generate)
            .map(key::EitherEd25519SecretKey::Extended)
            .map(PrivateKey)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
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

    pub fn from_extended_bytes(bytes: &[u8]) -> Result<PrivateKey, JsValue> {
        crypto::SecretKey::from_binary(bytes)
            .map(key::EitherEd25519SecretKey::Extended)
            .map(PrivateKey)
            .map_err(|_| JsValue::from_str("Invalid extended secret key"))
    }

    pub fn from_normal_bytes(bytes: &[u8]) -> Result<PrivateKey, JsValue> {
        crypto::SecretKey::from_binary(bytes)
            .map(key::EitherEd25519SecretKey::Normal)
            .map(PrivateKey)
            .map_err(|_| JsValue::from_str("Invalid normal secret key"))
    }

    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        Ed25519Signature(self.0.sign(&message.to_vec()))
    }
}

/// ED25519 key used as public key
#[wasm_bindgen]
#[derive(Clone)]
pub struct PublicKey(crypto::PublicKey<crypto::Ed25519>);

impl From<crypto::PublicKey<crypto::Ed25519>> for PublicKey {
    fn from(key: crypto::PublicKey<crypto::Ed25519>) -> PublicKey {
        PublicKey(key)
    }
}

#[wasm_bindgen]
impl PublicKey {
    /// Get private key from its bech32 representation
    /// Example:
    /// ```javascript
    /// const pkey = PublicKey.from_bech32(&#39;ed25519_pk1dgaagyh470y66p899txcl3r0jaeaxu6yd7z2dxyk55qcycdml8gszkxze2&#39;);
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PublicKey, JsValue> {
        crypto::PublicKey::try_from_bech32_str(&bech32_str)
            .map(PublicKey)
            .map_err(|_| JsValue::from_str("Malformed public key"))
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, JsValue> {
        crypto::PublicKey::from_binary(bytes)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
            .map(PublicKey)
    }

    pub fn verify(&self, data: &[u8], signature: &Ed25519Signature) -> bool {
        signature.0.verify_slice(&self.0, data) == crypto::Verification::Success
    }
}

#[wasm_bindgen]
pub struct PublicKeys(Vec<PublicKey>);

#[wasm_bindgen]
impl PublicKeys {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PublicKeys {
        PublicKeys(vec![])
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PublicKey {
        self.0[index].clone()
    }

    pub fn add(&mut self, key: &PublicKey) {
        self.0.push(key.clone());
    }
}

macro_rules! impl_signature {
    ($name:ident, $signee_type:ty, $verifier_type:ty) => {
        #[wasm_bindgen]
        pub struct $name(crypto::Signature<$signee_type, $verifier_type>);

        #[wasm_bindgen]
        impl $name {
            pub fn as_bytes(&self) -> Vec<u8> {
                self.0.as_ref().to_vec()
            }

            pub fn to_bech32(&self) -> String {
                self.0.to_bech32_str()
            }

            pub fn to_hex(&self) -> String {
                hex::encode(&self.0.as_ref())
            }

            pub fn from_bytes(bytes: &[u8]) -> Result<$name, JsValue> {
                crypto::Signature::from_binary(bytes)
                    .map($name)
                    .map_err(|e| JsValue::from_str(&format!("{}", e)))
            }

            pub fn from_bech32(bech32_str: &str) -> Result<$name, JsValue> {
                crypto::Signature::try_from_bech32_str(&bech32_str)
                    .map($name)
                    .map_err(|e| JsValue::from_str(&format!("{}", e)))
            }

            pub fn from_hex(input: &str) -> Result<$name, JsValue> {
                crypto::Signature::from_str(input)
                    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
                    .map($name)
            }
        }
    };
}

impl_signature!(Ed25519Signature, Vec<u8>, crypto::Ed25519);