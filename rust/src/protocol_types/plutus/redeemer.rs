use crate::*;

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct Redeemer {
    pub(crate) tag: RedeemerTag,
    pub(crate) index: BigNum,
    pub(crate) data: PlutusData,
    pub(crate) ex_units: ExUnits,
}

impl_to_from!(Redeemer);

#[wasm_bindgen]
impl Redeemer {
    pub fn tag(&self) -> RedeemerTag {
        self.tag.clone()
    }

    pub fn index(&self) -> BigNum {
        self.index.clone()
    }

    pub fn data(&self) -> PlutusData {
        self.data.clone()
    }

    pub fn ex_units(&self) -> ExUnits {
        self.ex_units.clone()
    }

    pub fn new(tag: &RedeemerTag, index: &BigNum, data: &PlutusData, ex_units: &ExUnits) -> Self {
        Self {
            tag: tag.clone(),
            index: index.clone(),
            data: data.clone(),
            ex_units: ex_units.clone(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn clone_with_index(&self, index: &BigNum) -> Self {
        Self {
            tag: self.tag.clone(),
            index: index.clone(),
            data: self.data.clone(),
            ex_units: self.ex_units.clone(),
        }
    }

    pub(crate) fn clone_with_index_and_tag(&self, index: &BigNum, tag: &RedeemerTag) -> Self {
        Self {
            tag: tag.clone(),
            index: index.clone(),
            data: self.data.clone(),
            ex_units: self.ex_units.clone(),
        }
    }
}
