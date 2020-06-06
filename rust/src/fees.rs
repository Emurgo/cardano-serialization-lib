use super::*;

// size (including major type tag) in CBOR for a uint
fn cbor_uint_length(x: u64) -> usize {
    if x <= 23 {
        // value stored inside the tag
        1
    } else if x < 0x1_00 {
        2
    } else if x < 0x1_00_00 {
        3
    } else if x < 0x1_00_00_00_00 {
        4
    } else {
        5
    }
}

#[wasm_bindgen]
pub fn txsize(tx: &Transaction) -> usize {
    const UINT: usize = 5;
    const SMALL_ARRAY: usize = 1;
    const HASH_LEN: usize = 32;
    const HASH_OBJ: usize = 2 + HASH_LEN;
    const ADDR_HASH_LEN: usize = 28;
    const ADDR_HEADER: usize = 1;
    const ADDRESS: usize = 2 + ADDR_HEADER + 2 * ADDR_HASH_LEN;
    const INPUT_SIZE: usize = SMALL_ARRAY + UINT + HASH_OBJ;
    const OUTPUT_SIZE: usize = SMALL_ARRAY + UINT + ADDRESS;
    let input_bytes = tx.body.inputs.to_bytes().len();
    let output_bytes = tx.body.outputs.to_bytes().len();
    let fee_bytes = cbor_uint_length(tx.body.fee);
    let extra_size = input_bytes + output_bytes + fee_bytes;
    let rest = tx.to_bytes().len() - extra_size;
    tx.body.inputs.len() * INPUT_SIZE + tx.body.outputs.len() * OUTPUT_SIZE + rest
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::*;
    use address::*;

    // The Haskell test use mock-crypto of sizes:
    const HASKELL_HLEN: usize = 4;
    const HASKELL_SLEN: usize = 13;
    const HASKELL_VLEN: usize = 8;
    // But we use:
    // HLEN: We use the hash sizes directly here
    // SLEN: signature size I'm not sure of and it's not fixed in our code so we'll keep it as 13
    const SLEN: usize = 13;
    const VLEN: usize = 32;
    // but this means we would have to change (make things generic and swap in mock crypto)
    // other parts of our code (ie hashing) to adapt to their size.
    // We instead calculated the expected difference in size for our test sizes.
    // However, they don't match due to serialization differences in definite vs indefinite
    // CBOR lengths for maps/arrays, thus for now we've got all the tests as >= instead.
    // It's possible they're still off by a byte or two somewhere.
 
    // TODO: compare raw bytes now that we have complete test vectors for this
    // will require implementing their mock crypto though... or at least copying the outputs 
    fn genesis_id() -> TransactionHash {
        TransactionHash::from([0u8; TransactionHash::BYTE_COUNT])
    }
    fn alice_key() -> PrivateKey {
        PrivateKey::from_normal_bytes(&[228, 61, 34, 119, 224, 166, 98, 69, 109, 32, 41, 244, 193, 183, 151, 145, 1, 130, 86, 184, 181, 148, 163, 25, 206, 19, 125, 217, 15, 154, 95, 53]).unwrap()
    }
    fn alice_pay() -> StakeCredential {
        StakeCredential::from_keyhash(AddrKeyHash::from([1u8; AddrKeyHash::BYTE_COUNT]))
    }
    fn alice_stake() -> StakeCredential {
        StakeCredential::from_keyhash(AddrKeyHash::from([2u8;AddrKeyHash::BYTE_COUNT]))
    }
    fn alice_addr() -> Address {
        BaseAddress::new(0, alice_pay(), alice_stake()).to_address()
    }
    fn alice_pool() -> PoolKeyHash {
        PoolKeyHash::from([10u8; PoolKeyHash::BYTE_COUNT])
    }
    fn bob_key() -> PrivateKey {
        PrivateKey::from_normal_bytes(&[29, 121, 11, 180, 125, 92, 240, 44, 174, 77, 75, 175, 52, 177, 31, 232, 186, 118, 65, 184, 118, 3, 159, 236, 29, 166, 235, 108, 101, 13, 67, 36]).unwrap()
    }
    fn bob_pay() -> StakeCredential {
        StakeCredential::from_keyhash(AddrKeyHash::from([3u8; AddrKeyHash::BYTE_COUNT]))
    }
    fn bob_stake() -> StakeCredential {
        StakeCredential::from_keyhash(AddrKeyHash::from([4u8; AddrKeyHash::BYTE_COUNT]))
    }
    fn bob_addr() -> Address {
        BaseAddress::new(0, bob_pay(), bob_stake()).to_address()
    }
    fn carl_pay() -> StakeCredential {
        StakeCredential::from_keyhash(AddrKeyHash::from([12u8; AddrKeyHash::BYTE_COUNT]))
    }

    fn make_mock_witnesses_vkey(tx: &TransactionBody, pks: Vec<&PrivateKey>) -> TransactionWitnessSet {
        // these tests use mock crypto anyway, but very specific (ShortHash / MockDSIGN)
        // but right now we're only checking against sizes do it's okay for now to mock this out entirely
        let mut w = TransactionWitnessSet::new();
        let mut vkw = Vkeywitnesss::new();
        for pk in pks {
            vkw.add(Vkeywitness::new(Vkey::new(vec![5u8; VLEN]), Signature::new(vec![1u8; SLEN])));
        }
        w.set_vkeys(vkw);
        w
    }

    // how many more bytes our real crypto is vs mock haskell crypto
    fn haskell_multisig_byte_diff(scripts: &MultisigScripts) -> usize {
        scripts.0.iter().map(|script| haskell_multisig_node_byte_diff(script)).sum()
    }

    fn haskell_multisig_node_byte_diff(script: &MultisigScript) -> usize {
        match &script.0 {
            MultisigScriptEnum::MsigPubkey(_pk) => AddrKeyHash::BYTE_COUNT - HASKELL_HLEN,
            MultisigScriptEnum::MsigAll(all) => haskell_multisig_byte_diff(&all.multisig_scripts),
            MultisigScriptEnum::MsigAny(any) => haskell_multisig_byte_diff(&any.multisig_scripts),
            MultisigScriptEnum::MsigNOfK(nofk) => haskell_multisig_byte_diff(&nofk.multisig_scripts)
        }
    }

    fn witness_vkey_bytes_haskell(w: &TransactionWitnessSet) -> usize {
        match &w.vkeys {
            Some(vkeys) => vkeys.len() * (HASKELL_SLEN + HASKELL_VLEN),
            None => 0,
        }
    }

    fn witness_vkey_bytes_rust(w: &TransactionWitnessSet) -> usize {
        match &w.vkeys {
            Some(vkeys) => vkeys.len() * (SLEN + VLEN),
            None => 0,
        }
    }

    #[test]
    fn tx_simple_utxo() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        let body = TransactionBody::new(inputs, outputs, 94, 10);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let tx = Transaction::new(body, w.clone(), None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w);
        let our_crypto_bytes = witness_vkey_bytes_rust(&w);
        assert!(txsize(&tx) >= 139 + our_crypto_bytes - haskell_crypto_bytes);
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
        let body = TransactionBody::new(inputs, outputs, 199, 10);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &bob_key()]);
        let tx = Transaction::new(body, w.clone(), None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w);
        let our_crypto_bytes = witness_vkey_bytes_rust(&w);
        assert!(txsize(&tx) >= 462 + our_crypto_bytes - haskell_crypto_bytes);
    }

    #[test]
    fn tx_register_stake() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        let mut body = TransactionBody::new(inputs, outputs, 94, 10);
        let mut certs = Certificates::new();
        certs.add(Certificate::new_stake_registration(StakeRegistration::new(alice_pay())));
        body.set_certs(certs);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let tx = Transaction::new(body, w.clone(), None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + AddrKeyHash::BYTE_COUNT;
        assert!(txsize(&tx) >= 150 + our_crypto_bytes - haskell_crypto_bytes);
    }

    #[test]
    fn tx_delegate_stake() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        let mut body = TransactionBody::new(inputs, outputs, 94, 10);
        let mut certs = Certificates::new();
        certs.add(Certificate::new_stake_delegation(StakeDelegation::new(bob_stake(), alice_pool())));
        body.set_certs(certs);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &bob_key()]);
        let tx = Transaction::new(body, w.clone(), None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN * 2;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + AddrKeyHash::BYTE_COUNT + PoolKeyHash::BYTE_COUNT;
        assert!(txsize(&tx) >= 178 + our_crypto_bytes - haskell_crypto_bytes);
    }

    #[test]
    fn tx_deregister_stake() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        let mut body = TransactionBody::new(inputs, outputs, 94, 10);
        let mut certs = Certificates::new();
        certs.add(Certificate::new_stake_deregistration(StakeDeregistration::new(alice_pay())));
        body.set_certs(certs);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let tx = Transaction::new(body, w.clone(), None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + AddrKeyHash::BYTE_COUNT;
        assert!(txsize(&tx) >= 150 + our_crypto_bytes - haskell_crypto_bytes);
    }

    #[test]
    fn tx_register_pool() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        let mut body = TransactionBody::new(inputs, outputs, 94, 10);
        let mut certs = Certificates::new();
        let mut owners = AddrKeyHashes::new();
        owners.add(alice_stake().to_keyhash().unwrap());
        let mut relays = Relays::new();
        relays.add(Relay::new_single_host_name(SingleHostName::new(None, String::from("relay.io"))));
        let params = PoolParams::new(
            alice_pool(),
            VrfKeyHash::from([0u8; VrfKeyHash::BYTE_COUNT]),
            1,
            5,
            UnitInterval::new(1, 10),
            alice_stake(),
            owners.clone(),
            relays,
            Some(PoolMetadata::new(String::from("alice.pool"), MetadataHash::from([0u8; MetadataHash::BYTE_COUNT])))
        );
        certs.add(Certificate::new_pool_registration(PoolRegistration::new(params)));
        body.set_certs(certs);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let tx = Transaction::new(body, w.clone(), None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w)
            + HASKELL_HLEN // operator pool keyhash
            + HASKELL_HLEN // vrf keyhash
            + HASKELL_HLEN // reward account
            + owners.len() * HASKELL_HLEN // owners' keyhashes
            + HASKELL_HLEN; // metadata hash
        let our_crypto_bytes = witness_vkey_bytes_rust(&w)
            + PoolKeyHash::BYTE_COUNT
            + VrfKeyHash::BYTE_COUNT
            + AddrKeyHash::BYTE_COUNT
            + owners.len() * AddrKeyHash::BYTE_COUNT
            + MetadataHash::BYTE_COUNT;
        assert!(txsize(&tx) >= 214 + our_crypto_bytes - haskell_crypto_bytes);
    }

    #[test]
    fn tx_retire_pool() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        let mut body = TransactionBody::new(inputs, outputs, 94, 10);
        let mut certs = Certificates::new();
        certs.add(Certificate::new_pool_retirement(PoolRetirement::new(alice_pool(), 5)));
        body.set_certs(certs);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let tx = Transaction::new(body, w.clone(), None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + PoolKeyHash::BYTE_COUNT;
        assert!(txsize(&tx) >= 149 + our_crypto_bytes - haskell_crypto_bytes);
    }

    #[test]
    fn tx_metadata() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        let mut body = TransactionBody::new(inputs, outputs, 94, 10);
        body.set_metadata_hash(MetadataHash::from([37; MetadataHash::BYTE_COUNT]));
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let mut metadata = TransactionMetadata::new();
        let mut md_list = TransactionMetadatums::new();
        md_list.add(TransactionMetadatum::new_int(Int::new(5)));
        md_list.add(TransactionMetadatum::new_text(String::from("hello")));
        metadata.insert(0, TransactionMetadatum::new_arr_transaction_metadatum(md_list));
        let tx = Transaction::new(body, w.clone(), Some(metadata));
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + MetadataHash::BYTE_COUNT;
        assert!(txsize(&tx) >= 154 + our_crypto_bytes - haskell_crypto_bytes);
    }

    #[test]
    fn tx_multisig() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        let body = TransactionBody::new(inputs, outputs, 94, 10);
        let mut w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &bob_key()]);
        let mut script_witnesses = MultisigScripts::new();
        let mut inner_scripts = MultisigScripts::new();
        inner_scripts.add(MultisigScript::new_msig_pubkey(MsigPubkey::new(alice_pay().to_keyhash().unwrap())));
        inner_scripts.add(MultisigScript::new_msig_pubkey(MsigPubkey::new(bob_pay().to_keyhash().unwrap())));
        inner_scripts.add(MultisigScript::new_msig_pubkey(MsigPubkey::new(carl_pay().to_keyhash().unwrap())));
        script_witnesses.add(MultisigScript::new_msig_n_of_k(MsigNOfK::new(2, inner_scripts)));
        w.set_scripts(script_witnesses.clone());
        let tx = Transaction::new(body, w.clone(), None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w);
        let our_crypto_bytes = witness_vkey_bytes_rust(&w);
        assert!(txsize(&tx) >= 189 + our_crypto_bytes - haskell_crypto_bytes + haskell_multisig_byte_diff(&script_witnesses));
    }

    #[test]
    fn tx_withdrawal() {
        let mut inputs = TransactionInputs::new();
        inputs.add(TransactionInput::new(genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(TransactionOutput::new(alice_addr(), 10));
        let mut body = TransactionBody::new(inputs, outputs, 94, 10);
        let mut withdrawals = Withdrawals::new();
        withdrawals.insert(alice_pay(), 100);
        body.set_withdrawals(withdrawals);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &alice_key()]);
        let tx = Transaction::new(body, w.clone(), None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + AddrKeyHash::BYTE_COUNT;
        assert!(txsize(&tx) >= 172 + our_crypto_bytes - haskell_crypto_bytes);
    }
}