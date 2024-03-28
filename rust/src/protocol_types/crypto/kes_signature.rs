use crate::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KESSignature(pub(crate) Vec<u8>);

#[wasm_bindgen]
impl KESSignature {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

// associated consts are not supported in wasm_bindgen
impl KESSignature {
    pub const BYTE_COUNT: usize = 448;
}

from_bytes!(KESSignature, bytes, {
    match bytes.len() {
        Self::BYTE_COUNT => Ok(KESSignature(bytes)),
        other_len => {
            let cbor_error = cbor_event::Error::WrongLen(
                Self::BYTE_COUNT as u64,
                cbor_event::Len::Len(other_len as u64),
                "hash length",
            );
            Err(DeserializeError::new(
                "KESSignature",
                DeserializeFailure::CBOR(cbor_error),
            ))
        }
    }
});

impl serde::Serialize for KESSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(self.to_bytes()))
    }
}

impl<'de> serde::de::Deserialize<'de> for KESSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        if let Ok(hex_bytes) = hex::decode(s.clone()) {
            if let Ok(sig) = KESSignature::from_bytes(hex_bytes) {
                return Ok(sig);
            }
        }
        Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(&s),
            &"hex bytes for KESSignature",
        ))
    }
}

impl JsonSchema for KESSignature {
    fn schema_name() -> String {
        String::from("KESSignature")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}