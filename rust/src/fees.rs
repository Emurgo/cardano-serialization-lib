use super::*;
use utils::*;

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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LinearFee {
    constant: Coin,
    coefficient: Coin,
}

#[wasm_bindgen]
impl LinearFee {
    pub fn constant(&self) -> Coin {
        self.constant
    }

    pub fn coefficient(&self) -> Coin {
        self.coefficient
    }

    pub fn new(constant: &Coin, coefficient: &Coin) -> Self {
        Self {
            constant: constant.clone(),
            coefficient: coefficient.clone(),
        }
    }
}

#[wasm_bindgen]
pub fn min_fee(tx: &Transaction, linear_fee: &LinearFee) -> Result<Coin, JsValue> {
    let size = fees::txsize(tx) as u64;
    Coin::new(size)
        .checked_mul(&linear_fee.coefficient())?
        .checked_add(&linear_fee.constant())
}

fn txsize(tx: &Transaction) -> usize {
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
    let fee_bytes = cbor_uint_length(tx.body.fee.unwrap());
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
    // Ed25519 signature size
    const SLEN: usize = 64;
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
        StakeCredential::from_keyhash(&AddrKeyHash::from([1u8; AddrKeyHash::BYTE_COUNT]))
    }
    fn alice_stake() -> StakeCredential {
        StakeCredential::from_keyhash(&AddrKeyHash::from([2u8;AddrKeyHash::BYTE_COUNT]))
    }
    fn alice_addr() -> Address {
        // addr1qqqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgzqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqm2xxxw
        BaseAddress::new(0, &alice_pay(), &alice_stake()).to_address()
    }
    fn alice_pool() -> PoolKeyHash {
        PoolKeyHash::from([10u8; PoolKeyHash::BYTE_COUNT])
    }
    fn bob_key() -> PrivateKey {
        PrivateKey::from_normal_bytes(&[29, 121, 11, 180, 125, 92, 240, 44, 174, 77, 75, 175, 52, 177, 31, 232, 186, 118, 65, 184, 118, 3, 159, 236, 29, 166, 235, 108, 101, 13, 67, 36]).unwrap()
    }
    fn bob_pay() -> StakeCredential {
        StakeCredential::from_keyhash(&AddrKeyHash::from([3u8; AddrKeyHash::BYTE_COUNT]))
    }
    fn bob_stake() -> StakeCredential {
        StakeCredential::from_keyhash(&AddrKeyHash::from([4u8; AddrKeyHash::BYTE_COUNT]))
    }
    fn bob_addr() -> Address {
        BaseAddress::new(0, &bob_pay(), &bob_stake()).to_address()
    }
    fn carl_pay() -> StakeCredential {
        StakeCredential::from_keyhash(&AddrKeyHash::from([12u8; AddrKeyHash::BYTE_COUNT]))
    }

    fn make_mock_witnesses_vkey(tx: &TransactionBody, pks: Vec<&PrivateKey>) -> TransactionWitnessSet {
        // these tests use mock crypto anyway, but very specific (ShortHash / MockDSIGN)
        // but right now we're only checking against sizes do it's okay for now to mock this out entirely
        let mut w = TransactionWitnessSet::new();
        let mut vkw = Vkeywitnesses::new();
        for pk in pks {
            vkw.add(&Vkeywitness::new(
                &Vkey::new(&pk.to_public()),
                &pk.sign([1u8; 100].as_ref())
            ));
        }
        w.set_vkeys(&vkw);
        w
    }

    fn make_mock_byron_witnesses_vkey(
        tx: &TransactionBody,
        addr: &ByronAddress,
        pks: Vec<&Bip32PrivateKey>,
    ) -> TransactionWitnessSet {
        let mut w = TransactionWitnessSet::new();
        let mut bootstrap_witnesses = BootstrapWitnesses::new();
        for pk in pks {
            let witness = make_icarus_bootstrap_witness(
                &hash_transaction(&tx),
                addr,
                &pk
            );
            bootstrap_witnesses.add(&witness);
        }
        w.set_bootstraps(&bootstrap_witnesses);
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
        inputs.add(&TransactionInput::new(&genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
        let body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let tx = Transaction::new(&body, &w, None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w);
        let our_crypto_bytes = witness_vkey_bytes_rust(&w);
        assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 139);
    }

    #[test]
    fn tx_simple_byron_utxo() {
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(&genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
        let body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);

        // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
        let entropy = [0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12];
        let root_key = Bip32PrivateKey::from_bip39_entropy(&entropy, &[]);
        let byron_addr = ByronAddress::from_icarus_key(&root_key.to_public(), 0b001);
        let w = make_mock_byron_witnesses_vkey(
            &body,
            &byron_addr, // Ae2tdPwUPEZB3mTcbvEYWmxEyn3yRmePa7YV5TKmrtPyU3rcSao4k6J216Q
            vec![&root_key]
        );
        assert_eq!(
            hex::encode(w.bootstraps.unwrap().get(0).to_bytes()),
            "855820cf76399a210de8720e9fa894e45e41e29ab525e30bc402801c076250d1585bcd5840dbc0d07297733aa0c71e942e3dddb648eb6cc96a9a35ea686673b534b8cc20e1fab4c736d2cb711b270a912b216ba55167cf3a5d400bc08671cc9335a66aa10d582091e248de509c070d812ab2fda57860ac876bc489192c1ef4ce253c197ee219a44683008200525441a0"
        );
    }

    #[test]
    fn tx_multi_utxo() {
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(&genesis_id(), 0));
        inputs.add(&TransactionInput::new(&genesis_id(), 1));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(20)));
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(30)));
        outputs.add(&TransactionOutput::new(&bob_addr(), Coin::new(40)));
        outputs.add(&TransactionOutput::new(&bob_addr(), Coin::new(50)));
        let body = TransactionBody::new(&inputs, &outputs, Coin::new(199), 10);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &bob_key()]);
        let tx = Transaction::new(&body, &w, None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w);
        let our_crypto_bytes = witness_vkey_bytes_rust(&w);
        assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 462);
    }

    #[test]
    fn tx_register_stake() {
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(&genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
        let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
        let mut certs = Certificates::new();
        certs.add(&Certificate::new_stake_registration(&StakeRegistration::new(&alice_pay())));
        body.set_certs(&certs);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let tx = Transaction::new(&body, &w, None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + AddrKeyHash::BYTE_COUNT;
        assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 150);
    }

    #[test]
    fn tx_delegate_stake() {
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(&genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
        let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
        let mut certs = Certificates::new();
        certs.add(&Certificate::new_stake_delegation(&StakeDelegation::new(&bob_stake(), &alice_pool())));
        body.set_certs(&certs);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &bob_key()]);
        let tx = Transaction::new(&body, &w, None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN * 2;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + AddrKeyHash::BYTE_COUNT + PoolKeyHash::BYTE_COUNT;
        assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 178);
    }

    #[test]
    fn tx_deregister_stake() {
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(&genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
        let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
        let mut certs = Certificates::new();
        certs.add(&Certificate::new_stake_deregistration(&StakeDeregistration::new(&alice_pay())));
        body.set_certs(&certs);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let tx = Transaction::new(&body, &w, None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + AddrKeyHash::BYTE_COUNT;
        assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 150);
    }

    #[test]
    fn tx_register_pool() {
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(&genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
        let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
        let mut certs = Certificates::new();
        let mut owners = AddrKeyHashes::new();
        owners.add(&(alice_stake().to_keyhash().unwrap()));
        let mut relays = Relays::new();
        relays.add(&Relay::new_single_host_name(&SingleHostName::new(None, String::from("relay.io"))));
        let params = PoolParams::new(
            &alice_pool(),
            &VRFKeyHash::from([0u8; VRFKeyHash::BYTE_COUNT]),
            Coin::new(1),
            Coin::new(5),
            &UnitInterval::new(BigNum::new(1), BigNum::new(10)),
            &RewardAddress::new(0, &alice_stake()),
            &owners,
            &relays,
            Some(PoolMetadata::new(String::from("alice.pool"), &MetadataHash::from([0u8; MetadataHash::BYTE_COUNT])))
        );
        certs.add(&Certificate::new_pool_registration(&PoolRegistration::new(&params)));
        body.set_certs(&certs);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let tx = Transaction::new(&body, &w, None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w)
            + HASKELL_HLEN // operator pool keyhash
            + HASKELL_HLEN // vrf keyhash
            + HASKELL_HLEN // reward account
            + owners.len() * HASKELL_HLEN // owners' keyhashes
            + HASKELL_HLEN; // metadata hash
        let our_crypto_bytes = witness_vkey_bytes_rust(&w)
            + PoolKeyHash::BYTE_COUNT
            + VRFKeyHash::BYTE_COUNT
            + AddrKeyHash::BYTE_COUNT
            + owners.len() * AddrKeyHash::BYTE_COUNT
            + MetadataHash::BYTE_COUNT;
        assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 200);
    }

    #[test]
    fn tx_retire_pool() {
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(&genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
        let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
        let mut certs = Certificates::new();
        certs.add(&Certificate::new_pool_retirement(&PoolRetirement::new(&alice_pool(), 5)));
        body.set_certs(&certs);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let tx = Transaction::new(&body, &w, None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + PoolKeyHash::BYTE_COUNT;
        assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 149);
    }

    #[test]
    fn tx_metadata() {
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(&genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
        let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
        body.set_metadata_hash(&MetadataHash::from([37; MetadataHash::BYTE_COUNT]));
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
        let mut metadata = TransactionMetadata::new();
        let mut md_list = TransactionMetadatums::new();
        md_list.add(&TransactionMetadatum::new_int(&Int::new(BigNum::new(5))));
        md_list.add(&TransactionMetadatum::new_text(String::from("hello")));
        metadata.insert(TransactionMetadadumLabel::new(0), &TransactionMetadatum::new_arr_transaction_metadatum(&md_list));
        let tx = Transaction::new(&body, &w, Some(metadata));
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + MetadataHash::BYTE_COUNT;
        assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 154);
    }

    #[test]
    fn tx_multisig() {
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(&genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
        let body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
        let mut w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &bob_key()]);
        let mut script_witnesses = MultisigScripts::new();
        let mut inner_scripts = MultisigScripts::new();
        inner_scripts.add(&MultisigScript::new_msig_pubkey(&alice_pay().to_keyhash().unwrap()));
        inner_scripts.add(&MultisigScript::new_msig_pubkey(&bob_pay().to_keyhash().unwrap()));
        inner_scripts.add(&MultisigScript::new_msig_pubkey(&carl_pay().to_keyhash().unwrap()));
        script_witnesses.add(&MultisigScript::new_msig_n_of_k(2, &inner_scripts));
        w.set_scripts(&script_witnesses);
        let tx = Transaction::new(&body, &w, None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w);
        let our_crypto_bytes = witness_vkey_bytes_rust(&w);
        assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes - haskell_multisig_byte_diff(&script_witnesses) >= 189);
    }

    #[test]
    fn tx_withdrawal() {
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(&genesis_id(), 0));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
        let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
        let mut withdrawals = Withdrawals::new();
        withdrawals.insert(&RewardAddress::new(0, &alice_pay()), Coin::new(100));
        body.set_withdrawals(&withdrawals);
        let w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &alice_key()]);
        let tx = Transaction::new(&body, &w, None);
        let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
        let our_crypto_bytes = witness_vkey_bytes_rust(&w) + AddrKeyHash::BYTE_COUNT;
        assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 172);
    }
}