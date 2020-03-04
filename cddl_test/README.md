# Experimental proof of concept for generating rust code for CBOR serialization from CDDL specs.

### Purpose ###

This would be used to get baseline code for a library we could compile to web-asm for use in Yoroi, Tangata, etc.
It would have the added benefit of being more prepared for the future when/if new CDDL specs are used.
It is highly experimental as we are not sure if the effort to get it working will be worth the development time,
or if another approach would be better.

### Current capacities

* Reads type references e.g. `foo = bar`
* Reads root-level maps and generates a struct with all keys as fields.
* Supports root-level maps-as-tables e.g. `foo = { * int => tstr }` [struct only, needs serialization]
* Supports array members (not not root-level yet) e.g. `foo = { bar: [int] }` [struct only, needs serialization]

The `supported.cddl` file contains all supported features thus far and outputs the following code:
```rust
use std::io::Write;
use wasm_bindgen::prelude::*;

use cbor_event::{self, de::{Deserialize, Deserializer}, se::{Serialize, Serializer}};

struct Array<T>(Vec<T>);

impl<T> std::ops::Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

#[derive(Clone)]
struct Bytes(Vec<u8>);

impl Serialize for Bytes {
    fn serialize<'a, W: Write + Sized>(&self, serializer: &'a mut Serializer<W>) -> cbor_event::Result<&'a mut Serializer<W>> {
        serializer.write_bytes(&self.0[..])
    }
}

type bar = u64;

#[wasm_bindgen]

struct foo {
    group: groups::foo,
}

impl cbor_event::se::Serialize for foo {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_map(serializer)
    }
}

#[wasm_bindgen]

impl foo {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }
}

#[wasm_bindgen]

struct block {
    group: groups::block,
}

impl cbor_event::se::Serialize for block {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_map(serializer)
    }
}

#[wasm_bindgen]

impl block {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }
}

#[wasm_bindgen]

struct mapper {
    group: groups::mapper,
}

impl cbor_event::se::Serialize for mapper {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_map(serializer)
    }
}

#[wasm_bindgen]

impl mapper {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }
}

type pointer = u64;

type $hash = Bytes;

type keyhash = $hash;

type scripthash = $hash;

mod groups {
    use super::*;

    pub (super) struct foo {
        bw_x: Option<bar>,
        bw_z: Option<u64>,
        key_0: Option<String>,
    }

    impl foo {
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            self.bw_x.clone().unwrap().serialize(serializer)?;
            self.bw_z.clone().unwrap().serialize(serializer)?;
            self.key_0.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }

        pub (super) fn serialize_as_map<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_text("x")?;
            self.bw_x.clone().unwrap().serialize(serializer)?;
            serializer.write_text("z")?;
            self.bw_z.clone().unwrap().serialize(serializer)?;
            serializer.write_unsigned_integer(0)?;
            self.key_0.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }
    }

    pub (super) struct block {
        bw_header: Option<u64>,
    }

    impl block {
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(1u64))?;
            self.bw_header.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }

        pub (super) fn serialize_as_map<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(1u64))?;
            serializer.write_text("header")?;
            self.bw_header.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
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
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_unsigned_integer(0)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            self.index_2.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }
    }

    pub (super) struct address1 {
        index_1: Option<keyhash>,
        index_2: Option<scripthash>,
    }

    impl address1 {
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_unsigned_integer(1)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            self.index_2.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }
    }

    pub (super) struct address2 {
        index_1: Option<scripthash>,
        index_2: Option<keyhash>,
    }

    impl address2 {
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_unsigned_integer(2)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            self.index_2.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }
    }

    pub (super) struct address3 {
        index_1: Option<scripthash>,
        index_2: Option<scripthash>,
    }

    impl address3 {
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_unsigned_integer(3)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            self.index_2.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }
    }

    pub (super) struct address4 {
        index_1: Option<keyhash>,
        index_2: Option<pointer>,
    }

    impl address4 {
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_unsigned_integer(4)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            self.index_2.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }
    }

    pub (super) struct address5 {
        index_1: Option<scripthash>,
        index_2: Option<pointer>,
    }

    impl address5 {
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(3u64))?;
            serializer.write_unsigned_integer(5)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            self.index_2.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }
    }

    pub (super) struct address6 {
        index_1: Option<keyhash>,
    }

    impl address6 {
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(2u64))?;
            serializer.write_unsigned_integer(6)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }
    }

    pub (super) struct address7 {
        index_1: Option<scripthash>,
    }

    impl address7 {
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(2u64))?;
            serializer.write_unsigned_integer(7)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }
    }

    pub (super) struct address8 {
        index_1: Option<keyhash>,
    }

    impl address8 {
        pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            serializer.write_array(cbor_event::Len::Len(2u64))?;
            serializer.write_unsigned_integer(8)?;
            self.index_1.clone().unwrap().serialize(serializer)?;
            serializer.write_special(cbor_event::Special::Break)
        }
    }

    enum address {
        address0(address0),
        address1(address1),
        address2(address2),
        address3(address3),
        address4(address4),
        address5(address5),
        address6(address6),
        address7(address7),
        address8(address8),
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
                address::address7(x) => x.serialize_as_array(serializer),
                address::address8(x) => x.serialize_as_array(serializer),
            }
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

### Current issues ###

*The only CDDL library for rust has several issues: Group choices don't parse correctly, optional members seem to be broken in parsing, some more complicated tagged values break parsing. An issue has been opened in the CDDL repo already.
