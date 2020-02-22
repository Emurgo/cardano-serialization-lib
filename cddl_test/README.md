# Experimental proof of concept for generating rust code for CBOR serialization from CDDL specs.

### Purpose ###

This would be used to get baseline code for a library we could compile to web-asm for use in Yoroi, Tangata, etc.
It would have the added benefit of being more prepared for the future when/if new CDDL specs are used.
It is highly experimental as we are not sure if the effort to get it working will be worth the development time,
or if another approach would be better.

### Current capacities

* Reads type references `(ie foo = bar`)
* Reads root-level maps and generates a struct with all keys as fields.

The `supported.cddl` file contains all supported features thus far and outputs the following code:
```rust
struct foo {
    x: Option<int>,
    y: Option<int>,
    z: Option<int>,
    value_0: Option<tstr>,
}

type bar = foo;
```
In a next commit, it will map those primitive types to rust-compatible ones. 

### Current issues ###

*The only CDDL library for rust has several issues: Group choices don't parse correctly, optional members seem to be broken in parsing, some more complicated tagged values break parsing.
