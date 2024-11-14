# Cardano Serialization Lib

This is a library, written in Rust, for serialization & deserialization of data structures used in Cardano's Haskell implementation of Alonzo along with useful utility functions.

##### NPM packages

- [NodeJS WASM package](https://www.npmjs.com/package/@emurgo/cardano-serialization-lib-nodejs)
- [Browser (chrome/firefox) WASM package](https://www.npmjs.com/package/@emurgo/cardano-serialization-lib-browser)
- [Browser (pure JS - no WASM) ASM.js package](https://www.npmjs.com/package/@emurgo/cardano-serialization-lib-asmjs)

##### NPM packages with GC support 
Note: This package uses [weak references flag from wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/reference/weak-references.html).
It uses `FinalizationRegistry` under the hood to automatically call "free" for each CSL struct when it is no longer needed. However, use this feature with caution as it may have unpredictable behaviors.
- [NodeJS WASM package with GC](https://www.npmjs.com/package/@emurgo/cardano-serialization-lib-nodejs-gc)
- [Browser (chrome/firefox) WASM package with GC](https://www.npmjs.com/package/@emurgo/cardano-serialization-lib-browser-gc)
- [Browser (pure JS - no WASM) ASM.js package with GC](https://www.npmjs.com/package/@emurgo/cardano-serialization-lib-asmjs-gc)


##### Rust crates

- [cardano-serialization-lib](https://crates.io/crates/cardano-serialization-lib)

##### Mobile bindings

- [React-Native mobile bindings](https://github.com/Emurgo/react-native-haskell-shelley)

## Documentation

You can find documentation [here](https://developers.cardano.org/docs/get-started/cardano-serialization-lib/overview)
