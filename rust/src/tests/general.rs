use crate::*;
use crate::tests::helpers::harden;
use crate::tests::fakes::{fake_plutus_script, fake_boostrap_witness, fake_tx_input, fake_vkey_witness};

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
    map.insert(&name2, &BigNum(1));
    map.insert(&name1, &BigNum(1));
    map.insert(&name3, &BigNum(1));

    assert_eq!(map.keys(), AssetNames(vec![name3, name1, name2]));

    let mut map2 = MintAssets::new();
    map2.insert(&name11, &Int::new_i32(1)).expect("insert failed");
    map2.insert(&name33, &Int::new_i32(1)).expect("insert failed");
    map2.insert(&name22, &Int::new_i32(1)).expect("insert failed");

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
    mass1.insert(&name1, &Int::new(&amount1)).expect("insert failed");
    mass1.insert(&name2, &Int::new(&amount2)).expect("insert failed");

    let mut mass2 = MintAssets::new();
    mass2.insert(&name1, &Int::new(&amount2)).expect("insert failed");
    mass2.insert(&name2, &Int::new(&amount1)).expect("insert failed");

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
    mass1.insert(&name1, &Int::new(&amount1)).expect("insert failed");
    mass1.insert(&name2, &Int::new_negative(&amount2)).expect("insert failed");

    let mut mass2 = MintAssets::new();
    mass2.insert(&name1, &Int::new_negative(&amount1)).expect("insert failed");
    mass2.insert(&name2, &Int::new(&amount2)).expect("insert failed");

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
    mass1.insert(&name1, &Int::new(&amount1)).expect("insert failed");

    let mut mass2 = MintAssets::new();
    mass2.insert(&name1, &Int::new_negative(&amount1)).expect("insert failed");

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

fn scripts_vec(scripts: Vec<&NativeScript>) -> NativeScripts {
    NativeScripts(scripts.iter().map(|s| (*s).clone()).collect())
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
        &scripts_vec(vec![&pkscript(&keyhash1), &pkscript(&keyhash2)]),
    )));
    assert_eq!(pks3.len(), 2);
    assert!(pks3.contains(&keyhash1));
    assert!(pks3.contains(&keyhash2));

    let pks4 = Ed25519KeyHashes::from(&NativeScript::new_script_any(&ScriptAny::new(
        &scripts_vec(vec![
            &NativeScript::new_script_n_of_k(&ScriptNOfK::new(
                1,
                &scripts_vec(vec![
                    &NativeScript::new_timelock_start(&TimelockStart::new(132)),
                    &pkscript(&keyhash3),
                ]),
            )),
            &NativeScript::new_script_all(&ScriptAll::new(&scripts_vec(vec![
                &NativeScript::new_timelock_expiry(&TimelockExpiry::new(132)),
                &pkscript(&keyhash1),
            ]))),
            &NativeScript::new_script_any(&ScriptAny::new(&scripts_vec(vec![
                &pkscript(&keyhash1),
                &pkscript(&keyhash2),
                &pkscript(&keyhash3),
            ]))),
        ]),
    )));
    assert_eq!(pks4.len(), 3);
    assert!(pks4.contains(&keyhash1));
    assert!(pks4.contains(&keyhash2));
    assert!(pks4.contains(&keyhash3));
}

#[test]
fn protocol_params_update_cbor_json_roundtrip() {
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
    orig_ppu.set_pool_pledge_influence(&UnitInterval::new(&BigNum::from(7u32), &BigNum::from(77u32)));
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
    orig_ppu.set_pool_voting_thresholds(&PoolVotingThresholds::new(
        &UnitInterval::new(&BigNum::from(26u32), &BigNum::from(27u32)),
        &UnitInterval::new(&BigNum::from(28u32), &BigNum::from(29u32)),
        &UnitInterval::new(&BigNum::from(30u32), &BigNum::from(31u32)),
        &UnitInterval::new(&BigNum::from(40u32), &BigNum::from(41u32)),
        &UnitInterval::new(&BigNum::from(50u32), &BigNum::from(51u32)),
    ));
    orig_ppu.set_drep_voting_thresholds(&DRepVotingThresholds::new(
        &UnitInterval::new(&BigNum::from(26u32), &BigNum::from(27u32)),
        &UnitInterval::new(&BigNum::from(28u32), &BigNum::from(29u32)),
        &UnitInterval::new(&BigNum::from(30u32), &BigNum::from(31u32)),
        &UnitInterval::new(&BigNum::from(40u32), &BigNum::from(41u32)),
        &UnitInterval::new(&BigNum::from(50u32), &BigNum::from(51u32)),
        &UnitInterval::new(&BigNum::from(60u32), &BigNum::from(61u32)),
        &UnitInterval::new(&BigNum::from(66u32), &BigNum::from(65u32)),
        &UnitInterval::new(&BigNum::from(70u32), &BigNum::from(71u32)),
        &UnitInterval::new(&BigNum::from(77u32), &BigNum::from(75u32)),
        &UnitInterval::new(&BigNum::from(80u32), &BigNum::from(81u32)),
    ));
    orig_ppu.set_min_committee_size(32);
    orig_ppu.set_committee_term_limit(33);
    orig_ppu.set_governance_action_validity_period(34);
    orig_ppu.set_governance_action_deposit(&Coin::from(35u32));
    orig_ppu.set_drep_deposit(&Coin::from(36u32));
    orig_ppu.set_drep_inactivity_period(37);
    orig_ppu.set_ref_script_coins_per_byte(&UnitInterval::new(&BigNum::from(38u32), &BigNum::from(39u32)));

    let encoded_cbor = orig_ppu.to_bytes();
    let decoded_from_cbor = ProtocolParamUpdate::from_bytes(encoded_cbor).unwrap();

    assert_eq!(decoded_from_cbor, orig_ppu);
    assert_eq!(decoded_from_cbor.to_bytes(), orig_ppu.to_bytes());

    let encoded_json = orig_ppu.to_json().unwrap();
    let decoded_from_json = ProtocolParamUpdate::from_json(&encoded_json).unwrap();

    assert_eq!(decoded_from_json, orig_ppu);
    assert_eq!(decoded_from_json.to_json().unwrap(), orig_ppu.to_json().unwrap());
}

#[test]
fn witnesses_deduplication_test(){
    let spend = tests::fakes::fake_root_key_15()
        .derive(harden(1854))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();

    let spending_hash = spend.to_raw_key().hash();

    let native_scripts_1 = NativeScript::new_script_pubkey(&ScriptPubkey::new(
        &spending_hash,
    ));

    let mut internal_scripts = NativeScripts::new();
    internal_scripts.add(&native_scripts_1);
    let native_scripts_2 = NativeScript::new_script_n_of_k(&ScriptNOfK::new(
        1,
        &internal_scripts,
    ));


    let native_scripts_1_1 = native_scripts_1.clone();

    let mut native_scripts = NativeScripts::new();
    native_scripts.add(&native_scripts_1);
    native_scripts.add(&native_scripts_2);
    native_scripts.add(&native_scripts_1_1);


    // recall: this includes keys for input, certs and withdrawals
    let vkey_witness_1 = fake_vkey_witness(1);
    let vkey_witness_1_1 = fake_vkey_witness(1);
    let vkey_witness_2 = fake_vkey_witness(2);

    let mut vkey_witnesses = Vkeywitnesses::new();
    vkey_witnesses.add(&vkey_witness_1);
    vkey_witnesses.add(&vkey_witness_1_1);
    vkey_witnesses.add(&vkey_witness_2);

    let plutus_v1_1 = fake_plutus_script(1, &Language::new_plutus_v1());
    let plutus_v1_1_1 = fake_plutus_script(1, &Language::new_plutus_v1());
    let plutus_v1_2 = fake_plutus_script(2, &Language::new_plutus_v1());

    let plutus_v2_1 = fake_plutus_script(1, &Language::new_plutus_v2());
    let plutus_v2_1_1 = fake_plutus_script(1, &Language::new_plutus_v2());
    let plutus_v2_2 = fake_plutus_script(2, &Language::new_plutus_v2());

    let plutus_v3_1 = fake_plutus_script(1, &Language::new_plutus_v3());
    let plutus_v3_1_1 = fake_plutus_script(1, &Language::new_plutus_v3());
    let plutus_v3_2 = fake_plutus_script(2, &Language::new_plutus_v3());

    let mut plutus_scripts = PlutusScripts::new();
    plutus_scripts.add(&plutus_v1_1);
    plutus_scripts.add(&plutus_v1_1_1);
    plutus_scripts.add(&plutus_v1_2);
    plutus_scripts.add(&plutus_v2_1);
    plutus_scripts.add(&plutus_v2_1_1);
    plutus_scripts.add(&plutus_v2_2);
    plutus_scripts.add(&plutus_v3_1);
    plutus_scripts.add(&plutus_v3_1_1);
    plutus_scripts.add(&plutus_v3_2);

    let mut datums = PlutusList::new();

    let datum_1 = PlutusData::new_integer(&BigInt::from(1));
    let datum_1_1 = PlutusData::new_integer(&BigInt::from(1));
    let datum_2 = PlutusData::new_integer(&BigInt::from(2));
    datums.add(&datum_1);
    datums.add(&datum_1_1);
    datums.add(&datum_2);

    let mut tx_wits_set = TransactionWitnessSet::new();
    tx_wits_set.set_vkeys(&vkey_witnesses);
    tx_wits_set.set_native_scripts(&native_scripts);
    tx_wits_set.set_plutus_scripts(&plutus_scripts);
    tx_wits_set.set_plutus_data(&datums);

    let roundtrip_tx_wits_set = TransactionWitnessSet::from_bytes(tx_wits_set.to_bytes()).unwrap();
    let roundtrip_vkeys = roundtrip_tx_wits_set.vkeys().unwrap();
    assert_eq!(roundtrip_vkeys.len(), 2);
    assert_eq!(roundtrip_vkeys.get(0), vkey_witness_1);
    assert_eq!(roundtrip_vkeys.get(1), vkey_witness_2);

    let roundtrip_native_scripts = roundtrip_tx_wits_set.native_scripts().unwrap();
    assert_eq!(roundtrip_native_scripts.len(), 2);
    assert_eq!(roundtrip_native_scripts.get(0), native_scripts_1);
    assert_eq!(roundtrip_native_scripts.get(1), native_scripts_2);

    let roundtrip_plutus_scripts = roundtrip_tx_wits_set.plutus_scripts().unwrap();
    assert_eq!(roundtrip_plutus_scripts.len(), 6);
    assert_eq!(roundtrip_plutus_scripts.get(0), plutus_v1_1);
    assert_eq!(roundtrip_plutus_scripts.get(1), plutus_v1_2);
    assert_eq!(roundtrip_plutus_scripts.get(2), plutus_v2_1);
    assert_eq!(roundtrip_plutus_scripts.get(3), plutus_v2_2);
    assert_eq!(roundtrip_plutus_scripts.get(4), plutus_v3_1);
    assert_eq!(roundtrip_plutus_scripts.get(5), plutus_v3_2);

    let roundtrip_plutus_data = roundtrip_tx_wits_set.plutus_data().unwrap();
    assert_eq!(roundtrip_plutus_data.len(), 2);
    assert_eq!(roundtrip_plutus_data.get(0), datum_1);
    assert_eq!(roundtrip_plutus_data.get(1), datum_2);
}

#[test]
fn min_ref_script_fee_test(){
    let cost = UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32));
    let total_size = 500;
    let min_fee = min_ref_script_fee(total_size, &cost).unwrap();
    assert_eq!(min_fee, BigNum(250));
}

#[test]
fn min_ref_script_fee_test_fail(){
    let cost = UnitInterval::new(&BigNum::from(1u32), &BigNum::from(0u32));
    let total_size = 500;
    let min_fee = min_ref_script_fee(total_size, &cost);
    assert!(min_fee.is_err());
}

#[test]
fn ed25519_key_hashes_dedup() {
    let mut key_hashes = Ed25519KeyHashes::new();
    let key_hash1 = keyhash(1);
    let key_hash2 = keyhash(2);

    assert!(key_hashes.add(&key_hash1));
    assert!(key_hashes.add(&key_hash2));
    assert_eq!(key_hashes.len(), 2);

    assert!(!key_hashes.add(&key_hash1));
    assert_eq!(key_hashes.len(), 2);
}

#[test]
fn bootstrap_witnesses_dedup() {
    let mut bootstrap_witnesses = BootstrapWitnesses::new();
    let bootstrap_witness1 = fake_boostrap_witness(1);
    let bootstrap_witness2 = fake_boostrap_witness(2);

    assert!(bootstrap_witnesses.add(&bootstrap_witness1));
    assert!(bootstrap_witnesses.add(&bootstrap_witness2));
    assert_eq!(bootstrap_witnesses.len(), 2);

    assert!(!bootstrap_witnesses.add(&bootstrap_witness1));
    assert_eq!(bootstrap_witnesses.len(), 2);
}

#[test]
fn credential_dedup() {
    let mut credentials = Credentials::new();
    let credential1 = Credential::from_keyhash(&keyhash(1));
    let credential2 = Credential::from_keyhash(&keyhash(2));

    assert!(credentials.add(&credential1));
    assert!(credentials.add(&credential2));
    assert_eq!(credentials.len(), 2);

    assert!(!credentials.add(&credential1));
    assert_eq!(credentials.len(), 2);
}

#[test]
fn vkeywitneses_dedup() {
    let mut vkeywitnesses = Vkeywitnesses::new();
    let vkeywitness1 = fake_vkey_witness(1);
    let vkeywitness2 = fake_vkey_witness(2);

    assert!(vkeywitnesses.add(&vkeywitness1));
    assert!(vkeywitnesses.add(&vkeywitness2));
    assert_eq!(vkeywitnesses.len(), 2);

    assert!(!vkeywitnesses.add(&vkeywitness1));
    assert_eq!(vkeywitnesses.len(), 2);
}

#[test]
fn plutus_scripts_dedup_on_tx_witnesses_set() {
    let plutus_script_v1_1 = fake_plutus_script(1, &Language::new_plutus_v1());
    let plutus_script_v1_2 = fake_plutus_script(2, &Language::new_plutus_v1());

    let plutus_script_v2_1 = fake_plutus_script(1, &Language::new_plutus_v2());
    let plutus_script_v2_2 = fake_plutus_script(2, &Language::new_plutus_v2());

    let plutus_script_v3_1 = fake_plutus_script(1, &Language::new_plutus_v3());
    let plutus_script_v3_2 = fake_plutus_script(2, &Language::new_plutus_v3());

    let mut plutus_scrips = PlutusScripts::new();
    plutus_scrips.add(&plutus_script_v1_1);
    plutus_scrips.add(&plutus_script_v1_2);
    plutus_scrips.add(&plutus_script_v1_1);

    plutus_scrips.add(&plutus_script_v2_1);
    plutus_scrips.add(&plutus_script_v2_2);
    plutus_scrips.add(&plutus_script_v2_1);

    plutus_scrips.add(&plutus_script_v3_1);
    plutus_scrips.add(&plutus_script_v3_2);
    plutus_scrips.add(&plutus_script_v3_1);
    assert_eq!(plutus_scrips.len(), 9);

    let mut  tx_wit_set = TransactionWitnessSet::new();
    tx_wit_set.set_plutus_scripts(&plutus_scrips);

    let plutus_scripts_from = tx_wit_set.plutus_scripts().unwrap();
    assert_eq!(plutus_scripts_from.len(), 6);
    assert!(plutus_scripts_from.contains(&plutus_script_v1_1));
    assert!(plutus_scripts_from.contains(&plutus_script_v1_2));
    assert!(plutus_scripts_from.contains(&plutus_script_v2_1));
    assert!(plutus_scripts_from.contains(&plutus_script_v2_2));
    assert!(plutus_scripts_from.contains(&plutus_script_v3_1));
    assert!(plutus_scripts_from.contains(&plutus_script_v3_2));

    let tx_wit_set_bytes = tx_wit_set.to_bytes();
    let tx_wit_set_from_bytes = TransactionWitnessSet::from_bytes(tx_wit_set_bytes).unwrap();
    let plutus_scripts_from_bytes = tx_wit_set_from_bytes.plutus_scripts().unwrap();
    assert_eq!(plutus_scripts_from_bytes.len(), 6);
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v1_1));
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v1_2));
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v2_1));
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v2_2));
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v3_1));
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v3_2));
}

#[test]
fn plutus_scripts_no_dedup_on_auxdata() {
    let plutus_script_v1_1 = fake_plutus_script(1, &Language::new_plutus_v1());
    let plutus_script_v1_2 = fake_plutus_script(2, &Language::new_plutus_v1());

    let plutus_script_v2_1 = fake_plutus_script(1, &Language::new_plutus_v2());
    let plutus_script_v2_2 = fake_plutus_script(2, &Language::new_plutus_v2());

    let plutus_script_v3_1 = fake_plutus_script(1, &Language::new_plutus_v3());
    let plutus_script_v3_2 = fake_plutus_script(2, &Language::new_plutus_v3());

    let mut plutus_scrips = PlutusScripts::new();
    plutus_scrips.add(&plutus_script_v1_1);
    plutus_scrips.add(&plutus_script_v1_2);
    plutus_scrips.add(&plutus_script_v1_1);

    plutus_scrips.add(&plutus_script_v2_1);
    plutus_scrips.add(&plutus_script_v2_2);
    plutus_scrips.add(&plutus_script_v2_1);

    plutus_scrips.add(&plutus_script_v3_1);
    plutus_scrips.add(&plutus_script_v3_2);
    plutus_scrips.add(&plutus_script_v3_1);
    assert_eq!(plutus_scrips.len(), 9);

    let mut  aux_data = AuxiliaryData::new();
    aux_data.set_plutus_scripts(&plutus_scrips);

    let plutus_scripts_from = aux_data.plutus_scripts().unwrap();
    assert_eq!(plutus_scripts_from.len(), 9);
    assert!(plutus_scripts_from.contains(&plutus_script_v1_1));
    assert!(plutus_scripts_from.contains(&plutus_script_v1_2));
    assert!(plutus_scripts_from.contains(&plutus_script_v2_1));
    assert!(plutus_scripts_from.contains(&plutus_script_v2_2));
    assert!(plutus_scripts_from.contains(&plutus_script_v3_1));
    assert!(plutus_scripts_from.contains(&plutus_script_v3_2));

    let aux_data_bytes = aux_data.to_bytes();
    let aux_data_from_bytes = AuxiliaryData::from_bytes(aux_data_bytes).unwrap();
    let plutus_scripts_from_bytes = aux_data_from_bytes.plutus_scripts().unwrap();
    assert_eq!(plutus_scripts_from_bytes.len(), 9);
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v1_1));
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v1_2));
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v2_1));
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v2_2));
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v3_1));
    assert!(plutus_scripts_from_bytes.contains(&plutus_script_v3_2));
}

#[test]
fn native_scripts_dedup_on_tx_witnesses_set() {
    let keyhash1 = keyhash(1);

    let native_scripts_1 = NativeScript::new_script_pubkey(&ScriptPubkey::new(
        &keyhash1,
    ));

    let mut internal_scripts = NativeScripts::new();
    internal_scripts.add(&native_scripts_1);
    let native_scripts_2 = NativeScript::new_script_n_of_k(&ScriptNOfK::new(
        1,
        &internal_scripts,
    ));

    let mut native_scripts = NativeScripts::new();
    native_scripts.add(&native_scripts_1);
    native_scripts.add(&native_scripts_2);
    native_scripts.add(&native_scripts_1);
    assert_eq!(native_scripts.len(), 3);

    let mut  tx_wit_set = TransactionWitnessSet::new();
    tx_wit_set.set_native_scripts(&native_scripts);

    let native_scripts_from = tx_wit_set.native_scripts().unwrap();
    assert_eq!(native_scripts_from.len(), 2);
    assert!(native_scripts_from.contains(&native_scripts_1));
    assert!(native_scripts_from.contains(&native_scripts_2));

    let tx_wit_set_bytes = tx_wit_set.to_bytes();
    let tx_wit_set_from_bytes = TransactionWitnessSet::from_bytes(tx_wit_set_bytes).unwrap();
    let native_scripts_from_bytes = tx_wit_set_from_bytes.native_scripts().unwrap();
    assert_eq!(native_scripts_from_bytes.len(), 2);
    assert!(native_scripts_from_bytes.contains(&native_scripts_1));
    assert!(native_scripts_from_bytes.contains(&native_scripts_2));
}

#[test]
fn native_scripts_no_dedup_on_auxdata() {
    let keyhash1 = keyhash(1);

    let native_scripts_1 = NativeScript::new_script_pubkey(&ScriptPubkey::new(
        &keyhash1,
    ));

    let mut internal_scripts = NativeScripts::new();
    internal_scripts.add(&native_scripts_1);
    let native_scripts_2 = NativeScript::new_script_n_of_k(&ScriptNOfK::new(
        1,
        &internal_scripts,
    ));

    let mut native_scripts = NativeScripts::new();
    native_scripts.add(&native_scripts_1);
    native_scripts.add(&native_scripts_2);
    native_scripts.add(&native_scripts_1);
    assert_eq!(native_scripts.len(), 3);

    let mut  aux_data = AuxiliaryData::new();
    aux_data.set_native_scripts(&native_scripts);

    let native_scripts_from = aux_data.native_scripts().unwrap();
    assert_eq!(native_scripts_from.len(), 3);
    assert!(native_scripts_from.contains(&native_scripts_1));
    assert!(native_scripts_from.contains(&native_scripts_2));

    let aux_data_bytes = aux_data.to_bytes();
    let aux_data_from_bytes = AuxiliaryData::from_bytes(aux_data_bytes).unwrap();
    let native_scripts_from_bytes = aux_data_from_bytes.native_scripts().unwrap();
    assert_eq!(native_scripts_from_bytes.len(), 3);
    assert!(native_scripts_from_bytes.contains(&native_scripts_1));
    assert!(native_scripts_from_bytes.contains(&native_scripts_2));
}

#[test]
fn plutus_data_dedup_on_tx_witnesses_set() {
    let datum_1 = PlutusData::new_integer(&BigInt::from(1));
    let datum_2 = PlutusData::new_integer(&BigInt::from(2));

    let mut datum = PlutusList::new();
    datum.add(&datum_1);
    datum.add(&datum_2);
    datum.add(&datum_1);
    assert_eq!(datum.len(), 3);

    let mut  tx_wit_set = TransactionWitnessSet::new();
    tx_wit_set.set_plutus_data(&datum);

    let datums_from = tx_wit_set.plutus_data().unwrap();
    assert_eq!(datums_from.len(), 2);
    assert!(datums_from.contains(&datum_1));
    assert!(datums_from.contains(&datum_2));

    let tx_wit_set_bytes = tx_wit_set.to_bytes();
    let tx_wit_set_from_bytes = TransactionWitnessSet::from_bytes(tx_wit_set_bytes).unwrap();
    let datums_from_bytes = tx_wit_set_from_bytes.plutus_data().unwrap();
    assert_eq!(datums_from_bytes.len(), 2);
    assert!(datums_from_bytes.contains(&datum_1));
    assert!(datums_from_bytes.contains(&datum_2));
}

#[test]
fn plutus_data_no_dedup_serialization() {
    let datum_1 = PlutusData::new_integer(&BigInt::from(1));
    let datum_2 = PlutusData::new_integer(&BigInt::from(2));

    let mut datum = PlutusList::new();
    datum.add(&datum_1);
    datum.add(&datum_2);
    datum.add(&datum_1);
    assert_eq!(datum.len(), 3);

    let datum_bytes = datum.to_bytes();
    let datum_from_bytes = PlutusList::from_bytes(datum_bytes).unwrap();
    assert_eq!(datum_from_bytes.len(), 3);
    assert!(datum_from_bytes.contains(&datum_1));
    assert!(datum_from_bytes.contains(&datum_2));
}

#[test]
fn tx_inputs_deduplication() {
    let tx_in1 = fake_tx_input(1);
    let tx_in2 = fake_tx_input(2);
    let tx_in3 = fake_tx_input(3);

    let mut txins = TransactionInputs::new();
    assert!(txins.add(&tx_in1));
    assert!(txins.add(&tx_in2));
    assert!(txins.add(&tx_in3));
    assert!(!txins.add(&tx_in1));

    assert_eq!(txins.len(), 3);

    let txins_bytes = txins.to_bytes();
    let txins_from_bytes = TransactionInputs::from_bytes(txins_bytes).unwrap();
    assert_eq!(txins_from_bytes.len(), 3);
    assert!(txins_from_bytes.contains(&tx_in1));
    assert!(txins_from_bytes.contains(&tx_in2));
    assert!(txins_from_bytes.contains(&tx_in3));
}

// Helper function to create a UnitInterval from a fraction
fn new_uinternal(numerator: u64, denominator: u64) -> UnitInterval {
    UnitInterval::new(&BigNum::from(numerator), &BigNum::from(denominator))
}

#[test]
fn min_ref_script_fee_zero_size_test() {
    let result = min_ref_script_fee(0, &new_uinternal(1, 1000)).unwrap();
    assert_eq!(result.to_str(), "0");
}

#[test]
fn min_ref_script_fee_small_size_test() {
    let result = min_ref_script_fee(1000, &new_uinternal(1, 1000)).unwrap();
    assert_eq!(result.to_str(), "1");
}

#[test]
fn min_ref_script_fee_exactly_one_tier_test() {
    let result = min_ref_script_fee(25600, &new_uinternal(1, 1000)).unwrap();
    assert_eq!(result.to_str(), "25");
}

#[test]
fn min_ref_script_fee_multiple_full_tiers_test() {
    let result = min_ref_script_fee(25600 * 2, &new_uinternal(1, 1000)).unwrap();
    let expected = ((25600f64 / 1000f64) + (25600f64 * 0.0012f64)) as u64;
    assert_eq!(result, BigNum(expected));
}

#[test]
fn min_ref_script_fee_partial_tier_test() {
    let result = min_ref_script_fee(30000, &new_uinternal(1, 1000)).unwrap();
    assert_eq!(result.to_str(), "30");
}

#[test]
fn min_ref_script_fee_large_size_test() {
    let result = min_ref_script_fee(1000000, &new_uinternal(1, 1000)).unwrap();
    assert_eq!(result.to_str(), "158607");
}

#[test]
fn min_ref_script_fee_different_cost_per_byte_test() {
    let result = min_ref_script_fee(50000, &new_uinternal(5, 1000)).unwrap();
    assert_eq!(result.to_str(), "274");
}

#[test]
fn min_ref_script_fee_one_cost_per_byte_test() {
    let result = min_ref_script_fee(10000, &new_uinternal(1, 1)).unwrap();
    assert_eq!(result.to_str(), "10000");
}

#[test]
fn min_ref_script_fee_zero_cost_per_byte_test() {
    let fee = min_ref_script_fee(10000, &new_uinternal(0, 1)).unwrap();
    assert_eq!(fee.to_str(), "0");
}

#[test]
fn test_multiple_tiers() {
    // Test cases with different numbers of tiers
    let test_cases = [
        (25600, "25"),   // Exactly 1 tier
        (25601, "25"),   // 1 full tier + 1 byte (at 1.2x price)
        (51200, "56"),   // Exactly 2 tiers
        (76800, "93"),   // Exactly 3 tiers
        (80000, "98"),   // 3 full tiers + partial tier
        (100000, "133"), // 3 full tiers + larger partial tier
        (128000, "190"), // Exactly 5 tiers
        (179200, "330"), // 7 full tiers
    ];

    for (size, expected) in test_cases.iter() {
        let result = min_ref_script_fee(*size, &new_uinternal(1, 1000)).unwrap();
        assert_eq!(result.to_str(), *expected, "Failed for size {}", size);
    }
}

#[test]
fn plutus_map_keys_duplication_test() {
    let mut map = PlutusMap::new();
    let key1 = PlutusData::new_integer(&BigInt::from(1));
    let key2 = PlutusData::new_integer(&BigInt::from(2));
    let value1 = PlutusData::new_integer(&BigInt::from(3));
    let value2 = PlutusData::new_integer(&BigInt::from(4));
    let value3 = PlutusData::new_integer(&BigInt::from(5));

    assert_eq!(map.len(), 0);
    assert_eq!(map.total_len(), 0);

    let mut plutus_map_value1 = PlutusMapValues::new();
    plutus_map_value1.add(&value1);
    plutus_map_value1.add(&value2);

    let mut plutus_map_value2 = PlutusMapValues::new();
    plutus_map_value2.add(&value3);

    map.insert(&key1, &plutus_map_value1);
    map.insert(&key2, &plutus_map_value2);

    assert_eq!(map.len(), 2);
    assert_eq!(map.total_len(), 3);

    let map_bytes = map.to_bytes();
    let map_from_bytes = PlutusMap::from_bytes(map_bytes).unwrap();
    assert_eq!(map_from_bytes.len(), 2);
    assert_eq!(map_from_bytes.total_len(), 3);

    assert_eq!(map, map_from_bytes)
}

#[test]
fn too_big_plutus_int_to_json() {
    let too_big_int = BigInt::from_str("999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999").unwrap();
    let plutus_data = PlutusData::new_integer(&too_big_int);
    let json = plutus_data.to_json(PlutusDatumSchema::DetailedSchema);
    #[cfg(feature = "arbitrary-precision-json")]
    {
        assert!(json.is_ok());
    }
    #[cfg(not(feature = "arbitrary-precision-json"))]
    {
        assert!(json.is_err());
    }
}