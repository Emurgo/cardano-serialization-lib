use crate::TransactionOutputBuilder;
use crate::*;

// based off tx test vectors (https://gist.github.com/KtorZ/5a2089df0915f21aca368d12545ab230)

// However, they don't match due to serialization differences in definite vs indefinite
// CBOR lengths for maps/arrays, thus for now we've got all the tests as >= instead.
// It's possible they're still off by a byte or two somewhere.

#[test]
fn tx_simple_utxo() {
    // # Vector #1: simple transaction
    let mut inputs = TransactionInputs::new();
    inputs.add(&TransactionInput::new(
        &TransactionHash::from_bytes(
            hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
                .unwrap(),
        )
        .unwrap(),
        0,
    ));
    let mut outputs = TransactionOutputs::new();

    outputs.add(
        &TransactionOutputBuilder::new()
            .with_address(
                &Address::from_bytes(
                    hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c")
                        .unwrap(),
                )
                .unwrap(),
            )
            .next()
            .unwrap()
            .with_coin(&BigNum(1))
            .build()
            .unwrap(),
    );
    let body = TransactionBody::new(&inputs, &outputs, &BigNum(94002), Some(10));

    let mut w = TransactionWitnessSet::new();
    let mut vkw = Vkeywitnesses::new();
    vkw.add(&make_vkey_witness(
        &hash_transaction(&body),
        &PrivateKey::from_normal_bytes(
            &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
                .unwrap(),
        )
        .unwrap(),
    ));
    w.set_vkeys(&vkw);

    let signed_tx = Transaction::new(&body, &w, None);

    let linear_fee = LinearFee::new(&BigNum(500), &BigNum(2));
    assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "84a400d90102818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a00016f32030aa100d9010281825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee58406d68d8b7b2ee54f1f46b64e3f61a14f840be2ec125c858ec917f634a1eb898a51660654839226016a2588d39920e6dfe1b66d917027f198b5eb887d20f4ac805f5f6"
        );
    assert_eq!(
        min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
        "97502" // todo: compare to Haskell fee to make sure the diff is not too big
    );
}

#[test]
fn tx_simple_byron_utxo() {
    let mut inputs = TransactionInputs::new();
    inputs.add(&TransactionInput::new(
        &TransactionHash::from_bytes(
            hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
                .unwrap(),
        )
        .unwrap(),
        0,
    ));
    let mut outputs = TransactionOutputs::new();

    outputs.add(
        &TransactionOutputBuilder::new()
            .with_address(
                &Address::from_bytes(
                    hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c")
                        .unwrap(),
                )
                .unwrap(),
            )
            .next()
            .unwrap()
            .with_coin(&BigNum(1))
            .build()
            .unwrap(),
    );
    let body = TransactionBody::new(&inputs, &outputs, &BigNum(112002), Some(10));

    let mut w = TransactionWitnessSet::new();
    let mut bootstrap_wits = BootstrapWitnesses::new();
    bootstrap_wits.add(&make_icarus_bootstrap_witness(
            &hash_transaction(&body),
            &ByronAddress::from_base58("Ae2tdPwUPEZ6r6zbg4ibhFrNnyKHg7SYuPSfDpjKxgvwFX9LquRep7gj7FQ").unwrap(),
            &Bip32PrivateKey::from_bytes(
                &hex::decode("d84c65426109a36edda5375ea67f1b738e1dacf8629f2bb5a2b0b20f3cd5075873bf5cdfa7e533482677219ac7d639e30a38e2e645ea9140855f44ff09e60c52c8b95d0d35fe75a70f9f5633a3e2439b2994b9e2bc851c49e9f91d1a5dcbb1a3").unwrap()
            ).unwrap()
        ));
    w.set_bootstraps(&bootstrap_wits);

    let signed_tx = Transaction::new(&body, &w, None);

    let linear_fee = LinearFee::new(&BigNum(500), &BigNum(2));
    assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "84a400d90102818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a0001b582030aa102d9010281845820473811afd4d939b337c9be1a2ceeb2cb2c75108bddf224c5c21c51592a7b204a58408b4ca7a71340bc6441f0e390122d53aba154b7e2b432ec2927ed8db7395d3d9347989aa1fca4823c991c1ef309570a0bbdf62155e3dba376fae9827cb465f5055820c8b95d0d35fe75a70f9f5633a3e2439b2994b9e2bc851c49e9f91d1a5dcbb1a341a0f5f6"
        );
    assert_eq!(
        min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
        "115502" // todo: compare to Haskell fee to make sure the diff is not too big
    );
}

#[test]
fn tx_multi_utxo() {
    // # Vector #2: multiple outputs and inputs
    let mut inputs = TransactionInputs::new();
    inputs.add(&TransactionInput::new(
        &TransactionHash::from_bytes(
            hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
                .unwrap(),
        )
        .unwrap(),
        42,
    ));
    inputs.add(&TransactionInput::new(
        &TransactionHash::from_bytes(
            hex::decode("82839f8200d81858248258203b40265111d8bb3c3c608d95b3a0bf83461ace32")
                .unwrap(),
        )
        .unwrap(),
        7,
    ));
    let mut outputs = TransactionOutputs::new();

    outputs.add(
        &TransactionOutputBuilder::new()
            .with_address(
                &Address::from_bytes(
                    hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c")
                        .unwrap(),
                )
                .unwrap(),
            )
            .next()
            .unwrap()
            .with_coin(&BigNum(289))
            .build()
            .unwrap(),
    );
    outputs.add(
        &TransactionOutputBuilder::new()
            .with_address(
                &Address::from_bytes(
                    hex::decode("61bcd18fcffa797c16c007014e2b8553b8b9b1e94c507688726243d611")
                        .unwrap(),
                )
                .unwrap(),
            )
            .next()
            .unwrap()
            .with_coin(&BigNum(874551452))
            .build()
            .unwrap(),
    );
    let body = TransactionBody::new(&inputs, &outputs, &BigNum(183502), Some(999));

    let mut w = TransactionWitnessSet::new();
    let mut vkw = Vkeywitnesses::new();
    vkw.add(&make_vkey_witness(
        &hash_transaction(&body),
        &PrivateKey::from_normal_bytes(
            &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
                .unwrap(),
        )
        .unwrap(),
    ));
    vkw.add(&make_vkey_witness(
        &hash_transaction(&body),
        &PrivateKey::from_normal_bytes(
            &hex::decode("13fe79205e16c09536acb6f0524d04069f380329d13949698c5f22c65c989eb4")
                .unwrap(),
        )
        .unwrap(),
    ));
    w.set_vkeys(&vkw);

    let signed_tx = Transaction::new(&body, &w, None);

    let linear_fee = LinearFee::new(&BigNum(500), &BigNum(2));
    assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "84a400d90102828258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7182a82582082839f8200d81858248258203b40265111d8bb3c3c608d95b3a0bf83461ace3207018282581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c19012182581d61bcd18fcffa797c16c007014e2b8553b8b9b1e94c507688726243d6111a3420989c021a0002ccce031903e7a100d9010282825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee584082eea9c7848c1136ebcb5fd5774d8dfc330c63b7f44b56a5cc5008887d3923df8785ebab92c230114099cf9b79a6c6c57ead026fa495d526731cc00caa3407088258206872b0a874acfe1cace12b20ea348559a7ecc912f2fc7f674f43481df973d92c5840f8861b68b3f966b6b63cbd3f7abf18efa18620aee9e730dea75d2d1cb0668988486f852f7743f6b5cc841c62d11440073706b52b408c0d776a411e2a0dd0da0af5f6"
        );
    assert_eq!(
        min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
        "187002" // todo: compare to Haskell fee to make sure the diff is not too big
    );
}

#[test]
fn tx_register_stake() {
    // # Vector #3: with stake pool registration certificate
    let network = 1;
    let mut inputs = TransactionInputs::new();
    inputs.add(&TransactionInput::new(
        &TransactionHash::from_bytes(
            hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
                .unwrap(),
        )
        .unwrap(),
        0,
    ));
    let mut outputs = TransactionOutputs::new();

    outputs.add(
        &TransactionOutputBuilder::new()
            .with_address(
                &Address::from_bytes(
                    hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c")
                        .unwrap(),
                )
                .unwrap(),
            )
            .next()
            .unwrap()
            .with_coin(&BigNum(1))
            .build()
            .unwrap(),
    );
    let mut body = TransactionBody::new(&inputs, &outputs, &BigNum(266002), Some(10));

    let mut certs = Certificates::new();

    let mut pool_owners = Ed25519KeyHashes::new();
    pool_owners.add(
        &PublicKey::from_bytes(
            &hex::decode("54d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f3")
                .unwrap(),
        )
        .unwrap()
        .hash(),
    );
    let registration_cert = PoolRegistration::new(&PoolParams::new(
        &PublicKey::from_bytes(
            &hex::decode("b24c040e65994bd5b0621a060166d32d356ef4be3cc1f848426a4cf386887089")
                .unwrap(),
        )
        .unwrap()
        .hash(), // operator
        &VRFKeyHash::from(blake2b256(
            &hex::decode("fbf6d41985670b9041c5bf362b5262cf34add5d265975de176d613ca05f37096")
                .unwrap(),
        )), // vrf_keyhash
        &BigNum(1000000),                             // pledge
        &BigNum(1000000),                             // cost
        &UnitInterval::new(&BigNum(3), &BigNum(100)), // margin
        &RewardAddress::new(
            network,
            &Credential::from_keyhash(
                &PublicKey::from_bytes(
                    &hex::decode(
                        "54d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f3",
                    )
                    .unwrap(),
                )
                .unwrap()
                .hash(),
            ),
        ), // reward_address
        &pool_owners,                                 // pool_owners
        &Relays::new(),                               // relays
        None,                                         // metadata
    ));
    certs.add(&Certificate::new_pool_registration(&registration_cert));
    body.set_certs(&certs);

    let mut w = TransactionWitnessSet::new();
    let mut vkw = Vkeywitnesses::new();
    // input key witness
    vkw.add(&make_vkey_witness(
        &hash_transaction(&body),
        &PrivateKey::from_normal_bytes(
            &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
                .unwrap(),
        )
        .unwrap(),
    ));
    // operator key witness
    vkw.add(&make_vkey_witness(
        &hash_transaction(&body),
        &PrivateKey::from_normal_bytes(
            &hex::decode("2363f3660b9f3b41685665bf10632272e2d03c258e8a5323436f0f3406293505")
                .unwrap(),
        )
        .unwrap(),
    ));
    // owner key witness
    vkw.add(&make_vkey_witness(
        &hash_transaction(&body),
        &PrivateKey::from_normal_bytes(
            &hex::decode("5ada7f4d92bce1ee1707c0a0e211eb7941287356e6ed0e76843806e307b07c8d")
                .unwrap(),
        )
        .unwrap(),
    ));
    w.set_vkeys(&vkw);

    let signed_tx = Transaction::new(&body, &w, None);

    let linear_fee = LinearFee::new(&BigNum(500), &BigNum(2));
    assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "84a500d90102818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a00040f12030a04d90102818a03581c1c13374874c68016df54b1339b6cacdd801098431e7659b24928efc15820bd0000f498ccacdc917c28274cba51c415f3f21931ff41ca8dc1197499f8e1241a000f42401a000f4240d81e82031864581de151df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af70d9010281581c51df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af7080f6a100d9010283825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee584081e223791960a07f401c378dce048ea658a155510ae6541c6cb692ed41ee45b40b913d1428a94af145885639f8acf99549f7b29af1e34997b9cb8ad05fe6e50a825820b24c040e65994bd5b0621a060166d32d356ef4be3cc1f848426a4cf38688708958401a246b6a4d63e83bd4904ac3b787797ba54238aab8e733b75b5ab2e465c46fce67e4403169a53239e7036f87f0518ec4414a10e45a1aa1788322d2777f59c30182582054d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f35840ea9518346e8515aea6c16e7076a1d0a637582f736c0b65abd1f4adccd8920f2da699e8602029cc608da2ba46b6a6e61f41166b12ae17c922114c040facd90b07f5f6"
        );
    assert_eq!(
        min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
        "275502" // todo: compare to Haskell fee to make sure the diff is not too big
    );
}

// #[test]
// fn tx_delegate_stake() {
//     let mut inputs = TransactionInputs::new();
//     inputs.add(&TransactionInput::new(&genesis_id(), 0));
//     let mut outputs = TransactionOutputs::new();
//     outputs.add(&TransactionOutput::new(&alice_addr(), BigNum(10)));
//     let mut body = TransactionBody::new(&inputs, &outputs, BigNum(94), 10);
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
//     outputs.add(&TransactionOutput::new(&alice_addr(), BigNum(10)));
//     let mut body = TransactionBody::new(&inputs, &outputs, BigNum(94), 10);
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
//     outputs.add(&TransactionOutput::new(&alice_addr(), BigNum(10)));
//     let mut body = TransactionBody::new(&inputs, &outputs, BigNum(94), 10);
//     let mut certs = Certificates::new();
//     let mut owners = Ed25519KeyHashes::new();
//     owners.add(&(alice_stake().to_keyhash().unwrap()));
//     let mut relays = Relays::new();
//     relays.add(&Relay::new_single_host_name(&SingleHostName::new(None, String::from("relay.io"))));
//     let params = PoolParams::new(
//         &alice_pool(),
//         &VRFKeyHash::from([0u8; VRFKeyHash::BYTE_COUNT]),
//         BigNum(1),
//         BigNum(5),
//         &UnitInterval::new(BigNum(1), BigNum(10)),
//         &RewardAddress::new(NetworkInfo::testnet_preprod().network_id(), &alice_stake()),
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
//     outputs.add(&TransactionOutput::new(&alice_addr(), BigNum(10)));
//     let mut body = TransactionBody::new(&inputs, &outputs, BigNum(94), 10);
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
//     outputs.add(&TransactionOutput::new(&alice_addr(), BigNum(10)));
//     let mut body = TransactionBody::new(&inputs, &outputs, BigNum(94), 10);
//     body.set_metadata_hash(&MetadataHash::from([37; MetadataHash::BYTE_COUNT]));
//     let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
//     let mut metadata = TransactionMetadata::new();
//     let mut md_list = TransactionMetadatums::new();
//     md_list.add(&TransactionMetadatum::new_int(&Int::new(&BigNum(5))));
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
//     outputs.add(&TransactionOutput::new(&alice_addr(), BigNum(10)));
//     let body = TransactionBody::new(&inputs, &outputs, BigNum(94), 10);
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

#[test]
fn tx_withdrawal() {
    // # Vector #8: with reward withdrawal
    let mut inputs = TransactionInputs::new();
    inputs.add(&TransactionInput::new(
        &TransactionHash::from_bytes(
            hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
                .unwrap(),
        )
        .unwrap(),
        0,
    ));
    let mut outputs = TransactionOutputs::new();

    outputs.add(
        &TransactionOutputBuilder::new()
            .with_address(
                &Address::from_bytes(
                    hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c")
                        .unwrap(),
                )
                .unwrap(),
            )
            .next()
            .unwrap()
            .with_coin(&BigNum(1))
            .build()
            .unwrap(),
    );
    let mut body = TransactionBody::new(&inputs, &outputs, &BigNum(162502), Some(10));
    let mut withdrawals = Withdrawals::new();
    withdrawals.insert(
        &RewardAddress::from_address(
            &Address::from_bytes(
                hex::decode("e151df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af70").unwrap(),
            )
            .unwrap(),
        )
        .unwrap(),
        &BigNum(1337),
    );
    body.set_withdrawals(&withdrawals);

    let mut w = TransactionWitnessSet::new();
    let mut vkw = Vkeywitnesses::new();
    // input key witness
    vkw.add(&make_vkey_witness(
        &hash_transaction(&body),
        &PrivateKey::from_normal_bytes(
            &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
                .unwrap(),
        )
        .unwrap(),
    ));
    // withdrawal key witness
    vkw.add(&make_vkey_witness(
        &hash_transaction(&body),
        &PrivateKey::from_normal_bytes(
            &hex::decode("5ada7f4d92bce1ee1707c0a0e211eb7941287356e6ed0e76843806e307b07c8d")
                .unwrap(),
        )
        .unwrap(),
    ));
    w.set_vkeys(&vkw);

    let signed_tx = Transaction::new(&body, &w, None);

    let linear_fee = LinearFee::new(&BigNum(500), &BigNum(2));
    assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "84a500d90102818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a00027ac6030a05a1581de151df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af70190539a100d9010282825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee58406dda4d88a17c7b888d15eb29f0871e85f3c50e1ea4efcc0d7781f4db0ae11dd418abae42f7f62637cc54d21887ccb60dc3ccae545e7a25c7c553b3a91e9e6d0082582054d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f35840a24b4863189d5872fdb98529bbe6feae375031162786bda1244d73c54adafea0ace2f087f23a794af4f232651ba66071246ab5bc1e1b0b9a39044d0531eeac0ef5f6"
        );
    assert_eq!(
        min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
        "166002" // todo: compare to Haskell fee to make sure the diff is not too big
    );
}

fn exunits(mem: u64, steps: u64) -> ExUnits {
    ExUnits::new(&BigNum(mem), &BigNum(steps))
}

fn subcoin(num: u64, denum: u64) -> SubCoin {
    SubCoin::new(&BigNum(num), &BigNum(denum))
}

fn exunit_prices(mem_prices: (u64, u64), step_prices: (u64, u64)) -> ExUnitPrices {
    ExUnitPrices::new(
        &subcoin(mem_prices.0, mem_prices.1),
        &subcoin(step_prices.0, step_prices.1),
    )
}

fn _calculate_ex_units_ceil_cost(
    mem: u64,
    steps: u64,
    mem_prices: (u64, u64),
    step_prices: (u64, u64),
) -> Coin {
    let ex_units = exunits(mem, steps);
    let ex_unit_prices = exunit_prices(mem_prices, step_prices);
    calculate_ex_units_ceil_cost(&ex_units, &ex_unit_prices).unwrap()
}

#[test]
fn test_calc_ex_units_cost() {
    // 10 * (2/1) + 20 * (3/1) = 10 * 2 + 20 * 3 = 20 + 60
    assert_eq!(
        _calculate_ex_units_ceil_cost(10, 20, (2, 1), (3, 1)),
        BigNum(80),
    );
    // 22 * (12/6) + 33 * (33/11) = 22 * 2 + 33 * 3 = 44 + 99 = 143
    assert_eq!(
        _calculate_ex_units_ceil_cost(22, 33, (12, 6), (33, 11)),
        BigNum(143),
    );
    // 10 * (5/7) + 20 * (9/13) = 50/7 + 180/13 = 650/91 + 1260/91 = 1910/91 = ceil(20.98) = 21
    assert_eq!(
        _calculate_ex_units_ceil_cost(10, 20, (5, 7), (9, 13)),
        BigNum(21),
    );
    // 22 * (7/5) + 33 * (13/9) = 154/5 + 429/9 = 1386/45 + 2145/45 = 3531/45 = ceil(78.46) = 79
    assert_eq!(
        _calculate_ex_units_ceil_cost(22, 33, (7, 5), (13, 9)),
        BigNum(79),
    );
}
