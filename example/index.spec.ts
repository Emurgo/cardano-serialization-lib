import wasm = require('rust-lib')
import { expect } from 'chai'
import 'mocha';
import { mnemonicToEntropy } from 'bip39';
import { initTransactionBuilder, toHex } from './index';
import { TEST_EPOCH_PARAMS } from './consts';

function harden (num: number): number {
  return 0x80000000 + num;
}

// Purpose derivation (See BIP43)
enum Purpose {
  CIP1852 = 1852, // see CIP 1852
}

// Cardano coin type (SLIP 44)
enum CoinTypes {
  CARDANO = 1815,
}

enum ChainDerivation {
  EXTERNAL = 0, // from BIP44
  INTERNAL = 1, // from BIP44
  CHIMERIC = 2, // from CIP1852
}

function getCip1852Account (): wasm.Bip32PrivateKey {
  const entropy = mnemonicToEntropy(
    ["test", "walk", "nut", "penalty", "hip", "pave", "soap", "entry", "language", "right", "filter", "choice"].join(' ')
  )
  const rootKey = wasm.Bip32PrivateKey.from_bip39_entropy(
    Buffer.from(entropy, 'hex'),
    Buffer.from(''),
  );
  return rootKey
    .derive(harden(Purpose.CIP1852))
    .derive(harden(CoinTypes.CARDANO))
    .derive(harden(0)); // account #0
}


describe('Addresses', () => {
  it('derive base address', () => {
    // from address test vectors

    const cip1852Account = getCip1852Account();

    const utxoPubKey = cip1852Account
      .derive(ChainDerivation.EXTERNAL)
      .derive(0)
      .to_public();
    const stakeKey = cip1852Account
      .derive(ChainDerivation.CHIMERIC)
      .derive(0)
      .to_public();

    const baseAddr = wasm.BaseAddress.new(
      1,
      wasm.StakeCredential.from_keyhash(utxoPubKey.to_raw_key().hash()),
      wasm.StakeCredential.from_keyhash(stakeKey.to_raw_key().hash()),
    );

    expect(baseAddr.to_address().to_bech32())
      .to.eq('addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3jcu5d8ps7zex2k2xt3uqxgjqnnj83ws8lhrn648jjxtwqfjkjv7');
  })
});

describe('Transactions', () => {
  it('creates a bundled nft transaction', () => {
    const txBuilder = initTransactionBuilder(TEST_EPOCH_PARAMS)

    const address = wasm.ByronAddress.from_base58("Ae2tdPwUPEZLs4HtbuNey7tK4hTKrwNwYtGqp7bDfCy2WdR3P6735W5Yfpe");
    txBuilder.add_bootstrap_input(
      address,
      wasm.TransactionInput.new(
        wasm.TransactionHash.from_bytes(
          Buffer.from("488afed67b342d41ec08561258e210352fba2ac030c98a8199bc22ec7a27ccf1", "hex"),
        ),
        0, // index
      ),
      wasm.Value.from_json(JSON.stringify({
        coin: '300000000',
        multiasset: {
          '7cfcafe81fc8f62e618cef2a18ec2fa68e1c6c6f8a1ecf6a19f89188': {
            [toHex('nft 01')]: '1',
            [toHex('nft 02')]: '1',
            [toHex('nft 03')]: '1',
            [toHex('nft 04')]: '1',
            [toHex('nft 05')]: '1',
            [toHex('nft 06')]: '1',
            [toHex('nft 07')]: '1',
            [toHex('nft 08')]: '1',
            [toHex('nft 09')]: '1',
            [toHex('nft 10')]: '1',
          }
        }
      }))
    );

    txBuilder.add_output(wasm.TransactionOutput.new(
      address.to_address(),
      wasm.Value.from_json(JSON.stringify({ coin: '10000000' }))
    ))

    txBuilder.set_ttl(410021);

    // calculate the min fee required and send any change to an address
    const changeAddress = wasm.ByronAddress.from_base58("Ae2tdPwUPEYxiWbAt3hUCJsZ9knze88qNhuTQ1MGCKqsVFo5ddNyoTDBymr").to_address()
    txBuilder.add_bundled_change_if_needed(changeAddress, 2)

    const txBody = txBuilder.build();
    const txHash = wasm.hash_transaction(txBody);
    const witnesses = wasm.TransactionWitnessSet.new();
    const bootstrapWitnesses = wasm.BootstrapWitnesses.new();
    const bootstrapWitness = wasm.make_icarus_bootstrap_witness(txHash, address, getCip1852Account());
    bootstrapWitnesses.add(bootstrapWitness);
    witnesses.set_bootstraps(bootstrapWitnesses);
    const transaction = wasm.Transaction.new(
      txBody,
      witnesses,
      undefined, // transaction metadata
    );

    console.log(`Change output was split into ${txBody.outputs().len() - 1}`)
    for (let i = 0; i < txBody.outputs().len(); i++) {
      console.log(i, txBody.outputs().get(i).amount().to_js_value())
    }
    expect(txBody.outputs().len() - 1).equal(6)

    const txHex = toHex(transaction.to_bytes());
    console.log(txHex);
  })

  it('creates a transaction', () => {
    const txBuilder = initTransactionBuilder(TEST_EPOCH_PARAMS)

    const address = wasm.ByronAddress.from_base58("Ae2tdPwUPEZLs4HtbuNey7tK4hTKrwNwYtGqp7bDfCy2WdR3P6735W5Yfpe");
    txBuilder.add_bootstrap_input(
      address,
      wasm.TransactionInput.new(
        wasm.TransactionHash.from_bytes(
          Buffer.from("488afed67b342d41ec08561258e210352fba2ac030c98a8199bc22ec7a27ccf1", "hex"),
        ),
        0, // index
      ),
      wasm.Value.from_json(JSON.stringify({
        coin: '10000000'
      }))
    );

    txBuilder.add_output(
      wasm.TransactionOutput.new(
        address.to_address(),
        wasm.Value.from_json(JSON.stringify({
          coin: '2000000',
        }))
      ),
    );

    txBuilder.set_ttl(410021);

    // calculate the min fee required and send any change to an address
    const changeAddress = wasm.ByronAddress.from_base58("Ae2tdPwUPEYxiWbAt3hUCJsZ9knze88qNhuTQ1MGCKqsVFo5ddNyoTDBymr").to_address()
    txBuilder.add_bundled_change_if_needed(changeAddress, 2)

    const txBody = txBuilder.build();
    const txHash = wasm.hash_transaction(txBody);
    const witnesses = wasm.TransactionWitnessSet.new();
    const bootstrapWitnesses = wasm.BootstrapWitnesses.new();
    const bootstrapWitness = wasm.make_icarus_bootstrap_witness(txHash, address, getCip1852Account());
    bootstrapWitnesses.add(bootstrapWitness);
    witnesses.set_bootstraps(bootstrapWitnesses);
    const transaction = wasm.Transaction.new(
      txBody,
      witnesses,
      undefined, // transaction metadata
    );

    const txHex = toHex(transaction.to_bytes());
    console.log(txHex);
  })

});
