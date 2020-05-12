use super::*;
// constants - are these right?
const ABSTRACT_SIZE_VKEY: usize = 8;//64; // from 
const ABSTRACT_SIZE_SIG: usize = 13;//64;
const HL: usize = 4;//16; // really unsure about this one, found https://github.com/input-output-hk/cardano-base/blob/501d91e426ae84ce0ae056be38bd3db594af9fc2/cardano-crypto-class/src/Cardano/Crypto/Hash/MD5.hs#L16
const HASH_OBJ: usize = 2 + HL;
// CBOR-encoding specific contants I believe
const UINT: usize = 5;
const ARRAY_PREFIX: usize = 2;
const SMALL_ARRAY: usize = 1;
const MAP_PREFIX: usize = 2;
const LABEL_SIZE: usize = 1;
const CBOR_TAG: usize = 2;
const ADDRESS: usize = 2 * HASH_OBJ;
const CREDENTIAL: usize = LABEL_SIZE + HASH_OBJ;
const UNIT_INTERVAL: usize = CBOR_TAG + SMALL_ARRAY + UINT + UINT;
const FEE_SIZE: usize = LABEL_SIZE + UINT;
const TTL_SIZE: usize = LABEL_SIZE + UINT;

fn multisig_node_count(root: &Script) -> usize {
    match &root.0 {
        ScriptEnum::ScriptKeyNode(_) => 1,
        ScriptEnum::ScriptAllOfNode(ScriptAllOfNode { scripts }) => 1usize + scripts.0.iter().map(multisig_node_count).sum::<usize>(),
        _ => 2,
    }
}

fn cert_size(cert: &DelegationCertificate) -> usize {
    let x = match &cert.0 {
        DelegationCertificateEnum::StakeKeyReg(_) |
        DelegationCertificateEnum::StakeScriptKeyReg(_) => {
            println!("wtf {} + {} + {} = {}", SMALL_ARRAY, LABEL_SIZE, HASH_OBJ, SMALL_ARRAY + LABEL_SIZE + HASH_OBJ);
            SMALL_ARRAY + LABEL_SIZE + HASH_OBJ
        },
        DelegationCertificateEnum::StakeKeyDereg(_) |
        DelegationCertificateEnum::StakeScriptKeyDereg(_) => SMALL_ARRAY + LABEL_SIZE + HASH_OBJ,
        DelegationCertificateEnum::StakeDeleg(_) |
        DelegationCertificateEnum::StakeScriptDeleg(_) => SMALL_ARRAY + LABEL_SIZE + 2 * HASH_OBJ,
        DelegationCertificateEnum::PoolRegistration(PoolRegistration{ pool_params, .. }) => {
            SMALL_ARRAY + LABEL_SIZE + HASH_OBJ
            // pool owners
            + CBOR_TAG + ARRAY_PREFIX + pool_params.owners.len() * HASH_OBJ
            // cost
            + UINT
            // margin
            + UNIT_INTERVAL
            // pledge
            + UINT
            // operator
            + HASH_OBJ
            // vrf keyhash
            + HASH_OBJ
            // reward acc
            + CREDENTIAL
            // relays (String::len() is in bytes already, no need for utf8 conversion)
            + pool_params.relays.0.iter().map(|url| url.len()).sum::<usize>()
            // metadata (String::len() is in bytes, and metadat should only have 1 element)
            + SMALL_ARRAY + pool_params.metadata.0.first().map_or(0, |pmd| pmd.url.len())
        },
        DelegationCertificateEnum::PoolRetirement(_) => SMALL_ARRAY + LABEL_SIZE + UINT + HASH_OBJ,
        DelegationCertificateEnum::GenesisKeyDeleg(_) => SMALL_ARRAY + 2 * LABEL_SIZE + HASH_OBJ,
        DelegationCertificateEnum::MoveRewardsCert(mir) => {
            SMALL_ARRAY + LABEL_SIZE + MAP_PREFIX + mir.move_instantaneous_reward.len() * (UINT + HASH_OBJ)
        },
    };
    println!("cert_size({:?}) = {}", cert, x);
    x
}

#[wasm_bindgen]
pub fn txsize(tx: &TransactionBody, witness: &TransactionWitnessSet) -> usize {
    // It seems that in Haskell we have Tx = {Body, vkeySigs, msigScripts, metadata}
    // but in the CDDL we have Body as one then vkeySigs/msigScripts as another
    // as the data is segwit.
    // We might want to make our own version that wraps these then.
    
    // witness size
    let signatures = witness.key_witnesses.as_ref().map_or(0, |keys| keys.len());
    let script_nodes = witness.script_witnesses.as_ref().map_or(0, |scripts| scripts.0.iter().map(multisig_node_count).sum());
    let witness_size = signatures * (ABSTRACT_SIZE_VKEY + ABSTRACT_SIZE_SIG)
                     + HASH_OBJ * script_nodes
                     + SMALL_ARRAY + LABEL_SIZE + MAP_PREFIX
                     + HASH_OBJ * witness.script_witnesses.as_ref().map_or(0, Scripts::len);

    let input_size = LABEL_SIZE + CBOR_TAG + ARRAY_PREFIX
                   + tx.inputs.len() * (SMALL_ARRAY + UINT + HASH_OBJ);
    let output_size = LABEL_SIZE + ARRAY_PREFIX
                    + tx.outputs.len() * (SMALL_ARRAY + UINT + ADDRESS);
    let certs_size = tx.certs.as_ref().map_or(0, |certs| certs.0.iter().map(cert_size).sum::<usize>());
    let withdrawals_size = LABEL_SIZE + MAP_PREFIX
                         + tx.withdrawals.as_ref().map_or(0, |wd| wd.len()) * (UINT + CREDENTIAL);
    // TODO: updates
    let update_size = ARRAY_PREFIX;
    // TODO: metdata/hash
    let metadata_size = ARRAY_PREFIX;
    let metdata_hash_size = ARRAY_PREFIX;
    
    println!("input_size = {}", input_size);
    println!("output_size = {}", output_size);
    println!("certs_size = {}", certs_size);
    println!("withdrawals_size = {}", withdrawals_size);
    println!("FEE_SIZE = {}", FEE_SIZE);
    println!("TTL_SIZE = {}", TTL_SIZE);
    println!("update_size = {}", update_size);
    println!("metdata_hash_size = {}", metdata_hash_size);
    println!("witness_size = {}", witness_size);
    println!("metadata_size = {}", metadata_size);
    input_size + output_size + certs_size + withdrawals_size + FEE_SIZE + TTL_SIZE + update_size + metdata_hash_size + witness_size + metadata_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use js_chain_libs::*;
    use address::*;

    const HLEN: usize = 4;
    const SLEN: usize = 13;

    // TODO: compare raw bytes now that we have complete test vectors for this
    // will require implementing their mock crypto though... or at least copying the outputs 
    fn genesis_id() -> Hash {
        Hash::new(vec![0; HLEN])
    }
    fn alice_key() -> PrivateKey {
        PrivateKey::from_normal_bytes(&[228, 61, 34, 119, 224, 166, 98, 69, 109, 32, 41, 244, 193, 183, 151, 145, 1, 130, 86, 184, 181, 148, 163, 25, 206, 19, 125, 217, 15, 154, 95, 53]).unwrap()
    }
    fn alice_pay() -> Keyhash {
        Keyhash::new(vec![2; HLEN])
    }
    fn alice_stake() -> Keyhash {
        Keyhash::new(vec![3; HLEN])
    }
    fn alice_addr() -> Address {
        BaseAddress::new(3, AddrCred::from_keyhash(alice_pay()), AddrCred::from_keyhash(alice_stake())).to_address()
    }
    fn bob_key() -> PrivateKey {
        PrivateKey::from_normal_bytes(&[29, 121, 11, 180, 125, 92, 240, 44, 174, 77, 75, 175, 52, 177, 31, 232, 186, 118, 65, 184, 118, 3, 159, 236, 29, 166, 235, 108, 101, 13, 67, 36]).unwrap()
    }
    fn bob_pay() -> Keyhash {
        Keyhash::new(vec![5; HLEN])
    }
    fn bob_stake() -> Keyhash {
        Keyhash::new(vec![6; HLEN])
    }
    fn bob_addr() -> Address {
        BaseAddress::new(3, AddrCred::from_keyhash(bob_pay()), AddrCred::from_keyhash(bob_stake())).to_address()
    }

    fn make_witnesses_vkey(tx: &TransactionBody, pks: Vec<&PrivateKey>) -> TransactionWitnessSet {
        // TODO: implement non-mock at some point?
        let mut w = TransactionWitnessSet::new();
        let mut vkw = Vkeywitnesss::new();
        for pk in pks {
            vkw.add(Vkeywitness::new(Vkey::new(pk.to_public().as_bytes()), Signature::new(vec![1; SLEN])));
        }
        w.set_key_witnesses(vkw);
        w
    }

    #[test]
    fn tx_simple_utxo() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        let body = TransactionBody::new(inputs, outputs, 94, 10);
        let w = make_witnesses_vkey(&body, vec![&alice_key()]);
        //assert_eq!(body.to_bytes().len() + w.to_bytes().len(), 94);
        assert_eq!(txsize(&body, &w), 84);
    }

    #[test]
    fn tx_multi_utxo() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        inputs.add(TransactionInput::new(genesis_id(), 1));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        outputs.add(TransactionOutput::new(alice_addr(), 20));
        outputs.add(TransactionOutput::new(alice_addr(), 30));
        outputs.add(TransactionOutput::new(bob_addr(), 40));
        outputs.add(TransactionOutput::new(bob_addr(), 50));
        let body = TransactionBody::new(inputs, outputs, 94, 10);
        let w = make_witnesses_vkey(&body, vec![&alice_key(), &bob_key()]);
        assert_eq!(txsize(&body, &w), 189);
    }

    #[test]
    fn tx_register_stake() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        let mut body = TransactionBody::new(inputs, outputs, 94, 10);
        let mut certs = DelegationCertificates::new();
        certs.add(DelegationCertificate::new_key_reg(alice_pay()));
        body.set_certs(certs);
        let w = make_witnesses_vkey(&body, vec![&alice_key()]);
        assert_eq!(txsize(&body, &w), 92);
    }

    // #[test]
    // fn tx_delegate_stake() {
    //     let mut inputs = TransactionInputs::new();
    //     inputs.add(TransactionInput::new(genesis_id(), 0));
    //     let mut outputs = TransactionOutputs::new();
    //     outputs.add(TransactionOutput::new(alice_addr(), 10));
    //     let mut body = TransactionBody::new(inputs, outputs, 94, 10);
    //     let mut certs = DelegationCertificates::new();
    //     certs.add(DelegationCertificate::new_delegation(bob_);
    //     body.set_certs(certs);
    //     let w = make_witnesses_vkey(&body, vec![&alice_key()]);
    //     assert_eq!(txsize(&body, &w), 92);
    // }
}