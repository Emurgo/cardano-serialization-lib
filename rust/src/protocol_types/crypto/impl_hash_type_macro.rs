#[macro_export]
macro_rules! impl_hash_type {
    ($name:ident, $byte_count:expr) => {
        #[wasm_bindgen]
        #[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(pub(crate) [u8; $byte_count]);

        // hash types are the only types in this library to not expect the entire CBOR structure.
        // There is no CBOR binary tag here just the raw hash bytes.
        from_bytes!($name, bytes, {
            use std::convert::TryInto;
            match bytes.len() {
                $byte_count => Ok($name(bytes[..$byte_count].try_into().unwrap())),
                other_len => {
                    let cbor_error = cbor_event::Error::WrongLen(
                        $byte_count,
                        cbor_event::Len::Len(other_len as u64),
                        "hash length",
                    );
                    Err(DeserializeError::new(
                        stringify!($name),
                        DeserializeFailure::CBOR(cbor_error),
                    ))
                }
            }
        });

        #[wasm_bindgen]
        impl $name {
            // hash types are the only types in this library to not give the entire CBOR structure.
            // There is no CBOR binary tag here just the raw hash bytes.
            pub fn to_bytes(&self) -> Vec<u8> {
                self.0.to_vec()
            }

            pub fn to_bech32(&self, prefix: &str) -> Result<String, JsError> {
                use bech32::ToBase32;
                bech32::encode(&prefix, self.to_bytes().to_base32())
                    .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
            }

            pub fn from_bech32(bech_str: &str) -> Result<$name, JsError> {
                let (_hrp, u5data) =
                    bech32::decode(bech_str).map_err(|e| JsError::from_str(&e.to_string()))?;
                let data: Vec<u8> = bech32::FromBase32::from_base32(&u5data).unwrap();
                Ok(Self::from_bytes(data)?)
            }

            pub fn to_hex(&self) -> String {
                hex::encode(&self.0)
            }

            pub fn from_hex(hex: &str) -> Result<$name, JsError> {
                let bytes = hex::decode(hex)
                    .map_err(|e| JsError::from_str(&format!("hex decode failed: {}", e)))?;
                Self::from_bytes(bytes).map_err(|e| JsError::from_str(&format!("{:?}", e)))
            }
        }

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
            fn serialize<'se, W: std::io::Write>(
                &self,
                serializer: &'se mut Serializer<W>,
            ) -> cbor_event::Result<&'se mut Serializer<W>> {
                serializer.write_bytes(self.0)
            }
        }

        impl Deserialize for $name {
            fn deserialize<R: std::io::BufRead>(
                raw: &mut Deserializer<R>,
            ) -> Result<Self, DeserializeError> {
                use std::convert::TryInto;
                (|| -> Result<Self, DeserializeError> {
                    let bytes = raw.bytes()?;
                    if bytes.len() != $byte_count {
                        return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                            $byte_count,
                            cbor_event::Len::Len(bytes.len() as u64),
                            "hash length",
                        ))
                        .into());
                    }
                    Ok($name(bytes[..$byte_count].try_into().unwrap()))
                })()
                .map_err(|e| e.annotate(stringify!($name)))
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
                        &"hex bytes for hash",
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

        impl Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.to_hex())
            }
        }
    };
}