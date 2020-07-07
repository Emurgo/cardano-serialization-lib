import CardanoWasm = require('rust-lib')
import { expect } from 'chai'
import 'mocha';
import { mnemonicToEntropy } from 'bip39';

function harden(num: number): number {
  return 0x80000000 + num;
}

// Purpose derivation (See BIP43)
enum Purpose {
  CIP1852=1852, // see CIP 1852
}

// Cardano coin type (SLIP 44)
enum CoinTypes {
  CARDANO=1815,
}

enum ChainDerivation {
  EXTERNAL=0, // from BIP44
  INTERNAL=1, // from BIP44
  CHIMERIC=2, // from CIP1852
}

function getCip1852Account(): CardanoWasm.Bip32PrivateKey {
  const entropy = mnemonicToEntropy(
    [ "test", "walk", "nut", "penalty", "hip", "pave", "soap", "entry", "language", "right", "filter", "choice" ].join(' ')
  )
  const rootKey = CardanoWasm.Bip32PrivateKey.from_bip39_entropy(
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

    const baseAddr = CardanoWasm.BaseAddress.new(
      0,
      CardanoWasm.StakeCredential.from_keyhash(utxoPubKey.hash()),
      CardanoWasm.StakeCredential.from_keyhash(stakeKey.hash()),
    );

    // this commented out test is what you would get with the wrong address hash.
    // expect(baseAddr.to_address().to_bech32()).to.eq('addr1qq8hc2lefnn0kuvur9jpnxnghnd9lm3jty0tsptjwvdsyrtpshk7al9e262uud767td297tp6qhqtlhg5q47q26fjcyw53g6a4mqsnj8nl6q9q');
    expect(baseAddr.to_address().to_bech32()).to.eq('addr1qz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3jcu5d8ps7zex2k2xt3uqxgjqnnj83ws8lhrn648jjxtwqcyl47r');
  })
});

describe('Transactions', () => {
  it('create transaction', () => {
    const txInputs = CardanoWasm.TransactionInputs.new();
    {
      txInputs.add(
        CardanoWasm.TransactionInput.new(
          CardanoWasm.TransactionHash.from_bytes(
            Buffer.from('3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7', 'hex'),
          ),
          0, // index
        )
      );
    }
    const txOutputs = CardanoWasm.TransactionOutputs.new();
    {
      txOutputs.add(
        CardanoWasm.TransactionOutput.new(
          CardanoWasm.Address.from_bytes(
            // Buffer.from('61a6274badf4c9ca583df893a73139625ff4dc73aaa3082e67d6d5d08e0ce3daa4', 'hex'),
            Buffer.from('61a6274badf4c9ca583df893a73139625ff4dc73aaa3082e67d6d5d08e', 'hex'),
          ),
          BigInt(1),
        )
      );
    }
    const txBody = CardanoWasm.TransactionBody.new(
      txInputs,
      txOutputs,
      BigInt(42), // fee
      10, // ttl
    );
    
    const witnesses = CardanoWasm.TransactionWitnessSet.new();
    {
      const vkeyWitnesses = CardanoWasm.Vkeywitnesses.new();

      const prvKey = CardanoWasm.PrivateKey.from_normal_bytes(
        Buffer.from('f7955ca7a24889e892a74851712975c924d536d503eeb1c900a7431900633fb8', 'hex')
      );
      vkeyWitnesses.add(
        txBody.sign(prvKey)
      );
      witnesses.set_vkeys(vkeyWitnesses);
    }
    CardanoWasm.Transaction.new(
      txBody,
      witnesses,
    );
  })
});
