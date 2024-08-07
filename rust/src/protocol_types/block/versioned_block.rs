use crate::*;

#[wasm_bindgen]
#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum BlockEra {
    Byron,
    Shelley,
    Allegra,
    Mary,
    Alonzo,
    Babbage,
    Conway,
    Unknown
}

#[wasm_bindgen]
#[derive(Clone, Eq, Debug, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct VersionedBlock {
    pub(crate) era_code: u32,
    pub(crate) block: Block,
}

impl_to_from!(VersionedBlock);

#[wasm_bindgen]
impl VersionedBlock {
    pub fn new(block: Block, era_code: u32) -> VersionedBlock {
        VersionedBlock {
            block,
            era_code,
        }
    }

    pub fn block(&self) -> Block {
        self.block.clone()
    }

    pub fn era(&self) -> BlockEra {
        match self.era_code {
            0 => BlockEra::Byron,
            1 => BlockEra::Byron,
            2 => BlockEra::Shelley,
            3 => BlockEra::Allegra,
            4 => BlockEra::Mary,
            5 => BlockEra::Alonzo,
            6 => BlockEra::Babbage,
            7 => BlockEra::Conway,
            _ => BlockEra::Unknown,
        }
    }
}
