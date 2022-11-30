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
pub fn min_fee(tx: &Transaction, linear_fee: &LinearFee) -> Result<Coin, JsError> {
    min_fee_for_size(tx.to_bytes().len(), linear_fee)
}

pub fn min_fee_for_size(size: usize, linear_fee: &LinearFee) -> Result<Coin, JsError> {
    BigNum::from(size)
        .checked_mul(&linear_fee.coefficient())?
        .checked_add(&linear_fee.constant())
}

#[wasm_bindgen]
pub fn calculate_ex_units_ceil_cost(
    ex_units: &ExUnits,
    ex_unit_prices: &ExUnitPrices,
) -> Result<Coin, JsError> {
    type Ratio = (BigInt, BigInt);
    fn mult(sc: &SubCoin, x: &BigNum) -> Result<Ratio, JsError> {
        let n: BigInt = BigInt::from_str(&sc.numerator.to_str())?;
        let d: BigInt = BigInt::from_str(&sc.denominator.to_str())?;
        let m: BigInt = BigInt::from_str(&x.to_str())?;
        Ok((n.mul(&m), d))
    }
    fn sum(a: &Ratio, b: &Ratio) -> Ratio {
        // Ratio Addition: a/x + b/y = ((a*y) + (b*x))/(x*y)
        let (a_num, a_denum) = &a;
        let (b_num, b_denum) = &b;
        if a_num.is_zero() {
            return b.clone();
        }
        if b_num.is_zero() {
            return a.clone();
        }
        let a_num_fixed = &a_num.mul(b_denum);
        let b_num_fixed = &b_num.mul(a_denum);
        let a_b_num_sum = a_num_fixed.add(b_num_fixed);
        let common_denum = a_denum.mul(b_denum);
        (a_b_num_sum, common_denum)
    }
    let mem_ratio: Ratio = mult(&ex_unit_prices.mem_price(), &ex_units.mem())?;
    let steps_ratio: Ratio = mult(&ex_unit_prices.step_price(), &ex_units.steps())?;
    let (total_num, total_denum) = sum(&mem_ratio, &steps_ratio);
    match total_num.div_ceil(&total_denum).as_u64() {
        Some(coin) => Ok(coin),
        _ => Err(JsError::from_str(&format!(
            "Failed to calculate ceil from ratio {}/{}",
            total_num.to_str(),
            total_denum.to_str(),
        ))),
    }
}

#[wasm_bindgen]
pub fn min_script_fee(tx: &Transaction, ex_unit_prices: &ExUnitPrices) -> Result<Coin, JsError> {
    if let Some(redeemers) = &tx.witness_set.redeemers {
        let total_ex_units: ExUnits = redeemers.total_ex_units()?;
        return calculate_ex_units_ceil_cost(&total_ex_units, ex_unit_prices);
    }
    Ok(Coin::zero())
}

#[cfg(test)]
mod tests {
    use super::output_builder::TransactionOutputBuilder;
    use super::*;
    use address::*;
    use crypto::*;

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
                .with_coin(&to_bignum(1))
                .build()
                .unwrap(),
        );
        let body = TransactionBody::new(&inputs, &outputs, &to_bignum(94002), Some(10));

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

        let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
        assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "84a400818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a00016f32030aa10081825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee5840fae5de40c94d759ce13bf9886262159c4f26a289fd192e165995b785259e503f6887bf39dfa23a47cf163784c6eee23f61440e749bc1df3c73975f5231aeda0ff5f6"
        );
        assert_eq!(
            min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
            "94502" // todo: compare to Haskell fee to make sure the diff is not too big
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
                .with_coin(&to_bignum(1))
                .build()
                .unwrap(),
        );
        let body = TransactionBody::new(&inputs, &outputs, &to_bignum(112002), Some(10));

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

        let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
        assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "84a400818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a0001b582030aa10281845820473811afd4d939b337c9be1a2ceeb2cb2c75108bddf224c5c21c51592a7b204a5840f0b04a852353eb23b9570df80b2aa6a61b723341ab45a2024a05b07cf58be7bdfbf722c09040db6cee61a0d236870d6ad1e1349ac999ec0db28f9471af25fb0c5820c8b95d0d35fe75a70f9f5633a3e2439b2994b9e2bc851c49e9f91d1a5dcbb1a341a0f5f6"
        );
        assert_eq!(
            min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
            "112502" // todo: compare to Haskell fee to make sure the diff is not too big
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
                .with_coin(&to_bignum(289))
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
                .with_coin(&to_bignum(874551452))
                .build()
                .unwrap(),
        );
        let body = TransactionBody::new(&inputs, &outputs, &to_bignum(183502), Some(999));

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

        let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
        assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "84a400828258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7182a82582082839f8200d81858248258203b40265111d8bb3c3c608d95b3a0bf83461ace3207018282581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c19012182581d61bcd18fcffa797c16c007014e2b8553b8b9b1e94c507688726243d6111a3420989c021a0002ccce031903e7a10082825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee58401ec3e56008650282ba2e1f8a20e81707810b2d0973c4d42a1b4df65b732bda81567c7824904840b2554d2f33861da5d70588a29d33b2b61042e3c3445301d8008258206872b0a874acfe1cace12b20ea348559a7ecc912f2fc7f674f43481df973d92c5840a0718fb5b37d89ddf926c08e456d3f4c7f749e91f78bb3e370751d5b632cbd20d38d385805291b1ef2541b02543728a235e01911f4b400bfb50e5fce589de907f5f6"
        );
        assert_eq!(
            min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
            "184002" // todo: compare to Haskell fee to make sure the diff is not too big
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
                .with_coin(&to_bignum(1))
                .build()
                .unwrap(),
        );
        let mut body = TransactionBody::new(&inputs, &outputs, &to_bignum(266002), Some(10));

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
            &to_bignum(1000000),                                // pledge
            &to_bignum(1000000),                                // cost
            &UnitInterval::new(&to_bignum(3), &to_bignum(100)), // margin
            &RewardAddress::new(
                network,
                &StakeCredential::from_keyhash(
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
            &pool_owners,                                       // pool_owners
            &Relays::new(),                                     // relays
            None,                                               // metadata
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

        let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
        assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "84a500818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a00040f12030a04818a03581c1c13374874c68016df54b1339b6cacdd801098431e7659b24928efc15820bd0000f498ccacdc917c28274cba51c415f3f21931ff41ca8dc1197499f8e1241a000f42401a000f4240d81e82031864581de151df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af7081581c51df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af7080f6a10083825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee5840a7f305d7e46abfe0f7bea6098bdf853ab9ce8e7aa381be5a991a871852f895a718e20614e22be43494c4dc3a8c78c56cd44fd38e0e5fff3e2fbd19f70402fc02825820b24c040e65994bd5b0621a060166d32d356ef4be3cc1f848426a4cf386887089584013c372f82f1523484eab273241d66d92e1402507760e279480912aa5f0d88d656d6f25d41e65257f2f38c65ac5c918a6735297741adfc718394994f20a1cfd0082582054d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f35840d326b993dfec21b9b3e1bd2f80adadc2cd673a1d8d033618cc413b0b02bc3b7efbb23d1ff99138abd05c398ce98e7983a641b50dcf0f64ed33f26c6e636b0b0ff5f6"
        );
        assert_eq!(
            min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
            "269502" // todo: compare to Haskell fee to make sure the diff is not too big
        );
    }

    // #[test]
    // fn tx_delegate_stake() {
    //     let mut inputs = TransactionInputs::new();
    //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
    //     let mut outputs = TransactionOutputs::new();
    //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
    //     let mut body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
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
    //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
    //     let mut body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
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
    //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
    //     let mut body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
    //     let mut certs = Certificates::new();
    //     let mut owners = Ed25519KeyHashes::new();
    //     owners.add(&(alice_stake().to_keyhash().unwrap()));
    //     let mut relays = Relays::new();
    //     relays.add(&Relay::new_single_host_name(&SingleHostName::new(None, String::from("relay.io"))));
    //     let params = PoolParams::new(
    //         &alice_pool(),
    //         &VRFKeyHash::from([0u8; VRFKeyHash::BYTE_COUNT]),
    //         to_bignum(1),
    //         to_bignum(5),
    //         &UnitInterval::new(to_bignum(1), to_bignum(10)),
    //         &RewardAddress::new(NetworkInfo::testnet().network_id(), &alice_stake()),
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
    //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
    //     let mut body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
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
    //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
    //     let mut body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
    //     body.set_metadata_hash(&MetadataHash::from([37; MetadataHash::BYTE_COUNT]));
    //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
    //     let mut metadata = TransactionMetadata::new();
    //     let mut md_list = TransactionMetadatums::new();
    //     md_list.add(&TransactionMetadatum::new_int(&Int::new(&to_bignum(5))));
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
    //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
    //     let body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
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
                .with_coin(&to_bignum(1))
                .build()
                .unwrap(),
        );
        let mut body = TransactionBody::new(&inputs, &outputs, &to_bignum(162502), Some(10));
        let mut withdrawals = Withdrawals::new();
        withdrawals.insert(
            &RewardAddress::from_address(
                &Address::from_bytes(
                    hex::decode("e151df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af70")
                        .unwrap(),
                )
                .unwrap(),
            )
            .unwrap(),
            &to_bignum(1337),
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

        let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
        assert_eq!(
            hex::encode(signed_tx.to_bytes()),
            "84a500818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a00027ac6030a05a1581de151df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af70190539a10082825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee5840fc0493f7121efe385d72830680e735ccdef99c3a31953fe877b89ad3a97fcdb871cc7f2cdd6a8104e52f6963bd9e10d814d4fabdbcdc8475bc63e872dcc94d0a82582054d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f35840a051ba927582004aedab736b9f1f9330ff867c260f4751135d480074256e83cd23d2a4bb109f955c43afdcdc5d1841b28d5c1ea2148dfbb6252693590692bb00f5f6"
        );
        assert_eq!(
            min_fee(&signed_tx, &linear_fee).unwrap().to_str(),
            "163002" // todo: compare to Haskell fee to make sure the diff is not too big
        );
    }

    fn exunits(mem: u64, steps: u64) -> ExUnits {
        ExUnits::new(&to_bignum(mem), &to_bignum(steps))
    }

    fn subcoin(num: u64, denum: u64) -> SubCoin {
        SubCoin::new(&to_bignum(num), &to_bignum(denum))
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
            to_bignum(80),
        );
        // 22 * (12/6) + 33 * (33/11) = 22 * 2 + 33 * 3 = 44 + 99 = 143
        assert_eq!(
            _calculate_ex_units_ceil_cost(22, 33, (12, 6), (33, 11)),
            to_bignum(143),
        );
        // 10 * (5/7) + 20 * (9/13) = 50/7 + 180/13 = 650/91 + 1260/91 = 1910/91 = ceil(20.98) = 21
        assert_eq!(
            _calculate_ex_units_ceil_cost(10, 20, (5, 7), (9, 13)),
            to_bignum(21),
        );
        // 22 * (7/5) + 33 * (13/9) = 154/5 + 429/9 = 1386/45 + 2145/45 = 3531/45 = ceil(78.46) = 79
        assert_eq!(
            _calculate_ex_units_ceil_cost(22, 33, (7, 5), (13, 9)),
            to_bignum(79),
        );
    }
}
