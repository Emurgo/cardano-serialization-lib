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

use cbor_event::{self, de::Deserializer, se::Serializer};

type bar = u64;

mod groups {
    use super::*;

    struct foo {
        x: Option<bar>,
        z: Option<u64>,
        value_0: Option<String>,
    }

    impl cbor_event::se::Serialize for foo {
        fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            self.x.clone().unwrap().serialize(serializer);
            self.z.clone().unwrap().serialize(serializer);
            self.value_0.clone().unwrap().serialize(serializer)
        }
    }

    struct block {
        header: Option<u64>,
    }

    impl cbor_event::se::Serialize for block {
        fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
            self.header.clone().unwrap().serialize(serializer)
        }
    }

    struct mapper {
        table: std::collections::BTreeMap<String, u64>,
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
