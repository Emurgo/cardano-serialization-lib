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
import cbor from "cbor";

const mintNft = async (
  privateKey,
  policy,
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

  const policyPubKey = policy.privateKey.to_public();
  const policyAddr = BaseAddress.new(
    NetworkInfo.testnet_preprod().network_id(),
    Credential.from_keyhash(policyPubKey.hash()),
    Credential.from_keyhash(policyPubKey.hash())
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
  const policyKeyHash = BaseAddress.from_address(policyAddr)
    .payment_cred()
    .to_keyhash();
  console.log( `POLICY_KEYHASH: ${Buffer.from(policyKeyHash.to_bytes()).toString("hex")}`
  );

  //==============add key hash script so only people with policy key can mint assets using this policyId
  const keyHashScript = NativeScript.new_script_pubkey(
    ScriptPubkey.new(policyKeyHash)
  );
  scripts.add(keyHashScript);

  const mintScript = NativeScript.new_script_all(
    ScriptAll.new(scripts)
  );

  const privKeyHash = BaseAddress.from_address(addr)
    .payment_cred()
    .to_keyhash();
  
  txBuilder.add_key_input(
    privKeyHash,
    TransactionInput.new(
      TransactionHash.from_bytes(Buffer.from(tx_hash, "hex")),
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

  const txBody = txBuilder.build();
  const txBodyBytes = FixedTransaction.new_from_body_bytes(txBody.to_bytes());
  let txHash = txBodyBytes.transaction_hash();
  // const txHash = hash_transaction(txBody);

  console.log(`TX_HASH: ${Buffer.from(txHash.to_bytes()).toString("hex")}`);

  //==============Creates a witness set to sign the transaction=====================
  const witnesses = TransactionWitnessSet.new();
  const vkeyWitnesses = Vkeywitnesses.new();
  vkeyWitnesses.add(make_vkey_witness(txHash, policy.privateKey));
  vkeyWitnesses.add(make_vkey_witness(txHash, privateKey));
  witnesses.set_vkeys(vkeyWitnesses);
  witnesses.set_native_scripts;
  const witnessScripts = NativeScripts.new();
  witnessScripts.add(mintScript);
  witnesses.set_native_scripts(witnessScripts);

  const unsignedTx = txBuilder.build_tx();

  //==============Create signed transaction=====================
  const tx = Transaction.new(
    unsignedTx.body(),
    witnesses,
    unsignedTx.auxiliary_data()
  );

  const signedTx = Buffer.from(tx.to_bytes()).toString("hex");
  console.log(`Minting NFT siged tx: ${signedTx}`)
};

try {
  const privateKey = PrivateKey.from_bech32(
  //"ed25519_sk1fde2u8u2qme8uau5ac3w6c082gvtnmxt6uke2w8e07xwzewxee3q3n0f8e"
  "ed25519e_sk1gphh7prmtr5gh542j9fcph5942jwxwh2x22c2zp0clf38dp4rfdsd2gah4y4hlwznnrscsm2kyt0kfzstch0fe3rsqfc6xfaa9evpfs5glzcf"
  );
  const policyPrivateKey = PrivateKey.from_normal_bytes(
  cbor.decodeFirstSync(
    "582009ca7f508dd5a5f9823d367e98170f25606799f49ae7363a47a11d7d3502c91f"
  ));
  
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
