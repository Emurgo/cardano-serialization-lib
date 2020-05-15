use std::io::{BufRead, Write};
use wasm_bindgen::prelude::*;

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::{self, de::{Deserializer}, se::{Serialize, Serializer}};

mod address;
mod serialization;
mod js_chain_libs;
mod prelude;

use address::Address;

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Keyhash([u8; 28]);

#[wasm_bindgen]
impl Keyhash {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    fn new(data: [u8; 28]) -> Self {
        Self(data)
    }
}


#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Scripthash([u8; 28]);

#[wasm_bindgen]
impl Scripthash {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    fn new(data: [u8; 28]) -> Self {
        Self(data)
    }
}