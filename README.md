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

Serialization/deserialization code is automatically generated from Cardano's official specification, which guarantees it can easily stay up to date! We do this using an EMURGO-written tool called [cddl-codegen](https://github.com/Emurgo/cddl-codegen) which can be re-used for other tasks such as automatically generate a Rust library for Cardano metadata specifications!

It is also very easy to create scripts in Rust or WASM to share with stake pools, or even embed inside an online tool! No more crazy cardano-cli bash scripts!

Powerful and flexible enough to be used to power wallets and exchanges! (Yes, it's used in production!)

## Documentation

This library generates both [Typescript](https://www.typescriptlang.org/) and [Flow](https://flow.org/) type definitions, so it's often easiest to see what is possible by just looking at the types!
You can find the Flow types [here](/rust/pkg/cardano_serialization_lib.js.flow)

You can also look in the [example](/example) folder to see how to use this library from Typescript or just experiment with the library.

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
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

To build this repository, do the following:
```
git submodule update --init --recursive
nvm use
npm install
npm run rust:build-nodejs
```

## Testing

```
npm run rust:test
```

## Publishing

```
npm run js:publish-nodejs
npm run js:publish-browser
```
