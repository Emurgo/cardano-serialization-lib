use crate::*;
use crate::fakes::{fake_plutus_script_and_hash, fake_script_hash, fake_tx_input};
use crate::tests::mock_objects::{create_reallistic_tx_builder, create_redeemer};

#[test]
fn plutus_mint_with_script_ref_test() {
    let mut tx_builder = create_reallistic_tx_builder();
    let colateral_adress = Address::from_bech32("addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w").unwrap();
    let colateral_input = fake_tx_input(0);
    let tx_input = fake_tx_input(1);
    let tx_input_ref = fake_tx_input(2);

    let (plutus_script, _) = fake_plutus_script_and_hash(1);

    let (plutus_script2, _) = fake_plutus_script_and_hash(2);

    let redeemer = create_redeemer(1);

    let redeemer2 = create_redeemer(2);

    let asset_name = AssetName::from_hex("44544e4654").unwrap();
    let mut mint_builder = MintBuilder::new();
    let plutus_script_source = PlutusScriptSource::new(&plutus_script);
    let plutus_script_source_ref = PlutusScriptSource::new_ref_input(
        &plutus_script2.hash(),
        &tx_input_ref,
        &Language::new_plutus_v2(),
        0,
    );
    let mint_witnes = MintWitness::new_plutus_script(&plutus_script_source, &redeemer);
    let mint_witnes_ref = MintWitness::new_plutus_script(&plutus_script_source_ref, &redeemer2);
    mint_builder.add_asset(&mint_witnes, &asset_name, &Int::new(&BigNum::from(100u64))).unwrap();
    mint_builder.add_asset(
        &mint_witnes_ref,
        &asset_name,
        &Int::new(&BigNum::from(100u64)),
    ).unwrap();

    let output_adress = Address::from_bech32("addr_test1qpm5njmgzf4t7225v6j34wl30xfrufzt3jtqtdzf3en9ahpmnhtmynpasyc8fq75zv0uaj86vzsr7g3g8q5ypgu5fwtqr9zsgj").unwrap();
    let mut output_assets = MultiAsset::new();
    let mut asset = Assets::new();
    asset.insert(&asset_name, &BigNum::from(100u64));
    output_assets.insert(&plutus_script.hash(), &asset);
    let output_value = Value::new_with_assets(&Coin::from(50000000u64), &output_assets);
    let output = TransactionOutput::new(&output_adress, &output_value);

    let mut col_builder = TxInputsBuilder::new();
    col_builder.add_regular_input(
        &colateral_adress,
        &colateral_input,
        &Value::new(&Coin::from(1000000000u64)),
    ).unwrap();
    tx_builder.set_collateral(&col_builder);
    tx_builder.add_output(&output).unwrap();
    tx_builder.add_regular_input(
        &output_adress,
        &tx_input,
        &Value::new(&BigNum::from(100000000000u64)),
    ).unwrap();
    tx_builder.set_mint_builder(&mint_builder);

    tx_builder
        .calc_script_data_hash(&TxBuilderConstants::plutus_vasil_cost_models())
        .unwrap();

    let change_res = tx_builder.add_change_if_needed(&output_adress);
    assert!(change_res.is_ok());

    let build_res = tx_builder.build_tx();
    assert!(build_res.is_ok());

    let tx = build_res.unwrap();
    assert_eq!(tx.witness_set.plutus_scripts.unwrap().len(), 1usize);
    assert_eq!(tx.witness_set.redeemers.unwrap().len(), 2usize);
    assert!(tx.witness_set.plutus_data.is_none());
    assert_eq!(tx.body.reference_inputs.unwrap().len(), 1usize);
    assert!(tx.body.mint.is_some());
    assert_eq!(tx.body.mint.unwrap().len(), 2usize);
}

#[test]
fn different_redeemers_error() {
    let (plutus_script, _) = fake_plutus_script_and_hash(1);
    let redeemer = create_redeemer(1);
    let redeemer2 = create_redeemer(2);

    let asset_name = AssetName::from_hex("44544e4654").unwrap();
    let mut mint_builder = MintBuilder::new();
    let plutus_script_source = PlutusScriptSource::new(&plutus_script);
    let mint_witnes = MintWitness::new_plutus_script(&plutus_script_source, &redeemer);
    let mint_witnes2 = MintWitness::new_plutus_script(&plutus_script_source, &redeemer2);

    let res1 = mint_builder.add_asset(&mint_witnes, &asset_name, &Int::new(&BigNum::from(100u64)));
    assert!(res1.is_ok());

    let res1 =  mint_builder.add_asset(&mint_witnes2, &asset_name, &Int::new(&BigNum::from(100u64)));
    assert!(res1.is_err());
}

#[test]
fn same_redeemers() {
    let (plutus_script, _) = fake_plutus_script_and_hash(1);
    let redeemer = create_redeemer(1);
    let mut redeemer2 = redeemer.clone();
    redeemer2.index = BigNum::from(77u64);
    redeemer2.tag = RedeemerTag::new_voting_proposal();

    let asset_name1 = AssetName::from_hex("44544e4654").unwrap();
    let asset_name2 = AssetName::from_hex("44544e4655").unwrap();
    let mut mint_builder = MintBuilder::new();
    let plutus_script_source = PlutusScriptSource::new(&plutus_script);
    let mint_witnes = MintWitness::new_plutus_script(&plutus_script_source, &redeemer);
    let mint_witnes2 = MintWitness::new_plutus_script(&plutus_script_source, &redeemer2);

    let res1 = mint_builder.add_asset(&mint_witnes, &asset_name1, &Int::new(&BigNum::from(100u64)));
    assert!(res1.is_ok());

    let res1 =  mint_builder.add_asset(&mint_witnes2, &asset_name2, &Int::new(&BigNum::from(100u64)));
    assert!(res1.is_ok());
}

#[test]
fn plutus_mint_test() {
    let mut tx_builder = create_reallistic_tx_builder();
    let colateral_adress = Address::from_bech32("addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w").unwrap();
    let colateral_input = fake_tx_input(0);

    let tx_input = fake_tx_input(1);
    let (plutus_script, _) = fake_plutus_script_and_hash(1);

    let redeemer = create_redeemer(1);
    let asset_name = AssetName::from_hex("44544e4654").unwrap();
    let mut mint_builder = MintBuilder::new();
    let plutus_script_source = PlutusScriptSource::new(&plutus_script);
    let mint_witnes = MintWitness::new_plutus_script(&plutus_script_source, &redeemer);
    mint_builder.add_asset(&mint_witnes, &asset_name, &Int::new(&BigNum::from(100u64))).unwrap();

    let output_adress = Address::from_bech32("addr_test1qpm5njmgzf4t7225v6j34wl30xfrufzt3jtqtdzf3en9ahpmnhtmynpasyc8fq75zv0uaj86vzsr7g3g8q5ypgu5fwtqr9zsgj").unwrap();
    let mut output_assets = MultiAsset::new();
    let mut asset = Assets::new();
    asset.insert(&asset_name, &BigNum::from(100u64));
    output_assets.insert(&plutus_script.hash(), &asset);
    let output_value = Value::new_with_assets(&Coin::from(5000000u64), &output_assets);
    let output = TransactionOutput::new(&output_adress, &output_value);

    let mut col_builder = TxInputsBuilder::new();
    col_builder.add_regular_input(
        &colateral_adress,
        &colateral_input,
        &Value::new(&Coin::from(1000000000000u64)),
    ).unwrap();
    tx_builder.set_collateral(&col_builder);
    tx_builder.add_output(&output).unwrap();
    tx_builder.add_regular_input(
        &output_adress,
        &tx_input,
        &Value::new(&BigNum::from(100000000000000u64)),
    ).unwrap();
    tx_builder.set_mint_builder(&mint_builder);

    tx_builder
        .calc_script_data_hash(&TxBuilderConstants::plutus_vasil_cost_models())
        .unwrap();

    let change_res = tx_builder.add_change_if_needed(&output_adress);
    assert!(change_res.is_ok());

    let build_res = tx_builder.build_tx();
    assert!(build_res.is_ok());

    assert_eq!(mint_builder.get_plutus_witnesses().len(), 1);

    let tx = build_res.unwrap();
    assert!(tx.body.mint.is_some());
    assert_eq!(
        tx.body.mint.unwrap().0.iter().next().unwrap().0,
        plutus_script.hash()
    );
}

#[test]
fn ref_inputs() {
    let script_hash_1 = fake_script_hash(1);
    let script_hash_2 = fake_script_hash(2);
    let tx_input_ref1 = fake_tx_input(2);
    let tx_input_ref2 = fake_tx_input(3);
    let redeemer = create_redeemer(1);

    let asset_name1 = AssetName::from_hex("44544e4654").unwrap();
    let asset_name2 = AssetName::from_hex("44544e4655").unwrap();

    let mut mint_builder = MintBuilder::new();
    let plutus_script_source = PlutusScriptSource::new_ref_input(
        &script_hash_1,
        &tx_input_ref1,
        &Language::new_plutus_v2(),
        0,
    );
    let native_script_source = NativeScriptSource::new_ref_input(
        &script_hash_2,
        &tx_input_ref2,
    );

    let mint_witnes = MintWitness::new_plutus_script(&plutus_script_source, &redeemer);
    let mint_witnes2 = MintWitness::new_native_script(&native_script_source);

    mint_builder.add_asset(&mint_witnes, &asset_name1, &Int::new(&BigNum::from(100u64))).unwrap();
    mint_builder.add_asset(&mint_witnes2, &asset_name2, &Int::new(&BigNum::from(100u64))).unwrap();

    let ref_inputs = mint_builder.get_ref_inputs();

    assert_eq!(ref_inputs.len(), 2);
    assert!(ref_inputs.contains(&tx_input_ref1));
    assert!(ref_inputs.contains(&tx_input_ref2));
}

#[test]
fn multiple_mints() {
    let script_hash_1 = fake_script_hash(1);
    let script_hash_2 = fake_script_hash(2);
    let tx_input_ref1 = fake_tx_input(2);
    let tx_input_ref2 = fake_tx_input(3);
    let redeemer = create_redeemer(1);

    let asset_name1 = AssetName::from_hex("44544e4654").unwrap();
    let asset_name2 = AssetName::from_hex("44544e4655").unwrap();
    let asset_name3 = AssetName::from_hex("44544e4656").unwrap();

    let mut mint_builder = MintBuilder::new();
    let plutus_script_source = PlutusScriptSource::new_ref_input(
        &script_hash_1,
        &tx_input_ref1,
        &Language::new_plutus_v2(),
        0,
    );

    let native_script_source = NativeScriptSource::new_ref_input(
        &script_hash_2,
        &tx_input_ref2,
    );

    let mint_witnes = MintWitness::new_plutus_script(&plutus_script_source, &redeemer);
    let mint_witnes2 = MintWitness::new_native_script(&native_script_source);

    mint_builder.add_asset(&mint_witnes, &asset_name1, &Int::new(&BigNum::from(100u64))).unwrap();
    mint_builder.add_asset(&mint_witnes2, &asset_name2, &Int::new(&BigNum::from(101u64))).unwrap();
    mint_builder.add_asset(&mint_witnes2, &asset_name3, &Int::new(&BigNum::from(102u64))).unwrap();

    let mint = mint_builder.build();
    assert_eq!(mint.len(), 2);

    let policy_mints_list = mint.get(&script_hash_1).unwrap();
    let policy_mints_list2 = mint.get(&script_hash_2).unwrap();

    assert_eq!(policy_mints_list.len(), 1);
    assert_eq!(policy_mints_list2.len(), 1);

    let policy_mints = policy_mints_list.get(0).unwrap();
    let policy_mints2 = policy_mints_list2.get(0).unwrap();

    assert_eq!(policy_mints.len(), 1);
    assert_eq!(policy_mints2.len(), 2);

    assert_eq!(policy_mints.get(&asset_name1).unwrap().to_str(), "100");
    assert_eq!(policy_mints2.get(&asset_name2).unwrap().to_str(), "101");
    assert_eq!(policy_mints2.get(&asset_name3).unwrap().to_str(), "102");
}

#[test]
fn native_script_mint() {
    let native_script = NativeScript::new_timelock_start(
        &TimelockStart::new_timelockstart(&BigNum::from(100u64)),
    );
    let script_hash = native_script.hash();

    let asset_name1 = AssetName::from_hex("44544e4654").unwrap();

    let mut mint_builder = MintBuilder::new();

    let native_script_source = NativeScriptSource::new(&native_script);
    let mint_witnes = MintWitness::new_native_script(&native_script_source);
    mint_builder.add_asset(&mint_witnes, &asset_name1, &Int::new(&BigNum::from(100u64))).unwrap();

    let mint = mint_builder.build();
    assert_eq!(mint.len(), 1);

    let policy_mints_list = mint.get(&script_hash).unwrap();
    assert_eq!(policy_mints_list.len(), 1);

    let policy_mints = policy_mints_list.get(0).unwrap();
    assert_eq!(policy_mints.len(), 1);
    assert_eq!(policy_mints.get(&asset_name1).unwrap().to_str(), "100");

    let native_scripts = mint_builder.get_native_scripts();
    assert_eq!(native_scripts.len(), 1);

    assert_eq!(native_scripts.get(0), native_script);
}

#[test]
fn different_script_type_error() {
    let script_hash_1 = fake_script_hash(1);
    let script_hash_2 = fake_script_hash(2);
    let tx_input_ref1 = fake_tx_input(2);
    let tx_input_ref2 = fake_tx_input(3);
    let redeemer = create_redeemer(1);

    let asset_name1 = AssetName::from_hex("44544e4654").unwrap();
    let asset_name2 = AssetName::from_hex("44544e4655").unwrap();

    let mut mint_builder = MintBuilder::new();
    let plutus_script_source1 = PlutusScriptSource::new_ref_input(
        &script_hash_1,
        &tx_input_ref1,
        &Language::new_plutus_v2(),
        0,
    );
    let plutus_script_source2 = PlutusScriptSource::new_ref_input(
        &script_hash_2,
        &tx_input_ref2,
        &Language::new_plutus_v2(),
        0,
    );

    let native_script_source1 = NativeScriptSource::new_ref_input(
        &script_hash_2,
        &tx_input_ref2,
    );
    let native_script_source2 = NativeScriptSource::new_ref_input(
        &script_hash_1,
        &tx_input_ref1,
    );

    let mint_witnes_plutus_1 = MintWitness::new_plutus_script(&plutus_script_source1, &redeemer);
    let mint_witnes_plutus_2 = MintWitness::new_plutus_script(&plutus_script_source2, &redeemer);
    let mint_witnes_native_1 = MintWitness::new_native_script(&native_script_source1);
    let mint_witnes_native_2 = MintWitness::new_native_script(&native_script_source2);

    mint_builder.add_asset(&mint_witnes_plutus_1, &asset_name1, &Int::new(&BigNum::from(100u64))).unwrap();
    mint_builder.add_asset(&mint_witnes_native_1, &asset_name2, &Int::new(&BigNum::from(101u64))).unwrap();

    let res = mint_builder.add_asset(&mint_witnes_plutus_2, &asset_name1, &Int::new(&BigNum::from(100u64)));
    assert!(res.is_err());

    let res = mint_builder.add_asset(&mint_witnes_native_2, &asset_name2, &Int::new(&BigNum::from(101u64)));
    assert!(res.is_err());
}

#[test]
fn wrong_witness_type_ref_error() {
    let native_script = NativeScript::new_timelock_start(
        &TimelockStart::new_timelockstart(&BigNum::from(100u64)),
    );
    let (plutus_script, script_hash_1) = fake_plutus_script_and_hash(5);
    let script_hash_2 = native_script.hash();
    let tx_input_ref1 = fake_tx_input(2);
    let tx_input_ref2 = fake_tx_input(3);
    let redeemer = create_redeemer(1);

    let asset_name1 = AssetName::from_hex("44544e4654").unwrap();
    let asset_name2 = AssetName::from_hex("44544e4655").unwrap();
    let asset_name3 = AssetName::from_hex("44544e4656").unwrap();
    let asset_name4 = AssetName::from_hex("44544e4657").unwrap();

    let mut mint_builder = MintBuilder::new();

    let plutus_script_source_1 = PlutusScriptSource::new_ref_input(
        &script_hash_1,
        &tx_input_ref1,
        &Language::new_plutus_v2(),
        0,
    );
    let plutus_script_source_2 = PlutusScriptSource::new(&plutus_script);

    let native_script_source_1 = NativeScriptSource::new_ref_input(
        &script_hash_2,
        &tx_input_ref2,
    );
    let native_script_source_2 = NativeScriptSource::new(&native_script);

    let mint_witness_plutus_1 = MintWitness::new_plutus_script(&plutus_script_source_1, &redeemer);
    let mint_witness_plutus_2 = MintWitness::new_plutus_script(&plutus_script_source_2, &redeemer);

    let mint_witness_native_1 = MintWitness::new_native_script(&native_script_source_1);
    let mint_witness_native_2 = MintWitness::new_native_script(&native_script_source_2);

    let res1 = mint_builder.add_asset(&mint_witness_plutus_1, &asset_name1, &Int::new(&BigNum::from(100u64)));
    assert!(res1.is_ok());

    let res2 = mint_builder.add_asset(&mint_witness_plutus_2, &asset_name2, &Int::new(&BigNum::from(101u64)));
    assert!(res2.is_err());

    let res3 = mint_builder.add_asset(&mint_witness_native_1, &asset_name3, &Int::new(&BigNum::from(102u64)));
    assert!(res3.is_ok());

    let res4= mint_builder.add_asset(&mint_witness_native_2, &asset_name4, &Int::new(&BigNum::from(103u64)));
    assert!(res4.is_err());

    let mint = mint_builder.build();
    assert_eq!(mint.len(), 2);

    let policy_mints_list = mint.get(&script_hash_1).unwrap();
    let policy_mints_list2 = mint.get(&script_hash_2).unwrap();

    assert_eq!(policy_mints_list.len(), 1);
    assert_eq!(policy_mints_list2.len(), 1);

    let policy_mints = policy_mints_list.get(0).unwrap();
    let policy_mints2 = policy_mints_list2.get(0).unwrap();

    assert_eq!(policy_mints.len(), 1);
    assert_eq!(policy_mints2.len(), 1);

    assert_eq!(policy_mints.get(&asset_name1).unwrap().to_str(), "100");
    assert_eq!(policy_mints2.get(&asset_name3).unwrap().to_str(), "102");
}

#[test]
fn wrong_witness_type_no_ref_error() {
    let native_script = NativeScript::new_timelock_start(
        &TimelockStart::new_timelockstart(&BigNum::from(100u64)),
    );
    let (plutus_script, script_hash_1) = fake_plutus_script_and_hash(5);
    let script_hash_2 = native_script.hash();
    let tx_input_ref1 = fake_tx_input(2);
    let tx_input_ref2 = fake_tx_input(3);
    let redeemer = create_redeemer(1);

    let asset_name1 = AssetName::from_hex("44544e4654").unwrap();
    let asset_name2 = AssetName::from_hex("44544e4655").unwrap();
    let asset_name3 = AssetName::from_hex("44544e4656").unwrap();
    let asset_name4 = AssetName::from_hex("44544e4657").unwrap();

    let mut mint_builder = MintBuilder::new();

    let plutus_script_source_1 = PlutusScriptSource::new_ref_input(
        &script_hash_1,
        &tx_input_ref1,
        &Language::new_plutus_v2(),
        0,
    );
    let plutus_script_source_2 = PlutusScriptSource::new(&plutus_script);

    let native_script_source_1 = NativeScriptSource::new_ref_input(
        &script_hash_2,
        &tx_input_ref2,
    );
    let native_script_source_2 = NativeScriptSource::new(&native_script);

    let mint_witness_plutus_1 = MintWitness::new_plutus_script(&plutus_script_source_2, &redeemer);
    let mint_witness_plutus_2 = MintWitness::new_plutus_script(&plutus_script_source_1, &redeemer);

    let mint_witness_native_1 = MintWitness::new_native_script(&native_script_source_2);
    let mint_witness_native_2 = MintWitness::new_native_script(&native_script_source_1);

    let res1 = mint_builder.add_asset(&mint_witness_plutus_1, &asset_name1, &Int::new(&BigNum::from(100u64)));
    assert!(res1.is_ok());

    let res2 = mint_builder.add_asset(&mint_witness_plutus_2, &asset_name2, &Int::new(&BigNum::from(101u64)));
    assert!(res2.is_err());

    let res3 = mint_builder.add_asset(&mint_witness_native_1, &asset_name3, &Int::new(&BigNum::from(102u64)));
    assert!(res3.is_ok());

    let res4= mint_builder.add_asset(&mint_witness_native_2, &asset_name4, &Int::new(&BigNum::from(103u64)));
    assert!(res4.is_err());

    let mint = mint_builder.build();
    assert_eq!(mint.len(), 2);

    let policy_mints_list = mint.get(&script_hash_1).unwrap();
    let policy_mints_list2 = mint.get(&script_hash_2).unwrap();

    assert_eq!(policy_mints_list.len(), 1);
    assert_eq!(policy_mints_list2.len(), 1);

    let policy_mints = policy_mints_list.get(0).unwrap();
    let policy_mints2 = policy_mints_list2.get(0).unwrap();

    assert_eq!(policy_mints.len(), 1);
    assert_eq!(policy_mints2.len(), 1);

    assert_eq!(policy_mints.get(&asset_name1).unwrap().to_str(), "100");
    assert_eq!(policy_mints2.get(&asset_name3).unwrap().to_str(), "102");
}

#[test]
fn zero_mint_error() {
    let native_script = NativeScript::new_timelock_start(
        &TimelockStart::new_timelockstart(&BigNum::from(100u64)),
    );
    let asset_name1 = AssetName::from_hex("44544e4654").unwrap();
    let mut mint_builder = MintBuilder::new();

    let native_script_source = NativeScriptSource::new(&native_script);


    let mint_witness_native = MintWitness::new_native_script(&native_script_source);

    let res= mint_builder.add_asset(&mint_witness_native, &asset_name1, &Int::new(&BigNum::from(0u64)));
    assert!(res.is_err());
}