use super::*;
use utils::*;

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

    pub fn new(coefficient: &Coin, constant: &Coin) -> Self {
        Self {
            constant: constant.clone(),
            coefficient: coefficient.clone(),
        }
    }
}

#[wasm_bindgen]
pub fn min_fee(tx: &Transaction, linear_fee: &LinearFee) -> Result<Coin, JsValue> {
    Coin::new(tx.to_bytes().len() as u64)
        .checked_mul(&linear_fee.coefficient())?
        .checked_add(&linear_fee.constant())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::*;
    use address::*;

    // based off tx test vectors (https://gist.github.com/KtorZ/5a2089df0915f21aca368d12545ab230)

    // However, they don't match due to serialization differences in definite vs indefinite
    // CBOR lengths for maps/arrays, thus for now we've got all the tests as >= instead.
    // It's possible they're still off by a byte or two somewhere.

    #[test]
    fn tx_simple_utxo() { // # Vector #1: simple transaction
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(
            &TransactionHash::from_bytes(hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7").unwrap()).unwrap(),
            0
        ));
        let mut outputs = TransactionOutputs::new();

        outputs.add(&TransactionOutput::new(
            &Address::from_bytes(
                hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c").unwrap()
            ).unwrap(),
            &Coin::new(1)
        ));
        let body = TransactionBody::new(&inputs, &outputs, &Coin::new(94002), 10);

        let mut w = TransactionWitnessSet::new();
        let mut vkw = Vkeywitnesses::new();
        vkw.add(&make_vkey_witness(
            &hash_transaction(&body),
            // TODO: not actually sure this is the right key to use when signing. Tx vectors is not clear
            &PrivateKey::from_normal_bytes(
                &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a").unwrap()
            ).unwrap()
        ));
        w.set_vkeys(&vkw);

        let signed_tx = Transaction::new(
            &body,
            &w,
            None,
        );

        let linear_fee = LinearFee::new(&Coin::new(500), &Coin::new(2));
        assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "83a400818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a00016f32030aa10081825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee5840fae5de40c94d759ce13bf9886262159c4f26a289fd192e165995b785259e503f6887bf39dfa23a47cf163784c6eee23f61440e749bc1df3c73975f5231aeda0ff6"
        );
        assert_eq!(
            min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
            "94002" // todo: compare to Haskell fee to make sure the diff is not too big
        );
    }

    // #[test]
    // fn tx_simple_byron_utxo() {
    //     let mut inputs = TransactionInputs::new();
    //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
    //     let mut outputs = TransactionOutputs::new();
    //     outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
    //     let body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);

    //     // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
    //     let entropy = [0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12];
    //     let root_key = Bip32PrivateKey::from_bip39_entropy(&entropy, &[]);
    //     let byron_addr = ByronAddress::from_icarus_key(&root_key.to_public(), 0b001);
    //     let w = make_mock_byron_witnesses_vkey(
    //         &body,
    //         &byron_addr, // Ae2tdPwUPEZB3mTcbvEYWmxEyn3yRmePa7YV5TKmrtPyU3rcSao4k6J216Q
    //         vec![&root_key]
    //     );
    //     assert_eq!(
    //         hex::encode(w.bootstraps.unwrap().get(0).to_bytes()),
    //         "855820cf76399a210de8720e9fa894e45e41e29ab525e30bc402801c076250d1585bcd5840dbc0d07297733aa0c71e942e3dddb648eb6cc96a9a35ea686673b534b8cc20e1fab4c736d2cb711b270a912b216ba55167cf3a5d400bc08671cc9335a66aa10d582091e248de509c070d812ab2fda57860ac876bc489192c1ef4ce253c197ee219a44683008200525441a0"
    //     );
    // }

    #[test]
    fn tx_multi_utxo() { // # Vector #2: multiple outputs and inputs
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(
            &TransactionHash::from_bytes(hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7").unwrap()).unwrap(),
            42
        ));
        inputs.add(&TransactionInput::new(
            &TransactionHash::from_bytes(hex::decode("82839f8200d81858248258203b40265111d8bb3c3c608d95b3a0bf83461ace32").unwrap()).unwrap(),
            7
        ));
        let mut outputs = TransactionOutputs::new();
        outputs.add(&TransactionOutput::new(
            &Address::from_bytes(
                hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c").unwrap()
            ).unwrap(),
            &Coin::new(289)
        ));
        outputs.add(&TransactionOutput::new(
            &Address::from_bytes(
                hex::decode("61bcd18fcffa797c16c007014e2b8553b8b9b1e94c507688726243d611").unwrap()
            ).unwrap(),
            &Coin::new(874551452)
        ));
        let body = TransactionBody::new(&inputs, &outputs, &Coin::new(183502), 999);

        let mut w = TransactionWitnessSet::new();
        let mut vkw = Vkeywitnesses::new();
        vkw.add(&make_vkey_witness(
            &hash_transaction(&body),
            &PrivateKey::from_normal_bytes(
                &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a").unwrap()
            ).unwrap()
        ));
        vkw.add(&make_vkey_witness(
            &hash_transaction(&body),
            &PrivateKey::from_normal_bytes(
                &hex::decode("13fe79205e16c09536acb6f0524d04069f380329d13949698c5f22c65c989eb4").unwrap()
            ).unwrap()
        ));
        w.set_vkeys(&vkw);

        let signed_tx = Transaction::new(
            &body,
            &w,
            None,
        );

        let linear_fee = LinearFee::new(&Coin::new(500), &Coin::new(2));
        assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "83a400828258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7182a82582082839f8200d81858248258203b40265111d8bb3c3c608d95b3a0bf83461ace3207018282581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c19012182581d61bcd18fcffa797c16c007014e2b8553b8b9b1e94c507688726243d6111a3420989c021a0002ccce031903e7a10082825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee58401ec3e56008650282ba2e1f8a20e81707810b2d0973c4d42a1b4df65b732bda81567c7824904840b2554d2f33861da5d70588a29d33b2b61042e3c3445301d8008258206872b0a874acfe1cace12b20ea348559a7ecc912f2fc7f674f43481df973d92c5840a0718fb5b37d89ddf926c08e456d3f4c7f749e91f78bb3e370751d5b632cbd20d38d385805291b1ef2541b02543728a235e01911f4b400bfb50e5fce589de907f6"
        );
        assert_eq!(
            min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
            "183502" // todo: compare to Haskell fee to make sure the diff is not too big
        );
    }

    #[test]
    fn tx_register_stake() { // # Vector #3: with stake pool registration certificate
        let network = 1;
        let mut inputs = TransactionInputs::new();
        inputs.add(&TransactionInput::new(
            &TransactionHash::from_bytes(hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7").unwrap()).unwrap(),
            0
        ));
        let mut outputs = TransactionOutputs::new();

        outputs.add(&TransactionOutput::new(
            &Address::from_bytes(
                hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c").unwrap()
            ).unwrap(),
            &Coin::new(1)
        ));
        let mut body = TransactionBody::new(&inputs, &outputs, &Coin::new(266002), 10);

        let mut certs = Certificates::new();

        let mut pool_owners = Ed25519KeyHashes::new();
        pool_owners.add(&PublicKey::from_bytes(
            &hex::decode("54d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f3").unwrap()
        ).unwrap().hash());
        let registration_cert = PoolRegistration::new(
            &PoolParams::new(
                &PublicKey::from_bytes(
                    &hex::decode("b24c040e65994bd5b0621a060166d32d356ef4be3cc1f848426a4cf386887089").unwrap()
                ).unwrap().hash(), // operator
                &VRFKeyHash::from(
                    blake2b224(&hex::decode("fbf6d41985670b9041c5bf362b5262cf34add5d265975de176d613ca05f37096").unwrap())
                ), // vrf_keyhash
                &Coin::new(1000000), // pledge
                &Coin::new(1000000), // cost
                &UnitInterval::new(
                    BigNum::new(3),
                    BigNum::new(100),
                ), // margin
                &RewardAddress::new(
                    network,
                    &StakeCredential::from_keyhash(
                        &PublicKey::from_bytes(
                            &hex::decode("54d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f3").unwrap()
                        ).unwrap().hash()
                    ),
                ), // reward_address
                &pool_owners, // pool_owners
                &Relays::new(), // relays
                None, // metadata
            )
        );
        certs.add(&Certificate::new_pool_registration(&registration_cert));
        body.set_certs(&certs);

        let mut w = TransactionWitnessSet::new();
        let mut vkw = Vkeywitnesses::new();
        // input key witness
        vkw.add(&make_vkey_witness(
            &hash_transaction(&body),
            &PrivateKey::from_normal_bytes(
                &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a").unwrap()
            ).unwrap()
        ));
        // operator key witness
        vkw.add(&make_vkey_witness(
            &hash_transaction(&body),
            &PrivateKey::from_normal_bytes(
                &hex::decode("2363f3660b9f3b41685665bf10632272e2d03c258e8a5323436f0f3406293505").unwrap()
            ).unwrap()
        ));
        // owner key witness
        vkw.add(&make_vkey_witness(
            &hash_transaction(&body),
            &PrivateKey::from_normal_bytes(
                &hex::decode("5ada7f4d92bce1ee1707c0a0e211eb7941287356e6ed0e76843806e307b07c8d").unwrap()
            ).unwrap()
        ));
        w.set_vkeys(&vkw);

        let signed_tx = Transaction::new(
            &body,
            &w,
            None,
        );

        let linear_fee = LinearFee::new(&Coin::new(500), &Coin::new(2));
        assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "83a500818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a00040f12030a04818a03581c1c13374874c68016df54b1339b6cacdd801098431e7659b24928efc1581ca3fd9f5904987ec1ea8001002833ea5013395ff62021c3e3fbb283aa1a000f42401a000f4240d81e82031864581de151df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af7081581c51df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af7080f6a10083825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee584042bf0438546b7e1489120c1a53372c383b5717a2d0f4811e30bbef648321efff4695cc5723361d55f7d9681cd2362023c74f964b3d947938f852910ec7649209825820b24c040e65994bd5b0621a060166d32d356ef4be3cc1f848426a4cf38688708958405a074e990bcdef2c181ece8050d40593f5666effed3180fa8cb84539e0a944ab1dfc13c3a257bb77e9183362955695483af8982bde3100f44b95299020d75b0682582054d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f358402ea853c2cd8628db5217afe27bbdd88944c73287c988b3b7c5f6123e2ee3e8ba3d7c437b4f16d6ac613a87258d99dfa49aae623a08b6da29a51ec18047d3a30af6"
        );
        assert_eq!(
            min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
            "267002" // todo: compare to Haskell fee to make sure the diff is not too big
        );
    }

    // #[test]
    // fn tx_delegate_stake() {
    //     let mut inputs = TransactionInputs::new();
    //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
    //     let mut outputs = TransactionOutputs::new();
    //     outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
    //     let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
    //     let mut certs = Certificates::new();
    //     certs.add(&Certificate::new_stake_delegation(&StakeDelegation::new(&bob_stake(), &alice_pool())));
    //     body.set_certs(&certs);
    //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &bob_key()]);
    //     let tx = Transaction::new(&body, &w, None);
    //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN * 2;
    //     let our_crypto_bytes = witness_vkey_bytes_rust(&w) + Ed25519KeyHash::BYTE_COUNT + Ed25519KeyHash::BYTE_COUNT;
    //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 178);
    // }

    // #[test]
    // fn tx_deregister_stake() {
    //     let mut inputs = TransactionInputs::new();
    //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
    //     let mut outputs = TransactionOutputs::new();
    //     outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
    //     let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
    //     let mut certs = Certificates::new();
    //     certs.add(&Certificate::new_stake_deregistration(&StakeDeregistration::new(&alice_pay())));
    //     body.set_certs(&certs);
    //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
    //     let tx = Transaction::new(&body, &w, None);
    //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
    //     let our_crypto_bytes = witness_vkey_bytes_rust(&w) + Ed25519KeyHash::BYTE_COUNT;
    //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 150);
    // }

    // #[test]
    // fn tx_register_pool() {
    //     let mut inputs = TransactionInputs::new();
    //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
    //     let mut outputs = TransactionOutputs::new();
    //     outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
    //     let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
    //     let mut certs = Certificates::new();
    //     let mut owners = Ed25519KeyHashes::new();
    //     owners.add(&(alice_stake().to_keyhash().unwrap()));
    //     let mut relays = Relays::new();
    //     relays.add(&Relay::new_single_host_name(&SingleHostName::new(None, String::from("relay.io"))));
    //     let params = PoolParams::new(
    //         &alice_pool(),
    //         &VRFKeyHash::from([0u8; VRFKeyHash::BYTE_COUNT]),
    //         Coin::new(1),
    //         Coin::new(5),
    //         &UnitInterval::new(BigNum::new(1), BigNum::new(10)),
    //         &RewardAddress::new(0, &alice_stake()),
    //         &owners,
    //         &relays,
    //         Some(PoolMetadata::new(String::from("alice.pool"), &MetadataHash::from([0u8; MetadataHash::BYTE_COUNT])))
    //     );
    //     certs.add(&Certificate::new_pool_registration(&PoolRegistration::new(&params)));
    //     body.set_certs(&certs);
    //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
    //     let tx = Transaction::new(&body, &w, None);
    //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w)
    //         + HASKELL_HLEN // operator pool keyhash
    //         + HASKELL_HLEN // vrf keyhash
    //         + HASKELL_HLEN // reward account
    //         + owners.len() * HASKELL_HLEN // owners' keyhashes
    //         + HASKELL_HLEN; // metadata hash
    //     let our_crypto_bytes = witness_vkey_bytes_rust(&w)
    //         + Ed25519KeyHash::BYTE_COUNT
    //         + VRFKeyHash::BYTE_COUNT
    //         + Ed25519KeyHash::BYTE_COUNT
    //         + owners.len() * Ed25519KeyHash::BYTE_COUNT
    //         + MetadataHash::BYTE_COUNT;
    //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 200);
    // }

    // #[test]
    // fn tx_retire_pool() {
    //     let mut inputs = TransactionInputs::new();
    //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
    //     let mut outputs = TransactionOutputs::new();
    //     outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
    //     let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
    //     let mut certs = Certificates::new();
    //     certs.add(&Certificate::new_pool_retirement(&PoolRetirement::new(&alice_pool(), 5)));
    //     body.set_certs(&certs);
    //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
    //     let tx = Transaction::new(&body, &w, None);
    //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
    //     let our_crypto_bytes = witness_vkey_bytes_rust(&w) + Ed25519KeyHash::BYTE_COUNT;
    //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 149);
    // }

    // #[test]
    // fn tx_metadata() {
    //     let mut inputs = TransactionInputs::new();
    //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
    //     let mut outputs = TransactionOutputs::new();
    //     outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
    //     let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
    //     body.set_metadata_hash(&MetadataHash::from([37; MetadataHash::BYTE_COUNT]));
    //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
    //     let mut metadata = TransactionMetadata::new();
    //     let mut md_list = TransactionMetadatums::new();
    //     md_list.add(&TransactionMetadatum::new_int(&Int::new(BigNum::new(5))));
    //     md_list.add(&TransactionMetadatum::new_text(String::from("hello")));
    //     metadata.insert(TransactionMetadatumLabel::new(0), &TransactionMetadatum::new_arr_transaction_metadatum(&md_list));
    //     let tx = Transaction::new(&body, &w, Some(metadata));
    //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
    //     let our_crypto_bytes = witness_vkey_bytes_rust(&w) + MetadataHash::BYTE_COUNT;
    //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 154);
    // }

    // #[test]
    // fn tx_multisig() {
    //     let mut inputs = TransactionInputs::new();
    //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
    //     let mut outputs = TransactionOutputs::new();
    //     outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
    //     let body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
    //     let mut w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &bob_key()]);
    //     let mut script_witnesses = MultisigScripts::new();
    //     let mut inner_scripts = MultisigScripts::new();
    //     inner_scripts.add(&MultisigScript::new_msig_pubkey(&alice_pay().to_keyhash().unwrap()));
    //     inner_scripts.add(&MultisigScript::new_msig_pubkey(&bob_pay().to_keyhash().unwrap()));
    //     inner_scripts.add(&MultisigScript::new_msig_pubkey(&carl_pay().to_keyhash().unwrap()));
    //     script_witnesses.add(&MultisigScript::new_msig_n_of_k(2, &inner_scripts));
    //     w.set_scripts(&script_witnesses);
    //     let tx = Transaction::new(&body, &w, None);
    //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w);
    //     let our_crypto_bytes = witness_vkey_bytes_rust(&w);
    //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes - haskell_multisig_byte_diff(&script_witnesses) >= 189);
    // }

    // #[test]
    // fn tx_withdrawal() {
    //     let mut inputs = TransactionInputs::new();
    //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
    //     let mut outputs = TransactionOutputs::new();
    //     outputs.add(&TransactionOutput::new(&alice_addr(), Coin::new(10)));
    //     let mut body = TransactionBody::new(&inputs, &outputs, Coin::new(94), 10);
    //     let mut withdrawals = Withdrawals::new();
    //     withdrawals.insert(&RewardAddress::new(0, &alice_pay()), Coin::new(100));
    //     body.set_withdrawals(&withdrawals);
    //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &alice_key()]);
    //     let tx = Transaction::new(&body, &w, None);
    //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
    //     let our_crypto_bytes = witness_vkey_bytes_rust(&w) + Ed25519KeyHash::BYTE_COUNT;
    //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 172);
    // }
}