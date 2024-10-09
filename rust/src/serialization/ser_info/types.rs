use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CborContainerType {
    Array = 0,
    Map = 1,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CborSetType {
    Tagged = 0,
    Untagged = 1,
}