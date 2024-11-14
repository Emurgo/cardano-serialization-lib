use crate::*;

#[derive(
    Debug, Clone, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub(crate) enum ScriptRefEnum {
    NativeScript(NativeScript),
    PlutusScript(PlutusScript),
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct ScriptRef(pub(crate) ScriptRefEnum);

impl_to_from!(ScriptRef);

#[wasm_bindgen]
impl ScriptRef {
    pub fn new_native_script(native_script: &NativeScript) -> Self {
        Self(ScriptRefEnum::NativeScript(native_script.clone()))
    }

    pub fn new_plutus_script(plutus_script: &PlutusScript) -> Self {
        Self(ScriptRefEnum::PlutusScript(plutus_script.clone()))
    }

    pub fn is_native_script(&self) -> bool {
        match &self.0 {
            ScriptRefEnum::NativeScript(_) => true,
            _ => false,
        }
    }

    pub fn is_plutus_script(&self) -> bool {
        match &self.0 {
            ScriptRefEnum::PlutusScript(_) => true,
            _ => false,
        }
    }

    pub fn native_script(&self) -> Option<NativeScript> {
        match &self.0 {
            ScriptRefEnum::NativeScript(native_script) => Some(native_script.clone()),
            _ => None,
        }
    }

    pub fn plutus_script(&self) -> Option<PlutusScript> {
        match &self.0 {
            ScriptRefEnum::PlutusScript(plutus_script) => Some(plutus_script.clone()),
            _ => None,
        }
    }

    /// Return bytes array of script ref struct but without wrapping into CBOR array under the tag
    /// to_bytes returns "#6.24(bytes .cbor script)" from CDDL
    /// to_unwrapped_bytes return "script" from CDDL
    pub fn to_unwrapped_bytes(&self) -> Vec<u8> {
        to_bytes(&self.0)
    }
}