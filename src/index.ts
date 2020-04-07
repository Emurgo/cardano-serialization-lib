const wasm = require('rust-lib')

const fs = require('fs')

let addr = wasm.Address.new_base(wasm.Keyhash.new([1, 3, 3, 7]), wasm.Keyhash.new([0, 1, 2, 3, 4]))

console.log(`addr.to_bytes() = ${addr.to_bytes()}`)

fs.writeFile('addr_out.bin', new Buffer(addr.to_bytes()), (err) => {
    if (err) throw err
    console.log('saved')
})

let inputs = wasm.TransactionInputs.new()
inputs.add(wasm.TransactionInput.new(wasm.Hash.new([5, 4, 6, 4, 5])), 0)
let outputs = wasm.TransactionOutputs.new()
outputs.add(wasm.TransactionOutput.new(addr, 666))
let tx_body = wasm.TransactionBody.new(inputs, outputs)

fs.writeFile('tx_body.bin', new Buffer(tx_body.to_bytes()), (err) => {
    if (err) throw err
    console.log('saved')
})
