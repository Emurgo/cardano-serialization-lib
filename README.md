# Cardano Serialization Lib

This is a library for serialization & deserialization of data structures used in Cardano's Haskell implementation of Shelley along with useful utility functions.

## How can I use this library

Rust is wonderfully portable! You can easily bind to the native Rust library from any common programming language (even C and WebAssembly)!

##### NPM packages

- [NodeJS WASM package](https://www.npmjs.com/package/@emurgo/cardano-serialization-lib-nodejs)
- [Browser (chrome/firefox) WASM package](https://www.npmjs.com/package/@emurgo/cardano-serialization-lib-browser)

##### Mobile bindings

- [React-Native mobile bindings](https://github.com/Emurgo/react-native-haskell-shelley)

## Benefits of using this library

Serialization/deserialization code is automatically generated from Cardano's official specification, which guarantees it can easily stay up to date! We do this using an EMURGO-written tool called [cddl-codegen](https://github.com/Emurgo/cddl-codegen) which be re-use to automatically generate a Rust library for Cardano metadata specifications!

It is also very easy to create scripts in Rust or WASM to share with stake pools of embed inside an online tool. No more crazy cardano-cli bash scripts!

Powerful and flexible enough to be used to power wallets and exchanges! (Yes, it's used in production!)

## What about other versions of Cardano?

If you are looking for legacy bindings, you can find them at the following:

- [Byron WASM bindings](https://github.com/input-output-hk/js-cardano-wasm/tree/master/cardano-wallet)
- [Jormungandr WASM bindings](https://github.com/emurgo/js-chain-libs)

## Original binary specifications

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
