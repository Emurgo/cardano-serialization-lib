use cbor_event::{self, de::{Deserialize, Deserializer}, se::{Serialize, Serializer}};
use std::io::Write;
use wasm_bindgen::prelude::*;

// TODO: handle this by not passing throught be barrior directly.
//       instead have it either:
//  1) generate 1 per each wrapped type
//  2) don't pass it, instead wrap it/unwrap it automatically
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TaggedData<T> {
    pub (crate) data: T,
    pub (crate) tag: u64,
}

impl<T> TaggedData<T> {
    pub fn new(data: T, tag: u64) -> Self {
        Self {
            data,
            tag,
        }
    }
}

impl<T: Serialize> Serialize for TaggedData<T> {
    fn serialize<'a, W: Write + Sized>(&self, serializer: &'a mut Serializer<W>) -> cbor_event::Result<&'a mut Serializer<W>> {
        serializer.write_tag(self.tag)?;
        self.data.serialize(serializer)
    }
}


#[wasm_bindgen]
#[derive(Clone)]
pub struct Bytes(Vec<u8>);

#[wasm_bindgen]
impl Bytes {
    pub fn new(data: &[u8]) -> Self {
        Self(data.into())
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl Serialize for Bytes {
    fn serialize<'a, W: Write + Sized>(&self, serializer: &'a mut Serializer<W>) -> cbor_event::Result<&'a mut Serializer<W>> {
        serializer.write_bytes(&self.0[..])
    }
}