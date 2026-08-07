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

/// JSON form: a bare hex string means Plutus V1; V2 and V3 carry the language.
#[derive(JsonSchema)]
#[serde(untagged)]
#[allow(dead_code)]
enum PlutusScriptJson {
    PlutusV1(String),
    Versioned {
        bytes: String,
        language: Language,
    },
}

fn plutus_script_from_hex<E: serde::de::Error>(
    hex_str: &str,
    language: LanguageKind,
) -> Result<PlutusScript, E> {
    hex::decode(hex_str)
        .map(|bytes| PlutusScript { bytes, language })
        .map_err(|_err| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(hex_str),
                &"PlutusScript as hex string e.g. F8AB28C2 (without CBOR bytes tag)",
            )
        })
}

impl serde::Serialize for PlutusScript {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        // Formats reporting is_human_readable() == false take a fixed pair instead.
        if !serializer.is_human_readable() {
            return (&self.bytes, Language(self.language)).serialize(serializer);
        }
        let bytes = hex::encode(&self.bytes);
        match self.language {
            LanguageKind::PlutusV1 => serializer.serialize_str(&bytes),
            language => {
                use serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct("PlutusScript", 2)?;
                state.serialize_field("bytes", &bytes)?;
                state.serialize_field("language", &Language(language))?;
                state.end()
            }
        }
    }
}

struct PlutusScriptVisitor;

impl<'de> serde::de::Visitor<'de> for PlutusScriptVisitor {
    type Value = PlutusScript;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            "a Plutus V1 script as a hex string, or an object with `bytes` and `language`",
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        plutus_script_from_hex(value, LanguageKind::PlutusV1)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
    {
        let mut bytes: Option<String> = None;
        let mut language: Option<Language> = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "bytes" => {
                    if bytes.is_some() {
                        return Err(serde::de::Error::duplicate_field("bytes"));
                    }
                    bytes = Some(map.next_value()?);
                }
                "language" => {
                    if language.is_some() {
                        return Err(serde::de::Error::duplicate_field("language"));
                    }
                    language = Some(map.next_value()?);
                }
                _ => {
                    map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }
        let bytes = bytes.ok_or_else(|| serde::de::Error::missing_field("bytes"))?;
        let language = language.ok_or_else(|| serde::de::Error::missing_field("language"))?;
        plutus_script_from_hex(&bytes, language.kind())
    }
}

impl<'de> serde::de::Deserialize<'de> for PlutusScript {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
    {
        if !deserializer.is_human_readable() {
            let (bytes, language) = <(Vec<u8>, Language)>::deserialize(deserializer)?;
            return Ok(PlutusScript {
                bytes,
                language: language.kind(),
            });
        }
        deserializer.deserialize_any(PlutusScriptVisitor)
    }
}

impl JsonSchema for PlutusScript {
    fn schema_name() -> String {
        String::from("PlutusScript")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        PlutusScriptJson::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        PlutusScriptJson::is_referenceable()
    }
}