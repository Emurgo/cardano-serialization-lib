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
} from "@emurgo/cardano-serialization-lib-nodejs-gc";
import { mnemonicToEntropy } from "bip39";
import { Buffer } from "node:buffer";
import cbor from "cbor";

const mintNft = async (
  privateKey,
  policyPrivateKey,
  assetName,
  description,
  imageUrl,
  mediaType,
  tx_hash,
  tx_index,
  amount
  ) => 
  {
  //==============Retrieve publicKey, addr, policyPubKey, PolicyAddr from private keys ===============
  const publicKey = privateKey.to_public();
  const addr = BaseAddress.new(
    NetworkInfo.testnet_preprod().network_id(),
    Credential.from_keyhash(publicKey.hash()),
    Credential.from_keyhash(publicKey.hash())
  ).to_address();

  const policyPubKey = policyPrivateKey.to_public();
  const policyKeyHash = policyPubKey.hash();
  const policyAddr = BaseAddress.new(
    NetworkInfo.testnet_preprod().network_id(),
    Credential.from_keyhash(policyPubKey),
    Credential.from_keyhash(policyPubKey)
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
  const scripts = NativeScripts.new();

  //==============add key hash script so only people with policy key can mint assets using this policyId
  const keyHashScript = NativeScript.new_script_pubkey(
    ScriptPubkey.new(policyKeyHash)
  );
  scripts.add(keyHashScript);

  const mintScript = NativeScript.new_script_all(
    ScriptAll.new(scripts)
  );

  const paymentKeyHash = BaseAddress.from_address(addr)
    .payment_cred()
    .to_keyhash();
  
  txBuilder.add_key_input(
    paymentKeyHash,
    TransactionInput.new(
      TransactionHash.from_hex(tx_hash),
      tx_index
    ),
    Value.new(BigNum.from_str(amount))
  );

  txBuilder.add_mint_asset_and_output_min_required_coin(
    mintScript,
    AssetName.new(Buffer.from(assetName)),
    Int.new_i32(1),
    TransactionOutputBuilder.new().with_address(addr).next()
  );
  const policyId = Buffer.from(mintScript.hash().to_bytes()).toString("hex");
  const metadata = {
    [policyId]: {
      [assetName]: {
        name: assetName,
        description,
        image: imageUrl,
        mediaType,
      },
    },
  };

  console.log(`METADATA: ${JSON.stringify(metadata, null, 4)}`);

  txBuilder.add_json_metadatum(
    BigNum.from_str("721"),
    JSON.stringify(metadata)
  );

  txBuilder.add_change_if_needed(addr);

  const tx = txBuilder.build_tx();
  const fixedTx = FixedTransaction.from_bytes(txBody.to_bytes());
  let txHash = fixedTx.transaction_hash();

  console.log(`TX_HASH: ${txHash.to_hex()}`);

  //==============Add signatures=====================
  fixedTx.sign_and_add_vkey_signature(privateKey);
  fixedTx.sign_and_add_vkey_signature(policyPrivateKey);
    
  const unsignedTx = txBuilder.build_tx();

  const signedTx = fixedTx.to_hex();
  console.log(`Minting NFT siged tx: ${signedTx}`)
};

try {
  const privateKey = PrivateKey.from_bech32(
    "ed25519e_sk1your_key"
  );
  const policyPrivateKey = PrivateKey.from_bech32(
    "ed25519e_sk1your_key"
  );
  
  //==============Select UTXO that is probably big enough to pay the tx fee===============
  const tx_hash="5a4925b330916e62307766802f5af4ce8b234c27de8271a901086c08733da0f1";
  const tx_index="1";
  const amount="31009807";
  await mintNft(
    privateKey, 
    {
      privateKey: policyPrivateKey 
      },
    "CSL_101", // assetName
    "Description about NFT", // description
    "ipfs://QmSUfz3aeFjufYo9RnQauBaoQhGD27BwMYzZSvtsJ714BP", // image url
    "image/jpeg", // mediaType
    tx_hash,
    tx_index,
    amount
);
}
catch (err) {
  console.error(`failed to mint nft: ${err.toString()}`);
}
