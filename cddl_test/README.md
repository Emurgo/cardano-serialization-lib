# Experimental proof of concept for generating rust code for CBOR serialization from CDDL specs.

### Purpose ###

This would be used to get baseline code for a library we could compile to web-asm for use in Yoroi, Tangata, etc.
It would have the added benefit of being more prepared for the future when/if new CDDL specs are used.
It is highly experimental as we are not sure if the effort to get it working will be worth the development time,
or if another approach would be better.

### Current capacities

* Reads type references e.g. `foo = bar`
* Reads root-level maps and generates a struct with all keys as fields.
* Supports root-level maps-as-tables e.g. `foo = { * int => tstr }`
* Supports array members (not not root-level yet) e.g. `foo = { bar: [int] }`

The `supported.cddl` file contains all supported features thus far and outputs the following code:
```rust
type bar = int;

struct foo {
    x: Option<bar>,
    y: Option<Vec<Vec<i32>>>,
    z: Option<i32>,
    value_0: Option<String>,
}

struct block {
    header: Option<i32>,
    bodies: Option<Vec<body>>,
}

struct body {
    txs: Option<Vec<i32>>,
}

struct mapper {
    table: std::collections::BTreeMap<String, i32>,
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

### Current issues ###

*The only CDDL library for rust has several issues: Group choices don't parse correctly, optional members seem to be broken in parsing, some more complicated tagged values break parsing. An issue has been opened in the CDDL repo already.
