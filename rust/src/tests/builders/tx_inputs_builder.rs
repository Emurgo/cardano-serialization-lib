use crate::tests::fakes::{fake_base_address, fake_base_script_address, fake_enterprise_address, fake_enterprise_script_address, fake_plutus_script, fake_pointer_address, fake_pointer_script_address, fake_redeemer, fake_reward_address, fake_key_hash, fake_script_hash, fake_tx_input};
use crate::*;

#[test]
fn regular_inputs() {
    let mut tx_inputs_builder = TxInputsBuilder::new();
    let base_address_1 = fake_base_address(1);
    let tx_input_1 = fake_tx_input(1);
    let input_value_1 = Value::new(&BigNum(100));
    tx_inputs_builder
        .add_regular_input(&base_address_1, &tx_input_1, &input_value_1)
        .unwrap();

    let enterprise_address_2 = fake_enterprise_address(2);
    let tx_input_2 = fake_tx_input(2);
    let input_value_2 = Value::new(&BigNum(200));
    tx_inputs_builder
        .add_regular_input(&enterprise_address_2, &tx_input_2, &input_value_2)
        .unwrap();

    let pointer_address_3 = fake_pointer_address(3);
    let tx_input_3 = fake_tx_input(3);
    let input_value_3 = Value::new(&BigNum(300));
    tx_inputs_builder
        .add_regular_input(&pointer_address_3, &tx_input_3, &input_value_3)
        .unwrap();

    let byron_address_4 =
        ByronAddress::from_base58("Ae2tdPwUPEZ3MHKkpT5Bpj549vrRH7nBqYjNXnCV8G2Bc2YxNcGHEa8ykDp")
            .unwrap()
            .to_address();
    let tx_input_4 = fake_tx_input(4);
    let input_value_4 = Value::new(&BigNum(400));
    tx_inputs_builder
        .add_regular_input(&byron_address_4, &tx_input_4, &input_value_4)
        .unwrap();

    let key_hash_5 = fake_key_hash(5);
    let tx_input_5 = fake_tx_input(5);
    let input_value_5 = Value::new(&BigNum(500));
    tx_inputs_builder.add_key_input(&key_hash_5, &tx_input_5, &input_value_5);

    let byron_address_6 =
        ByronAddress::from_base58("Ae2tdPwUPEZ6r6zbg4ibhFrNnyKHg7SYuPSfDpjKxgvwFX9LquRep7gj7FQ")
            .unwrap();
    let tx_input_6 = fake_tx_input(6);
    let input_value_6 = Value::new(&BigNum(600));
    tx_inputs_builder.add_bootstrap_input(&byron_address_6, &tx_input_6, &input_value_6);

    let key_hash_7 = fake_key_hash(7);
    tx_inputs_builder.add_required_signer(&key_hash_7);

    let ref_inputs = tx_inputs_builder.get_ref_inputs();
    assert_eq!(ref_inputs.len(), 0);

    let native_scripts = tx_inputs_builder.get_native_input_scripts();
    assert!(native_scripts.is_none());

    let plutus_scripts = tx_inputs_builder.get_plutus_input_scripts();
    assert!(plutus_scripts.is_none());

    assert_eq!(tx_inputs_builder.has_plutus_scripts(), false);

    let required_signatures: Ed25519KeyHashes = tx_inputs_builder.get_required_signers();
    assert_eq!(required_signatures.len(), 5);

    required_signatures.contains(&base_address_1.payment_cred().unwrap().to_keyhash().unwrap());
    required_signatures.contains(
        &enterprise_address_2
            .payment_cred()
            .unwrap()
            .to_keyhash()
            .unwrap(),
    );
    required_signatures.contains(
        &pointer_address_3
            .payment_cred()
            .unwrap()
            .to_keyhash()
            .unwrap(),
    );
    required_signatures.contains(&key_hash_5);
    required_signatures.contains(&key_hash_7);

    let tx_inputs = tx_inputs_builder.inputs();
    assert_eq!(tx_inputs.len(), 6);
    assert!(tx_inputs.contains(&tx_input_1));
    assert!(tx_inputs.contains(&tx_input_2));
    assert!(tx_inputs.contains(&tx_input_3));
    assert!(tx_inputs.contains(&tx_input_4));
    assert!(tx_inputs.contains(&tx_input_5));
    assert!(tx_inputs.contains(&tx_input_6));

    let bootstraps = get_bootstraps(&tx_inputs_builder);
    let boostrap_1 = ByronAddress::from_address(&byron_address_4)
        .unwrap()
        .to_bytes();
    let boostrap_2 = byron_address_6.to_bytes();
    assert_eq!(bootstraps.len(), 2);

    assert!(bootstraps.contains(&boostrap_1));
    assert!(bootstraps.contains(&boostrap_2));
}

#[test]
fn script_input_as_regular_input_error() {
    let mut tx_inputs_builder = TxInputsBuilder::new();
    let plutus_script = fake_tx_input(1);
    let input_value = Value::new(&BigNum(100));

    let base_address_1 = fake_base_script_address(1);
    let res_1 = tx_inputs_builder.add_regular_input(&base_address_1, &plutus_script, &input_value);
    assert!(res_1.is_err());

    let enterprise_address_2 = fake_enterprise_script_address(2);
    let res_2 =
        tx_inputs_builder.add_regular_input(&enterprise_address_2, &plutus_script, &input_value);
    assert!(res_2.is_err());

    let pointer_address_3 = fake_pointer_script_address(3);
    let res_3 =
        tx_inputs_builder.add_regular_input(&pointer_address_3, &plutus_script, &input_value);
    assert!(res_3.is_err());
}

#[test]
fn rewards_address_input_as_regular_input_error() {
    let mut tx_inputs_builder = TxInputsBuilder::new();
    let rewards_address = fake_reward_address(1).to_address();
    let tx_input = fake_tx_input(1);
    let input_value = Value::new(&BigNum(100));
    let res = tx_inputs_builder.add_regular_input(&rewards_address, &tx_input, &input_value);
    assert!(res.is_err());
}

#[test]
fn plutus_script_input() {
    let mut tx_inputs_builder = TxInputsBuilder::new();
    let tx_input_1 = fake_tx_input(1);
    let input_value_1 = Value::new(&BigNum(100));

    let plutus_script = fake_plutus_script(1, &Language::new_plutus_v2());
    let plutus_script_source = PlutusScriptSource::new(&plutus_script);
    let redeemer = fake_redeemer(1)
        .clone_with_index_and_tag(&BigNum(0), &RedeemerTag::new_spend());

    let datum = PlutusData::new_empty_constr_plutus_data(&BigNum::zero());
    let plutus_witness =
        PlutusWitness::new_with_ref(&plutus_script_source, &DatumSource::new(&datum), &redeemer);

    tx_inputs_builder.add_plutus_script_input(&plutus_witness, &tx_input_1, &input_value_1);

    let plutus_scripts = tx_inputs_builder.get_plutus_input_scripts().unwrap();
    assert_eq!(plutus_scripts.len(), 1);

    let plutus_wit_from_builder = plutus_scripts.get(0);
    assert_eq!(plutus_wit_from_builder.script().unwrap(), plutus_script);
    assert_eq!(plutus_wit_from_builder.datum().unwrap(), datum);
    assert_eq!(plutus_wit_from_builder.redeemer(), redeemer);

    assert!(tx_inputs_builder.has_plutus_scripts());

    let req_signers = tx_inputs_builder.get_required_signers();
    assert_eq!(req_signers.len(), 0);

    let inputs = tx_inputs_builder.inputs();
    assert_eq!(inputs.len(), 1);
    assert!(inputs.contains(&tx_input_1));

    let ref_inputs = tx_inputs_builder.get_ref_inputs();
    assert_eq!(ref_inputs.len(), 0);
}

#[test]
fn plutus_script_input_with_required_signers() {
    let mut tx_inputs_builder = TxInputsBuilder::new();
    let tx_input_1 = fake_tx_input(1);
    let input_value_1 = Value::new(&BigNum(100));

    let key_hash = fake_key_hash(1);
    let key_hashes = Ed25519KeyHashes::from_vec(vec![key_hash]);

    let plutus_script = fake_plutus_script(1, &Language::new_plutus_v2());
    let mut plutus_script_source = PlutusScriptSource::new(&plutus_script);
    plutus_script_source.set_required_signers(&key_hashes);

    let redeemer = fake_redeemer(1)
        .clone_with_index_and_tag(&BigNum(0), &RedeemerTag::new_spend());

    let datum = PlutusData::new_empty_constr_plutus_data(&BigNum::zero());
    let plutus_witness =
        PlutusWitness::new_with_ref(&plutus_script_source, &DatumSource::new(&datum), &redeemer);

    tx_inputs_builder.add_plutus_script_input(&plutus_witness, &tx_input_1, &input_value_1);

    let plutus_scripts = tx_inputs_builder.get_plutus_input_scripts().unwrap();
    assert_eq!(plutus_scripts.len(), 1);

    let plutus_wit_from_builder = plutus_scripts.get(0);
    assert_eq!(plutus_wit_from_builder.script().unwrap(), plutus_script);
    assert_eq!(plutus_wit_from_builder.datum().unwrap(), datum);
    assert_eq!(plutus_wit_from_builder.redeemer(), redeemer);

    assert!(tx_inputs_builder.has_plutus_scripts());

    let req_signers = tx_inputs_builder.get_required_signers();
    assert_eq!(req_signers.len(), 1);
    assert_eq!(req_signers, key_hashes);

    let inputs = tx_inputs_builder.inputs();
    assert_eq!(inputs.len(), 1);
    assert!(inputs.contains(&tx_input_1));

    let ref_inputs = tx_inputs_builder.get_ref_inputs();
    assert_eq!(ref_inputs.len(), 0);
}

#[test]
fn plutus_script_input_with_ref() {
    let mut tx_inputs_builder = TxInputsBuilder::new();
    let tx_input_1 = fake_tx_input(1);
    let input_value_1 = Value::new(&BigNum(100));

    let ref_input_1 = fake_tx_input(2);
    let script_hash = fake_script_hash(1);
    let lang_ver = Language::new_plutus_v2();
    let script_size = 100;

    let ref_input_2 = fake_tx_input(3);

    let plutus_script_source =
        PlutusScriptSource::new_ref_input(&script_hash, &ref_input_1, &lang_ver, script_size);

    let redeemer = fake_redeemer(1)
        .clone_with_index_and_tag(&BigNum(0), &RedeemerTag::new_spend());

    let plutus_witness = PlutusWitness::new_with_ref(
        &plutus_script_source,
        &DatumSource::new_ref_input(&ref_input_2),
        &redeemer,
    );

    tx_inputs_builder.add_plutus_script_input(&plutus_witness, &tx_input_1, &input_value_1);

    let plutus_scripts = tx_inputs_builder.get_plutus_input_scripts().unwrap();
    assert_eq!(plutus_scripts.len(), 1);

    let plutus_wit_from_builder = plutus_scripts.get(0);
    assert_eq!(plutus_wit_from_builder.script(), None);
    assert_eq!(plutus_wit_from_builder.datum(), None);
    assert_eq!(plutus_wit_from_builder.redeemer(), redeemer);

    assert!(tx_inputs_builder.has_plutus_scripts());

    let req_signers = tx_inputs_builder.get_required_signers();
    assert_eq!(req_signers.len(), 0);

    let inputs = tx_inputs_builder.inputs();
    assert_eq!(inputs.len(), 1);
    assert!(inputs.contains(&tx_input_1));

    let ref_inputs = tx_inputs_builder.get_ref_inputs();
    assert_eq!(ref_inputs.len(), 2);
    assert!(ref_inputs.contains(&ref_input_1));
    assert!(ref_inputs.contains(&ref_input_2));
}

#[test]
fn native_script_input() {
    let mut tx_inputs_builder = TxInputsBuilder::new();
    let tx_input_1 = fake_tx_input(1);
    let input_value_1 = Value::new(&BigNum(100));

    let key_hash_1 = fake_key_hash(1);
    let mut native_scripts = NativeScripts::new();
    native_scripts.add(
        &NativeScript::new_script_pubkey(
            &ScriptPubkey::new(&key_hash_1)
        )
    );
    let native_script = NativeScript::new_script_all(
        &ScriptAll::new(
            &native_scripts
        ),
    );

    let native_script_source = NativeScriptSource::new(&native_script);

    tx_inputs_builder.add_native_script_input(&native_script_source, &tx_input_1, &input_value_1);

    let native_scripts = tx_inputs_builder.get_native_input_scripts().unwrap();
    assert_eq!(native_scripts.len(), 1);

    let native_wit_from_builder = native_scripts.get(0);
    assert_eq!(native_wit_from_builder, native_script);

    let req_signers = tx_inputs_builder.get_required_signers();
    assert_eq!(req_signers.len(), 1);
    assert_eq!(req_signers.get(0), key_hash_1);

    let inputs = tx_inputs_builder.inputs();
    assert_eq!(inputs.len(), 1);
    assert!(inputs.contains(&tx_input_1));

    let ref_inputs = tx_inputs_builder.get_ref_inputs();
    assert_eq!(ref_inputs.len(), 0);
}

#[test]
fn native_script_custom_required_witness_input() {
    let mut tx_inputs_builder = TxInputsBuilder::new();
    let tx_input_1 = fake_tx_input(1);
    let input_value_1 = Value::new(&BigNum(100));

    let key_hash_1 = fake_key_hash(1);
    let key_hash_2 = fake_key_hash(2);

    let mut native_scripts = NativeScripts::new();
    native_scripts.add(
        &NativeScript::new_script_pubkey(
            &ScriptPubkey::new(&key_hash_1)
        )
    );
    let native_script = NativeScript::new_script_all(
        &ScriptAll::new(
            &native_scripts
        ),
    );

    let mut native_script_source = NativeScriptSource::new(&native_script);
    let mut key_hashes = Ed25519KeyHashes::new();
    key_hashes.add(&key_hash_2);
    native_script_source.set_required_signers(&key_hashes);

    tx_inputs_builder.add_native_script_input(&native_script_source, &tx_input_1, &input_value_1);

    let native_scripts = tx_inputs_builder.get_native_input_scripts().unwrap();
    assert_eq!(native_scripts.len(), 1);

    let native_wit_from_builder = native_scripts.get(0);
    assert_eq!(native_wit_from_builder, native_script);

    let req_signers = tx_inputs_builder.get_required_signers();
    assert_eq!(req_signers.len(), 1);
    assert_eq!(req_signers.get(0), key_hash_2);

    let inputs = tx_inputs_builder.inputs();
    assert_eq!(inputs.len(), 1);
    assert!(inputs.contains(&tx_input_1));

    let ref_inputs = tx_inputs_builder.get_ref_inputs();
    assert_eq!(ref_inputs.len(), 0);
}

#[test]
fn native_script_input_ref_script() {
    let mut tx_inputs_builder = TxInputsBuilder::new();
    let tx_input_1 = fake_tx_input(1);
    let input_value_1 = Value::new(&BigNum(100));

    let key_hash_1 = fake_key_hash(1);
    let ref_input = fake_tx_input(2);
    let script_hash = fake_script_hash(1);

    let mut native_script_source = NativeScriptSource::new_ref_input(&script_hash, &ref_input);
    let mut key_hashes = Ed25519KeyHashes::new();
    key_hashes.add(&key_hash_1);
    native_script_source.set_required_signers(&key_hashes);

    tx_inputs_builder.add_native_script_input(&native_script_source, &tx_input_1, &input_value_1);

    let native_scripts = tx_inputs_builder.get_native_input_scripts();
    assert_eq!(native_scripts, None);

    let req_signers = tx_inputs_builder.get_required_signers();
    assert_eq!(req_signers.len(), 1);
    assert_eq!(req_signers.get(0), key_hash_1);

    let inputs = tx_inputs_builder.inputs();
    assert_eq!(inputs.len(), 1);
    assert!(inputs.contains(&tx_input_1));

    let ref_inputs = tx_inputs_builder.get_ref_inputs();
    assert_eq!(ref_inputs.len(), 1);
    assert!(ref_inputs.contains(&ref_input));
}