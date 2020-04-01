# Experimental proof of concept for generating rust code for CBOR serialization from CDDL specs.

### Purpose ###

This would be used to get baseline code for a library we could compile to web-asm for use in Yoroi, Tangata, etc.
It would have the added benefit of being more prepared for the future when/if new CDDL specs are used.

### Current capacities

Generates a `/export/` folder with wasm-compilable rust code (including Cargo.toml, etc) which can then be compiled with `wasm-pack build`.
The `lib.rs` contains all wasm-exposable code that clients of the generated code can use, and `groups.rs` contians internal implementations for serialization, structure and such that are not exposed to clients. There is also a `prelude.rs` for helper code used by both.

All generated types contain a `new(...)` constructor as well as a `to_bytes()` function that serializes to a byte buffer as the CBOR structure.

* Primitives - `bytes`, `bstr`, `tstr`, `text`, `uint`, `nint` (last two truncated to 32 bytes for now)
* Array values - `[uint]`
* Inline groups at root level - `foo = ( a: uint, b: uint)`
* Array groups - `foo = [uint, tstr, 0, bytes]`
* Map groups (both struct-type and table-type) - `foo = { a: uint, b: tstr }` or `bar = { * uint => tstr }`
* Embedding groups in other groups - `foo = (0, bstr) bar = [uint, foo, foo]`
* Group choices - `foo = [ 0, uint // 1, tstr, uint // tstr }`
* Tagged major types - `rational =  #6.30([ numerator : uint, denominator : uint])`
* Optional fields - `foo = { ? 0 : bytes }`
* Type aliases - `foo = bar`

It should be noted that for our purposes when we encounter a type that is an alias or transitiviely an alias for binary bytes, we always create a wrapper type for it, as in our use cases those should not be mixed and are crypt
o keys, hashes, and so on.

Any array of non-primitives such as `[foo]` will generate another type called `foos` which supports all basic array operations.
This lets us get around the `wasm_bindgen` limitation (without implementing cross-boundary traits which could be inefficient/tedious/complicated) discussed in the limitations section.


### Limitations

* Primitive `int` not supported due to no type choice support
* There is no support for deserialization as this is not of immediate use for use.
* No accessor functions (easily added but we don't need them yet as the focus is constructing CBOR not deserializing)
* No type choices - `foo = uint / tstr`
* Does not support optional group `[(...)]` or `{(...)}` syntax - must use `[...]` for `{...}` for groups
* Ignores occurence specifiers: `*`, `+` or `n*m`
* No support for sockets
* CDDL generics not supported - just edit the cddl to inline it yourself for now

`wasm_bindgen` also cannot expose doubly-nested types like `Vec<Vec<T>` which can be a limitation if `T` was a non-byte primtive.
