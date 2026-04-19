use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CborContainerType {
    Array = 0,
    Map = 1,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum CborSetType {
    #[default]
    Tagged = 0,
    Untagged = 1,
}
