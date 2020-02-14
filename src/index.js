// @flow

var cbor = require('cbor')

// untested

// both keys must be binary buffers/byte arrays
// TODO: use flow
// TODO: debug to see if it expects array or Buffer
// Probably better to make our own Address object and then handle serialization in a separate step
const makeBaseAddress = (spendingKeyHash, delegationKeyHash) => {
    [0, spendingKeyHash, delegationKeyHash]
}

/*
type TxId = Buffer

type InputType = {
    txid: TxId,
    index: number
}

type OutputType = {
    address: ,
    value: number
}
*/

// untested

// It'd probably be better to have this take on some kind of Transaction object
// and serialize that instead in a separate function
const serializeTxBody = (inputs/*: [InputType]*/, outputs/* : [OutputType*/) => {
    // should we use CborMap instead?
    var body = new Map()

    body.set(0, new cbor.Tagged(258, inputs.map(input => [input.txid, input.index])))

    body.set(1, outputs.map(output => [output.address, output.value]))

    // TODO: certs
    //body.set(2, ...)
    
    // withdrawals are separate here?

    // fees are separate? - TODO

    // TODO: TTL

    // no updates

    // metadata is optional
    console.log(`body! = ${JSON.stringify(body)}`)

    return cbor.encode(body)
}

const main = () => {
    const fakeTxId = Buffer.alloc(64, 0)
    const fakeKeyHash = Buffer.alloc(64, 0)
    const fakeInputs = [{
        txid: fakeTxId,
        index: 0
    }];
    const fakeOutputs = [{
        address: makeBaseAddress(fakeKeyHash, fakeKeyHash),
        value: 100
    }]
    const body = serializeTxBody(fakeInputs, fakeOutputs)
    console.log(`body = ${body}`)
}

main()