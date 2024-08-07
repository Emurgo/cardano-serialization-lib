use crate::*;

#[wasm_bindgen]
#[derive(Clone, Eq, Debug, PartialEq)]
/// Warning: This is experimental and may be removed in the future.
pub struct FixedVersionedBlock {
    pub(crate) block: FixedBlock,
    pub(crate) era_code: u32,
}

from_bytes!(FixedVersionedBlock);

#[wasm_bindgen]
impl FixedVersionedBlock {
    pub fn block(&self) -> FixedBlock {
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