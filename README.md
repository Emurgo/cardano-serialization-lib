# Cardano Serialization Lib

**WARNING** this library is experimental.

This is a library for serialization & deserialization of data structures used in Cardano's Haskell implementation of Shelley along with useful utility functions.

Serialization/deserialization code is generated automatically from the CDDL specification using [cddl-codegen](https://github.com/Emurgo/cddl-codegen).

This code is available in:

- Native Rust (this repository)
- [React-Native mobile bindings](https://github.com/Emurgo/react-native-haskell-shelley)
- [nodejs WASM package](https://www.npmjs.com/package/@emurgo/cardano-serialization-lib-nodejs)
- [browser WASM package](https://www.npmjs.com/package/@emurgo/cardano-serialization-lib-browser)

If you are looking for legacy bindings, you can find them at the following:

- [Byron WASM bindings](https://github.com/input-output-hk/js-cardano-wasm/tree/master/cardano-wallet)
- [Jormungandr WASM bindings](https://github.com/emurgo/js-chain-libs)

## Original specs

Here are the location of the original [CDDL](http://cbor.io/tools.html) specifications:

- Byron: [link](https://github.com/input-output-hk/cardano-ledger-specs/tree/master/byron/cddl-spec)
- Shelley: [link](https://github.com/input-output-hk/cardano-ledger-specs/tree/master/shelley/chain-and-ledger/executable-spec/cddl-files)

## Building

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
npm run rust:build-nodejs
npm install
```

## Testing

```
npm run rust:test
npm run js:test
```

## Publishing

```
npm run js:publish-nodejs
npm run js:publish-browser
```
