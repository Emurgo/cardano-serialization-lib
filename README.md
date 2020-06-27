# Cardano Serialization Lib

**WARNING** this library is experimental.

This is a library for serialization & deserialization of types related to Cardano's Haskell implementation of Shelley.

## Original specs

Here are the location of the original [CDDL](http://cbor.io/tools.html) specifications:

- Byron: [link](https://github.com/input-output-hk/cardano-ledger-specs/blob/master/byron/cddl-spec/byron.cddl)
- Shelley: [link](https://github.com/input-output-hk/cardano-ledger-specs/blob/master/shelley/chain-and-ledger/cddl-spec/shelley.cddl#L72)

# Building

If you need to install Rust, do the following:
```
curl https://sh.rustup.rs -sSf | sh -s -- -y
echo 'export PATH=$HOME/.cargo/bin/:$PATH' >> $BASH_ENV
rustup install stable
rustup target add wasm32-unknown-unknown --toolchain stable
```

To build this repository, do the following:
```
git submodule update --init --recursive
nvm install && nvm use
npm run rust:build
npm install
```

# Testing

```
npm run js:test
```

# Generating CDDL instances

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
