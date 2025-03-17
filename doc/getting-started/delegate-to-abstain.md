# Delegating to Always Abstain

This example can be used to construct a CBOR offline for vote delegation to always abstain on testnet.

code taken from (here)[https://github.com/VladislavKudrin/csl_delegate_to_abstain]

```javascript
import {
  LinearFee,
  BigNum,
  TransactionBuilderConfigBuilder,
  TransactionBuilder,
  Bip32PrivateKey,
  BaseAddress,
  NetworkInfo,
  Credential,
  DRep,
  VoteDelegation,
  Certificates,
  Certificate,
  FixedTransaction,
  TxInputsBuilder,
  TransactionInput,
  TransactionHash,
  Value,
  ExUnitPrices,
  UnitInterval,
} from "@emurgo/cardano-serialization-lib-nodejs";
import { mnemonicToEntropy } from "bip39";

const MNEMONIC = "your mnemonic";
const INPUT_HASH = "UTXO hash";
const INPUT_INDEX = "UTXO index";
const INPUT_AMOUNT = "UTXO amount" //lovelace

function harden(num: number): number {
  return 0x80000000 + num;
}

function main(): void {
  // derive the keys from mnemonic
  const entropy = mnemonicToEntropy(MNEMONIC!);
  const rootKey = Bip32PrivateKey.from_bip39_entropy(
    Buffer.from(entropy, "hex"),
    Buffer.from("")
  );

  const accountKey = rootKey
    .derive(harden(1852))
    .derive(harden(1815))
    .derive(harden(0));
  const stakePrivKey = accountKey.derive(2).derive(0);
  const utxoPrivKey = accountKey.derive(0).derive(0);

  // get payment address
  const addr = BaseAddress.new(
    NetworkInfo.testnet_preprod().network_id(),
    Credential.from_keyhash(utxoPrivKey.to_public().to_raw_key().hash()),
    Credential.from_keyhash(stakePrivKey.to_public().to_raw_key().hash())
  );

  // instantiate the tx builder with the Cardano protocol parameters
  const linearFee = LinearFee.new(
    BigNum.from_str("44"),
    BigNum.from_str("155381")
  );

  // these parameters is used as an example, you should use the latest actual protocol params from cardano-node, cardano-cli or from 3rd party API providers
  const txBuilderCfg = TransactionBuilderConfigBuilder.new()
    .fee_algo(linearFee)
    .pool_deposit(BigNum.from_str("500000000"))
    .key_deposit(BigNum.from_str("2000000"))
    .max_value_size(5000)
    .max_tx_size(16384)
    .coins_per_utxo_byte(BigNum.from_str("4310"))
    .ex_unit_prices(ExUnitPrices.new(
      UnitInterval.new(
        BigNum.from_str("577"),
        BigNum.from_str("10000")
      ),
      UnitInterval.new(
        BigNum.from_str("721"),
        BigNum.from_str("10000000")
      )
    ))
    .build();
    .build();

  const txBuilder = TransactionBuilder.new(txBuilderCfg);

  // create new "Always Abstain" DRep and Vote Delegation
  const drep = DRep.new_always_abstain();
  const voteDelegation = VoteDelegation.new(
    Credential.from_keyhash(stakePrivKey.to_public().to_raw_key().hash()),
    drep
  );

  // add vote delegation certificate to the txBuilder
  const certs = Certificates.new();
  certs.add(Certificate.new_vote_delegation(voteDelegation));
  txBuilder.set_certs(certs);

  // add input for paying fees and add change
  const txInputsBuilder = TxInputsBuilder.new();
  txInputsBuilder.add_regular_input(
    addr.to_address(),
    TransactionInput.new(TransactionHash.from_hex(INPUT_HASH!), Number(INPUT_INDEX!)),
    Value.new(BigNum.from_str(INPUT_AMOUNT!))
  );
  txBuilder.set_inputs(txInputsBuilder);
  txBuilder.add_change_if_needed(addr.to_address());


  const txBody = txBuilder.build();

  //Use FixedTransaction for each time when you need to sign a transaction, especially if you recieved it from a third party
  const transaction = FixedTransaction.new_from_body_bytes(txBody.to_bytes());

  // sign the tx with stake and payment keys
  transaction.sign_and_add_vkey_signature(stakePrivKey.to_raw_key());
  transaction.sign_and_add_vkey_signature(utxoPrivKey.to_raw_key());

  // CBOR
  console.log(transaction.to_hex());

  }

```
