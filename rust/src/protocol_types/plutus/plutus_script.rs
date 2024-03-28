use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusScript {
    pub(crate) bytes: Vec<u8>,
    pub(crate) language: LanguageKind,
}

to_from_bytes!(PlutusScript);

#[wasm_bindgen]
impl PlutusScript {
    /**
     * Creates a new Plutus script from the RAW bytes of the compiled script.
     * This does NOT include any CBOR encoding around these bytes (e.g. from "cborBytes" in cardano-cli)
     * If you creating this from those you should use PlutusScript::from_bytes() instead.
     */
    pub fn new(bytes: Vec<u8>) -> PlutusScript {
        Self::new_with_version(bytes, &Language::new_plutus_v1())
    }

    /**
     * Creates a new Plutus script from the RAW bytes of the compiled script.
     * This does NOT include any CBOR encoding around these bytes (e.g. from "cborBytes" in cardano-cli)
     * If you creating this from those you should use PlutusScript::from_bytes() instead.
     */
    pub fn new_v2(bytes: Vec<u8>) -> PlutusScript {
        Self::new_with_version(bytes, &Language::new_plutus_v2())
    }

    /**
     * Creates a new Plutus script from the RAW bytes of the compiled script.
     * This does NOT include any CBOR encoding around these bytes (e.g. from "cborBytes" in cardano-cli)
     * If you creating this from those you should use PlutusScript::from_bytes() instead.
     */
    pub fn new_v3(bytes: Vec<u8>) -> PlutusScript {
        Self::new_with_version(bytes, &Language::new_plutus_v3())
    }

    /**
     * Creates a new Plutus script from the RAW bytes of the compiled script.
     * This does NOT include any CBOR encoding around these bytes (e.g. from "cborBytes" in cardano-cli)
     * If you creating this from those you should use PlutusScript::from_bytes() instead.
     */
    pub fn new_with_version(bytes: Vec<u8>, language: &Language) -> PlutusScript {
        Self {
            bytes,
            language: language.0.clone(),
        }
    }

    /**
     * The raw bytes of this compiled Plutus script.
     * If you need "cborBytes" for cardano-cli use PlutusScript::to_bytes() instead.
     */
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    /// Same as `.from_bytes` but will consider the script as requiring the Plutus Language V2
    pub fn from_bytes_v2(bytes: Vec<u8>) -> Result<PlutusScript, JsError> {
        Self::from_bytes_with_version(bytes, &Language::new_plutus_v2())
    }

    /// Same as `.from_bytes` but will consider the script as requiring the Plutus Language V3
    pub fn from_bytes_v3(bytes: Vec<u8>) -> Result<PlutusScript, JsError> {
        Self::from_bytes_with_version(bytes, &Language::new_plutus_v3())
    }

    /// Same as `.from_bytes` but will consider the script as requiring the specified language version
    pub fn from_bytes_with_version(
        bytes: Vec<u8>,
        language: &Language,
    ) -> Result<PlutusScript, JsError> {
        Ok(Self::new_with_version(
            Self::from_bytes(bytes)?.bytes,
            language,
        ))
    }

    /// Same as .from_hex but will consider the script as requiring the specified language version
    pub fn from_hex_with_version(
        hex_str: &str,
        language: &Language,
    ) -> Result<PlutusScript, JsError> {
        Ok(Self::new_with_version(
            Self::from_hex(hex_str)?.bytes,
            language,
        ))
    }

    pub fn hash(&self) -> ScriptHash {
        let mut bytes = Vec::with_capacity(self.bytes.len() + 1);
        // https://github.com/input-output-hk/cardano-ledger/blob/master/eras/babbage/test-suite/cddl-files/babbage.cddl#L413
        bytes.extend_from_slice(&vec![self.script_namespace() as u8]);
        bytes.extend_from_slice(&self.bytes);
        ScriptHash::from(blake2b224(bytes.as_ref()))
    }

    pub fn language_version(&self) -> Language {
        Language(self.language.clone())
    }

    pub(crate) fn script_namespace(&self) -> ScriptHashNamespace {
        match self.language {
            LanguageKind::PlutusV1 => ScriptHashNamespace::PlutusScript,
            LanguageKind::PlutusV2 => ScriptHashNamespace::PlutusScriptV2,
            LanguageKind::PlutusV3 => ScriptHashNamespace::PlutusScriptV3,
        }
    }

    pub(crate) fn clone_as_version(&self, language: &Language) -> PlutusScript {
        Self::new_with_version(self.bytes.clone(), language)
    }
}

impl serde::Serialize for PlutusScript {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.bytes))
    }
}

impl<'de> serde::de::Deserialize<'de> for PlutusScript {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        hex::decode(&s)
            .map(|bytes| PlutusScript::new(bytes))
            .map_err(|_err| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&s),
                    &"PlutusScript as hex string e.g. F8AB28C2 (without CBOR bytes tag)",
                )
            })
    }
}

impl JsonSchema for PlutusScript {
    fn schema_name() -> String {
        String::from("PlutusScript")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}