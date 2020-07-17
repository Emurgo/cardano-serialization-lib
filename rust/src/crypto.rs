use cbor_event::{de::Deserializer, se::Serializer};
use chain_impl_mockchain as chain;
use chain_crypto as crypto;
use chain::{key};
use crypto::bech32::Bech32 as _;
use rand_os::OsRng;
use std::io::{BufRead, Seek, Write};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use cryptoxide::blake2b::Blake2b;

use super::*;

pub (crate) fn blake2b224(data: &[u8]) -> [u8; 28] {
    let mut out = [0; 28];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

pub (crate) fn blake2b256(data: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

// All key structs were taken from js-chain-libs:
// https://github.com/Emurgo/js-chain-libs

#[wasm_bindgen]
pub struct Bip32PrivateKey(crypto::SecretKey<crypto::Ed25519Bip32>);

#[wasm_bindgen]
impl Bip32PrivateKey {
    /// derive this private key with the given index.
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
    pub fn derive(&self, index: u32) -> Bip32PrivateKey {
        Bip32PrivateKey(crypto::derive::derive_sk_ed25519(&self.0, index))
    }

    pub fn generate_ed25519_bip32() -> Result<Bip32PrivateKey, JsValue> {
        OsRng::new()
            .map(crypto::SecretKey::<crypto::Ed25519Bip32>::generate)
            .map(Bip32PrivateKey)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    pub fn to_raw_key(&self) -> PrivateKey {
        PrivateKey(key::EitherEd25519SecretKey::Extended(
            crypto::derive::to_raw_sk(&self.0),
        ))
    }

    pub fn to_public(&self) -> Bip32PublicKey {
        Bip32PublicKey(self.0.to_public().into())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Bip32PrivateKey, JsValue> {
        crypto::SecretKey::<crypto::Ed25519Bip32>::from_binary(bytes)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
            .map(Bip32PrivateKey)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PrivateKey, JsValue> {
        crypto::SecretKey::try_from_bech32_str(&bech32_str)
            .map(Bip32PrivateKey)
            .map_err(|_| JsValue::from_str("Invalid secret key"))
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn from_bip39_entropy(entropy: &[u8], password: &[u8]) -> Bip32PrivateKey {
        Bip32PrivateKey(crypto::derive::from_bip39_entropy(&entropy, &password))
    }

    pub fn chaincode(&self) -> Vec<u8> {
        let ED25519_PRIVATE_KEY_LENGTH = 64;
        let XPRV_SIZE = 96;
        self.0.as_ref()[ED25519_PRIVATE_KEY_LENGTH..XPRV_SIZE].to_vec()
    }
}

#[wasm_bindgen]
pub struct Bip32PublicKey(crypto::PublicKey<crypto::Ed25519Bip32>);

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
    pub fn derive(&self, index: u32) -> Result<Bip32PublicKey, JsValue> {
        crypto::derive::derive_pk_ed25519(&self.0, index)
            .map(Bip32PublicKey)
            .map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
    }

    pub fn to_raw_key(&self) -> PublicKey {
        PublicKey(crypto::derive::to_raw_pk(&self.0))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Bip32PublicKey, JsValue> {
        crypto::PublicKey::<crypto::Ed25519Bip32>::from_binary(bytes)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
            .map(Bip32PublicKey)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PublicKey, JsValue> {
        crypto::PublicKey::try_from_bech32_str(&bech32_str)
            .map(Bip32PublicKey)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn chaincode(&self) -> Vec<u8> {
        let ED25519_PRIVATE_KEY_LENGTH = 64;
        let XPRV_SIZE = 96;
        self.0.as_ref()[ED25519_PRIVATE_KEY_LENGTH..XPRV_SIZE].to_vec()
    }
}


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

    pub fn hash(&self) -> Ed25519KeyHash {
        Ed25519KeyHash::from(blake2b224(self.as_bytes().as_ref()))
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Vkey(PublicKey);

to_from_bytes!(Vkey);

#[wasm_bindgen]
impl Vkey {
    pub fn new(pk: &PublicKey) -> Self {
        Self(pk.clone())
    }

    pub fn public_key(&self) -> PublicKey {
        self.0.clone()
    }
}

impl cbor_event::se::Serialize for Vkey {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0.as_bytes())
    }
}

impl Deserialize for Vkey {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(PublicKey(crypto::PublicKey::from_binary(raw.bytes()?.as_ref())?)))
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Vkeywitness {
    vkey: Vkey,
    signature: Ed25519Signature,
}

to_from_bytes!(Vkeywitness);

#[wasm_bindgen]
impl Vkeywitness {
    pub fn new(vkey: &Vkey, signature: &Ed25519Signature) -> Self {
        Self {
            vkey: vkey.clone(),
            signature: signature.clone()
        }
    }

    pub fn vkey(&self) -> Vkey {
        self.vkey.clone()
    }

    pub fn signature(&self) -> Ed25519Signature {
        self.signature.clone()
    }
}

impl cbor_event::se::Serialize for Vkeywitness {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.vkey.serialize(serializer)?;
        self.signature.serialize(serializer)
    }
}

impl Deserialize for Vkeywitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let vkey = (|| -> Result<_, DeserializeError> {
                Ok(Vkey::deserialize(raw)?)
            })().map_err(|e| e.annotate("vkey"))?;
            let signature = (|| -> Result<_, DeserializeError> {
                Ok(Ed25519Signature::deserialize(raw)?)
            })().map_err(|e| e.annotate("signature"))?;
            let ret = Ok(Vkeywitness::new(&vkey, &signature));
            match len {
                cbor_event::Len::Len(n) => match n {
                    2 => (),
                    m => return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(2, len, "")).into()),
                },
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("Vkeywitness"))
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Vkeywitnesses(Vec<Vkeywitness>);

#[wasm_bindgen]
impl Vkeywitnesses {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Vkeywitness {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &Vkeywitness) {
        self.0.push(elem.clone());
    }
}

impl cbor_event::se::Serialize for Vkeywitnesses {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Vkeywitnesses {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == cbor_event::Type::Special {
                    assert_eq!(raw.special()?, cbor_event::Special::Break);
                    break;
                }
                arr.push(Vkeywitness::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("Vkeywitnesses"))?;
        Ok(Self(arr))
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct BootstrapWitness {
    vkey: Vkey,
    signature: Ed25519Signature,
    chain_code: Vec<u8>,
    pad_prefix: Vec<u8>,
    pad_suffix: Vec<u8>,
}

to_from_bytes!(BootstrapWitness);

#[wasm_bindgen]
impl BootstrapWitness {
    pub fn vkey(&self) -> Vkey {
        self.vkey.clone()
    }

    pub fn signature(&self) -> Ed25519Signature {
        self.signature.clone()
    }

    pub fn chain_code(&self) -> Vec<u8> {
        self.chain_code.clone()
    }

    pub fn pad_prefix(&self) -> Vec<u8> {
        self.pad_prefix.clone()
    }

    pub fn pad_suffix(&self) -> Vec<u8> {
        self.pad_suffix.clone()
    }

    pub fn new(vkey: &Vkey, signature: &Ed25519Signature, chain_code: Vec<u8>, pad_prefix: Vec<u8>, pad_suffix: Vec<u8>) -> Self {
        Self {
            vkey: vkey.clone(),
            signature: signature.clone(),
            chain_code: chain_code,
            pad_prefix: pad_prefix,
            pad_suffix: pad_suffix,
        }
    }
}

impl cbor_event::se::Serialize for BootstrapWitness {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(5))?;
        self.vkey.serialize(serializer)?;
        self.signature.serialize(serializer)?;
        serializer.write_bytes(&self.chain_code)?;
        serializer.write_bytes(&self.pad_prefix)?;
        serializer.write_bytes(&self.pad_suffix)?;
        Ok(serializer)
    }
}

impl Deserialize for BootstrapWitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("BootstrapWitness"))
    }
}

impl DeserializeEmbeddedGroup for BootstrapWitness {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let vkey = (|| -> Result<_, DeserializeError> {
            Ok(Vkey::deserialize(raw)?)
        })().map_err(|e| e.annotate("vkey"))?;
        let signature = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519Signature::deserialize(raw)?)
        })().map_err(|e| e.annotate("signature"))?;
        let chain_code = (|| -> Result<_, DeserializeError> {
            Ok(raw.bytes()?)
        })().map_err(|e| e.annotate("chain_code"))?;
        let pad_prefix = (|| -> Result<_, DeserializeError> {
            Ok(raw.bytes()?)
        })().map_err(|e| e.annotate("pad_prefix"))?;
        let pad_suffix = (|| -> Result<_, DeserializeError> {
            Ok(raw.bytes()?)
        })().map_err(|e| e.annotate("pad_suffix"))?;
        Ok(BootstrapWitness {
            vkey,
            signature,
            chain_code,
            pad_prefix,
            pad_suffix,
        })
    }
}


#[wasm_bindgen]
#[derive(Clone)]
pub struct BootstrapWitnesses(Vec<BootstrapWitness>);

#[wasm_bindgen]
impl BootstrapWitnesses {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> BootstrapWitness {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &BootstrapWitness) {
        self.0.push(elem.clone());
    }
}

impl cbor_event::se::Serialize for BootstrapWitnesses {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for BootstrapWitnesses {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == cbor_event::Type::Special {
                    assert_eq!(raw.special()?, cbor_event::Special::Break);
                    break;
                }
                arr.push(BootstrapWitness::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("BootstrapWitnesses"))?;
        Ok(Self(arr))
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
        #[derive(Clone)]
        pub struct $name(crypto::Signature<$signee_type, $verifier_type>);

        #[wasm_bindgen]
        impl $name {
            pub fn to_bytes(&self) -> Vec<u8> {
                self.0.as_ref().to_vec()
            }

            pub fn to_bech32(&self) -> String {
                self.0.to_bech32_str()
            }

            pub fn to_hex(&self) -> String {
                hex::encode(&self.0.as_ref())
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

        from_bytes!($name, bytes, {
            crypto::Signature::from_binary(bytes.as_ref())
                .map_err(|e| DeserializeError::new(stringify!($name), DeserializeFailure::SignatureError(e)))
                .map($name)
        });

        impl cbor_event::se::Serialize for $name {
            fn serialize<'se, W: std::io::Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
                serializer.write_bytes(self.0.as_ref())
            }
        }
        
        impl Deserialize for $name {
            fn deserialize<R: std::io::BufRead>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
                Ok(Self(crypto::Signature::from_binary(raw.bytes()?.as_ref())?))
            }
        }
    };
}

impl_signature!(Ed25519Signature, Vec<u8>, crypto::Ed25519);
macro_rules! impl_hash_type {
    ($name:ident, $byte_count:expr) => {
        #[wasm_bindgen]
        #[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $name(pub (crate) [u8; $byte_count]);

        #[wasm_bindgen]
        impl $name {
            pub fn to_bytes(&self) -> Vec<u8> {
                self.0.to_vec()
            }
        }

        from_bytes!($name, bytes, {
            use std::convert::TryInto;
            match bytes.len() {
                $byte_count => Ok($name(bytes[..$byte_count].try_into().unwrap())),
                other_len => {
                    let cbor_error = cbor_event::Error::WrongLen($byte_count, cbor_event::Len::Len(other_len as u64), "hash length");
                    Err(DeserializeError::new(stringify!($name), DeserializeFailure::CBOR(cbor_error)))
                },
            }
        });

        // associated consts are not supported in wasm_bindgen
        impl $name {
            pub const BYTE_COUNT: usize = $byte_count;
        }

        // can't expose [T; N] to wasm for new() but it's useful internally so we implement From trait
        impl From<[u8; $byte_count]> for $name {
            fn from(bytes: [u8; $byte_count]) -> Self {
                Self(bytes)
            }
        }

        impl cbor_event::se::Serialize for $name {
            fn serialize<'se, W: std::io::Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
                serializer.write_bytes(self.0)
            }
        }
        
        impl Deserialize for $name {
            fn deserialize<R: std::io::BufRead>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
                use std::convert::TryInto;
                (|| -> Result<Self, DeserializeError> {
                    let bytes = raw.bytes()?;
                    if bytes.len() != $byte_count {
                        return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen($byte_count, cbor_event::Len::Len(bytes.len() as u64), "hash length")).into());
                    }
                    Ok($name(bytes[..$byte_count].try_into().unwrap()))
                })().map_err(|e| e.annotate(stringify!($name)))
            }
        }
    }
}


#[wasm_bindgen]
pub struct LegacyDaedalusPrivateKey(pub (crate) crypto::SecretKey<crypto::LegacyDaedalus>);

#[wasm_bindgen]
impl LegacyDaedalusPrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<LegacyDaedalusPrivateKey, JsValue> {
        crypto::SecretKey::<crypto::LegacyDaedalus>::from_binary(bytes)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
            .map(LegacyDaedalusPrivateKey)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn chaincode(&self) -> Vec<u8> {
        let ED25519_PRIVATE_KEY_LENGTH = 64;
        let XPRV_SIZE = 96;
        self.0.as_ref()[ED25519_PRIVATE_KEY_LENGTH..XPRV_SIZE].to_vec()
    }
}

impl_hash_type!(Ed25519KeyHash, 28);
impl_hash_type!(ScriptHash, 28);
impl_hash_type!(TransactionHash, 32);
impl_hash_type!(GenesisDelegateHash, 28);
impl_hash_type!(GenesisHash, 28);
impl_hash_type!(MetadataHash, 32);
impl_hash_type!(VRFKeyHash, 28);