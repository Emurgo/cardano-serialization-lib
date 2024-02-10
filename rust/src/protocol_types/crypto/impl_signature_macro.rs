#[macro_export]
macro_rules! impl_signature {
    ($name:ident, $signee_type:ty, $verifier_type:ty) => {
        #[wasm_bindgen]
        #[derive(Clone, Debug, Hash, Eq, PartialEq)]
        pub struct $name(pub (crate) chain_crypto::Signature<$signee_type, $verifier_type>);

        #[wasm_bindgen]
        impl $name {
            pub fn to_bytes(&self) -> Vec<u8> {
                self.0.as_ref().to_vec()
            }

            pub fn to_bech32(&self) -> String {
                use crate::chain_crypto::bech32::Bech32;
                self.0.to_bech32_str()
            }

            pub fn to_hex(&self) -> String {
                hex::encode(&self.0.as_ref())
            }

            pub fn from_bech32(bech32_str: &str) -> Result<$name, JsError> {
                use crate::chain_crypto::bech32::Bech32;
                chain_crypto::Signature::try_from_bech32_str(&bech32_str)
                    .map($name)
                    .map_err(|e| JsError::from_str(&format!("{}", e)))
            }

            pub fn from_hex(input: &str) -> Result<$name, JsError> {
                chain_crypto::Signature::from_str(input)
                    .map_err(|e| JsError::from_str(&format!("{:?}", e)))
                    .map($name)
            }
        }

        from_bytes!($name, bytes, {
            chain_crypto::Signature::from_binary(bytes.as_ref())
                .map_err(|e| {
                    DeserializeError::new(stringify!($name), DeserializeFailure::SignatureError(e))
                })
                .map($name)
        });

        impl cbor_event::se::Serialize for $name {
            fn serialize<'se, W: std::io::Write>(
                &self,
                serializer: &'se mut Serializer<W>,
            ) -> cbor_event::Result<&'se mut Serializer<W>> {
                serializer.write_bytes(self.0.as_ref())
            }
        }

        impl Deserialize for $name {
            fn deserialize<R: std::io::BufRead>(
                raw: &mut Deserializer<R>,
            ) -> Result<Self, DeserializeError> {
                Ok(Self(chain_crypto::Signature::from_binary(raw.bytes()?.as_ref())?))
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_hex())
            }
        }

        impl<'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
                $name::from_hex(&s).map_err(|_e| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(&s),
                        &"hex bytes for signature",
                    )
                })
            }
        }

        impl JsonSchema for $name {
            fn schema_name() -> String {
                String::from(stringify!($name))
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                String::json_schema(gen)
            }
            fn is_referenceable() -> bool {
                String::is_referenceable()
            }
        }
    };
}