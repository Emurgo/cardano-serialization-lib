# Generating Keys

## BIP32 Keys

There are two main categories of keys in this library. There are the raw `PublicKey` and `PrivateKey` which are used for cryptographically signing/verifying, and `BIP32PrivateKey`/`BIP32PublicKey` which in addition to this have the ability to derive additional keys from them following the `[BIP32 derivation scheme]`(https://en.bitcoin.it/wiki/BIP_0032) variant called BIP32-Ed25519, which will be referred to as BIP32 for brevity. We use the [BIP44 spec](https://en.bitcoin.it/wiki/BIP_0044) variant for Ed25519 as well for the derivation paths using 1852 or 44 as the purpose consant and 1815 for the coin type depending on address type. See [this doc](https://github.com/input-output-hk/implementation-decisions/pull/18) for more details.

This is demonstrated with the below code
```javascript
function harden(num: number): number {
  return 0x80000000 + num;
}


const rootKey = CardanoWasm.BIP32PrivateKey.from_bech32("xprv17qx9vxm6060qjn5fgazfue9nwyf448w7upk60c3epln82vumg9r9kxzsud9uv5rfscxp382j2aku254zj3qfx9fx39t6hjwtmwq85uunsd8x0st3j66lzf5yn30hwq5n75zeuplepx8vxc502txx09ygjgx06n0p");
const accountKey = rootKey
  .derive(harden(1852)) // purpose
  .derive(harden(1815)) // coin type
  .derive(harden(0)); // account #0

const utxoPubKey = accountKey
  .derive(0) // external
  .derive(0)
  .to_public();

const stakeKey = accountKey
  .derive(2) // chimeric
  .derive(0)
  .to_public();
```

## BIP39 Entropy

To generate a `BIP32PrivateKey` from a BIP39 recovery phrase it must be first converted to entropy following the [BIP39 protocol](). This library does not directly handle that, but once entropy is created it is possible to use `Bip32PrivateKey.from_bip39_entropy(entropy, password)`. For more information see the [CIP3](https://github.com/cardano-foundation/CIPs/pull/3) Cardano improvement proposal. The code below uses the `bip39` npm package to generate a root `BIP32PrivateKey` from a BIP39 mnemonic.

```javascript
import { mnemonicToEntropy } from 'bip39';

const entropy = mnemonicToEntropy(
  [ "test", "walk", "nut", "penalty", "hip", "pave", "soap", "entry", "language", "right", "filter", "choice" ].join(' ')
);

const rootKey = CardanoWasm.Bip32PrivateKey.from_bip39_entropy(
  Buffer.from(entropy, 'hex'),
  Buffer.from(''),
);
```

## Use in Addresses

Once we have reached the desired derivation path, we must convert the `BIP32PrivateKey` or `BIP32PublicKey` to a `PrivateKey` or `PublicKey` by calling `.to_raw_key()` on them.
For example, to create an address using the `utxoPubKey` and `stakeKey` in the first example, we can do:
```javascript
// network id - 0? for mainnet, 1? for testnet
const networkId = 0;

// base address with staking key
const baseAddr = CardanoWasm.BaseAddress.new(
  networkId,
  CardanoWasm.StakeCredential.from_keyhash(utxoPubKey.to_raw_key().hash()),
  CardanoWasm.StakeCredential.from_keyhash(stakeKey.to_raw_key().hash()),
);

// enterprise address without staking ability, for use by exchanges/etc
const enterpriseAddr = CardanoWasm.EnterpriseAddress.new(
  networkId,
  CardanoWasm.StakeCredential.from_keyhash(utxoPubKey.to_raw_key().hash())
);

// pointer address - similar to Base address but can be shorter, see formal spec for explanation
const ptrAddr = CardanoWasm.PointerAddress.new(
  networkId,
  CardanoWasm.StakeCredential.from_keyhash(utxoPubKey.to_raw_key().hash()),
  CardanoWasm.Pointer.new(
    100, // slot
    2,   // tx index in slot
    0    // cert indiex in tx
  )
);
```

These are all address variant types with information specific to its address type. To be used with the library we must conver them to a generic `Address` like such:
```javascript
const address = baseAddress.to_address();

const output = CardanoWasm.TransactionOutput(address, BigNum.from_str("365"));
```

## Other Key Types

Conversion between `cardano-cli` 128-byte `XPrv` keys and `BIP32PrivateKey` is also supported:
```javascript
const bip32PrivateKey = CardanoWasm.BIP32PrivateKey.from_128_xprv(xprvBytes);
assert(xprvBytes == CardanoWasm.BIP32PrivateKey.to_128_xprv());
```
96-byte `XPrv` keys are identical to `BIP32PrivateKey`s byte-wise and no conversion is needed.


