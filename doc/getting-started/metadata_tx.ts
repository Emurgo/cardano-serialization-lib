import * as CSL from "@emurgo/cardano-serialization-lib-nodejs";
import { mnemonicToEntropy }  from "bip39";
import { Buffer } from "node:buffer";
 
const MNEMONIC = "key in your 24 words of your mnemonic here, words separated by spaces";
const INPUT_HASH ="9fc9bb3ea1f2540ae870076e6543b5d804566a548db9da9e16c5271596e8dc9d"
const INPUT_INDEX =1; 
const INPUT_AMOUNT = "113185492" ;  //Lovelace on your UTXO
const TO_ADDRESS="addr_test1qqew6jaz63u389gwnp8w92qntetzxs6j9222pn4cnej672vazs7a6wnrseqggj4d4ur43yq9e23r4q0m879t7efyhzjq8mvzua";
const AMOUNT="2000000";

function harden(num: number): number {
  return 0x80000000 + num;
}

//Step 1.========= Retrieve root key===============
const entropy = mnemonicToEntropy(MNEMONIC);
// retrieve root key in hex
const rootKey = CSL.Bip32PrivateKey.from_bip39_entropy(
  Buffer.from(entropy, "hex"),
  Buffer.from("")
);

// Retrieve child private key at m/1852H/1815H/0H/0/0 and m/1852H/1815H/0H/2/0
const accountKey = rootKey
  .derive(harden(1852))
  .derive(harden(1815))
  .derive(harden(0));
const utxoPrivKey = accountKey.derive(0).derive(0);
const stakePrivKey = accountKey.derive(2).derive(0);
const paymentKey = utxoPrivKey.to_raw_key();

// Retrieve payment credential + stake credential
const addr = CSL.BaseAddress.new(
  CSL.NetworkInfo.testnet_preprod().network_id(),
  CSL.Credential.from_keyhash(utxoPrivKey.to_public().to_raw_key().hash()),
  CSL.Credential.from_keyhash(stakePrivKey.to_public().to_raw_key().hash())  ///not need for non-stake tx
);

//Step 2.========= instantiate the tx builder with the Cardano protocol parameters - these may change later on===============
const linearFee = CSL.LinearFee.new(
  CSL.BigNum.from_str("44"),
  CSL.BigNum.from_str("155381")
);
const txBuilderCfg = CSL.TransactionBuilderConfigBuilder.new()
  .fee_algo(linearFee)
  .pool_deposit(CSL.BigNum.from_str("500000000"))
  .key_deposit(CSL.BigNum.from_str("2000000"))
  .max_value_size(5000)
  .max_tx_size(16384)
  .coins_per_utxo_byte(CSL.BigNum.from_str("4310"))
  .build();

const txBuilder = CSL.TransactionBuilder.new(txBuilderCfg);

//Step 3.========= Define and add inputs to transaction=================
const txInputsBuilder = CSL.TxInputsBuilder.new();
txInputsBuilder.add_regular_input(addr.to_address(),
CSL.TransactionInput.new(CSL.TransactionHash.from_hex(INPUT_HASH), INPUT_INDEX),
CSL.Value.new(CSL.BigNum.from_str(INPUT_AMOUNT))
);
txBuilder.set_inputs(txInputsBuilder);

//Step 4.========= Define and add output to the tx to transaction========
const DESTINATION_ADDRESS = CSL.Address.from_bech32(TO_ADDRESS);
txBuilder.add_output(
  CSL.TransactionOutput.new(DESTINATION_ADDRESS,
    CSL.Value.new(CSL.BigNum.from_str(AMOUNT))
  ),
);

//Step 5.========= Define and add metadata tx to transaction========
const metadata = CSL.GeneralTransactionMetadata.new();
    const metadataKey = CSL.BigNum.from_str("674"); 
    const metadataValue = CSL.encode_json_str_to_metadatum(
        JSON.stringify({ message: "hello metadata" }),
        CSL.MetadataJsonSchema.BasicConversions
    );
    metadata.insert(metadataKey, metadataValue);

// add metadata as auxiliary data to tx.
const auxData = CSL.AuxiliaryData.new();
auxData.set_metadata(metadata);
txBuilder.set_auxiliary_data(auxData);
 
// calculate the min fee required and send any change to an address
txBuilder.add_change_if_needed(addr.to_address());


const txBody = txBuilder.build()
const txBodyBytes = CSL.FixedTransaction.new_from_body_bytes(txBody.to_bytes());
let tx_hash = txBodyBytes.transaction_hash();

const witnesses = CSL.TransactionWitnessSet.new();
const vkeyWitnesses = CSL.Vkeywitnesses.new();
const vkeyWitness = CSL.make_vkey_witness(tx_hash,paymentKey);
    vkeyWitnesses.add(vkeyWitness);
    witnesses.set_vkeys(vkeyWitnesses);



// Step 6.=========Create Transaction with 03 components========
const transaction = CSL.Transaction.new(txBody, witnesses, auxData);

// Step 7.=========Serialize Transaction to hex========
const txSerialized = Buffer.from(transaction.to_bytes()).toString("hex");
console.log("Transaction serialized:", txSerialized);

 


