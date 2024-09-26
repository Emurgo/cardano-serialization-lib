use crate::*;

#[test]
fn subtract_values() {
    let policy1 = PolicyID::from([0; ScriptHash::BYTE_COUNT]);
    let policy2 = PolicyID::from([1; ScriptHash::BYTE_COUNT]);

    let asset1 = AssetName(vec![1]);
    let asset2 = AssetName(vec![2]);
    let asset3 = AssetName(vec![3]);
    let asset4 = AssetName(vec![4]);

    let mut token_bundle1 = MultiAsset::new();
    {
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(1));
        asset_list1.insert(&asset2, &BigNum(1));
        asset_list1.insert(&asset3, &BigNum(1));
        asset_list1.insert(&asset4, &BigNum(2));
        token_bundle1.insert(&policy1, &asset_list1);

        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset1, &BigNum(1));
        token_bundle1.insert(&policy2, &asset_list2);
    }
    let assets1 = Value {
        coin: BigNum(1555554),
        multiasset: Some(token_bundle1),
    };

    let mut token_bundle2 = MultiAsset::new();
    {
        let mut asset_list2 = Assets::new();
        // more than asset1 bundle
        asset_list2.insert(&asset1, &BigNum(2));
        // exactly equal to asset1 bundle
        asset_list2.insert(&asset2, &BigNum(1));
        // skip asset 3
        // less than in asset1 bundle
        asset_list2.insert(&asset4, &BigNum(1));
        token_bundle2.insert(&policy1, &asset_list2);

        // this policy should be removed entirely
        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset1, &BigNum(1));
        token_bundle2.insert(&policy2, &asset_list2);
    }

    let assets2 = Value {
        coin: BigNum(2555554),
        multiasset: Some(token_bundle2),
    };

    let result = assets1.clamped_sub(&assets2);
    assert_eq!(result.coin().to_str(), "0");
    assert_eq!(
        result.multiasset().unwrap().len(),
        1 // policy 2 was deleted successfully
    );
    let policy1_content = result.multiasset().unwrap().get(&policy1).unwrap();
    assert_eq!(policy1_content.len(), 2);
    assert_eq!(policy1_content.get(&asset3).unwrap().to_str(), "1");
    assert_eq!(policy1_content.get(&asset4).unwrap().to_str(), "1");
}

#[test]
fn compare_values() {
    let policy1 = PolicyID::from([0; ScriptHash::BYTE_COUNT]);

    let asset1 = AssetName(vec![1]);
    let asset2 = AssetName(vec![2]);

    // testing cases with no assets
    {
        let a = Value::new(&BigNum(1));
        let b = Value::new(&BigNum(1));
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Equal);
    }
    {
        let a = Value::new(&BigNum(2));
        let b = Value::new(&BigNum(1));
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
    }
    {
        let a = Value::new(&BigNum(1));
        let b = Value::new(&BigNum(2));
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
    }
    // testing case where one side has assets
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(1));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle1),
        };
        let b = Value::new(&BigNum(1));
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
    }
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(1));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value::new(&BigNum(1));
        let b = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle1),
        };
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
    }
    // testing case where both sides has assets
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(1));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset1, &BigNum(1));
        token_bundle2.insert(&policy1, &asset_list2);
        let b = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle2),
        };
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Equal);
    }
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(1));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value {
            coin: BigNum(2),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset1, &BigNum(1));
        token_bundle2.insert(&policy1, &asset_list2);
        let b = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle2),
        };
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
    }
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(1));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset1, &BigNum(1));
        token_bundle2.insert(&policy1, &asset_list2);
        let b = Value {
            coin: BigNum(2),
            multiasset: Some(token_bundle2),
        };
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
    }
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(2));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset1, &BigNum(1));
        token_bundle2.insert(&policy1, &asset_list2);
        let b = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle2),
        };
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
    }
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(2));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value {
            coin: BigNum(2),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset1, &BigNum(1));
        token_bundle2.insert(&policy1, &asset_list2);
        let b = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle2),
        };
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
    }
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(2));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset1, &BigNum(1));
        token_bundle2.insert(&policy1, &asset_list2);
        let b = Value {
            coin: BigNum(2),
            multiasset: Some(token_bundle2),
        };
        assert_eq!(a.partial_cmp(&b), None);
    }
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(1));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset1, &BigNum(2));
        token_bundle2.insert(&policy1, &asset_list2);
        let b = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle2),
        };
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
    }
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(1));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset1, &BigNum(2));
        token_bundle2.insert(&policy1, &asset_list2);
        let b = Value {
            coin: BigNum(2),
            multiasset: Some(token_bundle2),
        };
        assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
    }
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(1));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value {
            coin: BigNum(2),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset1, &BigNum(2));
        token_bundle2.insert(&policy1, &asset_list2);
        let b = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle2),
        };
        assert_eq!(a.partial_cmp(&b), None);
    }
    {
        let mut token_bundle1 = MultiAsset::new();
        let mut asset_list1 = Assets::new();
        asset_list1.insert(&asset1, &BigNum(1));
        token_bundle1.insert(&policy1, &asset_list1);
        let a = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        let mut asset_list2 = Assets::new();
        asset_list2.insert(&asset2, &BigNum(1));
        token_bundle2.insert(&policy1, &asset_list2);
        let b = Value {
            coin: BigNum(1),
            multiasset: Some(token_bundle2),
        };
        assert_eq!(a.partial_cmp(&b), None);
    }
}

#[test]
fn bigint_serialization() {
    let zero = BigInt::from_str("0").unwrap();
    let zero_rt = BigInt::from_bytes(zero.to_bytes()).unwrap();
    assert_eq!(zero.to_str(), zero_rt.to_str());
    assert_eq!(zero.to_bytes(), vec![0x00]);

    let pos_small = BigInt::from_str("100").unwrap();
    let pos_small_rt = BigInt::from_bytes(pos_small.to_bytes()).unwrap();
    assert_eq!(pos_small.to_str(), pos_small_rt.to_str());

    let pos_big = BigInt::from_str("123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890").unwrap();
    let pos_big_rt = BigInt::from_bytes(pos_big.to_bytes()).unwrap();
    assert_eq!(pos_big.to_str(), pos_big_rt.to_str());

    let neg_small = BigInt::from_str("-100").unwrap();
    let neg_small_rt = BigInt::from_bytes(neg_small.to_bytes()).unwrap();
    assert_eq!(neg_small.to_str(), neg_small_rt.to_str());

    let neg_big = BigInt::from_str("-123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890").unwrap();
    let neg_big_rt = BigInt::from_bytes(neg_big.to_bytes()).unwrap();
    assert_eq!(neg_big.to_str(), neg_big_rt.to_str());

    // taken from CBOR RFC examples
    // negative big int
    assert_eq!(
        hex::decode("c349010000000000000000").unwrap(),
        BigInt::from_str("-18446744073709551617")
            .unwrap()
            .to_bytes()
    );
    // positive big int
    assert_eq!(
        hex::decode("c249010000000000000000").unwrap(),
        BigInt::from_str("18446744073709551616").unwrap().to_bytes()
    );
    // uint
    assert_eq!(
        hex::decode("1b000000e8d4a51000").unwrap(),
        BigInt::from_str("1000000000000").unwrap().to_bytes()
    );
    // nint (lowest possible - used to be unsupported but works now)
    assert_eq!(
        hex::decode("3bffffffffffffffff").unwrap(),
        BigInt::from_str("-18446744073709551616")
            .unwrap()
            .to_bytes()
    );
    // this one fits in an i64 though
    assert_eq!(
        hex::decode("3903e7").unwrap(),
        BigInt::from_str("-1000").unwrap().to_bytes()
    );

    let x = BigInt::from_str("-18446744073709551617").unwrap();
    let x_rt = BigInt::from_bytes(x.to_bytes()).unwrap();
    assert_eq!(x.to_str(), x_rt.to_str());
}

#[test]
fn bounded_bytes_read_chunked() {
    use std::io::Cursor;
    let chunks = vec![
        vec![
            0x52, 0x73, 0x6F, 0x6D, 0x65, 0x20, 0x72, 0x61, 0x6E, 0x64, 0x6F, 0x6D, 0x20, 0x73,
            0x74, 0x72, 0x69, 0x6E, 0x67,
        ],
        vec![0x44, 0x01, 0x02, 0x03, 0x04],
    ];
    let mut expected = Vec::new();
    for chunk in chunks.iter() {
        expected.extend_from_slice(&chunk[1..]);
    }
    let mut vec = vec![0x5f];
    for mut chunk in chunks {
        vec.append(&mut chunk);
    }
    vec.push(0xff);
    let mut raw = Deserializer::from(Cursor::new(vec.clone()));
    let found = read_bounded_bytes(&mut raw).unwrap();
    assert_eq!(found, expected);
}

#[test]
fn bounded_bytes_write_chunked() {
    let mut chunk_64 = vec![0x58, crate::utils::BOUNDED_BYTES_CHUNK_SIZE as u8];
    chunk_64.extend(std::iter::repeat(37).take(crate::utils::BOUNDED_BYTES_CHUNK_SIZE));
    let chunks = vec![chunk_64, vec![0x44, 0x01, 0x02, 0x03, 0x04]];
    let mut input = Vec::new();
    input.extend_from_slice(&chunks[0][2..]);
    input.extend_from_slice(&chunks[1][1..]);
    let mut serializer = cbor_event::se::Serializer::new_vec();
    write_bounded_bytes(&mut serializer, &input).unwrap();
    let written = serializer.finalize();
    let mut expected = vec![0x5f];
    for mut chunk in chunks {
        expected.append(&mut chunk);
    }
    expected.push(0xff);
    assert_eq!(expected, written);
}

#[test]
fn correct_script_data_hash() {
    let mut datums = PlutusList::new();
    datums.add(&PlutusData::new_integer(&BigInt::from_str("1000").unwrap()));
    let mut redeemers = Redeemers::new();
    redeemers.add(&Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum::from_str("1").unwrap(),
        &PlutusData::new_integer(&BigInt::from_str("2000").unwrap()),
        &ExUnits::new(
            &BigNum::from_str("0").unwrap(),
            &BigNum::from_str("0").unwrap(),
        ),
    ));
    let plutus_cost_model = CostModel::from_bytes(vec![
        159, 26, 0, 3, 2, 89, 0, 1, 1, 26, 0, 6, 11, 199, 25, 2, 109, 0, 1, 26, 0, 2, 73, 240, 25,
        3, 232, 0, 1, 26, 0, 2, 73, 240, 24, 32, 26, 0, 37, 206, 168, 25, 113, 247, 4, 25, 116, 77,
        24, 100, 25, 116, 77, 24, 100, 25, 116, 77, 24, 100, 25, 116, 77, 24, 100, 25, 116, 77, 24,
        100, 25, 116, 77, 24, 100, 24, 100, 24, 100, 25, 116, 77, 24, 100, 26, 0, 2, 73, 240, 24,
        32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 25, 3, 232, 0,
        1, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 25, 3, 232, 0, 8, 26, 0, 2, 66, 32, 26, 0,
        6, 126, 35, 24, 118, 0, 1, 1, 26, 0, 2, 73, 240, 25, 3, 232, 0, 8, 26, 0, 2, 73, 240, 26,
        0, 1, 183, 152, 24, 247, 1, 26, 0, 2, 73, 240, 25, 39, 16, 1, 26, 0, 2, 21, 94, 25, 5, 46,
        1, 25, 3, 232, 26, 0, 2, 73, 240, 25, 3, 232, 1, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73,
        240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 1, 1, 26, 0, 2, 73, 240, 1, 26, 0, 2, 73, 240, 4,
        26, 0, 1, 148, 175, 24, 248, 1, 26, 0, 1, 148, 175, 24, 248, 1, 26, 0, 2, 55, 124, 25, 5,
        86, 1, 26, 0, 2, 189, 234, 25, 1, 241, 1, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24,
        32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0,
        2, 73, 240, 24, 32, 26, 0, 2, 66, 32, 26, 0, 6, 126, 35, 24, 118, 0, 1, 1, 25, 240, 76, 25,
        43, 210, 0, 1, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 66, 32, 26, 0, 6, 126, 35, 24, 118, 0,
        1, 1, 26, 0, 2, 66, 32, 26, 0, 6, 126, 35, 24, 118, 0, 1, 1, 26, 0, 37, 206, 168, 25, 113,
        247, 4, 0, 26, 0, 1, 65, 187, 4, 26, 0, 2, 73, 240, 25, 19, 136, 0, 1, 26, 0, 2, 73, 240,
        24, 32, 26, 0, 3, 2, 89, 0, 1, 1, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26,
        0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73,
        240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 51, 13, 167, 1, 1, 255,
    ])
    .unwrap();
    let mut cost_models = Costmdls::new();
    cost_models.insert(&Language::new_plutus_v1(), &plutus_cost_model);
    let script_data_hash = hash_script_data(&redeemers, &cost_models, Some(datums));

    assert_eq!(
        hex::encode(script_data_hash.to_bytes()),
        "8452337aed2f75d45838155503407b4241a75f021c3818ec90383c8e0faca5a4"
    );
}

#[test]
fn native_scripts_from_wallet_json() {
    let cosigner0_hex = "1423856bc91c49e928f6f30f4e8d665d53eb4ab6028bd0ac971809d514c92db11423856bc91c49e928f6f30f4e8d665d53eb4ab6028bd0ac971809d514c92db1";
    let cosigner1_hex = "a48d97f57ce49433f347d44ee07e54a100229b4f8e125d25f7bca9ad66d9707a25cd1331f46f7d6e279451637ca20802a25c441ba9436abf644fe5410d1080e3";
    let self_key_hex = "6ce83a12e9d4c783f54c0bb511303b37160a6e4f3f96b8e878a7c1f7751e18c4ccde3fb916d330d07f7bd51fb6bd99aa831d925008d3f7795033f48abd6df7f6";
    let native_script = encode_json_str_to_native_script(
        &format!(
            r#"
        {{
            "cosigners": {{
                "cosigner#0": "{}",
                "cosigner#1": "{}",
                "cosigner#2": "self"
            }},
            "template": {{
                "some": {{
                    "at_least": 2,
                    "from": [
                        {{
                            "all": [
                                "cosigner#0",
                                {{ "active_from": 120 }}
                            ]
                        }},
                        {{
                            "any": [
                                "cosigner#1",
                                {{ "active_until": 1000 }}
                            ]
                        }},
                        "cosigner#2"
                    ]
                }}
            }}
        }}"#,
            cosigner0_hex, cosigner1_hex
        ),
        self_key_hex,
        ScriptSchema::Wallet,
    );

    let n_of_k = native_script.unwrap().as_script_n_of_k().unwrap();
    let from = n_of_k.native_scripts();
    assert_eq!(n_of_k.n(), 2);
    assert_eq!(from.len(), 3);
    let all = from.get(0).as_script_all().unwrap().native_scripts();
    assert_eq!(all.len(), 2);
    let all_0 = all.get(0).as_script_pubkey().unwrap();
    assert_eq!(
        all_0.addr_keyhash(),
        Bip32PublicKey::from_bytes(&hex::decode(cosigner0_hex).unwrap())
            .unwrap()
            .to_raw_key()
            .hash()
    );
    let all_1 = all.get(1).as_timelock_start().unwrap();
    assert_eq!(all_1.slot().unwrap(), 120);
    let any = from.get(1).as_script_any().unwrap().native_scripts();
    assert_eq!(all.len(), 2);
    let any_0 = any.get(0).as_script_pubkey().unwrap();
    assert_eq!(
        any_0.addr_keyhash(),
        Bip32PublicKey::from_bytes(&hex::decode(cosigner1_hex).unwrap())
            .unwrap()
            .to_raw_key()
            .hash()
    );
    let any_1 = any.get(1).as_timelock_expiry().unwrap();
    assert_eq!(any_1.slot().unwrap(), 1000);
    let self_key = from.get(2).as_script_pubkey().unwrap();
    assert_eq!(
        self_key.addr_keyhash(),
        Bip32PublicKey::from_bytes(&hex::decode(self_key_hex).unwrap())
            .unwrap()
            .to_raw_key()
            .hash()
    );
}

#[test]
fn int_to_str() {
    assert_eq!(
        Int::new(&BigNum(u64::max_value())).to_str(),
        u64::max_value().to_string()
    );
    assert_eq!(
        Int::new(&BigNum(u64::min_value())).to_str(),
        u64::min_value().to_string()
    );
    assert_eq!(
        Int::new_negative(&BigNum(u64::max_value())).to_str(),
        (-(u64::max_value() as i128)).to_string()
    );
    assert_eq!(
        Int::new_negative(&BigNum(u64::min_value())).to_str(),
        (-(u64::min_value() as i128)).to_string()
    );
    assert_eq!(Int::new_i32(142).to_str(), "142");
    assert_eq!(Int::new_i32(-142).to_str(), "-142");
}

#[test]
fn int_as_i32_or_nothing() {
    let over_pos_i32 = (i32::max_value() as i64) + 1;
    assert!(Int::new(&BigNum(over_pos_i32 as u64))
        .as_i32_or_nothing()
        .is_none());

    let valid_pos_i32 = i32::max_value() as i64;
    assert_eq!(
        Int::new(&BigNum(valid_pos_i32 as u64))
            .as_i32_or_nothing()
            .unwrap(),
        i32::max_value()
    );

    let over_neg_i32 = (i32::min_value() as i64) - 1;
    assert!(Int::new_negative(&BigNum((-over_neg_i32) as u64))
        .as_i32_or_nothing()
        .is_none());

    let valid_neg_i32 = i32::min_value() as i64;
    assert_eq!(
        Int::new_negative(&BigNum((-valid_neg_i32) as u64))
            .as_i32_or_nothing()
            .unwrap(),
        i32::min_value()
    );

    assert!(Int::new(&BigNum(u64::max_value()))
        .as_i32_or_nothing()
        .is_none());
    assert_eq!(
        Int::new(&BigNum(i32::max_value() as u64))
            .as_i32_or_nothing()
            .unwrap(),
        i32::max_value()
    );
    assert_eq!(
        Int::new_negative(&BigNum(i32::max_value() as u64))
            .as_i32_or_nothing()
            .unwrap(),
        -i32::max_value()
    );

    assert_eq!(Int::new_i32(42).as_i32_or_nothing().unwrap(), 42);
    assert_eq!(Int::new_i32(-42).as_i32_or_nothing().unwrap(), -42);
}

#[test]
fn int_as_i32_or_fail() {
    let over_pos_i32 = (i32::max_value() as i64) + 1;
    assert!(Int::new(&BigNum(over_pos_i32 as u64))
        .as_i32_or_fail()
        .is_err());

    let valid_pos_i32 = i32::max_value() as i64;
    assert_eq!(
        Int::new(&BigNum(valid_pos_i32 as u64))
            .as_i32_or_fail()
            .unwrap(),
        i32::max_value()
    );

    let over_neg_i32 = (i32::min_value() as i64) - 1;
    assert!(Int::new_negative(&BigNum((-over_neg_i32) as u64))
        .as_i32_or_fail()
        .is_err());

    let valid_neg_i32 = i32::min_value() as i64;
    assert_eq!(
        Int::new_negative(&BigNum((-valid_neg_i32) as u64))
            .as_i32_or_fail()
            .unwrap(),
        i32::min_value()
    );

    assert!(Int::new(&BigNum(u64::max_value()))
        .as_i32_or_fail()
        .is_err());
    assert_eq!(
        Int::new(&BigNum(i32::max_value() as u64))
            .as_i32_or_fail()
            .unwrap(),
        i32::max_value()
    );
    assert_eq!(
        Int::new_negative(&BigNum(i32::max_value() as u64))
            .as_i32_or_fail()
            .unwrap(),
        -i32::max_value()
    );

    assert_eq!(Int::new_i32(42).as_i32_or_fail().unwrap(), 42);
    assert_eq!(Int::new_i32(-42).as_i32_or_fail().unwrap(), -42);
}

#[test]
fn int_full_range() {
    // cbor_event's nint API worked via i64 but we now have a workaround for it
    // so these tests are here to make sure that workaround works.

    // first nint below of i64::MIN
    let bytes_x = vec![0x3b, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let x = Int::from_bytes(bytes_x.clone()).unwrap();
    assert_eq!(x.to_str(), "-9223372036854775809");
    assert_eq!(bytes_x, x.to_bytes());

    // smallest possible nint which is -u64::MAX - 1
    let bytes_y = vec![0x3b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let y = Int::from_bytes(bytes_y.clone()).unwrap();
    assert_eq!(y.to_str(), "-18446744073709551616");
    assert_eq!(bytes_y, y.to_bytes());
}

#[test]
fn test_bigint_add() {
    assert_eq!(to_bigint(10).add(&to_bigint(20)), to_bigint(30),);
    assert_eq!(to_bigint(500).add(&to_bigint(800)), to_bigint(1300),);
}

#[test]
fn test_bigint_mul() {
    assert_eq!(to_bigint(10).mul(&to_bigint(20)), to_bigint(200),);
    assert_eq!(to_bigint(500).mul(&to_bigint(800)), to_bigint(400000),);
    assert_eq!(to_bigint(12).mul(&to_bigint(22)), to_bigint(264),);
}

#[test]
fn test_bigint_div_ceil() {
    assert_eq!(to_bigint(20).div_ceil(&to_bigint(10)), to_bigint(2),);
    assert_eq!(to_bigint(20).div_ceil(&to_bigint(2)), to_bigint(10),);
    assert_eq!(to_bigint(21).div_ceil(&to_bigint(2)), to_bigint(11),);
    assert_eq!(to_bigint(6).div_ceil(&to_bigint(3)), to_bigint(2),);
    assert_eq!(to_bigint(5).div_ceil(&to_bigint(3)), to_bigint(2),);
    assert_eq!(to_bigint(7).div_ceil(&to_bigint(3)), to_bigint(3),);
}

#[test]
fn test_bignum_div() {
    assert_eq!(BigNum(10).div_floor(&BigNum(1)), BigNum(10),);
    assert_eq!(BigNum(10).div_floor(&BigNum(3)), BigNum(3),);
    assert_eq!(BigNum(10).div_floor(&BigNum(4)), BigNum(2),);
    assert_eq!(BigNum(10).div_floor(&BigNum(5)), BigNum(2),);
    assert_eq!(BigNum(10).div_floor(&BigNum(6)), BigNum(1),);
    assert_eq!(BigNum(10).div_floor(&BigNum(12)), BigNum::zero(),);
}

#[test]
fn test_vasil_v1_costmodel_hashing() {
    let v1 = Language::new_plutus_v1();
    let v1_cost_model = TxBuilderConstants::plutus_vasil_cost_models()
        .get(&v1)
        .unwrap();
    let mut costmodels = Costmdls::new();
    costmodels.insert(&v1, &v1_cost_model);
    let hash = hash_script_data(
        &Redeemers::from(vec![Redeemer::new(
            &RedeemerTag::new_spend(),
            &BigNum::zero(),
            &PlutusData::new_integer(&BigInt::from_str("42").unwrap()),
            &ExUnits::new(&BigNum(1700), &BigNum(368100)),
        )]),
        &costmodels,
        Some(PlutusList::from(vec![PlutusData::new_integer(
            &BigInt::from_str("42").unwrap(),
        )])),
    );
    assert_eq!(
        hex::encode(hash.to_bytes()),
        "f173f8e25f385c61c33ab84c1e4a1af36fcd47dc7ab83d89f926828f618630f5"
    );
}

#[test]
fn bigint_as_int() {
    let zero = BigInt::from_str("0").unwrap();
    let zero_int = zero.as_int().unwrap();
    assert_eq!(zero_int.0, 0i128);

    let pos = BigInt::from_str("1024").unwrap();
    let pos_int = pos.as_int().unwrap();
    assert_eq!(pos_int.0, 1024i128);

    let neg = BigInt::from_str("-1024").unwrap();
    let neg_int = neg.as_int().unwrap();
    assert_eq!(neg_int.0, -1024i128);
}
