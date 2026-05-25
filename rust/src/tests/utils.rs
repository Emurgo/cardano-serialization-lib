use super::fakes::{fake_multiasset, fake_value_with_assets};
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

mod int_boundary {
    use crate::*;
    use std::convert::TryFrom;

    const MIN: i128 = -(u64::MAX as i128) - 1; // -2^64
    const MAX: i128 = u64::MAX as i128;        //  2^64 - 1

    #[test]
    fn min_max_constants() {
        assert_eq!(Int::CBOR_MIN, MIN);
        assert_eq!(Int::CBOR_MAX, MAX);
    }

    #[test]
    fn try_from_accepts_full_range() {
        assert!(Int::try_from(MIN).is_ok());
        assert!(Int::try_from(MAX).is_ok());
        assert!(Int::try_from(MIN + 1).is_ok());
        assert!(Int::try_from(0i128).is_ok());
    }

    #[test]
    fn try_from_rejects_out_of_range() {
        assert!(Int::try_from(MIN - 1).is_err());
        assert!(Int::try_from(MAX + 1).is_err());
        assert!(Int::try_from(i128::MIN).is_err());
        assert!(Int::try_from(i128::MAX).is_err());
    }

    #[test]
    fn from_str_boundaries() {
        assert!(Int::from_str("18446744073709551615").is_ok());      // u64::MAX
        assert!(Int::from_str("-18446744073709551615").is_ok());     // -(u64::MAX)
        assert!(Int::from_str("-18446744073709551616").is_ok());     // -2^64
        assert!(Int::from_str("18446744073709551616").is_err());     // u64::MAX + 1
        assert!(Int::from_str("-18446744073709551617").is_err());    // -2^64 - 1
    }

    #[test]
    fn infallible_from_native_types() {
        let _: Int = i32::MIN.into();
        let _: Int = i32::MAX.into();
        let _: Int = u32::MAX.into();
        let _: Int = i64::MIN.into();
        let _: Int = i64::MAX.into();
        let _: Int = u64::MAX.into();
    }

    #[test]
    fn cbor_roundtrip_full_range() {
        let cases = [MIN, MIN + 1, -1, 0, 1, i64::MIN as i128, i64::MAX as i128, MAX];
        for &x in &cases {
            let i = Int::try_from(x).unwrap();
            let bytes = i.to_bytes();
            let roundtrip = Int::from_bytes(bytes.clone())
                .unwrap_or_else(|e| panic!("roundtrip failed for {}: {:?}", x, e));
            assert_eq!(i, roundtrip, "mismatch for {}: bytes={}", x, hex::encode(&bytes));
        }
    }

    #[test]
    fn cbor_decode_rejects_out_of_range_uint() {
        // CBOR major-0 with 9-byte payload encoding > u64 is not valid CBOR uint,
        // so we don't need to test that explicitly. But a manually crafted nint
        // with payload > u64::MAX would be rejected by read_nint via u64 fit.
        // Just verify that the MIN value (payload = u64::MAX in nint) does decode.
        let i = Int::try_from(MIN).unwrap();
        let bytes = i.to_bytes();
        // last 8 bytes should be 0xFFFFFFFFFFFFFFFF (u64::MAX payload)
        assert!(bytes.last().copied() == Some(0xFF));
        let rt = Int::from_bytes(bytes).unwrap();
        assert_eq!(rt.to_str(), "-18446744073709551616");
    }

    #[test]
    fn checked_add_overflow() {
        let max = Int::try_from(MAX).unwrap();
        let one = Int::from(1u32);
        assert_eq!(max.checked_add(&one), None);
        let min = Int::try_from(MIN).unwrap();
        assert_eq!(min.checked_add(&Int::from(-1i64)), None);
    }

    #[test]
    fn checked_sub_underflow() {
        let min = Int::try_from(MIN).unwrap();
        let one = Int::from(1u32);
        assert_eq!(min.checked_sub(&one), None);
    }

    #[test]
    fn checked_mul_overflow() {
        let max = Int::try_from(MAX).unwrap();
        let two = Int::from(2u32);
        assert_eq!(max.checked_mul(&two), None);
    }

    #[test]
    fn checked_arithmetic_within_range() {
        let a = Int::from(100i32);
        let b = Int::from(42i32);
        assert_eq!(a.checked_add(&b), Some(Int::from(142i32)));
        assert_eq!(a.checked_sub(&b), Some(Int::from(58i32)));
        assert_eq!(a.checked_mul(&b), Some(Int::from(4200i32)));
    }

    #[test]
    fn saturating_arithmetic_clamps() {
        let max = Int::try_from(MAX).unwrap();
        let min = Int::try_from(MIN).unwrap();
        assert_eq!(max.saturating_add(&Int::from(1u32)), max);
        assert_eq!(min.saturating_sub(&Int::from(1u32)), min);
    }

    #[test]
    fn as_negative_handles_min() {
        // -2^64 has |x| = 2^64, doesn't fit u64 — None expected.
        let min = Int::try_from(MIN).unwrap();
        assert_eq!(min.as_negative(), None);
        assert_eq!(min.as_positive(), None);

        let near_min = Int::try_from(MIN + 1).unwrap(); // -(u64::MAX)
        assert_eq!(near_min.as_negative(), Some(BigNum(u64::MAX)));
    }

    #[test]
    fn cost_model_try_from_accepts_int_range() {
        // Values used by tx_builder_constants — all small i64, must succeed.
        let cm = CostModel::try_from(vec![812990i128, 1, -1, 0]).unwrap();
        assert_eq!(cm.len(), 4);

        // CBOR int boundary values still succeed.
        let cm = CostModel::try_from(vec![MIN, MAX]).unwrap();
        assert_eq!(cm.len(), 2);
    }

    #[test]
    fn cost_model_try_from_errors_on_out_of_range() {
        for &v in &[i128::MAX, i128::MIN, MAX + 1, MIN - 1] {
            let err = CostModel::try_from(vec![v]).unwrap_err();
            assert!(err.to_string().contains("out of CBOR int range"),
                    "unexpected error for {}: {}", v, err.to_string());
        }
    }

    #[test]
    fn mint_assets_reject_out_of_int64_range() {
        // Conway CDDL constrains mint amounts to `nonzero_int64`. MintAssets
        // must reject the lower CBOR-int corner (-2^64) and any other value
        // whose magnitude exceeds i64::MAX, as well as zero.
        let asset = AssetName::new(vec![1, 2, 3]).unwrap();

        let min_cbor = Int::try_from(MIN).unwrap();           // -2^64
        assert!(MintAssets::new_from_entry(&asset, &min_cbor).is_err());

        let too_negative = Int::try_from(i64::MIN as i128 - 1).unwrap();
        assert!(MintAssets::new_from_entry(&asset, &too_negative).is_err());

        let too_positive = Int::try_from(i64::MAX as i128 + 1).unwrap();
        assert!(MintAssets::new_from_entry(&asset, &too_positive).is_err());

        let mut ma = MintAssets::new();
        assert!(ma.insert(&asset, &Int::from(0i32)).is_err());

        // Boundary values that DO fit int64 must succeed.
        assert!(MintAssets::new_from_entry(&asset, &Int::from(i64::MIN)).is_ok());
        assert!(MintAssets::new_from_entry(&asset, &Int::from(i64::MAX)).is_ok());
    }

    #[test]
    fn mint_cbor_accepts_any_int_amount() {
        // Deserialization does not enforce nonzero_int64 — that check lives
        // at the construction boundary (MintAssets::insert / new_from_entry).
        // Out-of-range mint amounts on the wire are preserved verbatim so
        // that consumers can inspect or reject them with their own policy.
        let bytes: Vec<u8> = vec![
            0xa1, 0x41, 0x00,
            0x3b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // nint -2^64
        ];
        use cbor_event::de::Deserializer;
        use std::io::Cursor;
        let mut raw = Deserializer::from(Cursor::new(bytes));
        let ma = MintAssets::deserialize(&mut raw).unwrap();
        assert_eq!(ma.len(), 1);
    }

    // CDDL / RFC 8949 conformance vectors.
    //
    //   int  = uint / nint
    //   uint = 0 .. 2^64 - 1
    //   nint = -2^64 .. -1
    //
    // Canonical encoding (RFC 8949 §4.2.1) picks the smallest header form:
    //   value in 0..=23                → 1-byte inline
    //   value in 24..=255              → 1+1 bytes (0x18 / 0x38)
    //   value in 256..=u16::MAX        → 1+2 bytes (0x19 / 0x39)
    //   value in u16::MAX+1..=u32::MAX → 1+4 bytes (0x1a / 0x3a)
    //   value > u32::MAX               → 1+8 bytes (0x1b / 0x3b)
    // For nint, "value" above means the payload = -1 - n.
    fn assert_enc(n: i128, expected_hex: &str) {
        let i = Int::try_from(n).unwrap();
        let got = hex::encode(i.to_bytes());
        assert_eq!(got, expected_hex,
                   "encoding mismatch for {}: expected {} got {}", n, expected_hex, got);
        // round-trip
        let back = Int::from_bytes(i.to_bytes()).unwrap();
        assert_eq!(back, i, "round-trip mismatch for {}", n);
    }

    #[test]
    fn cbor_canonical_uint_vectors() {
        // RFC 8949 Appendix A examples.
        assert_enc(0, "00");
        assert_enc(1, "01");
        assert_enc(10, "0a");
        assert_enc(23, "17");
        assert_enc(24, "1818");
        assert_enc(25, "1819");
        assert_enc(100, "1864");
        assert_enc(255, "18ff");
        assert_enc(256, "190100");
        assert_enc(1000, "1903e8");
        assert_enc(65535, "19ffff");
        assert_enc(65536, "1a00010000");
        assert_enc(1_000_000, "1a000f4240");
        assert_enc(u32::MAX as i128, "1affffffff");
        assert_enc(u32::MAX as i128 + 1, "1b0000000100000000");
        assert_enc(1_000_000_000_000, "1b000000e8d4a51000");
        assert_enc(i64::MAX as i128, "1b7fffffffffffffff");
        assert_enc(i64::MAX as i128 + 1, "1b8000000000000000");
        assert_enc(u64::MAX as i128, "1bffffffffffffffff"); // 2^64 - 1, CBOR_MAX
    }

    #[test]
    fn cbor_canonical_nint_vectors() {
        // RFC 8949 Appendix A examples + Cardano-relevant boundaries.
        assert_enc(-1, "20");
        assert_enc(-10, "29");
        assert_enc(-24, "37");
        assert_enc(-25, "3818");
        assert_enc(-100, "3863");
        assert_enc(-256, "38ff");
        assert_enc(-257, "390100");
        assert_enc(-1000, "3903e7");
        assert_enc(-65536, "39ffff");
        assert_enc(-65537, "3a00010000");
        // payload boundary at u32::MAX (value = -(u32::MAX+1) = -2^32)
        assert_enc(-(u32::MAX as i128) - 1, "3affffffff");
        assert_enc(-(u32::MAX as i128) - 2, "3b0000000100000000");
        // i64::MIN: payload = i64::MAX = 0x7fff_ffff_ffff_ffff
        assert_enc(i64::MIN as i128, "3b7fffffffffffffff");
        // -(2^63 + 1): payload = 2^63 = 0x8000_0000_0000_0000 (out of i64 range)
        assert_enc(-(i64::MAX as i128) - 2, "3b8000000000000000");
        // -u64::MAX: payload = u64::MAX - 1
        assert_enc(-(u64::MAX as i128), "3bfffffffffffffffe");
        // CBOR_MIN = -2^64: payload = u64::MAX
        assert_enc(MIN, "3bffffffffffffffff");
    }

    #[test]
    fn cbor_canonical_header_size_selection() {
        // Both sides of every header-size boundary, positive and negative.
        let boundaries: &[i128] = &[
            // uint
            23, 24, 255, 256, 65535, 65536, u32::MAX as i128, u32::MAX as i128 + 1,
            // nint payload boundaries
            -24, -25, -256, -257, -65536, -65537,
            -(u32::MAX as i128) - 1, -(u32::MAX as i128) - 2,
        ];
        for &n in boundaries {
            let bytes = Int::try_from(n).unwrap().to_bytes();
            let expected_len = match n {
                v if (0..=23).contains(&v) || (-24..=-1).contains(&v) => 1,
                v if (24..=255).contains(&v) || (-256..=-25).contains(&v) => 2,
                v if (256..=65535).contains(&v) || (-65536..=-257).contains(&v) => 3,
                v if (65536..=u32::MAX as i128).contains(&v)
                    || (-(u32::MAX as i128 + 1)..=-65537).contains(&v) => 5,
                _ => 9,
            };
            assert_eq!(bytes.len(), expected_len,
                       "wrong header form for {}: bytes={}", n, hex::encode(&bytes));
        }
    }

    #[test]
    fn cbor_decode_known_vectors() {
        // Hand-crafted CBOR (canonical) decodes back to the right integer.
        let cases: &[(&str, i128)] = &[
            ("00", 0), ("17", 23), ("1818", 24), ("1864", 100),
            ("1bffffffffffffffff", u64::MAX as i128),
            ("20", -1), ("37", -24), ("3818", -25), ("3863", -100),
            ("3b7fffffffffffffff", i64::MIN as i128),
            ("3bffffffffffffffff", MIN),
        ];
        for (hexs, expected) in cases {
            let i = Int::from_bytes(hex::decode(hexs).unwrap()).unwrap();
            assert_eq!(i.to_str(), expected.to_string(),
                       "decode mismatch for {}", hexs);
        }
    }

    #[test]
    fn cbor_decode_rejects_non_canonical_non_int_types() {
        // Float, bytes, text — none are major-0 or major-1.
        for hex_bytes in ["f93c00" /* float16 1.0 */, "4100" /* bytes(1) */, "6161" /* text "a" */, "80" /* empty array */] {
            assert!(Int::from_bytes(hex::decode(hex_bytes).unwrap()).is_err(),
                    "expected reject for non-int CBOR: {}", hex_bytes);
        }
    }

    #[test]
    fn bigint_as_int_handles_min_i128() {
        let s = "-18446744073709551616"; // -2^64
        let bi = BigInt::from_str(s).unwrap();
        let i = bi.as_int().unwrap_or_else(|| panic!("as_int returned None for {}", s));
        assert_eq!(i.to_str(), s);

        let too_small = BigInt::from_str("-18446744073709551617").unwrap();
        assert!(too_small.as_int().is_none());

        let too_big = BigInt::from_str("18446744073709551616").unwrap();
        assert!(too_big.as_int().is_none());

        let max = BigInt::from_str("18446744073709551615").unwrap();
        assert_eq!(max.as_int().unwrap().to_str(), "18446744073709551615");
    }

    // Cover every branch of `BigInt::as_int`'s match:
    //   (Sign::NoSign, _)            → zero
    //   (Sign::Plus,  [lo])          → 0 < n ≤ u64::MAX
    //   (Sign::Minus, [lo])          → -u64::MAX ≤ n < 0
    //   (Sign::Minus, [0, 1])        → -2^64 exactly
    //   _                            → None
    #[test]
    fn bigint_as_int_branch_coverage() {
        // NoSign: 0
        let zero = BigInt::from_str("0").unwrap().as_int().unwrap();
        assert_eq!(zero.to_str(), "0");

        // Plus, single limb: small, mid, and u64::MAX upper edge
        for s in &["1", "1024", "9223372036854775807", "18446744073709551615"] {
            let i = BigInt::from_str(s).unwrap().as_int().unwrap();
            assert_eq!(i.to_str(), *s);
        }

        // Minus, single limb: -1, mid, -u64::MAX lower-but-still-single-limb edge
        for s in &["-1", "-1024", "-9223372036854775808", "-18446744073709551615"] {
            let i = BigInt::from_str(s).unwrap().as_int().unwrap();
            assert_eq!(i.to_str(), *s);
        }

        // Minus, two limbs [0, 1] is exactly -2^64 — covered by
        // `bigint_as_int_handles_min_i128`. Any other multi-limb BigInt
        // (positive >u64::MAX, negative <-2^64) must return None.
        for s in &[
            "18446744073709551616",      //  2^64 → Plus, [0, 1]
            "36893488147419103232",      //  2 * 2^64
            "-18446744073709551617",     // -(2^64 + 1)
            "-36893488147419103232",     // -2 * 2^64
        ] {
            assert!(BigInt::from_str(s).unwrap().as_int().is_none(),
                    "{} unexpectedly converted", s);
        }
    }
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

#[test]
fn has_transaction_set_tag_tx_with_only_tag() {
    let hex = "84a400d90102818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a00016f32030aa100d9010281825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee58406d68d8b7b2ee54f1f46b64e3f61a14f840be2ec125c858ec917f634a1eb898a51660654839226016a2588d39920e6dfe1b66d917027f198b5eb887d20f4ac805f5f6";
    let tx_sets = has_transaction_set_tag(hex::decode(hex).unwrap()).unwrap();
    assert_eq!(tx_sets, TransactionSetsState::AllSetsHaveTag);
}

#[test]
fn has_transaction_set_tag_tx_without_tag() {
    let hex = "84a400818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a00016f32030aa10081825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee5840fae5de40c94d759ce13bf9886262159c4f26a289fd192e165995b785259e503f6887bf39dfa23a47cf163784c6eee23f61440e749bc1df3c73975f5231aeda0ff5f6";
    let tx_sets = has_transaction_set_tag(hex::decode(hex).unwrap()).unwrap();
    assert_eq!(tx_sets, TransactionSetsState::AllSetsHaveNoTag);
}

#[test]
fn has_transaction_set_tag_mixed() {
    let hex = "84a400818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b700018182581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01021a00016f32030aa100d9010281825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee58406d68d8b7b2ee54f1f46b64e3f61a14f840be2ec125c858ec917f634a1eb898a51660654839226016a2588d39920e6dfe1b66d917027f198b5eb887d20f4ac805f5f6";
    let tx_sets = has_transaction_set_tag(hex::decode(hex).unwrap()).unwrap();
    assert_eq!(tx_sets, TransactionSetsState::MixedSets);
}


#[test]
fn value_empty_asset_equal() {
    let a = Value {
        coin: BigNum(0),
        multiasset: None,
    };
    let b = Value {
        coin: BigNum(0),
        multiasset: Some(MultiAsset::new()),
    };
    let c = Value {
        coin: BigNum(0),
        multiasset: None,
    };

    assert_eq!(a, b);
    assert_eq!(a, c);
}

mod value_checked_arithmetic {
    use super::*;
    use num::{CheckedAdd, CheckedSub};

    #[test]
    fn checked_add_coin_only() {
        let a = Value::from(BigNum(100));
        let b = Value::from(BigNum(200));
        let expected = Some(Value::from(BigNum(300)));
        assert_eq!(CheckedAdd::checked_add(&a, &b), expected);
    }

    #[test]
    fn checked_add_coin_overflow() {
        let a = Value::from(BigNum(u64::MAX));
        let b = Value::from(BigNum(1));
        assert_eq!(CheckedAdd::checked_add(&a, &b), None);
    }

    #[test]
    fn checked_add_both_have_multiasset() {
        let a = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let b = fake_value_with_assets(200, &[(0, &[(1, 5), (2, 20)])]);
        let expected = fake_value_with_assets(300, &[(0, &[(1, 15), (2, 20)])]);
        assert_eq!(CheckedAdd::checked_add(&a, &b), Some(expected));
    }

    #[test]
    fn checked_add_one_side_none_multiasset() {
        let a = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let b = Value::from(BigNum(200));
        let expected = fake_value_with_assets(300, &[(0, &[(1, 10)])]);
        assert_eq!(CheckedAdd::checked_add(&a, &b), Some(expected.clone()));
        assert_eq!(CheckedAdd::checked_add(&b, &a), Some(expected));
    }

    #[test]
    fn checked_add_asset_overflow() {
        let a = fake_value_with_assets(0, &[(0, &[(1, u64::MAX)])]);
        let b = fake_value_with_assets(0, &[(0, &[(1, 1)])]);
        assert_eq!(CheckedAdd::checked_add(&a, &b), None);
    }

    #[test]
    fn checked_sub_coin_only() {
        let a = Value::from(BigNum(300));
        let b = Value::from(BigNum(100));
        let expected = Some(Value::from(BigNum(200)));
        assert_eq!(CheckedSub::checked_sub(&a, &b), expected);
    }

    #[test]
    fn checked_sub_coin_underflow() {
        let a = Value::from(BigNum(100));
        let b = Value::from(BigNum(200));
        assert_eq!(CheckedSub::checked_sub(&a, &b), None);
    }

    #[test]
    fn checked_sub_both_have_multiasset() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 10), (2, 20)])]);
        let b = fake_value_with_assets(100, &[(0, &[(1, 3)])]);
        let expected = fake_value_with_assets(200, &[(0, &[(1, 7), (2, 20)])]);
        assert_eq!(CheckedSub::checked_sub(&a, &b), Some(expected));
    }

    #[test]
    fn checked_sub_asset_underflow() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 5)])]);
        let b = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        assert_eq!(CheckedSub::checked_sub(&a, &b), None);
    }

    #[test]
    fn checked_sub_lhs_none_rhs_nonzero_multiasset() {
        let a = Value::from(BigNum(300));
        let b = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        assert_eq!(CheckedSub::checked_sub(&a, &b), None);
    }

    #[test]
    fn checked_sub_lhs_none_rhs_zero_multiasset() {
        let a = Value::from(BigNum(300));
        let b = Value {
            coin: Coin::from(100u64),
            multiasset: Some(MultiAsset::new()),
        };
        let expected = Some(Value::from(BigNum(200)));
        assert_eq!(CheckedSub::checked_sub(&a, &b), expected);
    }

    #[test]
    fn checked_sub_lhs_has_multiasset_rhs_none() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 10)])]);
        let b = Value::from(BigNum(100));
        let expected = Some(fake_value_with_assets(200, &[(0, &[(1, 10)])]));
        assert_eq!(CheckedSub::checked_sub(&a, &b), expected);
    }

    #[test]
    fn add_operator() {
        let a = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let b = fake_value_with_assets(200, &[(0, &[(1, 5)])]);
        let expected = fake_value_with_assets(300, &[(0, &[(1, 15)])]);
        assert_eq!(a + b, expected);
    }

    #[test]
    #[should_panic(expected = "Value overflow")]
    fn add_operator_panics_on_overflow() {
        let a = Value::from(BigNum(u64::MAX));
        let b = Value::from(BigNum(1));
        let _ = a + b;
    }

    #[test]
    fn sub_operator() {
        let a = Value::from(BigNum(300));
        let b = Value::from(BigNum(100));
        let expected = Value::from(BigNum(200));
        assert_eq!(a - b, expected);
    }

    #[test]
    #[should_panic(expected = "Value underflow")]
    fn sub_operator_panics_on_underflow() {
        let a = Value::from(BigNum(100));
        let b = Value::from(BigNum(200));
        let _ = a - b;
    }

    #[test]
    fn checked_add_disjoint_policies() {
        let a = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let b = fake_value_with_assets(200, &[(1, &[(1, 20)])]);
        let expected = fake_value_with_assets(300, &[(0, &[(1, 10)]), (1, &[(1, 20)])]);
        assert_eq!(CheckedAdd::checked_add(&a, &b), Some(expected));
    }

    #[test]
    fn checked_sub_rhs_policy_not_in_lhs() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 10)])]);
        let b = fake_value_with_assets(100, &[(1, &[(1, 5)])]);
        assert_eq!(CheckedSub::checked_sub(&a, &b), None);
    }

    #[test]
    fn checked_sub_rhs_asset_name_not_in_lhs() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 10)])]);
        let b = fake_value_with_assets(100, &[(0, &[(2, 5)])]);
        assert_eq!(CheckedSub::checked_sub(&a, &b), None);
    }

    #[test]
    fn checked_sub_multiple_overlapping_policies() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 10)]), (1, &[(1, 20)])]);
        let b = fake_value_with_assets(100, &[(0, &[(1, 3)]), (1, &[(1, 5)])]);
        let expected = fake_value_with_assets(200, &[(0, &[(1, 7)]), (1, &[(1, 15)])]);
        assert_eq!(CheckedSub::checked_sub(&a, &b), Some(expected));
    }

    #[test]
    fn checked_sub_exact_assets_normalizes_to_none() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 10)])]);
        let b = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let expected = Some(Value::from(BigNum(200)));
        assert_eq!(CheckedSub::checked_sub(&a, &b), expected);
    }
}

mod value_saturating_arithmetic {
    use super::*;
    use num_traits::{SaturatingAdd, SaturatingSub};

    #[test]
    fn saturating_sub_coin_clamps_to_zero() {
        let a = Value::from(BigNum(100));
        let b = Value::from(BigNum(200));
        let expected = Value::from(BigNum(0));
        assert_eq!(a.saturating_sub(&b), expected);
    }

    #[test]
    fn saturating_sub_coin_within_range() {
        let a = Value::from(BigNum(300));
        let b = Value::from(BigNum(100));
        let expected = Value::from(BigNum(200));
        assert_eq!(a.saturating_sub(&b), expected);
    }

    #[test]
    fn saturating_sub_assets_clamp_to_zero_and_removed() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 5)])]);
        let b = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let expected = Value::from(BigNum(200));
        assert_eq!(a.saturating_sub(&b), expected);
    }

    #[test]
    fn saturating_sub_lhs_none_rhs_has_assets() {
        let a = Value::from(BigNum(300));
        let b = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let expected = Value::from(BigNum(200));
        assert_eq!(a.saturating_sub(&b), expected);
    }

    #[test]
    fn saturating_sub_lhs_has_assets_rhs_none() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 10)])]);
        let b = Value::from(BigNum(100));
        let expected = fake_value_with_assets(200, &[(0, &[(1, 10)])]);
        assert_eq!(a.saturating_sub(&b), expected);
    }

    #[test]
    fn saturating_sub_partial_asset_clamp() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 5), (2, 20)])]);
        let b = fake_value_with_assets(100, &[(0, &[(1, 10), (2, 3)])]);
        let expected = fake_value_with_assets(200, &[(0, &[(2, 17)])]);
        assert_eq!(a.saturating_sub(&b), expected);
    }

    #[test]
    fn saturating_add_coin_clamps_at_max() {
        let a = Value::from(BigNum(u64::MAX));
        let b = Value::from(BigNum(100));
        let expected = Value::from(BigNum(u64::MAX));
        assert_eq!(a.saturating_add(&b), expected);
    }

    #[test]
    fn saturating_add_asset_clamps_at_max() {
        let a = fake_value_with_assets(100, &[(0, &[(1, u64::MAX)])]);
        let b = fake_value_with_assets(200, &[(0, &[(1, 100), (2, 5)])]);
        let expected = fake_value_with_assets(300, &[(0, &[(1, u64::MAX), (2, 5)])]);
        assert_eq!(a.saturating_add(&b), expected);
    }

    #[test]
    fn saturating_add_one_side_none_multiasset() {
        let a = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let b = Value::from(BigNum(200));
        let expected = fake_value_with_assets(300, &[(0, &[(1, 10)])]);
        assert_eq!(a.saturating_add(&b), expected.clone());
        assert_eq!(b.saturating_add(&a), expected);
    }

    #[test]
    fn saturating_add_both_none_multiasset() {
        let a = Value::from(BigNum(100));
        let b = Value::from(BigNum(200));
        let expected = Value::from(BigNum(300));
        assert_eq!(a.saturating_add(&b), expected);
    }

    #[test]
    fn saturating_sub_disjoint_policies() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 10)])]);
        let b = fake_value_with_assets(100, &[(1, &[(1, 5)])]);
        let expected = fake_value_with_assets(200, &[(0, &[(1, 10)])]);
        assert_eq!(a.saturating_sub(&b), expected);
    }

    #[test]
    fn saturating_sub_rhs_asset_name_not_in_lhs() {
        let a = fake_value_with_assets(300, &[(0, &[(1, 10)])]);
        let b = fake_value_with_assets(100, &[(0, &[(2, 5)])]);
        let expected = fake_value_with_assets(200, &[(0, &[(1, 10)])]);
        assert_eq!(a.saturating_sub(&b), expected);
    }

    #[test]
    fn saturating_add_overlapping_assets() {
        let a = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let b = fake_value_with_assets(200, &[(0, &[(1, 5)])]);
        let expected = fake_value_with_assets(300, &[(0, &[(1, 15)])]);
        assert_eq!(a.saturating_add(&b), expected);
    }

    #[test]
    fn saturating_add_disjoint_policies() {
        let a = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let b = fake_value_with_assets(200, &[(1, &[(1, 20)])]);
        let expected = fake_value_with_assets(300, &[(0, &[(1, 10)]), (1, &[(1, 20)])]);
        assert_eq!(a.saturating_add(&b), expected);
    }
}

mod value_sub_components {
    use super::*;

    #[test]
    fn sub_coin_from_coin_only_value() {
        let a = Value::from(BigNum(150));
        let b = Coin::from(50u64);
        let expected = Value::from(BigNum(100));
        assert_eq!(a - b, expected);
    }

    #[test]
    fn sub_coin_from_value_with_assets() {
        let a = fake_value_with_assets(150, &[(0, &[(1, 10)])]);
        let b = Coin::from(50u64);
        let expected = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        assert_eq!(a - b, expected);
    }

    #[test]
    fn sub_multiasset_from_value_with_assets() {
        let a = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let b = fake_multiasset(&[(0, &[(1, 4)])]);
        let expected = fake_value_with_assets(100, &[(0, &[(1, 6)])]);
        assert_eq!(a - b, expected);
    }

    #[test]
    fn sub_multiasset_fully_removes_zeroed_asset() {
        let a = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let b = fake_multiasset(&[(0, &[(1, 10)])]);
        let expected = Value::from(BigNum(100));
        assert_eq!(a - b, expected);
    }
}

mod value_add_components {
    use super::*;

    #[test]
    fn add_coin_to_coin_only_value() {
        let a = Value::from(BigNum(100));
        let b = Coin::from(50u64);
        let expected = Value::from(BigNum(150));
        assert_eq!(a + b, expected);
    }

    #[test]
    fn add_coin_to_value_with_assets() {
        let a = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let b = Coin::from(50u64);
        let expected = fake_value_with_assets(150, &[(0, &[(1, 10)])]);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn add_multiasset_to_coin_only_value() {
        let a = Value::from(BigNum(100));
        let b = fake_multiasset(&[(0, &[(1, 10)])]);
        let expected = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn add_multiasset_to_value_with_assets() {
        let a = fake_value_with_assets(100, &[(0, &[(1, 10)])]);
        let b = fake_multiasset(&[(1, &[(2, 20)])]);
        let expected = fake_value_with_assets(100, &[(0, &[(1, 10)]), (1, &[(2, 20)])]);
        assert_eq!(a + b, expected);
    }
}

#[test]
fn value_empty_asset_not_equal() {
    let a = Value {
        coin: BigNum(1),
        multiasset: None,
    };
    let b = Value {
        coin: BigNum(2),
        multiasset: Some(MultiAsset::new()),
    };
    let c = Value {
        coin: BigNum(3),
        multiasset: None,
    };

    assert_ne!(a, b);
    assert_ne!(a, c);
}
