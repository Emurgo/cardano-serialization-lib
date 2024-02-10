use crate::*;

#[test]
fn native_script_hash() {
    let keyhash = Ed25519KeyHash::from_bytes(vec![
        143, 180, 186, 93, 223, 42, 243, 7, 81, 98, 86, 125, 97, 69, 110, 52, 130, 243, 244, 98,
        246, 13, 33, 212, 128, 168, 136, 40,
    ])
    .unwrap();
    assert_eq!(
        hex::encode(&keyhash.to_bytes()),
        "8fb4ba5ddf2af3075162567d61456e3482f3f462f60d21d480a88828"
    );

    let script = NativeScript::new_script_pubkey(&ScriptPubkey::new(&keyhash));

    let script_hash = script.hash();

    assert_eq!(
        hex::encode(&script_hash.to_bytes()),
        "187b8d3ddcb24013097c003da0b8d8f7ddcf937119d8f59dccd05a0f"
    );
}

#[test]
fn asset_name_ord() {
    let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
    let name11 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

    let name2 = AssetName::new(vec![0u8, 4, 5, 6]).unwrap();
    let name22 = AssetName::new(vec![0u8, 4, 5, 6]).unwrap();

    let name3 = AssetName::new(vec![0u8, 7, 8]).unwrap();
    let name33 = AssetName::new(vec![0u8, 7, 8]).unwrap();

    assert_eq!(name1.cmp(&name2), Ordering::Less);
    assert_eq!(name2.cmp(&name1), Ordering::Greater);
    assert_eq!(name1.cmp(&name3), Ordering::Greater);
    assert_eq!(name2.cmp(&name3), Ordering::Greater);
    assert_eq!(name3.cmp(&name1), Ordering::Less);
    assert_eq!(name3.cmp(&name2), Ordering::Less);

    assert_eq!(name1.cmp(&name11), Ordering::Equal);
    assert_eq!(name2.cmp(&name22), Ordering::Equal);
    assert_eq!(name3.cmp(&name33), Ordering::Equal);

    let mut map = Assets::new();
    map.insert(&name2, &to_bignum(1));
    map.insert(&name1, &to_bignum(1));
    map.insert(&name3, &to_bignum(1));

    assert_eq!(map.keys(), AssetNames(vec![name3, name1, name2]));

    let mut map2 = MintAssets::new();
    map2.insert(&name11, Int::new_i32(1));
    map2.insert(&name33, Int::new_i32(1));
    map2.insert(&name22, Int::new_i32(1));

    assert_eq!(map2.keys(), AssetNames(vec![name33, name11, name22]));
}

#[test]
fn mint_to_multiasset() {
    let policy_id1 = PolicyID::from([0u8; 28]);
    let policy_id2 = PolicyID::from([1u8; 28]);
    let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
    let name2 = AssetName::new(vec![0u8, 4, 5, 6]).unwrap();
    let amount1 = BigNum::from_str("1234").unwrap();
    let amount2 = BigNum::from_str("5678").unwrap();

    let mut mass1 = MintAssets::new();
    mass1.insert(&name1, Int::new(&amount1));
    mass1.insert(&name2, Int::new(&amount2));

    let mut mass2 = MintAssets::new();
    mass2.insert(&name1, Int::new(&amount2));
    mass2.insert(&name2, Int::new(&amount1));

    let mut mint = Mint::new();
    mint.insert(&policy_id1, &mass1);
    mint.insert(&policy_id2, &mass2);

    let multiasset = mint.as_positive_multiasset();
    assert_eq!(multiasset.len(), 2);

    let ass1 = multiasset.get(&policy_id1).unwrap();
    let ass2 = multiasset.get(&policy_id2).unwrap();

    assert_eq!(ass1.len(), 2);
    assert_eq!(ass2.len(), 2);

    assert_eq!(ass1.get(&name1).unwrap(), amount1);
    assert_eq!(ass1.get(&name2).unwrap(), amount2);

    assert_eq!(ass2.get(&name1).unwrap(), amount2);
    assert_eq!(ass2.get(&name2).unwrap(), amount1);
}

#[test]
fn mint_to_negative_multiasset() {
    let policy_id1 = PolicyID::from([0u8; 28]);
    let policy_id2 = PolicyID::from([1u8; 28]);
    let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
    let name2 = AssetName::new(vec![0u8, 4, 5, 6]).unwrap();
    let amount1 = BigNum::from_str("1234").unwrap();
    let amount2 = BigNum::from_str("5678").unwrap();

    let mut mass1 = MintAssets::new();
    mass1.insert(&name1, Int::new(&amount1));
    mass1.insert(&name2, Int::new_negative(&amount2));

    let mut mass2 = MintAssets::new();
    mass2.insert(&name1, Int::new_negative(&amount1));
    mass2.insert(&name2, Int::new(&amount2));

    let mut mint = Mint::new();
    mint.insert(&policy_id1, &mass1);
    mint.insert(&policy_id2, &mass2);

    let p_multiasset = mint.as_positive_multiasset();
    let n_multiasset = mint.as_negative_multiasset();

    assert_eq!(p_multiasset.len(), 2);
    assert_eq!(n_multiasset.len(), 2);

    let p_ass1 = p_multiasset.get(&policy_id1).unwrap();
    let p_ass2 = p_multiasset.get(&policy_id2).unwrap();

    let n_ass1 = n_multiasset.get(&policy_id1).unwrap();
    let n_ass2 = n_multiasset.get(&policy_id2).unwrap();

    assert_eq!(p_ass1.len(), 1);
    assert_eq!(p_ass2.len(), 1);
    assert_eq!(n_ass1.len(), 1);
    assert_eq!(n_ass2.len(), 1);

    assert_eq!(p_ass1.get(&name1).unwrap(), amount1);
    assert!(p_ass1.get(&name2).is_none());

    assert!(p_ass2.get(&name1).is_none());
    assert_eq!(p_ass2.get(&name2).unwrap(), amount2);

    assert!(n_ass1.get(&name1).is_none());
    assert_eq!(n_ass1.get(&name2).unwrap(), amount2);

    assert_eq!(n_ass2.get(&name1).unwrap(), amount1);
    assert!(n_ass2.get(&name2).is_none());
}

#[test]
fn mint_to_negative_multiasset_empty() {
    let policy_id1 = PolicyID::from([0u8; 28]);
    let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
    let amount1 = BigNum::from_str("1234").unwrap();

    let mut mass1 = MintAssets::new();
    mass1.insert(&name1, Int::new(&amount1));

    let mut mass2 = MintAssets::new();
    mass2.insert(&name1, Int::new_negative(&amount1));

    let mut mint1 = Mint::new();
    mint1.insert(&policy_id1, &mass1);

    let mut mint2 = Mint::new();
    mint2.insert(&policy_id1, &mass2);

    let p_multiasset_some = mint1.as_positive_multiasset();
    let p_multiasset_none = mint2.as_positive_multiasset();

    let n_multiasset_none = mint1.as_negative_multiasset();
    let n_multiasset_some = mint2.as_negative_multiasset();

    assert_eq!(p_multiasset_some.len(), 1);
    assert_eq!(p_multiasset_none.len(), 0);

    assert_eq!(n_multiasset_some.len(), 1);
    assert_eq!(n_multiasset_none.len(), 0);

    let p_ass = p_multiasset_some.get(&policy_id1).unwrap();
    let n_ass = n_multiasset_some.get(&policy_id1).unwrap();

    assert_eq!(p_ass.len(), 1);
    assert_eq!(n_ass.len(), 1);

    assert_eq!(p_ass.get(&name1).unwrap(), amount1);
    assert_eq!(n_ass.get(&name1).unwrap(), amount1);
}

fn keyhash(x: u8) -> Ed25519KeyHash {
    Ed25519KeyHash::from_bytes(vec![
        x, 180, 186, 93, 223, 42, 243, 7, 81, 98, 86, 125, 97, 69, 110, 52, 130, 243, 244, 98, 246,
        13, 33, 212, 128, 168, 136, 40,
    ])
    .unwrap()
}

fn pkscript(pk: &Ed25519KeyHash) -> NativeScript {
    NativeScript::new_script_pubkey(&ScriptPubkey::new(pk))
}

#[test]
fn native_scripts_get_pubkeys() {
    let keyhash1 = keyhash(1);
    let keyhash2 = keyhash(2);
    let keyhash3 = keyhash(3);

    let pks1 = Ed25519KeyHashes::from(&pkscript(&keyhash1));
    assert_eq!(pks1.len(), 1);
    assert!(pks1.contains(&keyhash1));

    let pks2 =
        Ed25519KeyHashes::from(&NativeScript::new_timelock_start(&TimelockStart::new(123)));
    assert_eq!(pks2.len(), 0);

    let pks3 = Ed25519KeyHashes::from(&NativeScript::new_script_all(&ScriptAll::new(
        &NativeScripts::from_vec(vec![pkscript(&keyhash1), pkscript(&keyhash2)]),
    )));
    assert_eq!(pks3.len(), 2);
    assert!(pks3.contains(&keyhash1));
    assert!(pks3.contains(&keyhash2));

    let pks4 = Ed25519KeyHashes::from(&NativeScript::new_script_any(&ScriptAny::new(
        &NativeScripts::from_vec(vec![
            NativeScript::new_script_n_of_k(&ScriptNOfK::new(
                1,
                &NativeScripts::from_vec(vec![
                    NativeScript::new_timelock_start(&TimelockStart::new(132)),
                    pkscript(&keyhash3),
                ]),
            )),
            NativeScript::new_script_all(&ScriptAll::new(&NativeScripts::from_vec(vec![
                NativeScript::new_timelock_expiry(&TimelockExpiry::new(132)),
                pkscript(&keyhash1),
            ]))),
            NativeScript::new_script_any(&ScriptAny::new(&NativeScripts::from_vec(vec![
                pkscript(&keyhash1),
                pkscript(&keyhash2),
                pkscript(&keyhash3),
            ]))),
        ]),
    )));
    assert_eq!(pks4.len(), 3);
    assert!(pks4.contains(&keyhash1));
    assert!(pks4.contains(&keyhash2));
    assert!(pks4.contains(&keyhash3));
}

#[test]
fn protocol_params_update_cbor_roundtrip() {
    let mut orig_ppu = ProtocolParamUpdate::new();
    orig_ppu.set_max_tx_size(1234);
    orig_ppu.set_max_block_body_size(5678);
    orig_ppu.set_max_block_header_size(91011);
    orig_ppu.set_minfee_a(&Coin::from(1u32));
    orig_ppu.set_minfee_b(&Coin::from(2u32));
    orig_ppu.set_key_deposit(&Coin::from(3u32));
    orig_ppu.set_pool_deposit(&Coin::from(4u32));
    orig_ppu.set_max_epoch(5);
    orig_ppu.set_n_opt(6);
    orig_ppu.set_pool_pledge_influence(&Rational::new(&BigNum::from(7u32), &BigNum::from(77u32)));
    orig_ppu.set_expansion_rate(&UnitInterval::new(&BigNum::from(8u32), &BigNum::from(9u32)));
    orig_ppu.set_treasury_growth_rate(&UnitInterval::new(
        &BigNum::from(10u32),
        &BigNum::from(11u32),
    ));
    orig_ppu.set_protocol_version(&ProtocolVersion::new(12u32, 13u32));
    orig_ppu.set_min_pool_cost(&Coin::from(14u32));
    orig_ppu.set_ada_per_utxo_byte(&Coin::from(15u32));
    orig_ppu.set_cost_models(&TxBuilderConstants::plutus_vasil_cost_models());
    orig_ppu.set_execution_costs(&ExUnitPrices::new(
        &SubCoin::new(&BigNum::from(16u32), &BigNum::from(17u32)),
        &SubCoin::new(&BigNum::from(18u32), &BigNum::from(19u32)),
    ));
    orig_ppu.set_max_tx_ex_units(&ExUnits::new(&BigNum::from(20u32), &BigNum::from(21u32)));
    orig_ppu.set_max_block_ex_units(&ExUnits::new(&BigNum::from(22u32), &BigNum::from(23u32)));
    orig_ppu.set_max_value_size(24);
    orig_ppu.set_collateral_percentage(25);
    orig_ppu.set_max_collateral_inputs(25);

    let encoded = orig_ppu.to_bytes();
    let dencoded = ProtocolParamUpdate::from_bytes(encoded).unwrap();

    assert_eq!(dencoded, orig_ppu);
    assert_eq!(dencoded.to_bytes(), orig_ppu.to_bytes());
}
