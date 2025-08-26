import {
  Bip32PrivateKey,
  FixedTransaction,
  Transaction
} from "@emurgo/cardano-serialization-lib-nodejs";
import { mnemonicToEntropy } from "bip39";
import { Buffer } from "node:buffer";
import { decode } from 'cbor'; 
 
// the mnemonic of your key which is used to sign transaction.
const MNEMONIC = "fill in 24 words of your mnemonic here";
// your rosetta-java unsign transaction which often starts with 82... or cbor of tx start with a4 or a5
//const rosetta_unsign_tx = "a400818258202138e16c10911f1c0e264909b0c8e6af903e3370b2a8a75ebf5443081100ec71000182825839000743d16cfe3c4fcc0c11c2403bbc10dbc7ecdd4477e053481a368e7a06e2ae44dff6770dc0f4ada3cf4cf2605008e27aecdb332ad349fda71a7735940082581d60d6dbfa87f1054da9d47d612b82634eff44871c371554c8e6732d53f41b00000001dcd3c37f021a00028c81031a026c822a";
const rosetta_unsign_tx = "8278f26134303064393031303238313832353832306633383131613830386135326362393366616362303538393465363861323066333266333438663131366436626231306236613234616266306639306630373330313031383138323538333930303332656434626132643437393133393530653938346565326138313335653536323334333532326139346130636562383965363561663239396431343364646433613633383634303834346161646166303735383930303563616132336138316662336638616266363532346238613431613038313661343438303231613030303238366139303331613034383335366531a16a6f7065726174696f6e7381a6746f7065726174696f6e5f6964656e746966696572a265696e646578006d6e6574776f726b5f696e64657800676163636f756e74a16761646472657373786c616464725f74657374317170726668356a6c357a666b6c616b68636d65657a6b7376727830796c70366861613234706b356d7434746477326634386e6c7237646d70716767337666303632367134336366616c65703973386b6e6c30337772786e6c6c3664736c723777666a66616d6f756e74a26863757272656e6379a26673796d626f6c6341444168646563696d616c73066576616c75656a2d3133353836373132316b636f696e5f6368616e6765a26f636f696e5f6964656e746966696572a16a6964656e7469666965727842663338313161383038613532636239336661636230353839346536386132306633326633343866313136643662623130623661323461626630663930663037333a316b636f696e5f616374696f6e6a636f696e5f7370656e74667374617475736773756363657373647479706565696e707574";

// Call the function to get the string value
const cborHex = remove_rosetta_payload(rosetta_unsign_tx);
 // Output the result
if (cborHex) {
    console.log("Unsigned Transaction: ", cborHex);
  } else {
    console.log("Your Hex string is not rosetta-java format.");
    process.exit(1);
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

//convert Cbor from Hex to Byte
const txBytes = Buffer.from(cborHex, 'hex')
const tx = Transaction.from_bytes(txBytes)
//Retrieve body of Transaction
const txBody = tx.body()

//
const transaction = FixedTransaction.new_from_body_bytes( txBody.to_bytes());

// sign transaction with payment vkey
transaction.sign_and_add_vkey_signature(utxoPrivKey.to_raw_key());

// in case your TX needs more than 1 key to sign, add their signatures by using transaction.sign_and_add_vkey_signature
// eg: adding signature signed by stake key
// transaction.sign_and_add_vkey_signature(stakePrivKey.to_raw_key());

// The CBOR Hex of your rosetta transaction.
console.log(`Signed tx: ${transaction.to_hex()}`)



function remove_rosetta_payload(rosetta_unsign_tx:string):string {
  if (rosetta_unsign_tx.startsWith("a5")|| rosetta_unsign_tx.startsWith("a4")) {
     // Define the prefix and suffix strings to convert rosetta-java unsign transaction to Array instead of Map
     const prefix = "84";    // Character to be added at the beginning
     const suffix = "a0f5f6"; // Character to be added at the end
     // Concatenate  the strings
     const cborHex = prefix + rosetta_unsign_tx + suffix;
     return cborHex  // Return the first string
  }
  else {
  // Convert the hex string to a byte array
   const rosetta_unsign_bytes = Buffer.from(rosetta_unsign_tx, 'hex');
  // Decode to CBOR object/array data
   const decodedData:(string | object)[] = decode(rosetta_unsign_bytes);
   if (decodedData.length > 0 && typeof decodedData[0] === "string") {
       // Define the prefix and suffix strings to convert rosetta-java unsign transaction to Array instead of Map
    const prefix = "84";    // Character to be added at the beginning
    const suffix = "a0f5f6"; // Character to be added at the end
    // Concatenate  the strings
    const cborHex = prefix + decodedData[0] + suffix;
    return cborHex  // Return the first string
  }

    return null;
  }
  }


function harden(num: number): number {
  return 0x80000000 + num;
}
 
