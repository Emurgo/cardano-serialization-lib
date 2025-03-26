import {
  LinearFee,
  PrivateKey,
  BigNum,
  TransactionBuilderConfigBuilder,
  TransactionBuilder,
  NativeScripts,
  NativeScript,
  ScriptPubkey,
  ScriptAll,
  AssetName,
  Int,
  TransactionOutputBuilder,
  TransactionWitnessSet,
  Vkeywitnesses,
  make_vkey_witness,
  Transaction,
  Bip32PrivateKey,
  BaseAddress,
  NetworkInfo,
  Credential,
  FixedTransaction,
  TransactionInput,
  TransactionHash,
  Value,
} from "@emurgo/cardano-serialization-lib-nodejs";
import { mnemonicToEntropy } from "bip39";
import { Buffer } from "node:buffer";


const MNEMONIC = "fill in 24 words of your mnemonic here";
const INPUT_HASH ="9171d747c58f1f582c0973d7d5c832e2ef5dd9738c208e0b72a10481ce187e45"
const INPUT_INDEX =1;   //tx_ix of your utxo
const INPUT_AMOUNT = "41611646" ;  //Lovelace on your UTXO
const TOKEN_NAME ="TOKEN_ABC";
const AMOUNT="2000000";

function harden(num: number): number {
  return 0x80000000 + num;
}

//==============Retrieve hierarchical deterministic (HD) root key ===============
const entropy = mnemonicToEntropy(MNEMONIC);
// retrieve root key in hex
const rootKey = Bip32PrivateKey.from_bip39_entropy(
  Buffer.from(entropy, "hex"),
  Buffer.from("")
);

//==============Derives child private keys from the root key using the CIP-1852 derivation path=====
const accountKey = rootKey
  .derive(harden(1852))
  .derive(harden(1815))
  .derive(harden(0));
const utxoPrivKey = accountKey.derive(0).derive(0);
const stakePrivKey = accountKey.derive(2).derive(0);

const payment_skey = utxoPrivKey.to_raw_key();
const payment_vkey = payment_skey.to_public();
const publicKeyHash = payment_vkey.hash();

//==============Creates a Cardano BaseAddress using the payment public key hash and staking public key hash====
const addr = BaseAddress.new(
  NetworkInfo.testnet_preprod().network_id(),
  Credential.from_keyhash(publicKeyHash),
  Credential.from_keyhash(stakePrivKey.to_raw_key().to_public().hash())  
).to_address();

//==============Sets transaction parameters like fees, deposits, and size limits================
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

//==============Creates a minting policy script by using the payment public key hash================
const policyScript = NativeScript.new_script_pubkey(
  ScriptPubkey.new(publicKeyHash));  // you could use other keys instead of using publickey
const scripts = NativeScripts.new();
scripts.add(policyScript);
const mintScript = NativeScript.new_script_all(
  ScriptAll.new(scripts)
);

//==============Specify assesst name and amount of token=============
const assetName = AssetName.new(Buffer.from(TOKEN_NAME, 'utf8'));
txBuilder.add_mint_asset_and_output_min_required_coin(
  mintScript,assetName,
  Int.new_i32(AMOUNT),
  TransactionOutputBuilder.new().with_address(addr).next()
);

//==============Adds an input to the transaction=====================
txBuilder.add_key_input(
  publicKeyHash,
TransactionInput.new(
  TransactionHash.from_bytes(Buffer.from(INPUT_HASH, "hex")),
  INPUT_INDEX
),Value.new(BigNum.from_str(INPUT_AMOUNT))
);

txBuilder.add_change_if_needed(addr);

//==============Builds the transaction body (txBody) and converts it to bytes==
const txBody = txBuilder.build();
const txBodyBytes = FixedTransaction.new_from_body_bytes(txBody.to_bytes());
let txHash = txBodyBytes.transaction_hash();
console.log(`Tx_hash of transaction: ${Buffer.from(txHash.to_bytes()).toString("hex")}`);

//==============Creates a witness set to sign the transaction=====================
const witnesses = TransactionWitnessSet.new();
const vkeyWitnesses = Vkeywitnesses.new();
vkeyWitnesses.add(make_vkey_witness(txHash,payment_skey));
// vkeyWitnesses.add(make_vkey_witness(txHash,Other_skey));//Add additional keys if you use them in policy
witnesses.set_vkeys(vkeyWitnesses);
witnesses.set_native_scripts;
const witnessScripts = NativeScripts.new();
witnessScripts.add(mintScript);
witnesses.set_native_scripts(witnessScripts);

//==============Create signed transaction=====================
const unsignedTx = txBuilder.build_tx();
const tx = Transaction.new(
  unsignedTx.body(),
  witnesses,
  null // using unsignedTx.auxiliary_data() for metadata
);
const signedTx = Buffer.from(tx.to_bytes()).toString("hex");

console.log(`CBOR of signed transaction: ${signedTx}`);
