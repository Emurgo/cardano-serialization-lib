// This file is just to test stuff quickly. Look at the spec.ts files for working examples

import wasm = require('rust-lib')

const metadata = wasm.TransactionMetadatum.from_bytes(
  Buffer.from("a200a16e4c615f52657073697374616e634568576173206865726501a56743686f6963657384a36b43616e6469646174654964782461616139353033612d366663352d343665612d396161302d62346339306633363161346368566f746552616e6b016a566f746557656967687401a36b43616e6469646174654964782438643634396331322d393336652d343662652d623635612d63313766333066353935373468566f746552616e6b026a566f746557656967687401a36b43616e6469646174654964782438316365376638652d393463332d343833352d393166632d32313436643531666131666368566f746552616e6b006a566f746557656967687400a36b43616e6469646174654964782434303735343061612d353862352d343063612d623438342d66343030343065623239393068566f746552616e6b036a566f746557656967687401694e6574776f726b49646f5468655265616c4164616d4465616e6a4f626a656374547970656a566f746542616c6c6f746a50726f706f73616c4964782438303036346332382d316230332d346631632d616266302d63613863356139386435623967566f7465724964782464393930613165382d636239302d346635392d623563662d613862353963396261386165", "hex")
);
const map = metadata.as_map();
const keys = map.keys();
for (let i = 0; i < keys.len(); i++) {
  const val = keys.get(i);
  console.log(Buffer.from(val.to_bytes()).toString('hex'));
}

const addr = wasm.Address.from_bech32('addr1qxy657awttf5avs2629f4hs6k5ulhw8f27akv30yws622dudj86zwkwhv3yjky5ntrmhcln5yxc05rcq0lhs8l78vd3qhc5eak');
console.log(Buffer.from(addr.to_bytes()).toString('hex'));
// const addr = wasm.Address.from_bytes(Buffer.from('615c619e192407b2e972f04f0dda7c52aa8013d45ee7ba69d57041cad0', 'hex'));
// console.log(addr.to_bech32());

// Ae2tdPwUPEZFAi4DxQaXeW9HAXYjdfvMWLgNXCJVjvweygZkAUiLjRwGfPr
// Ae2tdPwUPEZ3MHKkpT5Bpj549vrRH7nBqYjNXnCV8G2Bc2YxNcGHEa8ykDp
// console.log(wasm.ByronAddress.is_valid('Ae2tdPwUPEZFAi4DxQaXeW9HAXYjdfvMWLgNXCJVjvweygZkAUiLjRwGfPr'));
// console.log(wasm.ByronAddress.is_valid('Ae2tdPwUPEZ3MHKkpT5Bpj549vrRH7nBqYjNXnCV8G2Bc2YxNcGHEa8ykDp'));
// const addr = wasm.Address.from_bech32(
  // Buffer.from(
    // 'stake1uy5mdzcepk905jj5vqzgyxly57ldhzegugzp9y7fruc6d5sqapp2u',
  //   'hex'
  // ),
// );

// const enterpriseAddr = wasm.EnterpriseAddress.new(
//   0,
//   wasm.StakeCredential.from_keyhash(
//     wasm.Ed25519KeyHash.from_bytes(
//       Buffer.from(
//         'b861eeadde300385d88aaa98cad0f0ed1f95419bbb9971a0fb7c96fb',
//         'hex'
//       )
//     )
//   )
// );

const baseAddr = wasm.BaseAddress.from_address(addr);
if (baseAddr == null) throw new Error();

// const enterpriseAddr = wasm.EnterpriseAddress.from_address(addr);
// if (enterpriseAddr == null) throw new Error();

const keyHash = baseAddr.payment_cred().to_keyhash()?.to_bytes();
if (keyHash == null) throw new Error();
console.log(Buffer.from(keyHash).toString('hex'));

// const enterpriseAddr = wasm.BaseAddress.new(
//   addr.network_id(),
//   baseAddr.payment_cred(),
//   wasm.StakeCredential.from_keyhash(
//      wasm.Ed25519KeyHash.from_bytes(
//        Buffer.from(
//          '00000000000000000000000000000000000000000000000000000000',
//          'hex'
//        )
//      )
//   )
// );

// const enterpriseAddr = wasm.EnterpriseAddress.new(
//   0,
//   baseAddr.payment_cred()
// );

// console.log(enterpriseAddr.to_address().to_bech32());
// const baseAddr = wasm.RewardAddress.from_address(addr);
// console.log(addr.network_id());
// const baseAddr = wasm.BaseAddress.from_address(addr);
// if (baseAddr == null) throw new Error();
// const keyHash = baseAddr.stake_cred().to_keyhash();
// if (keyHash == null) throw new Error();
// console.log(Buffer.from(enterpriseAddr.to_address().to_bytes()).toString('hex'));

// const tx = wasm.Transaction.from_bytes(
//   Buffer.from(
//     'g6UAgYJYIDZ351x7ppm/3GzVfULyRvhvY679dgJQBqx4MT+tK7ohAQGBglg5Aceyi86pDUQLVFWmoConylm4aW8Gf8GWf0f5M+eVWOlpyqnletz8QLmQfreUNjtZD69C//SMOOuIGgBv0n8CGgACnmEDGhH+lM0EgYIBggBYHHVsMspvo8GlXypDW5P6AAkVsR41Lg4sbWgEAFShAIKCWCDMmAmUQVDADzkTzSsQPptC/mJD/Danb564AGkuK9o/LlhArA2Id/4V04BNncOIhWd1GMRLlX9q1lTmWTcQBsB7S7R0JvOV0u/2jMcBxIlFLkTSyvAywveQzAfc2uY+frBWB4JYIPOlcWS3t2uMM0elCy5Y8VhUv0SFmwE0b4nY9XZQ10F2WEDkGBk00hf2bU2gT2OVc5It9XVPa0BPF2QmpmOebAmHbBapE7xpMDPWckfUEBsNZ+B1OjHBtwIWOdI69y5PmWQJ9g==',
//     'base64'
//   )
// );
// console.log(tx);
// console.log(tx.body().withdrawals()?.len());
// console.log(tx.body().certs()?.len());
// import fs = require('fs');

// const addr = wasm.Address.new_base(wasm.Keyhash.new([1, 3, 3, 7]), wasm.Keyhash.new([0, 1, 2, 3, 4]))

// console.log(`addr.to_bytes() = ${addr.to_bytes()}`)

// fs.writeFile('addr_out.bin', new Buffer(addr.to_bytes()), (err) => {
//     if (err) throw err
//     console.log('saved')
// })

// const inputs = wasm.TransactionInputs.new()
// inputs.add(wasm.TransactionInput.new(wasm.Hash.new([5, 4, 6, 4, 5])), 0)
// const outputs = wasm.TransactionOutputs.new()
// outputs.add(wasm.TransactionOutput.new(addr, 666))
// const tx_body = wasm.TransactionBody.new(inputs, outputs)

// fs.writeFile('tx_body.bin', new Buffer(tx_body.to_bytes()), (err) => {
//     if (err) throw err
//     console.log('saved')
// })
