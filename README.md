# Cardano Serialization Lib

**WARNING** this library is experimental.

This is a library for serialization & deserialization of types related to Cardano's Haskell implementation of Shelley.

## Original specs

Here are the location of the original [CDDL](http://cbor.io/tools.html) specifications:

- Byron: [link](https://github.com/input-output-hk/cardano-ledger-specs/blob/master/byron/cddl-spec/byron.cddl)
- Shelley: [link](https://github.com/input-output-hk/cardano-ledger-specs/blob/master/shelley/chain-and-ledger/cddl-spec/shelley.cddl#L72)

# Testing

First you need to install `cddl`
```
sudo apt install ruby
sudo gem install cddl
sudo gem install cbor-diag
```

You can generate new tests with
1) `cddl specs/shelley.cddl generate 1 > test/name_here.diag`
2) `diag2cbor.rb test/name_here.diag > test/name_here.cbor`

You can combine these together with `cddl specs/shelley.cddl generate 1 | diag2cbor.rb > test/name_here.cbor`
