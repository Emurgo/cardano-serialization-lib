# Metadata

## Shelley Metadata format

Shelley metadata takes the form of a map of metadatums, which are recursive JSON-like structures.

It is defined in [CDDL](https://tools.ietf.org/html/rfc8610), a schema grammar for representing [CBOR](https://tools.ietf.org/html/rfc7049) binary encoding as:
```
transaction_metadatum =
    { * transaction_metadatum => transaction_metadatum }
  / [ * transaction_metadatum ]
  / int
  / bytes .size (0..64)
  / text .size (0..64)

transaction_metadatum_label = uint

transaction_metadata =
  { * transaction_metadatum_label => transaction_metadatum }
```

For each use we use a metadatum label specific to our use into the `TransactionMetadatum` map. If we had a JSON object such as
```json
{
  "receiver_id": "SJKdj34k3jjKFDKfjFUDfdjkfd",
  "sender_id": "jkfdsufjdk34h3Sdfjdhfduf873",
  "comment": "happy birthday",
  "tags": [0, 264, -1024, 32]
}
```

There are 4 ways we can achieve this with different trade-offs: Directly using the Metadata-related structures used in the library, conversion to/from JSON using our utility functions, writing a CDDL spec of this structure that is representable by that recursive metadatum CDDL, or encoding raw-bytes using our utility functions. Each section will give examples of how to encode a similar structure. Understanding CDDL is only necessary for the last 2 options, but it is fairly simple to understand.

## Direct use

Upsides:
* Flexible
* Readable by other methods

Downsides:
* Can be quite tedious to write
* Structural validation must be done by hand (partially)

As the metadatum structure is fairly expressive, we can directly use it using the structs in the metadata module of this library. These directly represent the types given in the CDDL. Namely:
* TransactionMetadatum - Can represent one of those 5 variant types.
* MetadataMap - The map variant that maps from metadatums to other metadatums. This is unordered and indexed by metadatums. This is like an object in JSON.
* MetadataList - An ordered list indexed starting at 0. This is like an array in JSON.

The variants for numbers, bytes and text are not specific to metadata and are directly used with the general `Int` type representing a signed or unsigned number, byte arrays accepting byte arrays/`Buffer`, and strings being JS strings.

We could construct the JSON example above with the following code:
```javascript
const map = CardanoWasm.MetadataMap.new();
map.insert(
  CardanoWasm.TransactionMetadatum.new_text("receiver_id"),
  CardanoWasm.TransactionMetadatum.new_text("SJKdj34k3jjKFDKfjFUDfdjkfd"),
);
map.insert(
  CardanoWasm.TransactionMetadatum.new_text("sender_id"),
  CardanoWasm.TransactionMetadatum.new_text("jkfdsufjdk34h3Sdfjdhfduf873"),
);
map.insert(
  CardanoWasm.TransactionMetadatum.new_text("comment"),
  CardanoWasm.TransactionMetadatum.new_text("happy birthday"),
);
const tags = CardanoWasm.MetadataList.new();
tags.add(CardanoWasm.TransactionMetadatum.new_int(CardanoWasm.Int.new(CardanoWasm.BigNum.from_str("0"))));
tags.add(CardanoWasm.TransactionMetadatum.new_int(CardanoWasm.Int.new(CardanoWasm.BigNum.from_str("264"))));
tags.add(CardanoWasm.TransactionMetadatum.new_int(CardanoWasm.Int.new_negative(CardanoWasm.BigNum.from_str("1024"))));
tags.add(CardanoWasm.TransactionMetadatum.new_int(CardanoWasm.Int.new(CardanoWasm.BigNum.from_str("32"))));
map.insert(
  CardanoWasm.TransactionMetadatum.new_text("tags"),
  CardanoWasm.TransactionMetadatum.new_list(tags),
);
const metadatum = CardanoWasm.TransactionMetadatum.new_map(map);
```

We could then parse the information back as such:
```javascript
try {
  const map = metadatum.as_map();
  const receiver = map.get(CardanoWasm.TransactionMetadatum.new_text("receiver_id"));
  const sender = map.get(CardanoWasm.TransactionMetadatum.new_text("sender_id"));
  const comment = map.get(CardanoWasm.TransactionMetadatum.new_text("comment"));
  const tags = map.get(CardanoWasm.TransactionMetadatum.new_text("tags"));
} catch (e) {
  // structure did not match
}
```

For decoding in a more exploratory manner we can check the types first as such:
```javascript
function parseMetadata(metadata) {
  // we must check the type first to know how to handle it
  switch (metadata.kind()) {
    case CardanoWasm.TransactionMetadatumKind.MetadataMap:
      const mapRet = new Map();
      const map = metadata.as_map();
      const keys = maps.keys();
      for (var i = 0; i < keys.len(); i += 1) {
        const key = keys.get(i);
        const value = parseMetadata(map.get(key);
        mapRet.set(key, value);
      }
      return mapRet;
    case CardanoWasm.TransactionMetadatumKind.MetadataList:
      let arrRet = [];
      const arr = metadata.as_list();
      for (var i = 0; i < arr.len(); i += 1) {
        const elem = parseMetadata(arr.get(i));
        arrRet.push(elem);
      }
      return arrRet;
    case CardanoWasm.TransactionMetadatumKind.Int:
      const x = metadata.as_int();
      // If the integer is too big as_i32() returns undefined
      // to handle larger numbers we need to use x.as_positive() / x.as_negative() and
      // convert from BigNums after checking x.is_positive() first
      return x.as_i32();
    case CardanoWasm.TransactionMetadatumKind.Bytes:
      return Buffer.from(metadata.as_bytes());
    case CardanoWasm.TransactionMetadatumKind.Text:
      return metadata.as_text();
  }
}
```
which recursively parses the `TransactionMetadatum` struct and transforms it into a JS `Map` / JS `object` structure by manually checking the types.


## JSON conversion

Upsides:
* Flexible
* Readable by other methods
* Lowest set-up work involved

Downsides:
* Does not support bytes (metdata side), or null/true/false (JSON side) to ensure unambiguous conversions
* Does not support negative integers between `-2^64 + 1` and `-2^63` (serde_json library restriction)
* Structural validation must be done by hand
* Can use more space as string keyed maps are likely to be used more than arrays would be in the CDDL solutions
* Does not support non-string map keys

```javascript
const obj = {
  receiver_id: "SJKdj34k3jjKFDKfjFUDfdjkfd",
  sender_id: "jkfdsufjdk34h3Sdfjdhfduf873",
  comment: "happy birthday",
  tags: [0, 264, -1024, 32]
};
const metadata = CardanoWasm.encode_json_str_to_metadatum(JSON.stringify(obj));
const metadataString = CardanoWasm.decode_metadatum_to_json_str(metadata);
```

## Using a CDDL Subset

Upsides:
* Automatic structural typing in deserialization
* Readable by other methods
* Possible reduced space due to array structs not serializing keys

Downsides:
* Requires additional set-up

For static or relatively static types this is probably the best choice, especially if you care about structural validation or need the binary types or more complex keys that JSON doesn't allow.

As we saw in the other examples, most reasonable structures can be encoded using the standard metadata CDDL as it is almost a superset of JSON outside of true/false/null. Not only this, but if we represent a struct using an array in CDDL such as:
```
foo = [
  receiver_id: text,
  sender_id: text,
  comment: text,
  tags: [*int]
]
```
there is space savings as the keys are not stored as it is represented as an ordered array of 4 elements instead of a direct map encoding of:
```
foo = {
  receiver_id: text,
  sender_id: text,
  comment: text,
  tags": [*int]
}
```
which would serialize the keys as strings inside the resulting CBOR. Using these CDDL definitions for the example JSON structure we had results in sizes of 89 bytes for the array definition and 124 bytes for the map one. Using the JSON encoding would also result in a metadata size of 124 bytes. Maps however do have the advantage of easy optional fields and a more readable metadata for external users who don't have access to the CDDL as the field names will be stored directly.

After you have created your CDDL definition, if you need to check that your CDDL conforms to the metadata CDDL we have a tool located in the `/tools/metadata-cddl-checker/` directory. Move to this directory and put your CDDL in a file called `input.cddl` there first, then run

```
cargo build
cargo run
```

Once we have the CDDL file and it has passed metadata format validation we can use the [cddl-codegen](https://github.com/Emurgo/cddl-codegen) tool that we used to initially generate the serialization/deserialization/structural code for the core Shelley structures from the [shelley cddl spec](https://github.com/input-output-hk/cardano-ledger-specs/blob/master/shelley/chain-and-ledger/shelley-spec-ledger-test/cddl-files/shelley.cddl).

Assuming we are in the `cddl-codegen` root directory and have created a `input.cddl` file in that directory containing the CDDL we wish to generate we can build and code-generate with
```
cargo build
cargo run
```
which should generate a wasm-convertible rust library for parsing our CDDL definition in the `/export/` directory.
After this we need to generate a wasm package from the rust code by running the following (you can do `--target=browser` too)
```
cd export
wasm-pack build --target=nodejs
wasm-pack pack
```

which should give you the library as a package in the `/pkg/` directory.

Once we have imported the library we can then use it as such:
```javascript
const tags = OurMetadataLib.Ints.new();
// if we have smaller (32-bit signed) numbers we can construct easier
tags.add(OurMetadataLib.Int.new_i32(0));
// but for bigger numbers we must use BigNum and specify the sign ourselves
tags.add(OurMetadataLib.Int.new(CardanoWasm.Int.from_str("264")));
// and for negative large numbers (here we construct -1024)
tags.add(OurMetadataLib.Int.new_negative(CardanoWasm.Int.from_str("1024")));
tags.add(OurMetadataLib.Int.new_i32(32));
const map = OurMetadataLib.Foo.new("SJKdj34k3jjKFDKfjFUDfdjkfd", "jkfdsufjdk34h3Sdfjdhfduf873", "happy birthday", tags)
let metadata;
try {
  metadata = CardanoWasm.TransactionMetadata.from_bytes(map.to_bytes());
} catch (e) {
  // this should never happen if OurMetadataLib was generated from compatible CDDL with the metadata definition
}
```

likewise you can parse the metadata back very simply with:
```javascript
let cddlMetadata;
try {
  cddlMetadata = OurMetadataLib.Foo.from_bytes(metadata.to_bytes());
} catch (e) {
  // this should never happen if OurMetadataLib was generated from compatible CDDL with the metadata definition
}
// we can now directly access the fields with cddlMetadata.receiver_id(), etc
```

If we take advantage of the additional primitives not defined in CDDL but defined for `cddl-codegen`, then we can specify precisions of `u32`, `u64`, `i64`, `i32` for specifying 32 or 64 bits instead of just a general purpose `uint`/`nint`/`int`.
If you know your metadata will always be within one of these ranges it can be much more convenient to work with, and if you have signed data this will also make it easier to work with instead of the `Int` class that CDDL `int` might generate, since that is either an up to 64-bit positive or an up to 64 negative numbers.
This is particularly useful here as lists of CDDL primitives can be exposed directly as `Vec<T>` to wasm from rust, but when we have `int` (converts to `Int` struct) or `uint` (converts to `BigNum` struct) a separate structure like that `Ints` one used above is used. Using the 32-bit versions allows direct js `number` conversions to/from wasm.

If we simply change the `tags` field to `tags: [+i32]` our code becomes:
```javascript
// notice how we can directly work with js numbers here now!
// but remember they must fit into a 32-bit number now - no 64-bit numbers like are allowed in the metadata
const tags = [0, 264, -1024, 32];
const map = OurMetadataLib.Foo.new("SJKdj34k3jjKFDKfjFUDfdjkfd", "jkfdsufjdk34h3Sdfjdhfduf873", "happy birthday", tags)
```

and deserializaing likewise is much simpler as `metadata.tags()` will return a JS array or numbers rather than a rust-wasm struct that must be accessed via the wasm boundary.

## Raw Bytes Encoding

Upsides:
* Can store arbitrary data
* Potential space-savings if the data is compressed

Downsides:
* Not readable by other methods - must be decoded using this method
* Requires additional set-up

While most data would likely conform to the metadata CDDL subset (or JSON), if your data does not fit there then this encoding style will be necessary.

If you still want to take advantage of CDDL type-checking it is possible to create a library just as in the CDDL subset section but without running the checker tool. This could be useful if you are using CDDL outside of the metadata CDDL structure. Otherwise, you can store whatever bytes you want.

```javascript
const bytes = /* whatever method you want - you can use the CDDL solution in the 3rd option here */
const metadata = CardanoWasm.encode_arbitrary_bytes_as_metadatum(bytes);
const decoded_bytes = CardanoWasm.decode_arbitrary_bytes_from_metadatum(metadata);
assertEquals(bytes, decoded_bytes);
```

