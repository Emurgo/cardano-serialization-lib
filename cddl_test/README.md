# Experimental proof of concept for generating rust code for CBOR serialization from CDDL specs.

### Purpose ###

This would be used to get baseline code for a library we could compile to web-asm for use in Yoroi, Tangata, etc.
It would have the added benefit of being more prepared for the future when/if new CDDL specs are used.
It is highly experimental as we are not sure if the effort to get it working will be worth the development time,
or if another approach would be better.

### Current capacities

TODO: list of features

Generates a `/export/` folder with wasm-compilable rust code (including Cargo.toml, etc) which can then be compiled with `wasm-pack build`.

The `supported.cddl` file contains all supported features thus far and outputs the following code:
```rust
use std::io::Write;
use wasm_bindgen::prelude::*;

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cardano-serialization-lib/tree/master/cddl_test

use cbor_event::{self, de::{Deserialize, Deserializer}, se::{Serialize, Serializer}};

struct Array<T>(Vec<T>);

impl<T> std::ops::Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
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

type bar = u64;

#[wasm_bindgen]

pub struct foo {
    group: groups::foo,
}

impl cbor_event::se::Serialize for foo {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_map(serializer)
    }
}

#[wasm_bindgen]

impl foo {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(x: u64, z: u64, key_0: String) -> foo {
        foo {
            group: groups::foo::new(Some(x.into()), Some(z.into()), Some(key_0.into()))
        }
    }
}

#[wasm_bindgen]

pub struct block {
    group: groups::block,
}

impl cbor_event::se::Serialize for block {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_map(serializer)
    }
}

#[wasm_bindgen]

impl block {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(header: u64) -> block {
        block {
            group: groups::block::new(Some(header.into()))
        }
    }
}

#[wasm_bindgen]

pub struct mapper {
    group: groups::mapper,
}

impl cbor_event::se::Serialize for mapper {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_map(serializer)
    }
}

#[wasm_bindgen]

impl mapper {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }
}

type hash = Bytes;

type keyhash = hash;

type scripthash = hash;

#[wasm_bindgen]

pub struct address {
    group: groups::address,
}

impl cbor_event::se::Serialize for address {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

#[wasm_bindgen]

impl address {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new_address0(index_1: Vec<u8>, index_2: Vec<u8>) -> Self {
        address {
            group: groups::address::address0(groups::address0::new(Some(index_1.into()), Some(index_2.into())))
        }
    }

    pub fn new_address1(index_1: Vec<u8>, index_2: Vec<u8>) -> Self {
        address {
            group: groups::address::address1(groups::address1::new(Some(index_1.into()), Some(index_2.into())))
        }
    }

    pub fn new_address2(index_1: Vec<u8>, index_2: Vec<u8>) -> Self {
        address {
            group: groups::address::address2(groups::address2::new(Some(index_1.into()), Some(index_2.into())))
        }
    }

    pub fn new_address3(index_1: Vec<u8>, index_2: Vec<u8>) -> Self {
        address {
            group: groups::address::address3(groups::address3::new(Some(index_1.into()), Some(index_2.into())))
        }
    }

    pub fn new_address4(index_1: Vec<u8>) -> Self {
        address {
            group: groups::address::address4(groups::address4::new(Some(index_1.into())))
        }
    }

    pub fn new_address5(index_1: Vec<u8>) -> Self {
        address {
            group: groups::address::address5(groups::address5::new(Some(index_1.into())))
        }
    }

    pub fn new_address6(index_1: Vec<u8>) -> Self {
        address {
            group: groups::address::address6(groups::address6::new(Some(index_1.into())))
        }
    }
}

#[wasm_bindgen]

pub struct transaction_input {
    group: groups::transaction_input,
}

impl cbor_event::se::Serialize for transaction_input {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

#[wasm_bindgen]

impl transaction_input {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(transaction_id: Vec<u8>, index: u64) -> transaction_input {
        transaction_input {
            group: groups::transaction_input::new(Some(transaction_id.into()), Some(index.into()))
        }
    }
}

#[wasm_bindgen]

pub struct transaction_output {
    group: groups::transaction_output,
}

impl cbor_event::se::Serialize for transaction_output {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

#[wasm_bindgen]

impl transaction_output {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(address: String, amount: u64) -> transaction_output {
        transaction_output {
            group: groups::transaction_output::new(Some(address.into()), Some(amount.into()))
        }
    }
}

mod groups {
    use super::*;

    pub (super) struct foo {
        x: Option<bar>,
        z: Option<u64>,
        key_0: Option<String>,
    }

    impl foo {
        pub (super) fn new(x: Option<bar>, z: Option<u64>, key_0: Option<String>) -> Self {
            foo {
                x: x,
                z: z,
                key_0: key_0,
            }
        }

        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            self.x.clone().unwrap().serialize(serializer)?;
            self.z.clone().unwrap().serialize(serializer)?;
            self.key_0.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }

        pub (super) fn serialize_as_map<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_text("x")?;
            self.x.clone().unwrap().serialize(serializer)?;
            serializer.write_text("z")?;
            self.z.clone().unwrap().serialize(serializer)?;
            serializer.write_unsigned_integer(0)?;
            self.key_0.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }
    }

    pub (super) struct block {
        header: Option<u64>,
    }

    impl block {
        pub (super) fn new(header: Option<u64>) -> Self {
            block {
                header: header,
            }
        }

        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(1u64))?;
            self.header.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }

        pub (super) fn serialize_as_map<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(1u64))?;
            serializer.write_text("header")?;
            self.header.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }
    }

    pub (super) struct mapper {
        table: std::collections::BTreeMap<String, u64>,
    }

    impl mapper {
        pub (super) fn serialize_as_map<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            panic!("TODO: implement");
        }
    }

    pub (super) struct address0 {
        index_1: Option<keyhash>,
        index_2: Option<keyhash>,
    }

    impl address0 {
        pub (super) fn new(index_1: Option<keyhash>, index_2: Option<keyhash>) -> Self {
            address0 {
                index_1: index_1,
                index_2: index_2,
            }
        }

        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_unsigned_integer(0)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            self.index_2.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }
    }

    pub (super) struct address1 {
        index_1: Option<keyhash>,
        index_2: Option<scripthash>,
    }

    impl address1 {
        pub (super) fn new(index_1: Option<keyhash>, index_2: Option<scripthash>) -> Self {
            address1 {
                index_1: index_1,
                index_2: index_2,
            }
        }

        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_unsigned_integer(1)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            self.index_2.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }
    }

    pub (super) struct address2 {
        index_1: Option<scripthash>,
        index_2: Option<keyhash>,
    }

    impl address2 {
        pub (super) fn new(index_1: Option<scripthash>, index_2: Option<keyhash>) -> Self {
            address2 {
                index_1: index_1,
                index_2: index_2,
            }
        }

        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_unsigned_integer(2)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            self.index_2.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }
    }

    pub (super) struct address3 {
        index_1: Option<scripthash>,
        index_2: Option<scripthash>,
    }

    impl address3 {
        pub (super) fn new(index_1: Option<scripthash>, index_2: Option<scripthash>) -> Self {
            address3 {
                index_1: index_1,
                index_2: index_2,
            }
        }

        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_unsigned_integer(3)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            self.index_2.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }
    }

    pub (super) struct address4 {
        index_1: Option<keyhash>,
    }

    impl address4 {
        pub (super) fn new(index_1: Option<keyhash>) -> Self {
            address4 {
                index_1: index_1,
            }
        }

        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(2u64))?;
            serializer.write_unsigned_integer(6)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }
    }

    pub (super) struct address5 {
        index_1: Option<scripthash>,
    }

    impl address5 {
        pub (super) fn new(index_1: Option<scripthash>) -> Self {
            address5 {
                index_1: index_1,
            }
        }

        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(2u64))?;
            serializer.write_unsigned_integer(7)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }
    }

    pub (super) struct address6 {
        index_1: Option<keyhash>,
    }

    impl address6 {
        pub (super) fn new(index_1: Option<keyhash>) -> Self {
            address6 {
                index_1: index_1,
            }
        }

        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(2u64))?;
            serializer.write_unsigned_integer(8)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }
    }

    pub (super) enum address {
        address0(address0),
        address1(address1),
        address2(address2),
        address3(address3),
        address4(address4),
        address5(address5),
        address6(address6),
    }

    impl address {
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            match self {
                address::address0(x) => x.serialize_as_array(serializer),
                address::address1(x) => x.serialize_as_array(serializer),
                address::address2(x) => x.serialize_as_array(serializer),
                address::address3(x) => x.serialize_as_array(serializer),
                address::address4(x) => x.serialize_as_array(serializer),
                address::address5(x) => x.serialize_as_array(serializer),
                address::address6(x) => x.serialize_as_array(serializer),
            }
        }
    }

    pub (super) struct transaction_input {
        transaction_id: Option<hash>,
        index: Option<u64>,
    }

    impl transaction_input {
        pub (super) fn new(transaction_id: Option<hash>, index: Option<u64>) -> Self {
            transaction_input {
                transaction_id: transaction_id,
                index: index,
            }
        }

        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(2u64))?;
            self.transaction_id.clone().unwrap().serialize(serializer)?;
            self.index.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }

        pub (super) fn serialize_as_map<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(2u64))?;
            serializer.write_text("transaction_id")?;
            self.transaction_id.clone().unwrap().serialize(serializer)?;
            serializer.write_text("index")?;
            self.index.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }
    }

    pub (super) struct transaction_output {
        address: Option<String>,
        amount: Option<u64>,
    }

    impl transaction_output {
        pub (super) fn new(address: Option<String>, amount: Option<u64>) -> Self {
            transaction_output {
                address: address,
                amount: amount,
            }
        }

        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(2u64))?;
            self.address.clone().unwrap().serialize(serializer)?;
            self.amount.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }

        pub (super) fn serialize_as_map<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(2u64))?;
            serializer.write_text("address")?;
            self.address.clone().unwrap().serialize(serializer)?;
            serializer.write_text("amount")?;
            self.amount.clone().unwrap().serialize(serializer)?;
            Ok(serializer)
        }
    }
}
```

The `Option<T>` types are to deal with polymorphic member types on maps but when not necessary
it could be possible to remove them. They are not necessary in any of the code above, but are
required to support stuff in `shelley.cddl` such as:
```cddl
transaction_body =
  { 4 : coin ; fee
  , 5 : uint ; ttl
  , 6 : full_update
  , 7 : metadata_hash
  }
```

The clones will be removed if possible.
The groups module will define the base representation of any group, and a wrapper type for either map or array will be defined for external use (wasm exposure).
