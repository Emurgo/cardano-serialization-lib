import {
  LinearFee,
  BigNum,
  TransactionBuilderConfigBuilder,
  TransactionBuilder,
  Bip32PrivateKey,
  BaseAddress,
  NetworkInfo,
  Credential,
  FixedTransaction,
  TxInputsBuilder,
  TransactionInput,
  TransactionOutput,
  TransactionHash,
  Address,
  Value,
} from "@emurgo/cardano-serialization-lib-nodejs";
import { mnemonicToEntropy } from "bip39";
import { Buffer } from "node:buffer";
 
//adding your a mnemomic, input_hash, tx_index
const MNEMONIC = "key in your 24 words of your mnemonic here, words separated by spaces";
const TX_HASH ="372467a317554bcf1e1d172b5418b9eed850fc7f2c1a1d15f91c06b05fc09499"
const TX_INDEX =0; 
const INPUT_AMOUNT = "397000000" ;  //Lovelace on your UTXO
const TO_ADDRESS="addr_test1qqew6jaz63u389gwnp8w92qntetzxs6j9222pn4cnej672vazs7a6wnrseqggj4d4ur43yq9e23r4q0m879t7efyhzjq8mvzua";
const AMOUNT="2000000";

function harden(num: number): number {
  return 0x80000000 + num;
}

// Retrieve root key
const entropy = mnemonicToEntropy(MNEMONIC);
const rootKey = Bip32PrivateKey.from_bip39_entropy(
  Buffer.from(entropy, "hex"),
  Buffer.from("")
);
const accountKey = rootKey
  .derive(harden(1852))
  .derive(harden(1815))
  .derive(harden(0));
const utxoPrivKey = accountKey.derive(0).derive(0);
const stakePrivKey = accountKey.derive(2).derive(0);

// Retrieve payment credential, stake credential
const addr = BaseAddress.new(
  NetworkInfo.testnet_preprod().network_id(),
  Credential.from_keyhash(utxoPrivKey.to_public().to_raw_key().hash()),
  Credential.from_keyhash(stakePrivKey.to_public().to_raw_key().hash())  ///not need for non-stake tx
);

// instantiate the tx builder with the Cardano protocol parameters - these may change later on
const linearFee = LinearFee.new(
  BigNum.from_str("44"),
  BigNum.from_str("155381")
);
const txBuilderCfg = TransactionBuilderConfigBuilder.new()
  .fee_algo(linearFee)
  .pool_deposit(BigNum.from_str("500000000"))
  .key_deposit(BigNum.from_str("2000000"))
  .max_value_size(5000)
  .max_tx_size(16384)
  .coins_per_utxo_byte(BigNum.from_str("4310"))
  .build();
const txBuilder = TransactionBuilder.new(txBuilderCfg);

//Define and add inputs to transaction
const txInputsBuilder = TxInputsBuilder.new();
txInputsBuilder.add_regular_input(
  addr.to_address(),
  TransactionInput.new(TransactionHash.from_hex(TX_HASH), TX_INDEX),
  Value.new(BigNum.from_str(INPUT_AMOUNT))
);
txBuilder.set_inputs(txInputsBuilder);

// add output to the transaction
const DESTINATION_ADDRESS = Address.from_bech32(TO_ADDRESS);
txBuilder.add_output(
  TransactionOutput.new(DESTINATION_ADDRESS,
      Value.new(BigNum.from_str(AMOUNT))
  ),
);
// calculate the min fee required and send any change to an address
txBuilder.add_change_if_needed(addr.to_address());

const txBody = txBuilder.build();
const transaction = FixedTransaction.new_from_body_bytes(txBody.to_bytes());

// sign transaction with payment vkey
transaction.sign_and_add_vkey_signature(utxoPrivKey.to_raw_key());
//Print CBOR of signed TX to console
console.log(transaction.to_hex());


 
