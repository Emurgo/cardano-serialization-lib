use crate::*;

#[wasm_bindgen]
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub enum LanguageKind {
    PlutusV1 = 0,
    PlutusV2 = 1,
    PlutusV3 = 2,
}

impl LanguageKind {
    pub(crate) fn from_u64(x: u64) -> Option<LanguageKind> {
        match x {
            0 => Some(LanguageKind::PlutusV1),
            1 => Some(LanguageKind::PlutusV2),
            2 => Some(LanguageKind::PlutusV3),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct Language(pub(crate) LanguageKind);

impl_to_from!(Language);

#[wasm_bindgen]
impl Language {
    pub fn new_plutus_v1() -> Self {
        Self(LanguageKind::PlutusV1)
    }

    pub fn new_plutus_v2() -> Self {
        Self(LanguageKind::PlutusV2)
    }

    pub fn new_plutus_v3() -> Self {
        Self(LanguageKind::PlutusV3)
    }

    pub fn kind(&self) -> LanguageKind {
        self.0.clone()
    }
}
