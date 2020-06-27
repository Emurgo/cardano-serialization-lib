// This file is just to test stuff quickly. Look at the spec.ts files for working examples

// import wasm = require('rust-lib')

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
