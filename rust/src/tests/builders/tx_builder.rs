use crate::fakes::{fake_base_address, fake_bytes_32, fake_data_hash, fake_key_hash, fake_plutus_script_and_hash, fake_policy_id, fake_script_hash, fake_tx_hash, fake_tx_input, fake_tx_input2, fake_value, fake_value2, fake_vkey_witness};
use crate::tests::helpers::harden;
use crate::tests::mock_objects::{byron_address, create_anchor, create_change_address, create_default_tx_builder, create_linear_fee, create_reallistic_tx_builder, create_redeemer, create_redeemer_zero_cost, create_rich_tx_builder, create_tx_builder, create_tx_builder_with_amount, create_tx_builder_with_fee, create_tx_builder_with_fee_and_pure_change, create_tx_builder_with_fee_and_val_size, create_tx_builder_with_key_deposit, root_key_15};
use crate::*;

use crate::builders::fakes::fake_private_key;
use fees::*;
use std::collections::{BTreeMap, HashMap, HashSet};

const MAX_TX_SIZE: u32 = 8000;

fn genesis_id() -> TransactionHash {
    TransactionHash::from([0u8; TransactionHash::BYTE_COUNT])
}

#[test]
fn check_fake_private_key() {
    let fpk = fake_private_key();
    assert_eq!(
            fpk.to_bech32(),
            "xprv1hretan5mml3tq2p0twkhq4tz4jvka7m2l94kfr6yghkyfar6m9wppc7h9unw6p65y23kakzct3695rs32z7vaw3r2lg9scmfj8ec5du3ufydu5yuquxcz24jlkjhsc9vsa4ufzge9s00fn398svhacse5su2awrw",
        );
    assert_eq!(
            fpk.to_public().to_bech32(),
            "xpub1eamrnx3pph58yr5l4z2wghjpu2dt2f0rp0zq9qquqa39p52ct0xercjgmegfcpcdsy4t9ld90ps2epmtcjy3jtq77n8z20qe0m3pnfqntgrgj",
        );
}

#[test]
fn build_tx_with_change() {
    let mut tx_builder = create_default_tx_builder();
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&BigNum(222))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();
    let added_change = tx_builder.add_change_if_needed(&change_addr);
    assert!(added_change.unwrap());
    assert_eq!(tx_builder.outputs.len(), 2);
    assert_eq!(
        tx_builder
            .get_explicit_input()
            .unwrap()
            .checked_add(&tx_builder.get_implicit_input().unwrap())
            .unwrap(),
        tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
            .unwrap()
    );
    assert_eq!(tx_builder.full_size().unwrap(), 285);
    assert_eq!(tx_builder.output_sizes(), vec![62, 65]);
    let _final_tx = tx_builder.build(); // just test that it doesn't throw
}

#[test]
fn build_tx_with_change_with_datum() {
    let mut tx_builder = create_default_tx_builder();
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&BigNum(222))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);

    let datum_hash = fake_data_hash(20);
    let data_option = OutputDatum::new_data_hash(&datum_hash);
    let (_, script_hash) = fake_plutus_script_and_hash(15);
    let change_cred = Credential::from_scripthash(&script_hash);
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();
    let added_change = tx_builder.add_change_if_needed_with_datum(&change_addr, &data_option);
    assert!(added_change.unwrap());
    assert_eq!(tx_builder.outputs.len(), 2);
    assert_eq!(
        tx_builder
            .get_explicit_input()
            .unwrap()
            .checked_add(&tx_builder.get_implicit_input().unwrap())
            .unwrap(),
        tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
            .unwrap()
    );
    assert_eq!(tx_builder.full_size().unwrap(), 319);
    assert_eq!(tx_builder.output_sizes(), vec![62, 99]);
    let _final_tx = tx_builder.build(); // just test that it doesn't throw
}

#[test]
fn build_tx_without_change() {
    let mut tx_builder = create_default_tx_builder();
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&BigNum(880_000))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();
    let added_change = tx_builder.add_change_if_needed(&change_addr);
    assert!(!added_change.unwrap());
    assert_eq!(tx_builder.outputs.len(), 1);
    assert_eq!(
        tx_builder
            .get_explicit_input()
            .unwrap()
            .checked_add(&tx_builder.get_implicit_input().unwrap())
            .unwrap(),
        tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
            .unwrap()
    );
    let _final_tx = tx_builder.build(); // just test that it doesn't throw
}

#[test]
fn build_tx_with_certs() {
    let mut tx_builder = create_tx_builder_with_key_deposit(1_000_000);
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(5_000_000)),
    );
    tx_builder.set_ttl(1000);

    let mut certs = Certificates::new();
    certs.add(&Certificate::new_stake_registration(
        &StakeRegistration::new(&stake_cred),
    ));
    certs.add(&Certificate::new_stake_delegation(&StakeDelegation::new(
        &stake_cred,
        &stake.to_raw_key().hash(), // in reality, this should be the pool owner's key, not ours
    )));
    tx_builder.set_certs(&certs).unwrap();

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(tx_builder.min_fee().unwrap().to_str(), "214002");
    assert_eq!(tx_builder.get_fee_if_set().unwrap().to_str(), "214002");
    assert_eq!(tx_builder.get_deposit().unwrap().to_str(), "1000000");
    assert_eq!(tx_builder.outputs.len(), 1);
    assert_eq!(
        tx_builder
            .get_explicit_input()
            .unwrap()
            .checked_add(&tx_builder.get_implicit_input().unwrap())
            .unwrap(),
        tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_deposit().unwrap()))
            .unwrap()
    );
    let _final_tx = tx_builder.build(); // just test that it doesn't throw
}

#[test]
fn build_tx_exact_amount() {
    // transactions where sum(input) == sum(output) exact should pass
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();
    tx_builder.add_key_input(
        &&spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(222)),
    );
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&BigNum(222))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(0);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();
    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(added_change, false);
    let final_tx = tx_builder.build().unwrap();
    assert_eq!(final_tx.outputs().len(), 1);
}

#[test]
fn build_tx_exact_change() {
    // transactions where we have exactly enough ADA to add change should pass
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();
    tx_builder.add_key_input(
        &&spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(700)),
    );
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&BigNum(222))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(0);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();
    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(added_change, true);
    let final_tx = tx_builder.build().unwrap();
    assert_eq!(final_tx.outputs().len(), 2);
    assert_eq!(final_tx.outputs().get(1).amount().coin().to_str(), "478");
}

#[test]
#[should_panic]
fn build_tx_insufficient_deposit() {
    // transactions should fail with insufficient fees if a deposit is required
    let mut tx_builder = create_tx_builder_with_key_deposit(5);
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();
    tx_builder.add_key_input(
        &&spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(5)),
    );
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&BigNum(5))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(0);

    // add a cert which requires a deposit
    let mut certs = Certificates::new();
    certs.add(&Certificate::new_stake_registration(
        &StakeRegistration::new(&stake_cred),
    ));
    tx_builder.set_certs(&certs);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    tx_builder.add_change_if_needed(&change_addr).unwrap();
}

#[test]
fn build_tx_with_inputs() {
    let mut tx_builder = create_default_tx_builder();
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());

    {
        assert_eq!(
            tx_builder
                .fee_for_input(
                    &EnterpriseAddress::new(
                        NetworkInfo::testnet_preprod().network_id(),
                        &spend_cred
                    )
                    .to_address(),
                    &TransactionInput::new(&genesis_id(), 0),
                    &Value::new(&BigNum(1_000_000))
                )
                .unwrap()
                .to_str(),
            "69500"
        );
        tx_builder.add_regular_input(
            &EnterpriseAddress::new(NetworkInfo::testnet_preprod().network_id(), &spend_cred)
                .to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&BigNum(1_000_000)),
        );
    }
    tx_builder.add_regular_input(
        &BaseAddress::new(
            NetworkInfo::testnet_preprod().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address(),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder.add_regular_input(
        &PointerAddress::new(
            NetworkInfo::testnet_preprod().network_id(),
            &spend_cred,
            &Pointer::new_pointer(&BigNum(0), &BigNum(0), &BigNum(0)),
        )
        .to_address(),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder.add_regular_input(
        &ByronAddress::icarus_from_key(&spend, NetworkInfo::testnet_preprod().protocol_magic())
            .to_address(),
        &TransactionInput::new(&genesis_id(), 3),
        &Value::new(&BigNum(1_000_000)),
    );

    assert_eq!(tx_builder.inputs.len(), 4);
}

#[test]
fn add_ref_inputs_to_builder() {
    let mut tx_builder = create_default_tx_builder();

    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 1));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 2));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 3));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 4));

    assert_eq!(tx_builder.reference_inputs.len(), 4);
}

#[test]
fn build_tx_with_script_ref() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());

    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 1));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 2));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 3));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 4));

    tx_builder.add_regular_input(
        &PointerAddress::new(
            NetworkInfo::testnet_preprod().network_id(),
            &spend_cred,
            &Pointer::new_pointer(&BigNum(0), &BigNum(0), &BigNum(0)),
        )
        .to_address(),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&BigNum(1_000_000)),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let output_amount = Value::new(&BigNum(500));

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(added_change, true);
    let final_tx = tx_builder.build().unwrap();
    assert_eq!(final_tx.outputs().len(), 2);
    assert_eq!(final_tx.reference_inputs().unwrap().len(), 4);
    assert_eq!(final_tx.outputs().get(1).amount().coin(), BigNum(999499));
}

#[test]
fn serialization_tx_body_with_script_ref() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());

    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 1));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 2));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 3));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 4));

    tx_builder.add_regular_input(
        &PointerAddress::new(
            NetworkInfo::testnet_preprod().network_id(),
            &spend_cred,
            &Pointer::new_pointer(&BigNum(0), &BigNum(0), &BigNum(0)),
        )
        .to_address(),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&BigNum(1_000_000)),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let output_amount = Value::new(&BigNum(500));

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    tx_builder.add_change_if_needed(&change_addr).unwrap();
    let final_tx = tx_builder.build().unwrap();

    let deser_t = TransactionBody::from_bytes(final_tx.to_bytes()).unwrap();

    assert_eq!(deser_t.to_bytes(), final_tx.to_bytes());
}

#[test]
fn json_serialization_tx_body_with_script_ref() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());

    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 1));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 2));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 3));
    tx_builder.add_reference_input(&TransactionInput::new(&genesis_id(), 4));

    tx_builder.add_regular_input(
        &PointerAddress::new(
            NetworkInfo::testnet_preprod().network_id(),
            &spend_cred,
            &Pointer::new_pointer(&BigNum(0), &BigNum(0), &BigNum(0)),
        )
        .to_address(),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&BigNum(1_000_000)),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let output_amount = Value::new(&BigNum(500));

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    tx_builder.add_change_if_needed(&change_addr).unwrap();
    let final_tx = tx_builder.build().unwrap();

    let json_tx_body = final_tx.to_json().unwrap();
    let deser_t = TransactionBody::from_json(json_tx_body.as_str()).unwrap();

    assert_eq!(deser_t.to_bytes(), final_tx.to_bytes());
    assert_eq!(deser_t.to_json().unwrap(), final_tx.to_json().unwrap());
}

#[test]
fn build_tx_with_mint_all_sent() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());

    // Input with 150 coins
    tx_builder.add_regular_input(
        &EnterpriseAddress::new(NetworkInfo::testnet_preprod().network_id(), &spend_cred)
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(500)),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let (min_script, policy_id) = mint_script_and_policy(0);
    let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
    let amount = BigNum(1234);

    // Adding mint of the asset - which should work as an input
    tx_builder.add_mint_asset(&min_script, &name, &Int::new(&amount));

    let mut ass = Assets::new();
    ass.insert(&name, &amount);
    let mut mass = MultiAsset::new();
    mass.insert(&policy_id, &ass);

    // One coin and the minted asset goes into the output
    let mut output_amount = Value::new(&BigNum(264));
    output_amount.set_multiasset(&mass);

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert!(added_change);
    assert_eq!(tx_builder.outputs.len(), 2);

    // Change must be one remaining coin because fee is one constant coin
    let change = tx_builder.outputs.get(1).amount();
    assert_eq!(change.coin(), BigNum(235));
    assert!(change.multiasset().is_none());
}

#[test]
fn build_tx_with_mint_in_change() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());

    // Input with 600 coins
    tx_builder.add_regular_input(
        &EnterpriseAddress::new(NetworkInfo::testnet_preprod().network_id(), &spend_cred)
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(600)),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let (min_script, policy_id) = mint_script_and_policy(0);
    let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

    let amount_minted = BigNum(1000);
    let amount_sent = BigNum(500);

    // Adding mint of the asset - which should work as an input
    tx_builder.add_mint_asset(&min_script, &name, &Int::new(&amount_minted));

    let mut ass = Assets::new();
    ass.insert(&name, &amount_sent);
    let mut mass = MultiAsset::new();
    mass.insert(&policy_id, &ass);

    // One coin and the minted asset goes into the output
    let mut output_amount = Value::new(&BigNum(300));
    output_amount.set_multiasset(&mass);

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert!(added_change);
    assert_eq!(tx_builder.outputs.len(), 2);

    // Change must be one remaining coin because fee is one constant coin
    let change = tx_builder.outputs.get(1).amount();
    assert_eq!(change.coin(), BigNum(299));
    assert!(change.multiasset().is_some());

    let change_assets = change.multiasset().unwrap();
    let change_asset = change_assets.get(&policy_id).unwrap();
    assert_eq!(
        change_asset.get(&name).unwrap(),
        amount_minted.checked_sub(&amount_sent).unwrap(),
    );
}

#[test]
fn change_with_input_and_mint_not_enough_ada() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 1));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());

    let (min_script, policy_id) = mint_script_and_policy(0);
    let asset_name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

    let amount_minted = BigNum(1000);
    let amount_sent = BigNum(500);
    let amount_input_amount = BigNum(600);

    let mut asset_input = Assets::new();
    asset_input.insert(&asset_name, &amount_input_amount);
    let mut mass_input = MultiAsset::new();
    mass_input.insert(&policy_id, &asset_input);

    // Input with 600 coins
    tx_builder.add_regular_input(
        &EnterpriseAddress::new(NetworkInfo::testnet_preprod().network_id(), &spend_cred)
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(600)),
    );

    tx_builder.add_regular_input(
        &EnterpriseAddress::new(NetworkInfo::testnet_preprod().network_id(), &spend_cred)
            .to_address(),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new_with_assets(&BigNum(1), &mass_input),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    // Adding mint of the asset - which should work as an input
    tx_builder.add_mint_asset(&min_script, &asset_name, &Int::new(&amount_minted));

    let mut asset = Assets::new();
    asset.insert(&asset_name, &amount_sent);
    let mut mass = MultiAsset::new();
    mass.insert(&policy_id, &asset);

    // One coin and the minted asset goes into the output
    let mut output_amount = Value::new(&BigNum(400));
    output_amount.set_multiasset(&mass);

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr);
    assert!(added_change.is_err());
}

#[test]
fn change_with_input_and_mint_not_enough_assets() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 1));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());

    let (min_script, policy_id) = mint_script_and_policy(0);
    let asset_name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

    let amount_minted = BigNum(1000);
    let amount_sent = BigNum(100000);
    let amount_input_amount = BigNum(600);

    let mut asset_input = Assets::new();
    asset_input.insert(&asset_name, &amount_input_amount);
    let mut mass_input = MultiAsset::new();
    mass_input.insert(&policy_id, &asset_input);

    // Input with 600 coins
    tx_builder.add_regular_input(
        &EnterpriseAddress::new(NetworkInfo::testnet_preprod().network_id(), &spend_cred)
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(100000)),
    );

    tx_builder.add_regular_input(
        &EnterpriseAddress::new(NetworkInfo::testnet_preprod().network_id(), &spend_cred)
            .to_address(),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new_with_assets(&BigNum(1), &mass_input),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    // Adding mint of the asset - which should work as an input
    tx_builder.add_mint_asset(&min_script, &asset_name, &Int::new(&amount_minted));

    let mut asset = Assets::new();
    asset.insert(&asset_name, &amount_sent);
    let mut mass = MultiAsset::new();
    mass.insert(&policy_id, &asset);

    // One coin and the minted asset goes into the output
    let mut output_amount = Value::new(&BigNum(400));
    output_amount.set_multiasset(&mass);

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr);
    assert!(added_change.is_err());
}

#[test]
fn build_tx_with_native_assets_change() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let policy_id = &PolicyID::from([0u8; 28]);
    let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

    let ma_input1 = 100;
    let ma_input2 = 200;
    let ma_output1 = 60;

    let multiassets = [ma_input1, ma_input2, ma_output1]
        .iter()
        .map(|input| {
            let mut multiasset = MultiAsset::new();
            multiasset.insert(policy_id, &{
                let mut assets = Assets::new();
                assets.insert(&name, &BigNum(*input));
                assets
            });
            multiasset
        })
        .collect::<Vec<MultiAsset>>();

    for (i, (multiasset, ada)) in multiassets
        .iter()
        .zip([100u64, 1000].iter().cloned().map(|x| BigNum(x)))
        .enumerate()
    {
        let mut input_amount = Value::new(&ada);
        input_amount.set_multiasset(multiasset);

        tx_builder.add_key_input(
            &&spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), i as u32),
            &input_amount,
        );
    }

    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let mut output_amount = Value::new(&BigNum(500));
    output_amount.set_multiasset(&multiassets[2]);

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(added_change, true);
    let final_tx = tx_builder.build().unwrap();
    assert_eq!(final_tx.outputs().len(), 2);
    assert_eq!(
        final_tx
            .outputs()
            .get(1)
            .amount()
            .multiasset()
            .unwrap()
            .get(policy_id)
            .unwrap()
            .get(&name)
            .unwrap(),
        BigNum(ma_input1 + ma_input2 - ma_output1)
    );
    assert_eq!(final_tx.outputs().get(1).amount().coin(), BigNum(599));
}

#[test]
fn build_tx_with_native_assets_change_and_purification() {
    let coin_per_utxo_byte = BigNum(1);
    // Prefer pure change!
    let mut tx_builder = create_tx_builder_with_fee_and_pure_change(&create_linear_fee(0, 1));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let policy_id = &PolicyID::from([0u8; 28]);
    let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

    let ma_input1 = 100;
    let ma_input2 = 200;
    let ma_output1 = 60;

    let multiassets = [ma_input1, ma_input2, ma_output1]
        .iter()
        .map(|input| {
            let mut multiasset = MultiAsset::new();
            multiasset.insert(policy_id, &{
                let mut assets = Assets::new();
                assets.insert(&name, &BigNum(*input));
                assets
            });
            multiasset
        })
        .collect::<Vec<MultiAsset>>();

    for (i, (multiasset, ada)) in multiassets
        .iter()
        .zip([100u64, 1000].iter().cloned().map(|x| BigNum(x)))
        .enumerate()
    {
        let mut input_amount = Value::new(&ada);
        input_amount.set_multiasset(multiasset);

        tx_builder.add_key_input(
            &&spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), i as u32),
            &input_amount,
        );
    }

    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let mut output_amount = Value::new(&BigNum(600));
    output_amount.set_multiasset(&multiassets[2]);

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(added_change, true);
    let final_tx = tx_builder.build().unwrap();
    assert_eq!(final_tx.outputs().len(), 3);
    assert_eq!(final_tx.outputs().get(0).amount().coin(), BigNum(600));
    assert_eq!(
        final_tx
            .outputs()
            .get(1)
            .amount()
            .multiasset()
            .unwrap()
            .get(policy_id)
            .unwrap()
            .get(&name)
            .unwrap(),
        BigNum(ma_input1 + ma_input2 - ma_output1)
    );
    // The first change output that contains all the tokens contain minimum required Coin
    let min_coin_for_dirty_change = min_ada_for_output(
        &final_tx.outputs().get(1),
        &DataCost::new_coins_per_byte(&coin_per_utxo_byte),
    )
    .unwrap();
    assert_eq!(
        final_tx.outputs().get(1).amount().coin(),
        min_coin_for_dirty_change
    );
    assert_eq!(final_tx.outputs().get(2).amount().coin(), BigNum(236));
    assert_eq!(final_tx.outputs().get(2).amount().multiasset(), None);
}

#[test]
fn build_tx_with_native_assets_change_and_no_purification_cuz_not_enough_pure_coin() {
    // Prefer pure change!
    let mut tx_builder = create_tx_builder_with_fee_and_pure_change(&create_linear_fee(1, 1));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let policy_id = &PolicyID::from([0u8; 28]);
    let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

    let ma_input1 = 100;
    let ma_input2 = 200;
    let ma_output1 = 60;

    let multiassets = [ma_input1, ma_input2, ma_output1]
        .iter()
        .map(|input| {
            let mut multiasset = MultiAsset::new();
            multiasset.insert(policy_id, &{
                let mut assets = Assets::new();
                assets.insert(&name, &BigNum(*input));
                assets
            });
            multiasset
        })
        .collect::<Vec<MultiAsset>>();

    for (i, (multiasset, ada)) in multiassets
        .iter()
        .zip([300u64, 900].iter().cloned().map(|x| BigNum(x)))
        .enumerate()
    {
        let mut input_amount = Value::new(&ada);
        input_amount.set_multiasset(multiasset);

        tx_builder.add_key_input(
            &&spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), i as u32),
            &input_amount,
        );
    }

    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let mut output_amount = Value::new(&BigNum(300));
    output_amount.set_multiasset(&multiassets[2]);

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(added_change, true);
    let final_tx = tx_builder.build().unwrap();
    assert_eq!(final_tx.outputs().len(), 2);
    assert_eq!(final_tx.outputs().get(0).amount().coin(), BigNum(300));
    assert_eq!(
        final_tx
            .outputs()
            .get(1)
            .amount()
            .multiasset()
            .unwrap()
            .get(policy_id)
            .unwrap()
            .get(&name)
            .unwrap(),
        BigNum(ma_input1 + ma_input2 - ma_output1)
    );
    // The single change output contains more Coin then minimal utxo value
    // But not enough to cover the additional fee for a separate output
    assert_eq!(final_tx.outputs().get(1).amount().coin(), BigNum(499));
}

#[test]
#[should_panic]
fn build_tx_leftover_assets() {
    let mut tx_builder = create_default_tx_builder();
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    // add an input that contains an asset not present in the output
    let policy_id = &PolicyID::from([0u8; 28]);
    let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
    let mut input_amount = Value::new(&BigNum(1_000_000));
    let mut input_multiasset = MultiAsset::new();
    input_multiasset.insert(policy_id, &{
        let mut assets = Assets::new();
        assets.insert(&name, &BigNum(100));
        assets
    });
    input_amount.set_multiasset(&input_multiasset);
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &input_amount,
    );

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&BigNum(880_000))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();
    let added_change = tx_builder.add_change_if_needed(&change_addr);
    assert!(!added_change.unwrap());
    assert_eq!(tx_builder.outputs.len(), 1);
    assert_eq!(
        tx_builder
            .get_explicit_input()
            .unwrap()
            .checked_add(&tx_builder.get_implicit_input().unwrap())
            .unwrap(),
        tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
            .unwrap()
    );
    let _final_tx = tx_builder.build(); // just test that it doesn't throw
}

#[test]
fn build_tx_burn_less_than_min_ada() {
    // with this mainnet value we should end up with a final min_ada_required of just under 1_000_000
    let mut tx_builder = create_reallistic_tx_builder();

    let output_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b")
            .unwrap();
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr.to_address())
                .next()
                .unwrap()
                .with_value(&Value::new(&BigNum(2_000_000)))
                .build()
                .unwrap(),
        )
        .unwrap();

    tx_builder.add_regular_input(
        &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(2_400_000)),
    );

    tx_builder.set_ttl(1);

    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap();
    let added_change = tx_builder.add_change_if_needed(&change_addr.to_address());
    assert!(!added_change.unwrap());
    assert_eq!(tx_builder.outputs.len(), 1);
    assert_eq!(
        tx_builder
            .get_explicit_input()
            .unwrap()
            .checked_add(&tx_builder.get_implicit_input().unwrap())
            .unwrap(),
        tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
            .unwrap()
    );
    let _final_tx = tx_builder.build(); // just test that it doesn't throw
}

#[test]
fn build_tx_burn_empty_assets() {
    let mut tx_builder = create_reallistic_tx_builder();

    let output_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b")
            .unwrap();
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr.to_address())
                .next()
                .unwrap()
                .with_value(&Value::new(&BigNum(2_000_000)))
                .build()
                .unwrap(),
        )
        .unwrap();

    let mut input_value = Value::new(&BigNum(2_400_000));
    input_value.set_multiasset(&MultiAsset::new());
    tx_builder.add_regular_input(
        &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &input_value,
    );

    tx_builder.set_ttl(1);

    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap();
    let added_change = tx_builder.add_change_if_needed(&change_addr.to_address());
    assert!(!added_change.unwrap());
    assert_eq!(tx_builder.outputs.len(), 1);
    assert_eq!(
        tx_builder
            .get_explicit_input()
            .unwrap()
            .checked_add(&tx_builder.get_implicit_input().unwrap())
            .unwrap()
            .coin(),
        tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
            .unwrap()
            .coin()
    );
    let _final_tx = tx_builder.build(); // just test that it doesn't throw
}

#[test]
fn build_tx_no_useless_multiasset() {
    let mut tx_builder = create_reallistic_tx_builder();

    let policy_id = &PolicyID::from([0u8; 28]);
    let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

    // add an output that uses up all the token but leaves ADA
    let mut input_amount = Value::new(&BigNum(5_000_000));
    let mut input_multiasset = MultiAsset::new();
    input_multiasset.insert(policy_id, &{
        let mut assets = Assets::new();
        assets.insert(&name, &BigNum(100));
        assets
    });
    input_amount.set_multiasset(&input_multiasset);

    tx_builder.add_regular_input(
        &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &input_amount,
    );

    // add an input that contains an asset & ADA
    let mut output_amount = Value::new(&BigNum(2_000_000));
    let mut output_multiasset = MultiAsset::new();
    output_multiasset.insert(policy_id, &{
        let mut assets = Assets::new();
        assets.insert(&name, &BigNum(100));
        assets
    });
    output_amount.set_multiasset(&output_multiasset);

    let output_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b")
            .unwrap();
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr.to_address())
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    tx_builder.set_ttl(1);

    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap();
    let added_change = tx_builder.add_change_if_needed(&change_addr.to_address());
    assert!(added_change.unwrap());
    assert_eq!(tx_builder.outputs.len(), 2);
    let final_tx = tx_builder.build().unwrap();
    let change_output = final_tx.outputs().get(1);
    let change_assets = change_output.amount().multiasset();

    // since all tokens got sent in the output
    // the change should be only ADA and not have any multiasset struct in it
    assert!(change_assets.is_none());
}

fn create_multiasset() -> (MultiAsset, [ScriptHash; 3], [AssetName; 3]) {
    let policy_ids = [fake_policy_id(0), fake_policy_id(1), fake_policy_id(2)];
    let names = [
        AssetName::new(vec![99u8; 32]).unwrap(),
        AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
        AssetName::new(vec![4u8, 5, 6, 7, 8, 9]).unwrap(),
    ];
    let multiasset = policy_ids.iter().zip(names.iter()).fold(
        MultiAsset::new(),
        |mut acc, (policy_id, name)| {
            acc.insert(policy_id, &{
                let mut assets = Assets::new();
                assets.insert(&name, &BigNum(500));
                assets
            });
            acc
        },
    );
    return (multiasset, policy_ids, names);
}

#[test]
fn build_tx_add_change_split_nfts() {
    let max_value_size = 100; // super low max output size to test with fewer assets
    let mut tx_builder =
        create_tx_builder_with_fee_and_val_size(&create_linear_fee(0, 1), max_value_size);

    let (multiasset, policy_ids, names) = create_multiasset();

    let mut input_value = Value::new(&BigNum(1000));
    input_value.set_multiasset(&multiasset);

    tx_builder.add_regular_input(
        &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &input_value,
    );

    let output_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b")
            .unwrap()
            .to_address();
    let output_amount = Value::new(&BigNum(208));

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap()
            .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(added_change, true);
    let final_tx = tx_builder.build().unwrap();
    assert_eq!(final_tx.outputs().len(), 3);
    for (policy_id, asset_name) in policy_ids.iter().zip(names.iter()) {
        assert!(final_tx
            .outputs
            .0
            .iter()
            .find(|output| output.amount.multiasset.as_ref().map_or_else(
                || false,
                |ma| ma
                    .0
                    .iter()
                    .find(|(pid, a)| *pid == policy_id
                        && a.0.iter().find(|(name, _)| *name == asset_name).is_some())
                    .is_some()
            ))
            .is_some());
    }
    for output in final_tx.outputs.0.iter() {
        assert!(output.amount.to_bytes().len() <= max_value_size as usize);
    }
}

#[test]
fn build_tx_too_big_output() {
    let mut tx_builder = create_tx_builder_with_fee_and_val_size(&create_linear_fee(0, 1), 10);

    tx_builder.add_regular_input(
        &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(500)),
    );

    let output_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b")
            .unwrap()
            .to_address();
    let mut output_amount = Value::new(&BigNum(50));
    output_amount.set_multiasset(&create_multiasset().0);

    assert!(tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap()
        )
        .is_err());
}

#[test]
fn build_tx_add_change_nfts_not_enough_ada() {
    let mut tx_builder = create_tx_builder_with_fee_and_val_size(
        &create_linear_fee(0, 1),
        150, // super low max output size to test with fewer assets
    );

    let policy_ids = [
        PolicyID::from([0u8; 28]),
        PolicyID::from([1u8; 28]),
        PolicyID::from([2u8; 28]),
    ];
    let names = [
        AssetName::new(vec![99u8; 32]).unwrap(),
        AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
        AssetName::new(vec![4u8, 5, 6, 7, 8, 9]).unwrap(),
    ];

    let multiasset = policy_ids.iter().zip(names.iter()).fold(
        MultiAsset::new(),
        |mut acc, (policy_id, name)| {
            acc.insert(policy_id, &{
                let mut assets = Assets::new();
                assets.insert(&name, &BigNum(500));
                assets
            });
            acc
        },
    );

    let mut input_value = Value::new(&BigNum(58));
    input_value.set_multiasset(&multiasset);

    tx_builder.add_regular_input(
        &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &input_value,
    );

    let output_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b")
            .unwrap()
            .to_address();
    let output_amount = Value::new(&BigNum(208));

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap()
            .to_address();

    assert!(tx_builder.add_change_if_needed(&change_addr).is_err())
}

fn make_input(input_hash_byte: u8, value: Value) -> TransactionUnspentOutput {
    TransactionUnspentOutput::new(
        &fake_tx_input(input_hash_byte),
        &TransactionOutputBuilder::new()
            .with_address(
                &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z")
                    .unwrap(),
            )
            .next()
            .unwrap()
            .with_value(&value)
            .build()
            .unwrap(),
    )
}

#[test]
fn tx_builder_cip2_largest_first_increasing_fees() {
    // we have a = 1 to test increasing fees when more inputs are added
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 0));
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(
                    &Address::from_bech32(
                        "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                    )
                    .unwrap(),
                )
                .next()
                .unwrap()
                .with_coin(&BigNum(9000))
                .build()
                .unwrap(),
        )
        .unwrap();
    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(0u8, Value::new(&BigNum(1200))));
    available_inputs.add(&make_input(1u8, Value::new(&BigNum(1600))));
    available_inputs.add(&make_input(2u8, Value::new(&BigNum(6400))));
    available_inputs.add(&make_input(3u8, Value::new(&BigNum(2400))));
    available_inputs.add(&make_input(4u8, Value::new(&BigNum(800))));
    tx_builder
        .add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::LargestFirst)
        .unwrap();
    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap()
            .to_address();
    let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert!(change_added);
    let tx = tx_builder.build().unwrap();
    // change needed
    assert_eq!(2, tx.outputs().len());
    assert_eq!(3, tx.inputs().len());
    // confirm order of only what is necessary
    assert_eq!(1u8, tx.inputs().get(0).transaction_id().0[0]);
    assert_eq!(2u8, tx.inputs().get(1).transaction_id().0[0]);
    assert_eq!(3u8, tx.inputs().get(2).transaction_id().0[0]);
}

#[test]
fn tx_builder_cip2_largest_first_static_fees() {
    // we have a = 0 so we know adding inputs/outputs doesn't change the fee so we can analyze more
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(
                    &Address::from_bech32(
                        "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                    )
                    .unwrap(),
                )
                .next()
                .unwrap()
                .with_coin(&BigNum(1200))
                .build()
                .unwrap(),
        )
        .unwrap();
    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(0u8, Value::new(&BigNum(150))));
    available_inputs.add(&make_input(1u8, Value::new(&BigNum(200))));
    available_inputs.add(&make_input(2u8, Value::new(&BigNum(800))));
    available_inputs.add(&make_input(3u8, Value::new(&BigNum(400))));
    available_inputs.add(&make_input(4u8, Value::new(&BigNum(100))));
    tx_builder
        .add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::LargestFirst)
        .unwrap();
    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap()
            .to_address();
    let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert!(!change_added);
    let tx = tx_builder.build().unwrap();
    // change not needed - should be exact
    assert_eq!(1, tx.outputs().len());
    assert_eq!(2, tx.inputs().len());
    // confirm order of only what is necessary
    assert_eq!(2u8, tx.inputs().get(0).transaction_id().0[0]);
    assert_eq!(3u8, tx.inputs().get(1).transaction_id().0[0]);
}

#[test]
fn tx_builder_cip2_largest_first_multiasset() {
    // we have a = 0 so we know adding inputs/outputs doesn't change the fee so we can analyze more
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
    let pid1 = PolicyID::from([1u8; 28]);
    let pid2 = PolicyID::from([2u8; 28]);
    let asset_name1 = AssetName::new(vec![1u8; 8]).unwrap();
    let asset_name2 = AssetName::new(vec![2u8; 11]).unwrap();
    let asset_name3 = AssetName::new(vec![3u8; 9]).unwrap();

    let mut output_value = Value::new(&BigNum(415));
    let mut output_ma = MultiAsset::new();
    output_ma.set_asset(&pid1, &asset_name1, BigNum(5));
    output_ma.set_asset(&pid1, &asset_name2, BigNum(1));
    output_ma.set_asset(&pid2, &asset_name2, BigNum(2));
    output_ma.set_asset(&pid2, &asset_name3, BigNum(4));
    output_value.set_multiasset(&output_ma);
    tx_builder
        .add_output(&TransactionOutput::new(
            &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z")
                .unwrap(),
            &output_value,
        ))
        .unwrap();

    let mut available_inputs = TransactionUnspentOutputs::new();
    // should not be taken
    available_inputs.add(&make_input(0u8, Value::new(&BigNum(150))));

    // should not be taken
    let mut input1 = make_input(1u8, Value::new(&BigNum(200)));
    let mut ma1 = MultiAsset::new();
    ma1.set_asset(&pid1, &asset_name1, BigNum(10));
    ma1.set_asset(&pid1, &asset_name2, BigNum(1));
    ma1.set_asset(&pid2, &asset_name2, BigNum(2));
    input1.output.amount.set_multiasset(&ma1);
    available_inputs.add(&input1);

    // taken first to satisfy pid1:asset_name1 (but also satisfies pid2:asset_name3)
    let mut input2 = make_input(2u8, Value::new(&BigNum(10)));
    let mut ma2 = MultiAsset::new();
    ma2.set_asset(&pid1, &asset_name1, BigNum(20));
    ma2.set_asset(&pid2, &asset_name3, BigNum(4));
    input2.output.amount.set_multiasset(&ma2);
    available_inputs.add(&input2);

    // taken second to satisfy pid1:asset_name2 (but also satisfies pid2:asset_name1)
    let mut input3 = make_input(3u8, Value::new(&BigNum(50)));
    let mut ma3 = MultiAsset::new();
    ma3.set_asset(&pid2, &asset_name1, BigNum(5));
    ma3.set_asset(&pid1, &asset_name2, BigNum(15));
    input3.output.amount.multiasset = Some(ma3);
    available_inputs.add(&input3);

    // should not be taken either
    let mut input4 = make_input(4u8, Value::new(&BigNum(10)));
    let mut ma4 = MultiAsset::new();
    ma4.set_asset(&pid1, &asset_name1, BigNum(10));
    ma4.set_asset(&pid1, &asset_name2, BigNum(10));
    input4.output.amount.multiasset = Some(ma4);
    available_inputs.add(&input4);

    // taken third to satisfy pid2:asset_name_2
    let mut input5 = make_input(5u8, Value::new(&BigNum(10)));
    let mut ma5 = MultiAsset::new();
    ma5.set_asset(&pid1, &asset_name2, BigNum(10));
    ma5.set_asset(&pid2, &asset_name2, BigNum(3));
    input5.output.amount.multiasset = Some(ma5);
    available_inputs.add(&input5);

    // should be taken to get enough ADA
    let input6 = make_input(6u8, Value::new(&BigNum(900)));
    available_inputs.add(&input6);

    // should not be taken
    available_inputs.add(&make_input(7u8, Value::new(&BigNum(100))));
    tx_builder
        .add_inputs_from(
            &available_inputs,
            CoinSelectionStrategyCIP2::LargestFirstMultiAsset,
        )
        .unwrap();
    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap()
            .to_address();
    let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert!(change_added);
    let tx = tx_builder.build().unwrap();

    assert_eq!(2, tx.outputs().len());
    assert_eq!(4, tx.inputs().len());
    // check order expected per-asset
    assert_eq!(2u8, tx.inputs().get(0).transaction_id().0[0]);
    assert_eq!(3u8, tx.inputs().get(1).transaction_id().0[0]);
    assert_eq!(5u8, tx.inputs().get(2).transaction_id().0[0]);
    assert_eq!(6u8, tx.inputs().get(3).transaction_id().0[0]);

    let change = tx.outputs().get(1).amount;
    assert_eq!(u64::from(change.coin), 555);
    let change_ma = change.multiasset().unwrap();
    assert_eq!(15, u64::from(change_ma.get_asset(&pid1, &asset_name1)));
    assert_eq!(24, u64::from(change_ma.get_asset(&pid1, &asset_name2)));
    assert_eq!(1, u64::from(change_ma.get_asset(&pid2, &asset_name2)));
    assert_eq!(0, u64::from(change_ma.get_asset(&pid2, &asset_name3)));
    let expected_input = input2
        .output
        .amount
        .checked_add(&input3.output.amount)
        .unwrap()
        .checked_add(&input5.output.amount)
        .unwrap()
        .checked_add(&input6.output.amount)
        .unwrap();
    let expected_change = expected_input.checked_sub(&output_value).unwrap();
    assert_eq!(expected_change, change);
}

#[test]
fn tx_builder_cip2_random_improve_multiasset() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
    let pid1 = PolicyID::from([1u8; 28]);
    let pid2 = PolicyID::from([2u8; 28]);
    let asset_name1 = AssetName::new(vec![1u8; 8]).unwrap();
    let asset_name2 = AssetName::new(vec![2u8; 11]).unwrap();
    let asset_name3 = AssetName::new(vec![3u8; 9]).unwrap();

    let mut output_value = Value::new(&BigNum(415));
    let mut output_ma = MultiAsset::new();
    output_ma.set_asset(&pid1, &asset_name1, BigNum(5));
    output_ma.set_asset(&pid1, &asset_name2, BigNum(1));
    output_ma.set_asset(&pid2, &asset_name2, BigNum(2));
    output_ma.set_asset(&pid2, &asset_name3, BigNum(4));
    output_value.set_multiasset(&output_ma);
    tx_builder
        .add_output(&TransactionOutput::new(
            &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z")
                .unwrap(),
            &output_value,
        ))
        .unwrap();

    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(0u8, Value::new(&BigNum(150))));

    let mut input1 = make_input(1u8, Value::new(&BigNum(200)));
    let mut ma1 = MultiAsset::new();
    ma1.set_asset(&pid1, &asset_name1, BigNum(10));
    ma1.set_asset(&pid1, &asset_name2, BigNum(1));
    ma1.set_asset(&pid2, &asset_name2, BigNum(2));
    input1.output.amount.set_multiasset(&ma1);
    available_inputs.add(&input1);

    let mut input2 = make_input(2u8, Value::new(&BigNum(10)));
    let mut ma2 = MultiAsset::new();
    ma2.set_asset(&pid1, &asset_name1, BigNum(20));
    ma2.set_asset(&pid2, &asset_name3, BigNum(4));
    input2.output.amount.set_multiasset(&ma2);
    available_inputs.add(&input2);

    let mut input3 = make_input(3u8, Value::new(&BigNum(50)));
    let mut ma3 = MultiAsset::new();
    ma3.set_asset(&pid2, &asset_name1, BigNum(5));
    ma3.set_asset(&pid1, &asset_name2, BigNum(15));
    input3.output.amount.multiasset = Some(ma3);
    available_inputs.add(&input3);

    let mut input4 = make_input(4u8, Value::new(&BigNum(10)));
    let mut ma4 = MultiAsset::new();
    ma4.set_asset(&pid1, &asset_name1, BigNum(10));
    ma4.set_asset(&pid1, &asset_name2, BigNum(10));
    input4.output.amount.multiasset = Some(ma4);
    available_inputs.add(&input4);

    let mut input5 = make_input(5u8, Value::new(&BigNum(10)));
    let mut ma5 = MultiAsset::new();
    ma5.set_asset(&pid1, &asset_name2, BigNum(10));
    ma5.set_asset(&pid2, &asset_name2, BigNum(3));
    input5.output.amount.multiasset = Some(ma5);
    available_inputs.add(&input5);

    let input6 = make_input(6u8, Value::new(&BigNum(1000)));
    available_inputs.add(&input6);
    available_inputs.add(&make_input(7u8, Value::new(&BigNum(100))));

    let mut input8 = make_input(8u8, Value::new(&BigNum(10)));
    let mut ma8 = MultiAsset::new();
    ma8.set_asset(&pid2, &asset_name2, BigNum(10));
    input8.output.amount.multiasset = Some(ma8);
    available_inputs.add(&input8);

    let mut input9 = make_input(9u8, Value::new(&BigNum(10)));
    let mut ma9 = MultiAsset::new();
    ma9.set_asset(&pid2, &asset_name3, BigNum(10));
    input9.output.amount.multiasset = Some(ma9);
    available_inputs.add(&input9);

    tx_builder
        .add_inputs_from(
            &available_inputs,
            CoinSelectionStrategyCIP2::RandomImproveMultiAsset,
        )
        .unwrap();

    let input_for_cover_change = make_input(10u8, Value::new(&BigNum(1000)));
    tx_builder.add_regular_input(
        &input_for_cover_change.output.address,
        &input_for_cover_change.input,
        &input_for_cover_change.output.amount,
    );

    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap()
            .to_address();
    let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert!(change_added);
    let tx = tx_builder.build().unwrap();

    assert_eq!(2, tx.outputs().len());

    let input_total = tx_builder.get_explicit_input().unwrap();
    assert!(input_total >= output_value);
}

#[test]
fn tx_builder_cip2_random_improve() {
    // we have a = 1 to test increasing fees when more inputs are added
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 0));
    const COST: u64 = 10000;
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(
                    &Address::from_bech32(
                        "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                    )
                    .unwrap(),
                )
                .next()
                .unwrap()
                .with_coin(&BigNum(COST))
                .build()
                .unwrap(),
        )
        .unwrap();
    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(0u8, Value::new(&BigNum(1500))));
    available_inputs.add(&make_input(1u8, Value::new(&BigNum(2000))));
    available_inputs.add(&make_input(2u8, Value::new(&BigNum(8000))));
    available_inputs.add(&make_input(3u8, Value::new(&BigNum(4000))));
    available_inputs.add(&make_input(4u8, Value::new(&BigNum(1000))));
    available_inputs.add(&make_input(5u8, Value::new(&BigNum(2000))));
    available_inputs.add(&make_input(6u8, Value::new(&BigNum(1500))));
    let add_inputs_res =
        tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::RandomImprove);
    assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap()
            .to_address();
    let add_change_res = tx_builder.add_change_if_needed(&change_addr);
    assert!(add_change_res.is_ok(), "{:?}", add_change_res.err());
    let tx_build_res = tx_builder.build();
    assert!(tx_build_res.is_ok(), "{:?}", tx_build_res.err());
    let tx = tx_build_res.unwrap();
    // we need to look up the values to ensure there's enough
    let mut input_values = BTreeMap::new();
    for utxo in available_inputs.0.iter() {
        input_values.insert(utxo.input.transaction_id(), utxo.output.amount.clone());
    }
    let mut encountered = std::collections::HashSet::new();
    let mut input_total = Value::new(&Coin::zero());
    for input in tx.inputs.0.iter() {
        let txid = input.transaction_id();
        if !encountered.insert(txid.clone()) {
            panic!("Input {:?} duplicated", txid);
        }
        let value = input_values.get(&txid).unwrap();
        input_total = input_total.checked_add(value).unwrap();
    }
    assert!(
        input_total
            >= Value::new(
                &tx_builder
                    .min_fee()
                    .unwrap()
                    .checked_add(&BigNum(COST))
                    .unwrap()
            )
    );
}

#[test]
fn tx_builder_cip2_random_improve_when_using_all_available_inputs() {
    // we have a = 1 to test increasing fees when more inputs are added
    let linear_fee = LinearFee::new(&BigNum(1), &BigNum(0));
    let cfg = TransactionBuilderConfigBuilder::new()
        .fee_algo(&linear_fee)
        .pool_deposit(&BigNum(0))
        .key_deposit(&BigNum(0))
        .max_value_size(9999)
        .max_tx_size(9999)
        .coins_per_utxo_byte(&Coin::zero())
        .build()
        .unwrap();
    let mut tx_builder = TransactionBuilder::new(&cfg);
    const COST: u64 = 1000;
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(
                    &Address::from_bech32(
                        "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                    )
                    .unwrap(),
                )
                .next()
                .unwrap()
                .with_coin(&BigNum(COST))
                .build()
                .unwrap(),
        )
        .unwrap();
    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(1u8, Value::new(&BigNum(800))));
    available_inputs.add(&make_input(2u8, Value::new(&BigNum(800))));
    let add_inputs_res =
        tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::RandomImprove);
    assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
}

#[test]
fn tx_builder_cip2_random_improve_adds_enough_for_fees() {
    // we have a = 1 to test increasing fees when more inputs are added
    let linear_fee = LinearFee::new(&BigNum(1), &BigNum(0));
    let cfg = TransactionBuilderConfigBuilder::new()
        .fee_algo(&linear_fee)
        .pool_deposit(&BigNum(0))
        .key_deposit(&BigNum(0))
        .max_value_size(9999)
        .max_tx_size(9999)
        .coins_per_utxo_byte(&Coin::zero())
        .build()
        .unwrap();
    let mut tx_builder = TransactionBuilder::new(&cfg);
    const COST: u64 = 100;
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(
                    &Address::from_bech32(
                        "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                    )
                    .unwrap(),
                )
                .next()
                .unwrap()
                .with_coin(&BigNum(COST))
                .build()
                .unwrap(),
        )
        .unwrap();
    assert_eq!(tx_builder.min_fee().unwrap(), BigNum(53));
    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(1u8, Value::new(&BigNum(150))));
    available_inputs.add(&make_input(2u8, Value::new(&BigNum(150))));
    available_inputs.add(&make_input(3u8, Value::new(&BigNum(150))));
    let add_inputs_res =
        tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::RandomImprove);
    assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
    assert_eq!(tx_builder.min_fee().unwrap(), BigNum(264));
    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap()
            .to_address();
    let add_change_res = tx_builder.add_change_if_needed(&change_addr);
    assert!(add_change_res.is_ok(), "{:?}", add_change_res.err());
}

#[test]
fn build_tx_pay_to_multisig() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(10, 2));
    let spend = root_key_15()
        .derive(harden(1854))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();

    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&BigNum(999_000))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);
    tx_builder.set_fee(&BigNum(1_000));

    assert_eq!(tx_builder.outputs.len(), 1);
    assert_eq!(
        tx_builder
            .get_explicit_input()
            .unwrap()
            .checked_add(&tx_builder.get_implicit_input().unwrap())
            .unwrap(),
        tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
            .unwrap()
    );

    let _final_tx = tx_builder.build().unwrap();
    let _deser_t = TransactionBody::from_bytes(_final_tx.to_bytes()).unwrap();

    assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
}

fn build_full_tx(
    body: &TransactionBody,
    witness_set: &TransactionWitnessSet,
    auxiliary_data: Option<AuxiliaryData>,
) -> Transaction {
    return Transaction::new(body, witness_set, auxiliary_data);
}

#[test]
fn build_tx_multisig_spend_1on1_unsigned() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(10, 2));

    let spend = root_key_15() //multisig
        .derive(harden(1854))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let stake = root_key_15() //multisig
        .derive(harden(1854))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let addr_multisig = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    let addr_output = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    tx_builder.add_regular_input(
        &addr_multisig,
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_output)
                .next()
                .unwrap()
                .with_coin(&BigNum(999_000))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);
    tx_builder.set_fee(&BigNum(1_000));

    let mut auxiliary_data = AuxiliaryData::new();
    let mut pubkey_native_scripts = NativeScripts::new();
    let mut oneof_native_scripts = NativeScripts::new();

    let spending_hash = spend.to_raw_key().hash();
    pubkey_native_scripts.add(&NativeScript::new_script_pubkey(&ScriptPubkey::new(
        &spending_hash,
    )));
    oneof_native_scripts.add(&NativeScript::new_script_n_of_k(&ScriptNOfK::new(
        1,
        &pubkey_native_scripts,
    )));
    auxiliary_data.set_native_scripts(&oneof_native_scripts);
    tx_builder.set_auxiliary_data(&auxiliary_data);

    assert_eq!(tx_builder.outputs.len(), 1);
    assert_eq!(
        tx_builder
            .get_explicit_input()
            .unwrap()
            .checked_add(&tx_builder.get_implicit_input().unwrap())
            .unwrap(),
        tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
            .unwrap()
    );

    let _final_tx = tx_builder.build().unwrap();
    let _deser_t = TransactionBody::from_bytes(_final_tx.to_bytes()).unwrap();

    assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
    assert_eq!(
        _deser_t.auxiliary_data_hash.unwrap(),
        utils::hash_auxiliary_data(&auxiliary_data)
    );
}

#[test]
fn build_tx_multisig_1on1_signed() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(10, 2));
    let spend = root_key_15()
        .derive(harden(1854)) //multisig
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1854)) //multisig
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&BigNum(999_000))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);
    tx_builder.set_fee(&BigNum(1_000));

    let mut auxiliary_data = AuxiliaryData::new();
    let mut pubkey_native_scripts = NativeScripts::new();
    let mut oneof_native_scripts = NativeScripts::new();

    let spending_hash = spend.to_raw_key().hash();
    pubkey_native_scripts.add(&NativeScript::new_script_pubkey(&ScriptPubkey::new(
        &spending_hash,
    )));
    oneof_native_scripts.add(&NativeScript::new_script_n_of_k(&ScriptNOfK::new(
        1,
        &pubkey_native_scripts,
    )));
    auxiliary_data.set_native_scripts(&oneof_native_scripts);
    tx_builder.set_auxiliary_data(&auxiliary_data);

    let body = tx_builder.build().unwrap();

    assert_eq!(tx_builder.outputs.len(), 1);
    assert_eq!(
        tx_builder
            .get_explicit_input()
            .unwrap()
            .checked_add(&tx_builder.get_implicit_input().unwrap())
            .unwrap(),
        tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
            .unwrap()
    );

    let mut witness_set = TransactionWitnessSet::new();
    let mut vkw = Vkeywitnesses::new();
    vkw.add(&make_vkey_witness(
        &hash_transaction(&body),
        &PrivateKey::from_normal_bytes(
            &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
                .unwrap(),
        )
        .unwrap(),
    ));
    witness_set.set_vkeys(&vkw);

    let _final_tx = build_full_tx(&body, &witness_set, None);
    let _deser_t = Transaction::from_bytes(_final_tx.to_bytes()).unwrap();
    assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
    assert_eq!(
        _deser_t.body().auxiliary_data_hash.unwrap(),
        utils::hash_auxiliary_data(&auxiliary_data)
    );
}

#[test]
fn add_change_splits_change_into_multiple_outputs_when_nfts_overflow_output_size() {
    let linear_fee = LinearFee::new(&BigNum(0), &BigNum(1));
    let max_value_size = 100; // super low max output size to test with fewer assets
    let mut tx_builder = TransactionBuilder::new(
        &TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&BigNum(0))
            .key_deposit(&BigNum(0))
            .max_value_size(max_value_size)
            .max_tx_size(MAX_TX_SIZE)
            .coins_per_utxo_byte(&BigNum(1))
            .prefer_pure_change(true)
            .build()
            .unwrap(),
    );

    let policy_id = PolicyID::from([0u8; 28]);
    let names = [
        AssetName::new(vec![99u8; 32]).unwrap(),
        AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
        AssetName::new(vec![4u8, 5, 6, 7]).unwrap(),
        AssetName::new(vec![5u8, 5, 6, 7]).unwrap(),
        AssetName::new(vec![6u8, 5, 6, 7]).unwrap(),
    ];
    let assets = names.iter().fold(Assets::new(), |mut a, name| {
        a.insert(&name, &BigNum(500));
        a
    });
    let mut multiasset = MultiAsset::new();
    multiasset.insert(&policy_id, &assets);

    let mut input_value = Value::new(&BigNum(1200));
    input_value.set_multiasset(&multiasset);

    tx_builder.add_regular_input(
        &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &input_value,
    );

    let output_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b")
            .unwrap()
            .to_address();
    let output_amount = Value::new(&BigNum(208));

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr)
                .next()
                .unwrap()
                .with_value(&output_amount)
                .build()
                .unwrap(),
        )
        .unwrap();

    let change_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho")
            .unwrap()
            .to_address();

    let add_change_result = tx_builder.add_change_if_needed(&change_addr);
    assert!(add_change_result.is_ok());
    assert_eq!(tx_builder.outputs.len(), 4);

    let change1 = tx_builder.outputs.get(1);
    let change2 = tx_builder.outputs.get(2);
    let change3 = tx_builder.outputs.get(3);

    assert_eq!(change1.address, change_addr);
    assert_eq!(change1.address, change2.address);
    assert_eq!(change1.address, change3.address);

    assert_eq!(change1.amount.coin, BigNum(288));
    assert_eq!(change2.amount.coin, BigNum(293));
    assert_eq!(change3.amount.coin, BigNum(410));

    assert!(change1.amount.multiasset.is_some());
    assert!(change2.amount.multiasset.is_some());
    assert!(change3.amount.multiasset.is_none()); // purified

    let masset1 = change1.amount.multiasset.unwrap();
    let masset2 = change2.amount.multiasset.unwrap();

    assert_eq!(masset1.keys().len(), 1);
    assert_eq!(masset1.keys(), masset2.keys());

    let asset1 = masset1.get(&policy_id).unwrap();
    let asset2 = masset2.get(&policy_id).unwrap();
    assert_eq!(asset1.len(), 4);
    assert_eq!(asset2.len(), 1);

    names.iter().for_each(|name| {
        let v1 = asset1.get(name);
        let v2 = asset2.get(name);
        assert_ne!(v1.is_some(), v2.is_some());
        assert_eq!(v1.or(v2).unwrap(), BigNum(500));
    });
}

fn create_json_metadatum_string() -> String {
    String::from("{ \"qwe\": 123 }")
}

fn create_json_metadatum() -> TransactionMetadatum {
    encode_json_str_to_metadatum(
        create_json_metadatum_string(),
        MetadataJsonSchema::NoConversions,
    )
    .unwrap()
}

fn create_aux_with_metadata(metadatum_key: &TransactionMetadatumLabel) -> AuxiliaryData {
    let mut metadata = GeneralTransactionMetadata::new();
    metadata.insert(metadatum_key, &create_json_metadatum());

    let mut aux = AuxiliaryData::new();
    aux.set_metadata(&metadata);

    let mut nats = NativeScripts::new();
    nats.add(&NativeScript::new_timelock_start(&TimelockStart::new(123)));
    aux.set_native_scripts(&nats);

    return aux;
}

fn assert_json_metadatum(dat: &TransactionMetadatum) {
    let map = dat.as_map().unwrap();
    assert_eq!(map.len(), 1);
    let key = TransactionMetadatum::new_text(String::from("qwe")).unwrap();
    let val = map.get(&key).unwrap();
    assert_eq!(val.as_int().unwrap(), Int::new_i32(123));
}

#[test]
fn set_metadata_with_empty_auxiliary() {
    let mut tx_builder = create_default_tx_builder();

    let num = BigNum(42);
    tx_builder.set_metadata(&create_aux_with_metadata(&num).metadata().unwrap());

    assert!(tx_builder.auxiliary_data.is_some());

    let aux = tx_builder.auxiliary_data.unwrap();
    assert!(aux.metadata().is_some());
    assert!(aux.native_scripts().is_none());
    assert!(aux.plutus_scripts().is_none());

    let met = aux.metadata().unwrap();

    assert_eq!(met.len(), 1);
    assert_json_metadatum(&met.get(&num).unwrap());
}

#[test]
fn set_metadata_with_existing_auxiliary() {
    let mut tx_builder = create_default_tx_builder();

    let num1 = BigNum(42);
    tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

    let num2 = BigNum(84);
    tx_builder.set_metadata(&create_aux_with_metadata(&num2).metadata().unwrap());

    let aux = tx_builder.auxiliary_data.unwrap();
    assert!(aux.metadata().is_some());
    assert!(aux.native_scripts().is_some());
    assert!(aux.plutus_scripts().is_none());

    let met = aux.metadata().unwrap();
    assert_eq!(met.len(), 1);
    assert!(met.get(&num1).is_none());
    assert_json_metadatum(&met.get(&num2).unwrap());
}

#[test]
fn add_metadatum_with_empty_auxiliary() {
    let mut tx_builder = create_default_tx_builder();

    let num = BigNum(42);
    tx_builder.add_metadatum(&num, &create_json_metadatum());

    assert!(tx_builder.auxiliary_data.is_some());

    let aux = tx_builder.auxiliary_data.unwrap();
    assert!(aux.metadata().is_some());
    assert!(aux.native_scripts().is_none());
    assert!(aux.plutus_scripts().is_none());

    let met = aux.metadata().unwrap();

    assert_eq!(met.len(), 1);
    assert_json_metadatum(&met.get(&num).unwrap());
}

#[test]
fn add_metadatum_with_existing_auxiliary() {
    let mut tx_builder = create_default_tx_builder();

    let num1 = BigNum(42);
    tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

    let num2 = BigNum(84);
    tx_builder.add_metadatum(&num2, &create_json_metadatum());

    let aux = tx_builder.auxiliary_data.unwrap();
    assert!(aux.metadata().is_some());
    assert!(aux.native_scripts().is_some());
    assert!(aux.plutus_scripts().is_none());

    let met = aux.metadata().unwrap();
    assert_eq!(met.len(), 2);
    assert_json_metadatum(&met.get(&num1).unwrap());
    assert_json_metadatum(&met.get(&num2).unwrap());
}

#[test]
fn add_json_metadatum_with_empty_auxiliary() {
    let mut tx_builder = create_default_tx_builder();

    let num = BigNum(42);
    tx_builder
        .add_json_metadatum(&num, create_json_metadatum_string())
        .unwrap();

    assert!(tx_builder.auxiliary_data.is_some());

    let aux = tx_builder.auxiliary_data.unwrap();
    assert!(aux.metadata().is_some());
    assert!(aux.native_scripts().is_none());
    assert!(aux.plutus_scripts().is_none());

    let met = aux.metadata().unwrap();

    assert_eq!(met.len(), 1);
    assert_json_metadatum(&met.get(&num).unwrap());
}

#[test]
fn add_json_metadatum_with_existing_auxiliary() {
    let mut tx_builder = create_default_tx_builder();

    let num1 = BigNum(42);
    tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

    let num2 = BigNum(84);
    tx_builder
        .add_json_metadatum(&num2, create_json_metadatum_string())
        .unwrap();

    let aux = tx_builder.auxiliary_data.unwrap();
    assert!(aux.metadata().is_some());
    assert!(aux.native_scripts().is_some());
    assert!(aux.plutus_scripts().is_none());

    let met = aux.metadata().unwrap();
    assert_eq!(met.len(), 2);
    assert_json_metadatum(&met.get(&num1).unwrap());
    assert_json_metadatum(&met.get(&num2).unwrap());
}

fn create_asset_name() -> AssetName {
    AssetName::new(vec![0u8, 1, 2, 3]).unwrap()
}

fn create_mint_asset() -> MintAssets {
    MintAssets::new_from_entry(&create_asset_name(), &Int::new_i32(1234)).unwrap()
}

fn create_assets() -> Assets {
    let mut assets = Assets::new();
    assets.insert(&create_asset_name(), &BigNum(1234));
    return assets;
}

fn create_mint_with_one_asset(policy_id: &PolicyID) -> Mint {
    Mint::new_from_entry(policy_id, &create_mint_asset())
}

fn create_multiasset_one_asset(policy_id: &PolicyID) -> MultiAsset {
    let mut mint = MultiAsset::new();
    mint.insert(policy_id, &create_assets());
    return mint;
}

fn assert_mint_asset(mint: &Mint, policy_id: &PolicyID) {
    assert!(mint.get(&policy_id).is_some());
    let result_asset = mint.get(&policy_id).unwrap();
    assert_eq!(result_asset.len(), 1);
    assert_eq!(
        result_asset
            .get(0)
            .unwrap()
            .get(&create_asset_name())
            .unwrap(),
        Int::new_i32(1234)
    );
}

fn mint_script_and_policy_and_hash(x: u8) -> (NativeScript, PolicyID, Ed25519KeyHash) {
    let hash = fake_key_hash(x);
    let mint_script = NativeScript::new_script_pubkey(&ScriptPubkey::new(&hash));
    let policy_id = mint_script.hash();
    (mint_script, policy_id, hash)
}

fn mint_script_and_policy(x: u8) -> (NativeScript, ScriptHash) {
    let (m, p, _) = mint_script_and_policy_and_hash(x);
    (m, p)
}

#[test]
fn set_mint_asset_with_empty_mint() {
    let mut tx_builder = create_default_tx_builder();

    let (mint_script, policy_id) = mint_script_and_policy(0);
    tx_builder.set_mint_asset(&mint_script, &create_mint_asset());

    assert!(tx_builder.mint.is_some());
    let mint_scripts = tx_builder.mint.as_ref().unwrap().get_native_scripts();
    assert!(mint_scripts.len() > 0);

    let mint = tx_builder.mint.unwrap().build();

    assert_eq!(mint.len(), 1);
    assert_mint_asset(&mint, &policy_id);

    assert_eq!(mint_scripts.len(), 1);
    assert_eq!(mint_scripts.get(0), mint_script);
}

#[test]
fn set_mint_asset_with_existing_mint() {
    let mut tx_builder = create_default_tx_builder();

    let (mint_script1, policy_id1) = mint_script_and_policy(0);
    let (mint_script2, policy_id2) = mint_script_and_policy(1);

    tx_builder
        .set_mint(
            &create_mint_with_one_asset(&policy_id1),
            &NativeScripts::from(vec![mint_script1.clone()]),
        )
        .unwrap();

    tx_builder.set_mint_asset(&mint_script2, &create_mint_asset());

    assert!(tx_builder.mint.is_some());
    let mint_scripts = tx_builder.mint.as_ref().unwrap().get_native_scripts();
    assert!(mint_scripts.len() > 0);

    let mint = tx_builder.mint.unwrap().build();

    assert_eq!(mint.len(), 2);
    assert_mint_asset(&mint, &policy_id1);
    assert_mint_asset(&mint, &policy_id2);

    // Only second script is present in the scripts
    assert_eq!(mint_scripts.len(), 2);
    let actual_scripts = mint_scripts
        .0
        .iter()
        .cloned()
        .collect::<BTreeSet<NativeScript>>();
    let expected_scripts = vec![mint_script1, mint_script2]
        .iter()
        .cloned()
        .collect::<BTreeSet<NativeScript>>();
    assert_eq!(actual_scripts, expected_scripts);
}

#[test]
fn add_mint_asset_with_empty_mint() {
    let mut tx_builder = create_default_tx_builder();

    let (mint_script, policy_id) = mint_script_and_policy(0);

    tx_builder.add_mint_asset(&mint_script, &create_asset_name(), &Int::new_i32(1234));

    assert!(tx_builder.mint.is_some());
    let mint_scripts = tx_builder.mint.as_ref().unwrap().get_native_scripts();
    assert!(mint_scripts.len() > 0);

    let mint = tx_builder.mint.unwrap().build();

    assert_eq!(mint.len(), 1);
    assert_mint_asset(&mint, &policy_id);

    assert_eq!(mint_scripts.len(), 1);
    assert_eq!(mint_scripts.get(0), mint_script);
}

#[test]
fn add_mint_asset_with_existing_mint() {
    let mut tx_builder = create_default_tx_builder();

    let (mint_script1, policy_id1) = mint_script_and_policy(0);
    let (mint_script2, policy_id2) = mint_script_and_policy(1);

    tx_builder
        .set_mint(
            &create_mint_with_one_asset(&policy_id1),
            &NativeScripts::from(vec![mint_script1.clone()]),
        )
        .unwrap();
    tx_builder.add_mint_asset(&mint_script2, &create_asset_name(), &Int::new_i32(1234));

    assert!(tx_builder.mint.is_some());
    let mint_scripts = tx_builder.mint.as_ref().unwrap().get_native_scripts();
    assert!(mint_scripts.len() > 0);

    let mint = tx_builder.mint.unwrap().build();

    assert_eq!(mint.len(), 2);
    assert_mint_asset(&mint, &policy_id1);
    assert_mint_asset(&mint, &policy_id2);

    assert_eq!(mint_scripts.len(), 2);
    let actual_scripts = mint_scripts
        .0
        .iter()
        .cloned()
        .collect::<BTreeSet<NativeScript>>();
    let expected_scripts = vec![mint_script1, mint_script2]
        .iter()
        .cloned()
        .collect::<BTreeSet<NativeScript>>();
    assert_eq!(actual_scripts, expected_scripts);
}

#[test]
fn add_output_amount() {
    let mut tx_builder = create_default_tx_builder();

    let policy_id1 = PolicyID::from([0u8; 28]);
    let multiasset = create_multiasset_one_asset(&policy_id1);
    let mut value = Value::new(&BigNum(249));
    value.set_multiasset(&multiasset);

    let address = byron_address();
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&address)
                .next()
                .unwrap()
                .with_value(&value)
                .build()
                .unwrap(),
        )
        .unwrap();

    assert_eq!(tx_builder.outputs.len(), 1);
    let out = tx_builder.outputs.get(0);

    assert_eq!(out.address.to_bytes(), address.to_bytes());
    assert_eq!(out.amount, value);
}

#[test]
fn add_output_coin() {
    let mut tx_builder = create_default_tx_builder();

    let address = byron_address();
    let coin = BigNum(208);
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&address)
                .next()
                .unwrap()
                .with_coin(&coin)
                .build()
                .unwrap(),
        )
        .unwrap();

    assert_eq!(tx_builder.outputs.len(), 1);
    let out = tx_builder.outputs.get(0);

    assert_eq!(out.address.to_bytes(), address.to_bytes());
    assert_eq!(out.amount.coin, coin);
    assert!(out.amount.multiasset.is_none());
}

#[test]
fn add_output_coin_and_multiasset() {
    let mut tx_builder = create_default_tx_builder();

    let policy_id1 = PolicyID::from([0u8; 28]);
    let multiasset = create_multiasset_one_asset(&policy_id1);

    let address = byron_address();
    let coin = BigNum(249);

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&address)
                .next()
                .unwrap()
                .with_coin_and_asset(&coin, &multiasset)
                .build()
                .unwrap(),
        )
        .unwrap();

    assert_eq!(tx_builder.outputs.len(), 1);
    let out = tx_builder.outputs.get(0);

    assert_eq!(out.address.to_bytes(), address.to_bytes());
    assert_eq!(out.amount.coin, coin);
    assert_eq!(out.amount.multiasset.unwrap(), multiasset);
}

#[test]
fn add_output_asset_and_min_required_coin() {
    let mut tx_builder = create_reallistic_tx_builder();

    let policy_id1 = PolicyID::from([0u8; 28]);
    let multiasset = create_multiasset_one_asset(&policy_id1);

    let address = byron_address();

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&address)
                .next()
                .unwrap()
                .with_asset_and_min_required_coin_by_utxo_cost(
                    &multiasset,
                    &tx_builder.config.utxo_cost(),
                )
                .unwrap()
                .build()
                .unwrap(),
        )
        .unwrap();

    assert_eq!(tx_builder.outputs.len(), 1);
    let out = tx_builder.outputs.get(0);

    assert_eq!(out.address.to_bytes(), address.to_bytes());
    assert_eq!(out.amount.multiasset.unwrap(), multiasset);
    assert_eq!(out.amount.coin, BigNum(1146460));
}

#[test]
fn add_mint_asset_and_output() {
    let mut tx_builder = create_default_tx_builder();

    let (mint_script0, policy_id0) = mint_script_and_policy(0);
    let (mint_script1, policy_id1) = mint_script_and_policy(1);

    let name = create_asset_name();
    let amount = Int::new_i32(1234);

    let address = byron_address();
    let coin = BigNum(249);

    // Add unrelated mint first to check it is NOT added to output later
    tx_builder.add_mint_asset(&mint_script0, &name, &amount.clone());

    tx_builder
        .add_mint_asset_and_output(
            &mint_script1,
            &name,
            &amount,
            &TransactionOutputBuilder::new()
                .with_address(&address)
                .next()
                .unwrap(),
            &coin,
        )
        .unwrap();

    assert!(tx_builder.mint.is_some());
    let mint_scripts = &tx_builder.mint.as_ref().unwrap().get_native_scripts();
    assert!(mint_scripts.len() > 0);

    let mint = &tx_builder.mint.unwrap().build();

    // Mint contains two entries
    assert_eq!(mint.len(), 2);
    assert_mint_asset(mint, &policy_id0);
    assert_mint_asset(mint, &policy_id1);

    assert_eq!(mint_scripts.len(), 2);
    let actual_scripts = mint_scripts
        .0
        .iter()
        .cloned()
        .collect::<BTreeSet<NativeScript>>();
    let expected_scripts = vec![mint_script0, mint_script1]
        .iter()
        .cloned()
        .collect::<BTreeSet<NativeScript>>();
    assert_eq!(actual_scripts, expected_scripts);

    // One new output is created
    assert_eq!(tx_builder.outputs.len(), 1);
    let out = tx_builder.outputs.get(0);

    assert_eq!(out.address.to_bytes(), address.to_bytes());
    assert_eq!(out.amount.coin, coin);

    let multiasset = out.amount.multiasset.unwrap();

    // Only second mint entry was added to the output
    assert_eq!(multiasset.len(), 1);
    assert!(multiasset.get(&policy_id0).is_none());
    assert!(multiasset.get(&policy_id1).is_some());

    let asset = multiasset.get(&policy_id1).unwrap();
    assert_eq!(asset.len(), 1);
    assert_eq!(asset.get(&name).unwrap(), BigNum(1234));
}

#[test]
fn add_mint_asset_and_min_required_coin() {
    let mut tx_builder = create_reallistic_tx_builder();

    let (mint_script0, policy_id0) = mint_script_and_policy(0);
    let (mint_script1, policy_id1) = mint_script_and_policy(1);

    let name = create_asset_name();
    let amount = Int::new_i32(1234);

    let address = byron_address();

    // Add unrelated mint first to check it is NOT added to output later
    tx_builder.add_mint_asset(&mint_script0, &name, &amount);

    tx_builder
        .add_mint_asset_and_output_min_required_coin(
            &mint_script1,
            &name,
            &amount,
            &TransactionOutputBuilder::new()
                .with_address(&address)
                .next()
                .unwrap(),
        )
        .unwrap();

    assert!(tx_builder.mint.is_some());
    let mint_scripts = tx_builder.mint.as_ref().unwrap().get_native_scripts();
    assert!(mint_scripts.len() > 0);

    let mint = &tx_builder.mint.unwrap().build();

    // Mint contains two entries
    assert_eq!(mint.len(), 2);
    assert_mint_asset(mint, &policy_id0);
    assert_mint_asset(mint, &policy_id1);

    assert_eq!(mint_scripts.len(), 2);
    let actual_scripts = mint_scripts
        .0
        .iter()
        .cloned()
        .collect::<BTreeSet<NativeScript>>();
    let expected_scripts = vec![mint_script0, mint_script1]
        .iter()
        .cloned()
        .collect::<BTreeSet<NativeScript>>();
    assert_eq!(actual_scripts, expected_scripts);

    // One new output is created
    assert_eq!(tx_builder.outputs.len(), 1);
    let out = tx_builder.outputs.get(0);

    assert_eq!(out.address.to_bytes(), address.to_bytes());
    assert_eq!(out.amount.coin, BigNum(1146460));

    let multiasset = out.amount.multiasset.unwrap();

    // Only second mint entry was added to the output
    assert_eq!(multiasset.len(), 1);
    assert!(multiasset.get(&policy_id0).is_none());
    assert!(multiasset.get(&policy_id1).is_some());

    let asset = multiasset.get(&policy_id1).unwrap();
    assert_eq!(asset.len(), 1);
    assert_eq!(asset.get(&name).unwrap(), BigNum(1234));
}

#[test]
fn add_mint_includes_witnesses_into_fee_estimation() {
    let mut tx_builder = create_reallistic_tx_builder();

    let hash0 = fake_key_hash(0);

    let (mint_script1, _, hash1) = mint_script_and_policy_and_hash(1);
    let (mint_script2, _, _) = mint_script_and_policy_and_hash(2);
    let (mint_script3, _, _) = mint_script_and_policy_and_hash(3);

    let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
    let name2 = AssetName::new(vec![1u8, 1, 2, 3]).unwrap();
    let name3 = AssetName::new(vec![2u8, 1, 2, 3]).unwrap();
    let name4 = AssetName::new(vec![3u8, 1, 2, 3]).unwrap();
    let amount = Int::new_i32(1234);

    // One input from unrelated address
    tx_builder.add_key_input(
        &hash0,
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(10_000_000)),
    );

    // One input from same address as mint
    tx_builder.add_key_input(
        &hash1,
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&BigNum(10_000_000)),
    );

    // Original tx fee now assumes two VKey signatures for two inputs
    let original_tx_fee = tx_builder.min_fee().unwrap();
    assert_eq!(original_tx_fee, BigNum(168361));

    // Add minting four assets from three different policies
    tx_builder.add_mint_asset(&mint_script1, &name1, &amount);
    tx_builder.add_mint_asset(&mint_script2, &name2, &amount);
    tx_builder.add_mint_asset(&mint_script3, &name3, &amount);
    tx_builder.add_mint_asset(&mint_script3, &name4, &amount);

    let mint = tx_builder.get_mint().unwrap();
    let mint_len = mint.to_bytes().len();

    let mint_scripts = tx_builder.get_witness_set();
    let mint_scripts_len =
        mint_scripts.to_bytes().len() - TransactionWitnessSet::new().to_bytes().len();

    let fee_coefficient = tx_builder.config.fee_algo.coefficient();

    let raw_mint_fee = fee_coefficient
        .checked_mul(&BigNum(mint_len as u64))
        .unwrap();

    let raw_mint_script_fee = fee_coefficient
        .checked_mul(&BigNum(mint_scripts_len as u64))
        .unwrap();

    assert_eq!(raw_mint_fee, BigNum(5544));
    assert_eq!(raw_mint_script_fee, BigNum(4312));

    let new_tx_fee = tx_builder.min_fee().unwrap();

    let fee_diff_from_adding_mint = new_tx_fee.checked_sub(&original_tx_fee).unwrap();

    let witness_fee_increase = fee_diff_from_adding_mint
        .checked_sub(&raw_mint_fee)
        .unwrap()
        .checked_sub(&raw_mint_script_fee)
        .unwrap();

    assert_eq!(witness_fee_increase, BigNum(8932));

    let fee_increase_bytes = u64::from(&witness_fee_increase)
        .checked_div(u64::from(&fee_coefficient))
        .unwrap();

    // Two vkey witnesses 96 bytes each (32 byte pubkey + 64 byte signature)
    // Plus 11 bytes overhead for CBOR wrappers
    // This is happening because we have three different minting policies
    // but the same key-hash from one of them is already also used in inputs
    // so no suplicate witness signature is require for that one
    assert_eq!(fee_increase_bytes, 203);
}

#[test]
fn fee_estimation_fails_on_missing_mint_scripts() {
    let mut tx_builder = create_reallistic_tx_builder();

    // No error estimating fee without mint
    assert!(tx_builder.min_fee().is_ok());

    let (mint_script1, policy_id1) = mint_script_and_policy(0);
    let (mint_script2, _) = mint_script_and_policy(1);

    let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
    let amount = Int::new_i32(1234);

    let mut mint = Mint::new();
    mint.insert(
        &policy_id1,
        &MintAssets::new_from_entry(&name1, &amount.clone()).unwrap(),
    );

    tx_builder
        .set_mint(&mint, &NativeScripts::from(vec![mint_script1]))
        .unwrap();

    let est1 = tx_builder.min_fee();
    assert!(est1.is_ok());

    tx_builder.add_mint_asset(&mint_script2, &name1, &amount);

    let est2 = tx_builder.min_fee();
    assert!(est2.is_ok());

    // Native script assertion has been commented out in `.min_fee`
    // Until implemented in a more performant manner
    // TODO: these test parts might be returned back when it's done

    // // Remove one mint script
    // tx_builder.mint_scripts =
    //     Some(NativeScripts::from(vec![tx_builder.mint_scripts.unwrap().get(1)]));
    //
    // // Now two different policies are minted but only one witness script is present
    // let est3 = tx_builder.min_fee();
    // assert!(est3.is_err());
    // assert!(est3.err().unwrap().to_string().contains(&format!("{:?}", hex::encode(policy_id1.to_bytes()))));
    //
    // // Remove all mint scripts
    // tx_builder.mint_scripts = Some(NativeScripts::new());
    //
    // // Mint exists but no witness scripts at all present
    // let est4 = tx_builder.min_fee();
    // assert!(est4.is_err());
    // assert!(est4.err().unwrap().to_string().contains("witness scripts are not provided"));
    //
    // // Remove all mint scripts
    // tx_builder.mint_scripts = None;
    //
    // // Mint exists but no witness scripts at all present
    // let est5 = tx_builder.min_fee();
    // assert!(est5.is_err());
    // assert!(est5.err().unwrap().to_string().contains("witness scripts are not provided"));
}

#[test]
fn total_input_output_with_mint_and_burn() {
    let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();

    let (mint_script1, policy_id1) = mint_script_and_policy(0);
    let (mint_script2, policy_id2) = mint_script_and_policy(1);

    let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

    let ma_input1 = 100;
    let ma_input2 = 200;
    let ma_output1 = 60;

    let multiassets = [ma_input1, ma_input2, ma_output1]
        .iter()
        .map(|input| {
            let mut multiasset = MultiAsset::new();
            multiasset.insert(&policy_id1, &{
                let mut assets = Assets::new();
                assets.insert(&name, &BigNum(*input));
                assets
            });
            multiasset.insert(&policy_id2, &{
                let mut assets = Assets::new();
                assets.insert(&name, &BigNum(*input));
                assets
            });
            multiasset
        })
        .collect::<Vec<MultiAsset>>();

    for (i, (multiasset, ada)) in multiassets
        .iter()
        .zip([100u64, 100, 100].iter().cloned().map(BigNum::from))
        .enumerate()
    {
        let mut input_amount = Value::new(&ada);
        input_amount.set_multiasset(multiasset);

        tx_builder.add_key_input(
            &&spend.to_raw_key().hash(),
            &TransactionInput::new(&genesis_id(), i as u32),
            &input_amount,
        );
    }

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&byron_address())
                .next()
                .unwrap()
                .with_coin(&BigNum(208))
                .build()
                .unwrap(),
        )
        .unwrap();

    let total_input_before_mint = tx_builder.get_total_input().unwrap();
    let total_output_before_mint = tx_builder.get_total_output().unwrap();

    assert_eq!(total_input_before_mint.coin, BigNum(300));
    assert_eq!(total_output_before_mint.coin, BigNum(208));
    let ma1_input = total_input_before_mint.multiasset.unwrap();
    let ma1_output = total_output_before_mint.multiasset;
    assert_eq!(
        ma1_input.get(&policy_id1).unwrap().get(&name).unwrap(),
        BigNum(360)
    );
    assert_eq!(
        ma1_input.get(&policy_id2).unwrap().get(&name).unwrap(),
        BigNum(360)
    );
    assert!(ma1_output.is_none());

    // Adding mint
    tx_builder.add_mint_asset(&mint_script1, &name, &Int::new_i32(40));

    // Adding burn
    tx_builder.add_mint_asset(&mint_script2, &name, &Int::new_i32(-40));

    let total_input_after_mint = tx_builder.get_total_input().unwrap();
    let total_output_after_mint = tx_builder.get_total_output().unwrap();

    assert_eq!(total_input_after_mint.coin, BigNum(300));
    assert_eq!(total_output_before_mint.coin, BigNum(208));
    let ma2_input = total_input_after_mint.multiasset.unwrap();
    let ma2_output = total_output_after_mint.multiasset.unwrap();
    assert_eq!(
        ma2_input.get(&policy_id1).unwrap().get(&name).unwrap(),
        BigNum(400)
    );
    assert_eq!(
        ma2_input.get(&policy_id2).unwrap().get(&name).unwrap(),
        BigNum(360)
    );
    assert_eq!(
        ma2_output.get(&policy_id2).unwrap().get(&name).unwrap(),
        BigNum(40)
    );
}

fn create_base_address_from_script_hash(sh: &ScriptHash) -> Address {
    BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &Credential::from_scripthash(sh),
        &Credential::from_keyhash(&fake_key_hash(0)),
    )
    .to_address()
}

#[test]
fn test_add_native_script_input() {
    let mut tx_builder = create_reallistic_tx_builder();
    let (script1, _) = mint_script_and_policy(0);
    let (script2, _) = mint_script_and_policy(1);

    // Adding two script inputs using script1 and script2 hashes
    tx_builder.add_native_script_input(
        &script1,
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder.add_native_script_input(
        &script2,
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );

    assert_eq!(
        tx_builder.inputs.get_native_input_scripts().unwrap().len(),
        2
    );
}

fn unsafe_tx_len(b: &TransactionBuilder) -> usize {
    b.build_tx_unsafe().unwrap().to_bytes().len()
}

#[test]
fn test_native_input_scripts_are_added_to_the_witnesses() {
    let mut tx_builder = create_reallistic_tx_builder();
    let (script1, _) = mint_script_and_policy(0);
    let (script2, _) = mint_script_and_policy(1);
    tx_builder.set_fee(&BigNum(42));

    tx_builder.add_native_script_input(
        &script1,
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder.add_native_script_input(
        &script2,
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&BigNum(1_000_000)),
    );

    let tx: Transaction = tx_builder.build_tx_unsafe().unwrap();
    assert!(tx.witness_set.native_scripts.is_some());
    let native_scripts = tx.witness_set.native_scripts.unwrap();
    assert_eq!(native_scripts.len(), 2);
    assert_eq!(native_scripts.get(0), script1);
    assert_eq!(native_scripts.get(1), script2);
}

#[test]
fn test_adding_plutus_script_input() {
    let mut tx_builder = create_reallistic_tx_builder();
    let (script1, _) = fake_plutus_script_and_hash(0);
    let datum = PlutusData::new_bytes(fake_bytes_32(1));
    let redeemer_datum = PlutusData::new_bytes(fake_bytes_32(2));
    let redeemer = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &redeemer_datum,
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum, &redeemer),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder.set_fee(&BigNum(42));
    // There are no missing script witnesses
    let tx: Transaction = tx_builder.build_tx_unsafe().unwrap();
    assert!(tx.witness_set.plutus_scripts.is_some());
    assert_eq!(tx.witness_set.plutus_scripts.unwrap().get(0), script1);
    assert!(tx.witness_set.plutus_data.is_some());
    assert_eq!(tx.witness_set.plutus_data.unwrap().get(0), datum);
    assert!(tx.witness_set.redeemers.is_some());
    assert_eq!(tx.witness_set.redeemers.unwrap().get(0), redeemer);
}

#[test]
fn test_adding_plutus_script_witnesses() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(42));
    let (script1, _) = fake_plutus_script_and_hash(0);
    let (script2, _) = fake_plutus_script_and_hash(1);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &PlutusData::new_bytes(fake_bytes_32(20)),
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(1),
        &PlutusData::new_bytes(fake_bytes_32(21)),
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum1, &redeemer1),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script2, &datum2, &redeemer2),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&BigNum(1_000_000)),
    );

    let tx: Transaction = tx_builder.build_tx_unsafe().unwrap();
    // Check there are two correct scripts
    assert!(tx.witness_set.plutus_scripts.is_some());
    let pscripts = tx.witness_set.plutus_scripts.unwrap();
    assert_eq!(pscripts.len(), 2);
    assert_eq!(pscripts.get(0), script1);
    assert_eq!(pscripts.get(1), script2);
    // Check there are two correct datums
    assert!(tx.witness_set.plutus_data.is_some());
    let datums = tx.witness_set.plutus_data.unwrap();
    assert_eq!(datums.len(), 2);
    assert_eq!(datums.get(0), datum1);
    assert_eq!(datums.get(1), datum2);
    // Check there are two correct redeemers
    assert!(tx.witness_set.redeemers.is_some());
    let redeems = tx.witness_set.redeemers.unwrap();
    assert_eq!(redeems.len(), 2);
    assert_eq!(redeems.get(0), redeemer1);
    assert_eq!(redeems.get(1), redeemer2);
}

fn create_collateral() -> TxInputsBuilder {
    let mut collateral_builder = TxInputsBuilder::new();
    collateral_builder
        .add_regular_input(
            &byron_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&BigNum(1_000_000)),
        )
        .unwrap();
    collateral_builder
}

#[test]
fn test_existing_plutus_scripts_require_data_hash() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(42));
    tx_builder.set_collateral(&create_collateral());
    let (script1, _) = fake_plutus_script_and_hash(0);
    let datum = PlutusData::new_bytes(fake_bytes_32(1));
    let redeemer_datum = PlutusData::new_bytes(fake_bytes_32(2));
    let redeemer = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &redeemer_datum,
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum, &redeemer),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );

    // Using SAFE `.build_tx`
    let res = tx_builder.build_tx();
    assert!(res.is_err());
    if let Err(e) = res {
        assert!(e.as_string().unwrap().contains("script data hash"));
    }

    // Setting script data hash removes the error
    tx_builder.set_script_data_hash(&ScriptDataHash::from_bytes(fake_bytes_32(42)).unwrap());
    // Using SAFE `.build_tx`
    let res2 = tx_builder.build_tx();
    assert!(res2.is_ok());

    // Removing script data hash will cause error again
    tx_builder.remove_script_data_hash();
    // Using SAFE `.build_tx`
    let res3 = tx_builder.build_tx();
    assert!(res3.is_err());
}

#[test]
fn test_calc_script_hash_data() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(42));
    tx_builder.set_collateral(&create_collateral());

    let (script1, _) = fake_plutus_script_and_hash(0);
    let datum = PlutusData::new_bytes(fake_bytes_32(1));
    let redeemer_datum = PlutusData::new_bytes(fake_bytes_32(2));
    let redeemer = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &redeemer_datum,
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum, &redeemer),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );

    // Setting script data hash removes the error
    tx_builder
        .calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models())
        .unwrap();

    // Using SAFE `.build_tx`
    let res2 = tx_builder.build_tx();
    assert!(res2.is_ok());

    let mut used_langs = Languages::new();
    used_langs.add(Language::new_plutus_v1());

    let data_hash = hash_script_data(
        &Redeemers::from(vec![redeemer.clone()]),
        &TxBuilderConstants::plutus_default_cost_models().retain_language_versions(&used_langs),
        Some(PlutusList::from(vec![datum])),
    );
    assert_eq!(tx_builder.script_data_hash.unwrap(), data_hash);
}

#[test]
fn test_plutus_witness_redeemer_index_auto_changing() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(42));
    tx_builder.set_collateral(&create_collateral());
    let (script1, _) = fake_plutus_script_and_hash(0);
    let (script2, _) = fake_plutus_script_and_hash(1);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));

    // Creating redeemers with indexes ZERO
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &PlutusData::new_bytes(fake_bytes_32(20)),
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &PlutusData::new_bytes(fake_bytes_32(21)),
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );

    // Add a regular NON-script input first
    tx_builder.add_regular_input(
        &byron_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );

    // Adding two plutus inputs then
    // both have redeemers with index ZERO
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum1, &redeemer1),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&BigNum(1_000_000)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script2, &datum2, &redeemer2),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&BigNum(1_000_000)),
    );

    // Calc the script data hash
    tx_builder
        .calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models())
        .unwrap();

    let tx: Transaction = tx_builder.build_tx().unwrap();
    assert!(tx.witness_set.redeemers.is_some());
    let redeems = tx.witness_set.redeemers.unwrap();
    assert_eq!(redeems.len(), 2);

    fn compare_redeems(r1: Redeemer, r2: Redeemer) {
        assert_eq!(r1.tag(), r2.tag());
        assert_eq!(r1.data(), r2.data());
        assert_eq!(r1.ex_units(), r2.ex_units());
    }

    compare_redeems(redeems.get(0), redeemer1);
    compare_redeems(redeems.get(1), redeemer2);

    // Note the redeemers from the result transaction are equal with source redeemers
    // In everything EXCEPT the index field, the indexes have changed to 1 and 2
    // To match the position of their corresponding input
    assert_eq!(redeems.get(0).index(), BigNum(1));
    assert_eq!(redeems.get(1).index(), BigNum(2));
}

#[test]
fn test_native_and_plutus_scripts_together() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(42));
    tx_builder.set_collateral(&create_collateral());
    let (pscript1, _) = fake_plutus_script_and_hash(0);
    let (pscript2, phash2) = fake_plutus_script_and_hash(1);
    let (nscript1, _) = mint_script_and_policy(0);
    let (nscript2, nhash2) = mint_script_and_policy(1);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
    // Creating redeemers with indexes ZERO
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &PlutusData::new_bytes(fake_bytes_32(20)),
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &PlutusData::new_bytes(fake_bytes_32(21)),
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );

    // Add one plutus input directly with witness
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    // Add one native input directly with witness
    tx_builder.add_native_script_input(
        &nscript1,
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&BigNum(1_000_000)),
    );
    // Add one plutus input generically without witness
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript2, &datum2, &redeemer2),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&BigNum(1_000_000)),
    );
    // Add one native input generically without witness
    tx_builder.add_native_script_input(
        &nscript2,
        &TransactionInput::new(&genesis_id(), 3),
        &Value::new(&BigNum(1_000_000)),
    );

    tx_builder
        .calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models())
        .unwrap();

    let tx: Transaction = tx_builder.build_tx().unwrap();

    let wits = tx.witness_set;
    assert!(wits.native_scripts.is_some());
    assert!(wits.plutus_scripts.is_some());
    assert!(wits.plutus_data.is_some());
    assert!(wits.redeemers.is_some());

    let nscripts = wits.native_scripts.unwrap();
    assert_eq!(nscripts.len(), 2);
    assert_eq!(nscripts.get(0), nscript1);
    assert_eq!(nscripts.get(1), nscript2);

    let pscripts = wits.plutus_scripts.unwrap();
    assert_eq!(pscripts.len(), 2);
    assert_eq!(pscripts.get(0), pscript1);
    assert_eq!(pscripts.get(1), pscript2);

    let datums = wits.plutus_data.unwrap();
    assert_eq!(datums.len(), 2);
    assert_eq!(datums.get(0), datum1);
    assert_eq!(datums.get(1), datum2);

    let redeems = wits.redeemers.unwrap();
    assert_eq!(redeems.len(), 2);
    assert_eq!(redeems.get(0), redeemer1);

    // The second plutus input redeemer index has automatically changed to 2
    // because it was added on the third position
    assert_eq!(redeems.get(1), redeemer2.clone_with_index(&BigNum(2)));
}

#[test]
fn test_json_serialization_native_and_plutus_scripts_together() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(42));
    tx_builder.set_collateral(&create_collateral());
    let (pscript1, _) = fake_plutus_script_and_hash(0);
    let (pscript2, phash2) = fake_plutus_script_and_hash(1);
    let (nscript1, _) = mint_script_and_policy(0);
    let (nscript2, nhash2) = mint_script_and_policy(1);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
    // Creating redeemers with indexes ZERO
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &PlutusData::new_bytes(fake_bytes_32(20)),
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &PlutusData::new_bytes(fake_bytes_32(21)),
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );

    // Add one plutus input directly with witness
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    // Add one native input directly with witness
    tx_builder.add_native_script_input(
        &nscript1,
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&BigNum(1_000_000)),
    );
    // Add one plutus input generically without witness
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript2, &datum2, &redeemer2),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&BigNum(1_000_000)),
    );
    // Add one native input generically without witness
    tx_builder.add_native_script_input(
        &nscript2,
        &TransactionInput::new(&genesis_id(), 3),
        &Value::new(&BigNum(1_000_000)),
    );

    tx_builder.calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models());

    let tx: Transaction = tx_builder.build_tx().unwrap();

    let json_tx = tx.to_json().unwrap();
    let deser_tx = Transaction::from_json(json_tx.as_str()).unwrap();

    assert_eq!(deser_tx.to_bytes(), tx.to_bytes());
    assert_eq!(deser_tx.to_json().unwrap(), tx.to_json().unwrap());
}

#[test]
fn test_regular_and_collateral_inputs_same_keyhash() {
    let mut input_builder = TxInputsBuilder::new();
    let mut collateral_builder = TxInputsBuilder::new();

    // Add a single input of both kinds with the SAME keyhash
    input_builder.add_regular_input(
        &fake_base_address(0),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    collateral_builder.add_regular_input(
        &fake_base_address(0),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );

    fn get_fake_vkeys_count(i: &TxInputsBuilder, c: &TxInputsBuilder) -> usize {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&BigNum(42));
        tx_builder.set_inputs(i);
        tx_builder.set_collateral(c);
        let tx: Transaction = fake_full_tx(&tx_builder, tx_builder.build().unwrap()).unwrap();
        tx.witness_set.vkeys.unwrap().len()
    }

    // There's only one fake witness in the builder
    // because a regular and a collateral inputs both use the same keyhash
    assert_eq!(get_fake_vkeys_count(&input_builder, &collateral_builder), 1);

    // Add a new input of each kind with DIFFERENT keyhashes
    input_builder.add_regular_input(
        &fake_base_address(1),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    collateral_builder.add_regular_input(
        &fake_base_address(2),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );

    // There are now three fake witnesses in the builder
    // because all three unique keyhashes got combined
    assert_eq!(get_fake_vkeys_count(&input_builder, &collateral_builder), 3);
}

#[test]
fn test_regular_and_collateral_inputs_together() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(42));
    let (pscript1, _) = fake_plutus_script_and_hash(0);
    let (pscript2, _) = fake_plutus_script_and_hash(1);
    let (nscript1, _) = mint_script_and_policy(0);
    let (nscript2, _) = mint_script_and_policy(1);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
    // Creating redeemers with indexes ZERO
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &PlutusData::new_bytes(fake_bytes_32(20)),
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &PlutusData::new_bytes(fake_bytes_32(21)),
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );

    let mut input_builder = TxInputsBuilder::new();
    let mut collateral_builder = TxInputsBuilder::new();

    input_builder.add_native_script_input(
        &NativeScriptSource::new(&nscript1),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );
    collateral_builder.add_native_script_input(
        &NativeScriptSource::new(&nscript2),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&BigNum(1_000_000)),
    );

    input_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&BigNum(1_000_000)),
    );
    collateral_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript2, &datum2, &redeemer2),
        &TransactionInput::new(&genesis_id(), 3),
        &Value::new(&BigNum(1_000_000)),
    );

    tx_builder.set_inputs(&input_builder);
    tx_builder.set_collateral(&collateral_builder);

    tx_builder
        .calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models())
        .unwrap();

    let w: &TransactionWitnessSet = &tx_builder.build_tx().unwrap().witness_set;

    assert!(w.native_scripts.is_some());
    let nscripts = w.native_scripts.as_ref().unwrap();
    assert_eq!(nscripts.len(), 2);
    assert_eq!(nscripts.get(0), nscript1);
    assert_eq!(nscripts.get(1), nscript2);

    assert!(w.plutus_scripts.is_some());
    let pscripts = w.plutus_scripts.as_ref().unwrap();
    assert_eq!(pscripts.len(), 2);
    assert_eq!(pscripts.get(0), pscript1);
    assert_eq!(pscripts.get(1), pscript2);

    assert!(w.plutus_data.is_some());
    let datums = w.plutus_data.as_ref().unwrap();
    assert_eq!(datums.len(), 2);
    assert_eq!(datums.get(0), datum1);
    assert_eq!(datums.get(1), datum2);

    assert!(w.redeemers.is_some());
    let redeemers = w.redeemers.as_ref().unwrap();
    assert_eq!(redeemers.len(), 2);
    assert_eq!(redeemers.get(0), redeemer1.clone_with_index(&BigNum(1)));
    assert_eq!(redeemers.get(1), redeemer2.clone_with_index(&BigNum(1)));
}

#[test]
fn test_ex_unit_costs_are_added_to_the_fees() {
    fn calc_fee_with_ex_units(mem: u64, step: u64) -> Coin {
        let mut input_builder = TxInputsBuilder::new();
        let mut collateral_builder = TxInputsBuilder::new();

        // Add a single input of both kinds with the SAME keyhash
        input_builder.add_regular_input(
            &fake_base_address(0),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&BigNum(1_000_000)),
        );
        collateral_builder.add_regular_input(
            &fake_base_address(0),
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new(&BigNum(1_000_000)),
        );

        let (pscript1, _) = fake_plutus_script_and_hash(0);
        let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
        let redeemer1 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &BigNum(0),
            &PlutusData::new_bytes(fake_bytes_32(20)),
            &ExUnits::new(&BigNum(mem), &BigNum(step)),
        );
        input_builder.add_plutus_script_input(
            &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
            &TransactionInput::new(&genesis_id(), 2),
            &Value::new(&BigNum(1_000_000)),
        );

        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_inputs(&input_builder);
        tx_builder.set_collateral(&collateral_builder);

        tx_builder
            .add_change_if_needed(&fake_base_address(42))
            .unwrap();

        tx_builder.get_fee_if_set().unwrap()
    }

    assert_eq!(calc_fee_with_ex_units(0, 0), BigNum(173509));
    assert_eq!(calc_fee_with_ex_units(10000, 0), BigNum(174174));
    assert_eq!(calc_fee_with_ex_units(0, 10000000), BigNum(174406));
    assert_eq!(calc_fee_with_ex_units(10000, 10000000), BigNum(175071));
}

#[test]
fn test_script_inputs_ordering() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(42));
    let (nscript1, _) = mint_script_and_policy(0);
    let (pscript1, _) = fake_plutus_script_and_hash(0);
    let (pscript2, _) = fake_plutus_script_and_hash(1);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
    // Creating redeemers with indexes ZERO
    let pdata1 = PlutusData::new_bytes(fake_bytes_32(20));
    let pdata2 = PlutusData::new_bytes(fake_bytes_32(21));
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &pdata1,
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &pdata2,
        &ExUnits::new(&BigNum(1), &BigNum(2)),
    );

    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
        &fake_tx_input2(2, 1),
        &fake_value(),
    );
    tx_builder.add_native_script_input(&nscript1, &fake_tx_input2(1, 0), &fake_value());
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript2, &datum2, &redeemer2),
        &fake_tx_input2(2, 0),
        &fake_value(),
    );

    let tx: Transaction = tx_builder.build_tx_unsafe().unwrap();

    let ins = tx.body.inputs;
    assert_eq!(ins.len(), 3);
    assert_eq!(ins.get(0).transaction_id.0[0], 1);
    assert_eq!(ins.get(1).transaction_id.0[0], 2);
    assert_eq!(ins.get(1).index, 0);
    assert_eq!(ins.get(2).transaction_id.0[0], 2);
    assert_eq!(ins.get(2).index, 1);

    let r: Redeemers = tx.witness_set.redeemers.unwrap();
    assert_eq!(r.len(), 2);

    // Redeemer1 now has the index 2 even tho the input was added first
    assert_eq!(r.get(0).data(), pdata1);
    assert_eq!(r.get(0).index(), BigNum(2));

    // Redeemer1 now has the index 1 even tho the input was added last
    assert_eq!(r.get(1).data(), pdata2);
    assert_eq!(r.get(1).index(), BigNum(1));
}

#[test]
fn test_required_signers() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(42));
    let tx1: TransactionBody = tx_builder.build().unwrap();
    assert!(tx1.required_signers.is_none());

    let s1 = fake_key_hash(1);
    let s2 = fake_key_hash(22);
    let s3 = fake_key_hash(133);

    tx_builder.add_required_signer(&s1);
    tx_builder.add_required_signer(&s3);
    tx_builder.add_required_signer(&s2);

    let tx1: TransactionBody = tx_builder.build().unwrap();
    assert!(tx1.required_signers.is_some());

    let rs: Ed25519KeyHashes = tx1.required_signers.unwrap();
    assert_eq!(rs.len(), 3);
    assert!(rs.contains(&s1));
    assert!(rs.contains(&s2));
    assert!(rs.contains(&s3));
}

#[test]
fn test_required_signers_are_added_to_the_witness_estimate() {
    fn count_fake_witnesses_with_required_signers(keys: &Ed25519KeyHashes) -> usize {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&BigNum(42));
        tx_builder.add_regular_input(
            &fake_base_address(0),
            &TransactionInput::new(&fake_tx_hash(0), 0),
            &Value::new(&BigNum(10_000_000)),
        );

        keys.to_vec().iter().for_each(|k| {
            tx_builder.add_required_signer(k);
        });

        let tx: Transaction = fake_full_tx(&tx_builder, tx_builder.build().unwrap()).unwrap();
        tx.witness_set.vkeys.unwrap().len()
    }

    assert_eq!(
        count_fake_witnesses_with_required_signers(&Ed25519KeyHashes::new(),),
        1
    );

    assert_eq!(
        count_fake_witnesses_with_required_signers(&Ed25519KeyHashes::from_vec(vec![
            fake_key_hash(1)
        ]),),
        2
    );

    assert_eq!(
        count_fake_witnesses_with_required_signers(&Ed25519KeyHashes::from_vec(vec![
            fake_key_hash(1),
            fake_key_hash(2)
        ]),),
        3
    );

    // This case still produces only 3 fake signatures, because the same key is already used in the input address
    assert_eq!(
        count_fake_witnesses_with_required_signers(&Ed25519KeyHashes::from_vec(vec![
            fake_key_hash(1),
            fake_key_hash(2),
            fake_key_hash(0)
        ]),),
        3
    );

    // When a different key is used - 4 fake witnesses are produced
    assert_eq!(
        count_fake_witnesses_with_required_signers(&Ed25519KeyHashes::from_vec(vec![
            fake_key_hash(1),
            fake_key_hash(2),
            fake_key_hash(3)
        ]),),
        4
    );
}

#[test]
fn collateral_return_and_total_collateral_setters() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let mut inp = TxInputsBuilder::new();
    inp.add_regular_input(&fake_base_address(0), &fake_tx_input(0), &fake_value());

    tx_builder.set_inputs(&inp);
    tx_builder.set_collateral(&inp);

    let col_return = TransactionOutput::new(&fake_base_address(1), &fake_value2(123123));
    let col_total = BigNum(234234);

    tx_builder.set_collateral_return(&col_return);
    tx_builder.set_total_collateral(&col_total);

    let tx: Transaction = tx_builder.build_tx_unsafe().unwrap();
    assert!(tx.body.collateral_return.is_some());
    assert_eq!(tx.body.collateral_return.unwrap(), col_return);
    assert!(tx.body.total_collateral.is_some());
    assert_eq!(tx.body.total_collateral.unwrap(), col_total);
}

fn fake_multiasset(amount: u64) -> MultiAsset {
    let (_, policy_id) = mint_script_and_policy(234);
    let mut assets = Assets::new();
    assets.insert(
        &AssetName::new(fake_bytes_32(235)).unwrap(),
        &BigNum(amount),
    );
    let mut masset = MultiAsset::new();
    masset.insert(&policy_id, &assets);
    masset
}

#[test]
fn inputs_builder_total_value() {
    let mut b = TxInputsBuilder::new();
    assert_eq!(b.total_value().unwrap(), Value::zero());

    b.add_regular_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &fake_value2(100_000),
    );
    assert_eq!(b.total_value().unwrap(), Value::new(&BigNum(100_000)));

    b.add_regular_input(
        &fake_base_address(1),
        &fake_tx_input(1),
        &fake_value2(200_000),
    );
    assert_eq!(b.total_value().unwrap(), Value::new(&BigNum(300_000)));

    let masset = fake_multiasset(123);

    b.add_regular_input(
        &fake_base_address(2),
        &fake_tx_input(2),
        &Value::new_with_assets(&BigNum(300_000), &masset),
    );
    assert_eq!(
        b.total_value().unwrap(),
        Value::new_with_assets(&BigNum(600_000), &masset)
    );
}

#[test]
fn test_auto_calc_total_collateral() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_regular_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &fake_value2(collateral_input_value.clone()),
    );

    tx_builder.set_collateral(&inp);

    let collateral_return_value = 1_234_567;
    let col_return = TransactionOutput::new(
        &fake_base_address(1),
        &fake_value2(collateral_return_value.clone()),
    );

    tx_builder
        .set_collateral_return_and_total(&col_return)
        .unwrap();

    assert!(tx_builder.collateral_return.is_some());
    assert_eq!(tx_builder.collateral_return.unwrap(), col_return,);

    assert!(tx_builder.total_collateral.is_some());
    assert_eq!(
        tx_builder.total_collateral.unwrap(),
        BigNum(collateral_input_value - collateral_return_value),
    );
}

#[test]
fn test_auto_calc_total_collateral_with_assets() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let masset = fake_multiasset(123);

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_regular_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new_with_assets(&BigNum(collateral_input_value.clone()), &masset),
    );

    tx_builder.set_collateral(&inp);

    let collateral_return_value = 1_345_678;
    let col_return = TransactionOutput::new(
        &fake_base_address(1),
        &Value::new_with_assets(&BigNum(collateral_return_value.clone()), &masset),
    );

    tx_builder
        .set_collateral_return_and_total(&col_return)
        .unwrap();

    assert!(tx_builder.collateral_return.is_some());
    assert_eq!(tx_builder.collateral_return.unwrap(), col_return,);

    assert!(tx_builder.total_collateral.is_some());
    assert_eq!(
        tx_builder.total_collateral.unwrap(),
        BigNum(collateral_input_value - collateral_return_value),
    );
}

#[test]
fn test_auto_calc_total_collateral_fails_with_assets() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let masset = fake_multiasset(123);

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_regular_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new_with_assets(&BigNum(collateral_input_value.clone()), &masset),
    );

    tx_builder.set_collateral(&inp);

    // Collateral return does not handle ALL the assets from collateral input
    let collateral_return_value = 1_345_678;
    let col_return = TransactionOutput::new(
        &fake_base_address(1),
        &fake_value2(collateral_return_value.clone()),
    );

    let res = tx_builder.set_collateral_return_and_total(&col_return);

    // Function call returns an error
    assert!(res.is_err());

    // NEITHER total collateral nor collateral return are changed in the builder
    assert!(tx_builder.total_collateral.is_none());
    assert!(tx_builder.collateral_return.is_none());
}

#[test]
fn test_auto_calc_total_collateral_fails_on_no_collateral() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let res = tx_builder.set_collateral_return_and_total(&TransactionOutput::new(
        &fake_base_address(1),
        &fake_value2(1_345_678),
    ));

    // Function call returns an error
    assert!(res.is_err());

    // NEITHER total collateral nor collateral return are changed in the builder
    assert!(tx_builder.total_collateral.is_none());
    assert!(tx_builder.collateral_return.is_none());
}

#[test]
fn test_auto_calc_total_collateral_fails_on_no_ada() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_regular_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new(&BigNum(collateral_input_value.clone())),
    );

    tx_builder.set_collateral(&inp);

    let res = tx_builder.set_collateral_return_and_total(&TransactionOutput::new(
        &fake_base_address(1),
        &fake_value2(1),
    ));

    // Function call returns an error
    assert!(res.is_err());

    // NEITHER total collateral nor collateral return are changed in the builder
    assert!(tx_builder.total_collateral.is_none());
    assert!(tx_builder.collateral_return.is_none());
}

#[test]
fn test_auto_calc_collateral_return() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_regular_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &fake_value2(collateral_input_value.clone()),
    );

    tx_builder.set_collateral(&inp);

    let total_collateral_value = 234_567;
    let collateral_return_address = fake_base_address(1);

    tx_builder
        .set_total_collateral_and_return(
            &BigNum(total_collateral_value.clone()),
            &collateral_return_address,
        )
        .unwrap();

    assert!(tx_builder.total_collateral.is_some());
    assert_eq!(
        tx_builder.total_collateral.unwrap(),
        BigNum(total_collateral_value.clone()),
    );

    assert!(tx_builder.collateral_return.is_some());
    let col_return: TransactionOutput = tx_builder.collateral_return.unwrap();
    assert_eq!(col_return.address, collateral_return_address);
    assert_eq!(
        col_return.amount,
        Value::new(&BigNum(collateral_input_value - total_collateral_value),)
    );
}

#[test]
fn test_auto_calc_collateral_return_with_assets() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let masset = fake_multiasset(123);

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_regular_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new_with_assets(&BigNum(collateral_input_value.clone()), &masset),
    );

    tx_builder.set_collateral(&inp);

    let total_collateral_value = 345_678;
    let collateral_return_address = fake_base_address(1);

    tx_builder
        .set_total_collateral_and_return(
            &BigNum(total_collateral_value.clone()),
            &collateral_return_address,
        )
        .unwrap();

    assert!(tx_builder.total_collateral.is_some());
    assert_eq!(
        tx_builder.total_collateral.unwrap(),
        BigNum(total_collateral_value.clone()),
    );

    assert!(tx_builder.collateral_return.is_some());
    let col_return: TransactionOutput = tx_builder.collateral_return.unwrap();
    assert_eq!(col_return.address, collateral_return_address);
    assert_eq!(
        col_return.amount,
        Value::new_with_assets(
            &BigNum(collateral_input_value - total_collateral_value),
            &masset,
        )
    );
}

#[test]
fn test_add_collateral_return_succeed_with_border_amount() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let masset = fake_multiasset(123);

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_regular_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new_with_assets(&BigNum(collateral_input_value.clone()), &masset),
    );

    tx_builder.set_collateral(&inp);

    let collateral_return_address = fake_base_address(1);

    let possible_ret = Value::new_from_assets(&masset);
    let fake_out = TransactionOutput::new(&collateral_return_address, &possible_ret);
    let min_ada = min_ada_for_output(&fake_out, &tx_builder.config.utxo_cost()).unwrap();

    let total_collateral_value = BigNum(collateral_input_value)
        .checked_sub(&min_ada)
        .unwrap();

    tx_builder
        .set_total_collateral_and_return(&total_collateral_value, &collateral_return_address)
        .unwrap();

    assert!(tx_builder.total_collateral.is_some());
    assert!(tx_builder.collateral_return.is_some());
    let col_return: TransactionOutput = tx_builder.collateral_return.unwrap();
    assert_eq!(col_return.address, collateral_return_address);
    assert_eq!(
        col_return.amount,
        Value::new_with_assets(&min_ada, &masset,)
    );
}

#[test]
fn test_add_zero_collateral_return() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_regular_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new(&BigNum(collateral_input_value.clone())),
    );

    tx_builder.set_collateral(&inp);

    let collateral_return_address = fake_base_address(1);

    tx_builder
        .set_total_collateral_and_return(
            &BigNum(collateral_input_value.clone()),
            &collateral_return_address,
        )
        .unwrap();

    assert!(tx_builder.total_collateral.is_some());
    assert!(tx_builder.collateral_return.is_none());
}

#[test]
fn test_add_collateral_return_fails_no_enough_ada() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let masset = fake_multiasset(123);

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_regular_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new_with_assets(&BigNum(collateral_input_value.clone()), &masset),
    );

    tx_builder.set_collateral(&inp);

    let collateral_return_address = fake_base_address(1);

    let possible_ret = Value::new_from_assets(&masset);
    let fake_out = TransactionOutput::new(&collateral_return_address, &possible_ret);
    let min_ada = min_ada_for_output(&fake_out, &tx_builder.config.utxo_cost()).unwrap();
    let mut total_collateral_value = BigNum(collateral_input_value)
        .checked_sub(&min_ada)
        .unwrap();
    //make total collateral value bigger for make collateral return less then min ada
    total_collateral_value = total_collateral_value.checked_add(&BigNum(1)).unwrap();

    let coll_add_res = tx_builder
        .set_total_collateral_and_return(&total_collateral_value, &collateral_return_address);

    assert!(coll_add_res.is_err());
    assert!(tx_builder.total_collateral.is_none());
    assert!(tx_builder.collateral_return.is_none());
}

#[test]
fn test_auto_calc_collateral_return_fails_on_no_collateral() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(123456));

    let res =
        tx_builder.set_total_collateral_and_return(&BigNum(345_678.clone()), &fake_base_address(1));

    assert!(res.is_err());
    assert!(tx_builder.total_collateral.is_none());
    assert!(tx_builder.collateral_return.is_none());
}

#[test]
fn test_costmodel_retaining_for_v1() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(42));
    tx_builder.set_collateral(&create_collateral());

    let (script1, _) = fake_plutus_script_and_hash(0);
    let datum = PlutusData::new_integer(&BigInt::from_str("42").unwrap());
    let redeemer = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &datum,
        &ExUnits::new(&BigNum(1700), &BigNum(368100)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum, &redeemer),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );

    // Setting script data hash removes the error
    tx_builder
        .calc_script_data_hash(&TxBuilderConstants::plutus_vasil_cost_models())
        .unwrap();

    // Using SAFE `.build_tx`
    let res2 = tx_builder.build_tx();
    assert!(res2.is_ok());

    let v1 = Language::new_plutus_v1();
    let v1_costmodel = TxBuilderConstants::plutus_vasil_cost_models()
        .get(&v1)
        .unwrap();
    let mut retained_cost_models = Costmdls::new();
    retained_cost_models.insert(&v1, &v1_costmodel);

    let data_hash = hash_script_data(
        &Redeemers::from(vec![redeemer.clone()]),
        &retained_cost_models,
        Some(PlutusList::from(vec![datum])),
    );
    assert_eq!(tx_builder.script_data_hash.unwrap(), data_hash);
}

#[test]
fn test_costmodel_retaining_fails_on_missing_costmodel() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&BigNum(42));
    tx_builder.set_collateral(&create_collateral());

    let (script1, _) = fake_plutus_script_and_hash(0);
    let datum = PlutusData::new_integer(&BigInt::from_str("42").unwrap());
    let redeemer = Redeemer::new(
        &RedeemerTag::new_spend(),
        &BigNum(0),
        &datum,
        &ExUnits::new(&BigNum(1700), &BigNum(368100)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum, &redeemer),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(1_000_000)),
    );

    let v2 = Language::new_plutus_v2();
    let v2_costmodel = TxBuilderConstants::plutus_vasil_cost_models()
        .get(&v2)
        .unwrap();
    let mut retained_cost_models = Costmdls::new();
    retained_cost_models.insert(&v2, &v2_costmodel);

    // Setting script data hash removes the error
    let calc_result = tx_builder.calc_script_data_hash(&retained_cost_models);
    assert!(calc_result.is_err());
}

#[test]
fn coin_selection_random_improve_multi_asset() {
    let utoxs = TransactionUnspentOutputs::from_json("[ { \"input\": {
  \"transaction_id\": \"96631bf40bc2ae1e10b3c9157a4c711562c664b9744ed1f580b725e0589efcd0\",
  \"index\": 1
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"661308571\",
    \"multiasset\": null
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"89da149fa162eca7212493f2bcc8415ed070832e053ac0ec335d3501f901ad77\",
  \"index\": 1
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"555975153\",
    \"multiasset\": null
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"0124993c20ea0fe626d96a644773225202fb442238c38206242d26a1131e0a6e\",
  \"index\": 1
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"1899495\",
    \"multiasset\": {
      \"07e8df329b724e4be48ee32738125c06000de5448aaf93ed46d59e28\": {
        \"44696e6f436f696e\": \"750\"
      }
    }
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"c15c423d624b3af3f032c079a1b390c472b8ba889b48dd581d0ea28f96a36875\",
  \"index\": 0
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"1804315\",
    \"multiasset\": {
      \"07e8df329b724e4be48ee32738125c06000de5448aaf93ed46d59e28\": {
        \"44696e6f436f696e\": \"2000\"
      }
    }
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"5894bf9c9125859d29770bf43e4018f4f34a69edee49a7c9488c6707ab523c9b\",
  \"index\": 1
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"440573428\",
    \"multiasset\": null
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"168404afd4e9927d7775c8f40c0f749fc7634832d6931c5d51a507724cf44420\",
  \"index\": 0
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"1804315\",
    \"multiasset\": {
      \"07e8df329b724e4be48ee32738125c06000de5448aaf93ed46d59e28\": {
        \"44696e6f436f696e\": \"1000\"
      }
    }
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"3e6138498b721ee609a4c289768b2accad39cd4f00448540a95ba3362578a2f7\",
  \"index\": 4
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"1508500\",
    \"multiasset\": {
      \"07e8df329b724e4be48ee32738125c06000de5448aaf93ed46d59e28\": {
        \"44696e6f436f696e\": \"750\"
      }
    }
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"3e6138498b721ee609a4c289768b2accad39cd4f00448540a95ba3362578a2f7\",
  \"index\": 5
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"664935092\",
    \"multiasset\": null
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"046cf1bc21c23c59975714b520dd7ed22b63dab592cb0449e0ee6cc96eefde69\",
  \"index\": 2
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"7094915\",
    \"multiasset\": null
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"e16f195105db5f84621af4f7ea57c7156b8699cba94d4fdb72a6fb09e31db7a8\",
  \"index\": 1
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"78400000\",
    \"multiasset\": null
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"e16f195105db5f84621af4f7ea57c7156b8699cba94d4fdb72a6fb09e31db7a8\",
  \"index\": 2
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"2000000\",
    \"multiasset\": null
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"006697ef0c9285b7001ebe5a9e356fb50441e0af803773a99b7cbb0e9b728570\",
  \"index\": 1
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"15054830\",
    \"multiasset\": {
      \"07e8df329b724e4be48ee32738125c06000de5448aaf93ed46d59e28\": {
        \"44696e6f436f696e\": \"56250\"
      },
      \"3320679b145d683b9123f0626360699fcd7408b4d3ec3bd9cc79398c\": {
        \"44696e6f436f696e\": \"287000\"
      },
      \"57fca08abbaddee36da742a839f7d83a7e1d2419f1507fcbf3916522\": {
        \"4d494e54\": \"91051638\",
        \"534245525259\": \"27198732\"
      },
      \"e61bfc106338ed4aeba93036324fbea8150fd9750fcffca1cd9f1a19\": {
        \"44696e6f536176696f723030303639\": \"1\",
        \"44696e6f536176696f723030303936\": \"1\",
        \"44696e6f536176696f723030313737\": \"1\",
        \"44696e6f536176696f723030333033\": \"1\",
        \"44696e6f536176696f723030333531\": \"1\",
        \"44696e6f536176696f723030333931\": \"1\",
        \"44696e6f536176696f723030343336\": \"1\",
        \"44696e6f536176696f723030343434\": \"1\",
        \"44696e6f536176696f723030353232\": \"1\",
        \"44696e6f536176696f723030353337\": \"1\",
        \"44696e6f536176696f723030363334\": \"1\",
        \"44696e6f536176696f723030373332\": \"1\",
        \"44696e6f536176696f723030373430\": \"1\",
        \"44696e6f536176696f723030373435\": \"1\",
        \"44696e6f536176696f723031303139\": \"1\",
        \"44696e6f536176696f723031303631\": \"1\",
        \"44696e6f536176696f723031333432\": \"1\",
        \"44696e6f536176696f723031333832\": \"1\",
        \"44696e6f536176696f723031353333\": \"1\",
        \"44696e6f536176696f723031353732\": \"1\",
        \"44696e6f536176696f723031363337\": \"1\",
        \"44696e6f536176696f723031363430\": \"1\",
        \"44696e6f536176696f723031373631\": \"1\",
        \"44696e6f536176696f723031393436\": \"1\",
        \"44696e6f536176696f723032313237\": \"1\",
        \"44696e6f536176696f723032323232\": \"1\",
        \"44696e6f536176696f723032333230\": \"1\",
        \"44696e6f536176696f723032333239\": \"1\",
        \"44696e6f536176696f723032333534\": \"1\",
        \"44696e6f536176696f723032333631\": \"1\",
        \"44696e6f536176696f723032333935\": \"1\",
        \"44696e6f536176696f723032333938\": \"1\",
        \"44696e6f536176696f723032343037\": \"1\",
        \"44696e6f536176696f723032343434\": \"1\",
        \"44696e6f536176696f723032353039\": \"1\",
        \"44696e6f536176696f723032363334\": \"1\",
        \"44696e6f536176696f723032363430\": \"1\",
        \"44696e6f536176696f723032373537\": \"1\",
        \"44696e6f536176696f723032373832\": \"1\",
        \"44696e6f536176696f723032383933\": \"1\",
        \"44696e6f536176696f723033323430\": \"1\",
        \"44696e6f536176696f723033343937\": \"1\",
        \"44696e6f536176696f723033353437\": \"1\",
        \"44696e6f536176696f723033353738\": \"1\",
        \"44696e6f536176696f723033363638\": \"1\",
        \"44696e6f536176696f723033363836\": \"1\",
        \"44696e6f536176696f723033363930\": \"1\",
        \"44696e6f536176696f723033383638\": \"1\",
        \"44696e6f536176696f723033383731\": \"1\",
        \"44696e6f536176696f723033383931\": \"1\",
        \"44696e6f536176696f723034313936\": \"1\",
        \"44696e6f536176696f723034323538\": \"1\",
        \"44696e6f536176696f723034323733\": \"1\",
        \"44696e6f536176696f723034363235\": \"1\",
        \"44696e6f536176696f723034373132\": \"1\",
        \"44696e6f536176696f723034373932\": \"1\",
        \"44696e6f536176696f723034383831\": \"1\",
        \"44696e6f536176696f723034393936\": \"1\",
        \"44696e6f536176696f723035303432\": \"1\",
        \"44696e6f536176696f723035313539\": \"1\",
        \"44696e6f536176696f723035333138\": \"1\",
        \"44696e6f536176696f723035333532\": \"1\",
        \"44696e6f536176696f723035343433\": \"1\",
        \"44696e6f536176696f723035343639\": \"1\",
        \"44696e6f536176696f723035373434\": \"1\",
        \"44696e6f536176696f723035373638\": \"1\",
        \"44696e6f536176696f723035373830\": \"1\",
        \"44696e6f536176696f723035383435\": \"1\",
        \"44696e6f536176696f723035383538\": \"1\",
        \"44696e6f536176696f723035393632\": \"1\",
        \"44696e6f536176696f723036303032\": \"1\",
        \"44696e6f536176696f723036303337\": \"1\",
        \"44696e6f536176696f723036303738\": \"1\",
        \"44696e6f536176696f723036323033\": \"1\",
        \"44696e6f536176696f723036323036\": \"1\",
        \"44696e6f536176696f723036323236\": \"1\",
        \"44696e6f536176696f723036333130\": \"1\",
        \"44696e6f536176696f723036333935\": \"1\",
        \"44696e6f536176696f723036343932\": \"1\",
        \"44696e6f536176696f723036353532\": \"1\",
        \"44696e6f536176696f723036363735\": \"1\",
        \"44696e6f536176696f723036363839\": \"1\",
        \"44696e6f536176696f723036373233\": \"1\",
        \"44696e6f536176696f723036383731\": \"1\",
        \"44696e6f536176696f723036383830\": \"1\",
        \"44696e6f536176696f723036393137\": \"1\",
        \"44696e6f536176696f723037303339\": \"1\",
        \"44696e6f536176696f723037323638\": \"1\",
        \"44696e6f536176696f723037333434\": \"1\",
        \"44696e6f536176696f723037343232\": \"1\",
        \"44696e6f536176696f723037343731\": \"1\",
        \"44696e6f536176696f723037353431\": \"1\",
        \"44696e6f536176696f723037363032\": \"1\",
        \"44696e6f536176696f723037363136\": \"1\",
        \"44696e6f536176696f723037363430\": \"1\",
        \"44696e6f536176696f723037373635\": \"1\",
        \"44696e6f536176696f723037373732\": \"1\",
        \"44696e6f536176696f723037393039\": \"1\",
        \"44696e6f536176696f723037393234\": \"1\",
        \"44696e6f536176696f723037393430\": \"1\",
        \"44696e6f536176696f723037393632\": \"1\",
        \"44696e6f536176696f723038303130\": \"1\",
        \"44696e6f536176696f723038303338\": \"1\",
        \"44696e6f536176696f723038303339\": \"1\",
        \"44696e6f536176696f723038303636\": \"1\",
        \"44696e6f536176696f723038313735\": \"1\",
        \"44696e6f536176696f723038323032\": \"1\",
        \"44696e6f536176696f723038323131\": \"1\",
        \"44696e6f536176696f723038323536\": \"1\",
        \"44696e6f536176696f723038333532\": \"1\",
        \"44696e6f536176696f723038333536\": \"1\",
        \"44696e6f536176696f723038333538\": \"1\",
        \"44696e6f536176696f723038333539\": \"1\",
        \"44696e6f536176696f723038333830\": \"1\",
        \"44696e6f536176696f723038343932\": \"1\",
        \"44696e6f536176696f723038353231\": \"1\",
        \"44696e6f536176696f723038353736\": \"1\",
        \"44696e6f536176696f723038353836\": \"1\",
        \"44696e6f536176696f723038363130\": \"1\",
        \"44696e6f536176696f723039303231\": \"1\",
        \"44696e6f536176696f723039303735\": \"1\",
        \"44696e6f536176696f723039313039\": \"1\",
        \"44696e6f536176696f723039313231\": \"1\",
        \"44696e6f536176696f723039323238\": \"1\",
        \"44696e6f536176696f723039333138\": \"1\",
        \"44696e6f536176696f723039333731\": \"1\",
        \"44696e6f536176696f723039343035\": \"1\",
        \"44696e6f536176696f723039343136\": \"1\",
        \"44696e6f536176696f723039353039\": \"1\",
        \"44696e6f536176696f723039353635\": \"1\",
        \"44696e6f536176696f723039363331\": \"1\",
        \"44696e6f536176696f723039363932\": \"1\",
        \"44696e6f536176696f723039383839\": \"1\",
        \"44696e6f536176696f723039393038\": \"1\",
        \"44696e6f536176696f723039393935\": \"1\"
      },
      \"ee8e37676f6ebb8e031dff493f88ff711d24aa68666a09d61f1d3fb3\": {
        \"43727970746f44696e6f3030303135\": \"1\",
        \"43727970746f44696e6f3030313335\": \"1\",
        \"43727970746f44696e6f3030323634\": \"1\",
        \"43727970746f44696e6f3030333932\": \"1\",
        \"43727970746f44696e6f3030353834\": \"1\",
        \"43727970746f44696e6f3030373136\": \"1\",
        \"43727970746f44696e6f3030373837\": \"1\",
        \"43727970746f44696e6f3030383438\": \"1\",
        \"43727970746f44696e6f3031303537\": \"1\",
        \"43727970746f44696e6f3031313134\": \"1\",
        \"43727970746f44696e6f3031323237\": \"1\",
        \"43727970746f44696e6f3031323330\": \"1\",
        \"43727970746f44696e6f3031343031\": \"1\",
        \"43727970746f44696e6f3031353138\": \"1\",
        \"43727970746f44696e6f3031353734\": \"1\",
        \"43727970746f44696e6f3031373635\": \"1\",
        \"43727970746f44696e6f3031383037\": \"1\",
        \"43727970746f44696e6f3031383231\": \"1\",
        \"43727970746f44696e6f3032303830\": \"1\",
        \"43727970746f44696e6f3032313133\": \"1\",
        \"43727970746f44696e6f3032323835\": \"1\",
        \"43727970746f44696e6f3032343238\": \"1\",
        \"43727970746f44696e6f3032363738\": \"1\",
        \"43727970746f44696e6f3032393034\": \"1\",
        \"43727970746f44696e6f3032393333\": \"1\",
        \"43727970746f44696e6f3032393537\": \"1\",
        \"43727970746f44696e6f3032393632\": \"1\",
        \"43727970746f44696e6f3032393735\": \"1\",
        \"43727970746f44696e6f3033303434\": \"1\",
        \"43727970746f44696e6f3033333338\": \"1\",
        \"43727970746f44696e6f3033393535\": \"1\",
        \"43727970746f44696e6f3034303630\": \"1\",
        \"43727970746f44696e6f3034313939\": \"1\",
        \"43727970746f44696e6f3034373439\": \"1\",
        \"43727970746f44696e6f3034383134\": \"1\",
        \"43727970746f44696e6f3034393530\": \"1\",
        \"43727970746f44696e6f3035303630\": \"1\",
        \"43727970746f44696e6f3035333230\": \"1\",
        \"43727970746f44696e6f2d312d3030303030\": \"1\",
        \"43727970746f44696e6f2d312d3030303032\": \"1\"
      }
    }
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"006697ef0c9285b7001ebe5a9e356fb50441e0af803773a99b7cbb0e9b728570\",
  \"index\": 2
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"2279450\",
    \"multiasset\": null
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"017962634cf8fa87835256a80b8374c6f75687c34d8694480cb071648551c3a7\",
  \"index\": 0
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"2000000\",
    \"multiasset\": {
      \"ee8e37676f6ebb8e031dff493f88ff711d24aa68666a09d61f1d3fb3\": {
        \"43727970746f44696e6f3031353039\": \"1\"
      }
    }
  },
  \"plutus_data\": null,
  \"script_ref\": null
}},
{ \"input\": {
  \"transaction_id\": \"017962634cf8fa87835256a80b8374c6f75687c34d8694480cb071648551c3a7\",
  \"index\": 1
},
\"output\": {
  \"address\": \"addr_test1qp03v9yeg0vcfdhyn65ets2juearxpc3pmdhr0sxs0w6wh3sjf67h3yhrpxpv00zqfc7rtmr6mnmrcplfdkw5zhnl49qmyf0q5\",
  \"amount\": {
    \"coin\": \"725669617\",
    \"multiasset\": null
  },
  \"plutus_data\": null,
  \"script_ref\": null
}}]")
            .unwrap();
    let output = TransactionOutput::from_json(
        "{
  \"address\": \"addr_test1wpv93hm9sqx0ar7pgxwl9jn3xt6lwmxxy27zd932slzvghqg8fe0n\",
  \"amount\": {
    \"coin\": \"20000000\",
    \"multiasset\": {
      \"07e8df329b724e4be48ee32738125c06000de5448aaf93ed46d59e28\": {
        \"44696e6f436f696e\": \"1000\"
      },
      \"ee8e37676f6ebb8e031dff493f88ff711d24aa68666a09d61f1d3fb3\": {
        \"43727970746f44696e6f2d312d3030303030\": \"1\",
        \"43727970746f44696e6f2d312d3030303032\": \"1\"
      }
    }
  },
  \"plutus_data\": {
    \"DataHash\": \"979f68de9e070e75779f80ce5e6cc74f8d77661d65f2895c01d0a6f66eceb791\"
  },
  \"script_ref\": null
}",
    )
    .unwrap();
    let mut builder = create_reallistic_tx_builder();
    builder.add_output(&output).unwrap();
    let res = builder.add_inputs_from(&utoxs, CoinSelectionStrategyCIP2::RandomImproveMultiAsset);
    assert!(res.is_ok());
}

#[test]
fn multiple_plutus_inputs_test() {
    let mut tx_builder = create_reallistic_tx_builder();
    let (plutus_script, _) = fake_plutus_script_and_hash(1);
    let redeemer1 = create_redeemer(1);
    let redeemer2 = create_redeemer(2);

    let mut in_builder = TxInputsBuilder::new();
    let input_1 = TransactionInput::new(
        &TransactionHash::from_bytes(
            hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
                .unwrap(),
        )
        .unwrap(),
        1,
    );
    let input_2 = TransactionInput::new(
        &TransactionHash::from_bytes(
            hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
                .unwrap(),
        )
        .unwrap(),
        2,
    );

    let colateral_adress = Address::from_bech32("addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w").unwrap();
    let colateral_input = TransactionInput::new(
        &TransactionHash::from_bytes(
            hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
                .unwrap(),
        )
        .unwrap(),
        3,
    );

    let output_adress = Address::from_bech32("addr_test1qpm5njmgzf4t7225v6j34wl30xfrufzt3jtqtdzf3en9ahpmnhtmynpasyc8fq75zv0uaj86vzsr7g3g8q5ypgu5fwtqr9zsgj").unwrap();
    let output_value = Value::new(&Coin::from(500000u64));
    let output = TransactionOutput::new(&output_adress, &output_value);

    tx_builder.add_output(&output);
    let mut col_builder = TxInputsBuilder::new();
    col_builder.add_regular_input(
        &colateral_adress,
        &colateral_input,
        &Value::new(&Coin::from(1000000000u64)),
    );
    tx_builder.set_collateral(&col_builder);

    let datum = PlutusData::new_bytes(fake_bytes_32(11));
    let plutus_wit1 = PlutusWitness::new(&plutus_script, &datum, &redeemer1);

    let plutus_wit2 = PlutusWitness::new(&plutus_script, &datum, &redeemer2);

    let value = Value::new(&Coin::from(100000000u64));

    in_builder.add_plutus_script_input(&plutus_wit1, &input_1, &value);
    in_builder.add_plutus_script_input(&plutus_wit2, &input_2, &value);

    tx_builder.set_inputs(&in_builder);
    tx_builder.calc_script_data_hash(&TxBuilderConstants::plutus_vasil_cost_models());
    tx_builder.add_change_if_needed(&output_adress);
    let build_res = tx_builder.build_tx();
    assert!(&build_res.is_ok());
    let tx = build_res.unwrap();
    assert_eq!(tx.witness_set.plutus_scripts.unwrap().len(), 1usize);
    assert_eq!(tx.witness_set.redeemers.unwrap().len(), 2usize);
}

#[test]
fn build_tx_with_certs_withdrawals_plutus_script_address() {
    let mut tx_builder = create_tx_builder_with_key_deposit(1_000_000);
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let change_key = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();
    let reward = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(3)
        .derive(1)
        .to_public();

    let redeemer_cert1 = create_redeemer(1);
    let redeemer_cert2 = create_redeemer(2);
    let redeemer_cert3 = create_redeemer(3);
    let redeemer_withdraw1 = create_redeemer(4);
    let redeemer_withdraw2 = create_redeemer(5);

    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&BigNum(5_000_000)),
    );
    tx_builder.set_ttl(1000);
    let (cert_script1, cert_script_hash1) = fake_plutus_script_and_hash(1);
    let cert_script_cred1 = Credential::from_scripthash(&cert_script_hash1);

    let (cert_script2, cert_script_hash2) = fake_plutus_script_and_hash(2);
    let cert_script_cred2 = Credential::from_scripthash(&cert_script_hash2);

    let cert_script_hash3 = fake_script_hash(3);
    let cert_script_cred3 = Credential::from_scripthash(&cert_script_hash3);

    let (withdraw_script1, withdraw_script_hash1) = fake_plutus_script_and_hash(3);
    let withdraw_script_cred1 = Credential::from_scripthash(&withdraw_script_hash1);

    let withdraw_script_hash2 = fake_script_hash(3);
    let withdraw_script_cred2 = Credential::from_scripthash(&withdraw_script_hash2);

    let cert_witness_1 = PlutusWitness::new_without_datum(&cert_script1, &redeemer_cert1);
    let cert_witness_2 = PlutusWitness::new_without_datum(&cert_script2, &redeemer_cert2);

    let ref_cert_script_input_3 = fake_tx_input(1);
    let ref_cert_withdrawal_input_2 = fake_tx_input(2);
    let plutus_cert_source = PlutusScriptSource::new_ref_input(
        &cert_script_hash3,
        &ref_cert_script_input_3,
        &Language::new_plutus_v2(),
        0,
    );
    let plutus_withdrawal_source = PlutusScriptSource::new_ref_input(
        &withdraw_script_hash2,
        &ref_cert_withdrawal_input_2,
        &Language::new_plutus_v2(),
        0,
    );

    let cert_witness_3 =
        PlutusWitness::new_with_ref_without_datum(&plutus_cert_source, &redeemer_cert3);
    let withdraw_witness1 =
        PlutusWitness::new_without_datum(&withdraw_script1, &redeemer_withdraw1);
    let withdraw_witness2 =
        PlutusWitness::new_with_ref_without_datum(&plutus_withdrawal_source, &redeemer_withdraw2);

    let mut certs = CertificatesBuilder::new();
    certs
        .add(&Certificate::new_stake_registration(
            &StakeRegistration::new(&stake_cred),
        ))
        .unwrap();
    certs
        .add_with_plutus_witness(
            &Certificate::new_stake_delegation(&StakeDelegation::new(
                &cert_script_cred1,
                &stake.to_raw_key().hash(), // in reality, this should be the pool owner's key, not ours
            )),
            &cert_witness_1,
        )
        .unwrap();
    certs
        .add_with_plutus_witness(
            &Certificate::new_stake_delegation(&StakeDelegation::new(
                &cert_script_cred2,
                &stake.to_raw_key().hash(), // in reality, this should be the pool owner's key, not ours
            )),
            &cert_witness_2,
        )
        .unwrap();
    certs
        .add(&Certificate::new_stake_delegation(&StakeDelegation::new(
            &stake_cred,
            &stake.to_raw_key().hash(), // in reality, this should be the pool owner's key, not ours
        )))
        .unwrap();
    certs
        .add_with_plutus_witness(
            &Certificate::new_stake_deregistration(&StakeDeregistration::new(&cert_script_cred3)),
            &cert_witness_3,
        )
        .unwrap();

    tx_builder.set_certs_builder(&certs);

    let mut withdrawals = WithdrawalsBuilder::new();
    let reward_cred = Credential::from_keyhash(&reward.to_raw_key().hash());
    withdrawals
        .add(
            &RewardAddress::new(NetworkInfo::testnet_preprod().network_id(), &reward_cred),
            &Coin::from(1u32),
        )
        .unwrap();
    withdrawals
        .add_with_plutus_witness(
            &RewardAddress::new(
                NetworkInfo::testnet_preprod().network_id(),
                &withdraw_script_cred1,
            ),
            &Coin::from(2u32),
            &withdraw_witness1,
        )
        .unwrap();
    withdrawals
        .add_with_plutus_witness(
            &RewardAddress::new(
                NetworkInfo::testnet_preprod().network_id(),
                &withdraw_script_cred2,
            ),
            &Coin::from(3u32),
            &withdraw_witness2,
        )
        .unwrap();
    tx_builder.set_withdrawals_builder(&withdrawals);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();
    let cost_models = TxBuilderConstants::plutus_default_cost_models();
    let collateral_input = fake_tx_input(1);
    let collateral_addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Credential::from_keyhash(&fake_key_hash(2)),
    )
    .to_address();
    let mut collateral_builder = TxInputsBuilder::new();
    collateral_builder.add_regular_input(
        &collateral_addr,
        &collateral_input,
        &Value::new(&Coin::from(123u32)),
    );
    tx_builder.set_collateral(&collateral_builder);
    tx_builder.calc_script_data_hash(&cost_models).unwrap();
    tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(tx_builder.outputs.len(), 1);
    assert_eq!(
        tx_builder
            .get_explicit_input()
            .unwrap()
            .checked_add(&tx_builder.get_implicit_input().unwrap())
            .unwrap(),
        tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap()))
            .unwrap()
            .checked_add(&Value::new(&tx_builder.get_deposit().unwrap()))
            .unwrap()
    );
    let final_tx = tx_builder.build_tx().unwrap();
    let final_tx_body = final_tx.body();
    let final_tx_wits = final_tx.witness_set();

    assert_eq!(final_tx_body.reference_inputs().unwrap().len(), 2);
    assert_eq!(final_tx_body.withdrawals().unwrap().len(), 3);
    assert_eq!(final_tx_body.certs().unwrap().len(), 5);
    assert_eq!(final_tx_wits.plutus_scripts().unwrap().len(), 3);
    assert_eq!(final_tx_wits.redeemers().unwrap().len(), 5);

    let certs = final_tx_body.certs().unwrap().0;
    let withdraws = final_tx_body
        .withdrawals()
        .unwrap()
        .0
        .iter()
        .map(|(k, _)| k.clone())
        .collect::<Vec<RewardAddress>>();
    let redeemers = final_tx_wits.redeemers().unwrap();
    let mut indexes = HashMap::new();
    indexes.insert(RedeemerTag::new_cert(), HashSet::new());
    indexes.insert(RedeemerTag::new_reward(), HashSet::new());
    for redeemer in &redeemers.redeemers {
        let tag_set = indexes.get_mut(&redeemer.tag()).unwrap();
        assert_ne!(tag_set.contains(&redeemer.index()), true);
        tag_set.insert(redeemer.index());
        let index: usize = redeemer.index().into();
        if redeemer.tag().kind() == RedeemerTagKind::Cert {
            let cert = &certs[index];
            assert!(cert.has_required_script_witness());
        } else if redeemer.tag().kind() == RedeemerTagKind::Reward {
            let withdraw = &withdraws[index];
            assert!(withdraw.payment_cred().has_script_hash());
        }
    }
}

#[test]
pub fn test_extra_datum() {
    let mut tx_builder = create_tx_builder(&create_linear_fee(1, 100000), 1, 1, 1);

    let datum = PlutusData::new_bytes(fake_bytes_32(1));
    tx_builder.add_extra_witness_datum(&datum);

    let mut inp = TxInputsBuilder::new();
    inp.add_regular_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new(&BigNum(100000000000000u64)),
    )
    .unwrap();

    tx_builder.set_inputs(&inp);
    tx_builder
        .calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models())
        .unwrap();

    let change_address = create_change_address();

    tx_builder.add_change_if_needed(&change_address).unwrap();
    let res = tx_builder.build_tx();
    assert!(res.is_ok());

    let tx = res.unwrap();

    let tx_size = tx.to_bytes().len();
    let fake_input_wit_size = fake_vkey_witness(1).to_bytes().len();
    let real_fee = min_fee_for_size(
        tx_size + fake_input_wit_size,
        &LinearFee::new(&Coin::from(1u64), &Coin::from(100000u64)),
    )
    .unwrap();

    assert!(real_fee.less_than(&tx.body.fee));

    let data_hash = hash_script_data(
        &Redeemers::new(),
        &Costmdls::new(),
        Some(PlutusList::from(vec![datum.clone()])),
    );

    let tx_builder_script_data_hash = tx_builder.script_data_hash.clone();
    assert_eq!(tx_builder_script_data_hash.unwrap(), data_hash);

    let extra_datums = tx_builder.get_extra_witness_datums().unwrap();
    assert_eq!(&extra_datums.get(0), &datum);
    assert_eq!(extra_datums.len(), 1usize);
    assert_eq!(
        tx_builder.get_witness_set().plutus_data().unwrap().len(),
        1usize
    );
    assert_eq!(tx.witness_set().plutus_data().unwrap().len(), 1usize);
    assert_eq!(tx.witness_set().plutus_data().unwrap().get(0), datum);
}

#[test]
fn current_treasure_value_test() {
    let input_amount = 10000000000;
    let mut builder = create_tx_builder_with_amount(input_amount, false);
    let treasure_value = Coin::from(1000000000u64);

    assert_eq!(builder.get_current_treasury_value(), None);

    builder.set_current_treasury_value(&treasure_value).unwrap();
    builder
        .add_change_if_needed(&create_change_address())
        .unwrap();

    assert_eq!(
        builder.get_current_treasury_value().unwrap(),
        treasure_value
    );

    let tx = builder.build_tx().unwrap();
    assert_eq!(tx.body().outputs().len(), 1);

    let mut total_out = tx.body().outputs().get(0).amount().coin();
    total_out = total_out.checked_add(&tx.body().fee()).unwrap();

    assert_eq!(total_out, Coin::from(input_amount));
}

#[test]
fn current_treasure_value_zero_error_test() {
    let mut builder = create_rich_tx_builder(false);
    let treasure_value = Coin::from(0u64);

    assert_eq!(builder.get_current_treasury_value(), None);

    let res = builder.set_current_treasury_value(&treasure_value);
    assert!(res.is_err());
}

#[test]
fn donation_test() {
    let input_amount = 10000000000;
    let mut builder = create_tx_builder_with_amount(input_amount, false);
    let donation = Coin::from(1000u64);

    assert_eq!(builder.get_donation(), None);

    builder.set_donation(&donation);
    builder
        .add_change_if_needed(&create_change_address())
        .unwrap();

    assert_eq!(builder.get_donation().unwrap(), donation);

    let tx = builder.build_tx().unwrap();
    assert_eq!(tx.body().outputs().len(), 1);

    let mut total_out = tx.body().outputs().get(0).amount().coin();
    total_out = total_out.checked_add(&tx.body().fee()).unwrap();
    total_out = total_out.checked_add(&donation).unwrap();

    assert_eq!(total_out, Coin::from(input_amount));
}

#[test]
fn ref_script_fee_from_all_builders() {
    let mut mint_builder = MintBuilder::new();
    let mut cert_builder = CertificatesBuilder::new();
    let mut withdrawal_builder = WithdrawalsBuilder::new();
    let mut voting_builder = VotingBuilder::new();
    let mut voting_proposal_builder = VotingProposalBuilder::new();
    let mut tx_input_builder = TxInputsBuilder::new();

    let tx_in_1 = fake_tx_input(1);
    let tx_in_2 = fake_tx_input(2);
    let tx_in_3 = fake_tx_input(3);
    let tx_in_4 = fake_tx_input(4);
    let tx_in_5 = fake_tx_input(5);
    let tx_in_6 = fake_tx_input(6);
    let tx_in_7 = fake_tx_input(7);
    let tx_in_8 = fake_tx_input(8);
    let tx_in_9 = fake_tx_input(9);

    let script_hash_1 = fake_script_hash(1);
    let script_hash_2 = fake_script_hash(2);
    let script_hash_3 = fake_script_hash(3);
    let script_hash_4 = fake_script_hash(4);
    let script_hash_5 = fake_script_hash(5);
    let script_hash_6 = fake_script_hash(6);
    let script_hash_7 = fake_script_hash(7);
    let script_hash_8 = fake_script_hash(8);

    let redeemer_1 = create_redeemer_zero_cost(1);
    let redeemer_2 = create_redeemer_zero_cost(2);
    let redeemer_3 = create_redeemer_zero_cost(3);
    let redeemer_4 = create_redeemer_zero_cost(4);
    let redeemer_5 = create_redeemer_zero_cost(5);
    let redeemer_6 = create_redeemer_zero_cost(6);
    let redeemer_8 = create_redeemer_zero_cost(8);

    let plutus_source_1 = PlutusScriptSource::new_ref_input(&script_hash_1, &tx_in_1, &Language::new_plutus_v2(), 10);
    let plutus_source_2 = PlutusScriptSource::new_ref_input(&script_hash_2, &tx_in_2, &Language::new_plutus_v2(), 100);
    let plutus_source_3 = PlutusScriptSource::new_ref_input(&script_hash_3, &tx_in_3, &Language::new_plutus_v2(), 1000);
    let plutus_source_4 = PlutusScriptSource::new_ref_input(&script_hash_4, &tx_in_4, &Language::new_plutus_v2(), 10000);
    let plutus_source_5 = PlutusScriptSource::new_ref_input(&script_hash_5, &tx_in_5, &Language::new_plutus_v2(), 100000);
    let plutus_source_6 = PlutusScriptSource::new_ref_input(&script_hash_6, &tx_in_6, &Language::new_plutus_v2(), 1000000);
    let native_script_source = NativeScriptSource::new_ref_input(&script_hash_7, &tx_in_7);
    let plutus_source_8 = PlutusScriptSource::new_ref_input(&script_hash_8, &tx_in_8, &Language::new_plutus_v2(), 10000000);

    mint_builder.add_asset(
        &MintWitness::new_plutus_script(&plutus_source_1, &redeemer_1),
        &AssetName::from_hex("44544e4654").unwrap(),
        &Int::new(&BigNum::from(100u64))
    ).unwrap();

    mint_builder.add_asset(
        &MintWitness::new_plutus_script(&plutus_source_2, &redeemer_2),
        &AssetName::from_hex("44544e4654").unwrap(),
        &Int::new(&BigNum::from(100u64))
    ).unwrap();

    withdrawal_builder.add_with_plutus_witness(
        &RewardAddress::new(NetworkInfo::testnet_preprod().network_id(), &Credential::from_scripthash(&script_hash_3)),
        &Coin::from(1u64),
        &PlutusWitness::new_with_ref_without_datum(&plutus_source_3, &redeemer_3)
    ).unwrap();

    withdrawal_builder.add_with_native_script(
        &RewardAddress::new(NetworkInfo::testnet_preprod().network_id(), &Credential::from_scripthash(&script_hash_7)),
        &Coin::from(1u64),
        &native_script_source
    ).unwrap();

    cert_builder.add_with_plutus_witness(
        &Certificate::new_stake_delegation(&StakeDelegation::new(&Credential::from_scripthash(&script_hash_4), &fake_key_hash(1))),
        &PlutusWitness::new_with_ref_without_datum(&plutus_source_4, &redeemer_4)
    ).unwrap();

    voting_builder.add_with_plutus_witness(
        &Voter::new_drep(&Credential::from_scripthash(&script_hash_5)),
        &GovernanceActionId::new(&fake_tx_hash(1), 1),
        &VotingProcedure::new(VoteKind::Abstain),
        &PlutusWitness::new_with_ref_without_datum(&plutus_source_5, &redeemer_5)
    ).unwrap();

    voting_proposal_builder.add_with_plutus_witness(
        &VotingProposal::new(
            &GovernanceAction::new_new_constitution_action(
                &NewConstitutionAction::new(
                    &Constitution::new_with_script_hash(&create_anchor(), &script_hash_6)
                )
            ),
            &create_anchor(),
            &RewardAddress::new(NetworkInfo::testnet_preprod().network_id(), &Credential::from_keyhash(&fake_key_hash(1))),
            &Coin::from(0u64),
        ),
        &PlutusWitness::new_with_ref_without_datum(&plutus_source_6, &redeemer_6)
    ).unwrap();

    let input_coin = Coin::from(1000000000u64);
    tx_input_builder.add_plutus_script_input(
        &PlutusWitness::new_with_ref_without_datum(&plutus_source_8, &redeemer_8),
        &tx_in_8,
        &Value::new(&input_coin)
    );

    let mut tx_builder = create_reallistic_tx_builder();
    let change_address = fake_base_address(1);

    tx_builder.set_mint_builder(&mint_builder);
    tx_builder.set_certs_builder(&cert_builder);
    tx_builder.set_withdrawals_builder(&withdrawal_builder);
    tx_builder.set_voting_builder(&voting_builder);
    tx_builder.set_voting_proposal_builder(&voting_proposal_builder);
    tx_builder.set_inputs(&tx_input_builder);
    tx_builder.add_script_reference_input(&tx_in_9, 100000000);

    let fake_collateral = fake_tx_input(99);
    let mut collateral_builder = TxInputsBuilder::new();
    collateral_builder.add_regular_input(
        &fake_base_address(99),
        &fake_collateral,
        &Value::new(&Coin::from(1000000000u64))
    ).unwrap();

    tx_builder.set_collateral(&collateral_builder);
    tx_builder.calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models()).unwrap();
    tx_builder.add_change_if_needed(&change_address).unwrap();

    let res = tx_builder.build_tx();
    assert!(res.is_ok());

    let mut tx = res.unwrap();
    let mut vkey_witneses = Vkeywitnesses::new();
    vkey_witneses.add(&fake_vkey_witness(1));
    let mut wit_set = tx.witness_set();
    wit_set.set_vkeys(&vkey_witneses);
    tx = Transaction::new(&tx.body(), &wit_set, tx.auxiliary_data());

    let ref_script_fee = BigNum::from(111111110u64 / 2);
    let total_tx_fee = tx.body().fee();

    //TODO: check change calculation for pessimistic size estimation.
    let tx_size = tx.to_bytes().len() + 4;

    let min_tx_fee = min_fee_for_size(tx_size, &create_linear_fee(44, 155381)).unwrap();
    let fee_leftover = total_tx_fee.checked_sub(&min_tx_fee).unwrap();
    assert_eq!(ref_script_fee, fee_leftover);

    let tx_out_coin = tx.body().outputs().get(0).amount().coin();
    let total_out = tx_out_coin
        .checked_add(&total_tx_fee).unwrap()
        .checked_sub(&Coin::from(2u64)).unwrap(); // withdrawals

    assert_eq!(total_out, input_coin);

    let ref_inputs = tx.body().reference_inputs().unwrap();
    assert!(ref_inputs.contains(&tx_in_1));
    assert!(ref_inputs.contains(&tx_in_2));
    assert!(ref_inputs.contains(&tx_in_3));
    assert!(ref_inputs.contains(&tx_in_4));
    assert!(ref_inputs.contains(&tx_in_5));
    assert!(ref_inputs.contains(&tx_in_6));
    assert!(ref_inputs.contains(&tx_in_7));
    assert!(ref_inputs.contains(&tx_in_8));
    assert!(ref_inputs.contains(&tx_in_9));
    assert_eq!(ref_inputs.len(), 9);
}


#[test]
fn utxo_selection_accounts_for_change_min_utxo_test() {
    let mut tx_builder = create_reallistic_tx_builder();
    let hex_utxos = [
        "82825820731224c9d2bc3528578009fec9f9e34a67110aca2bd4dde0f050845a2daf660d0082583900436075347d6a452eba4289ae345a8eb15e73eb80979a7e817d988fc56c8e2cfd5a9478355fa1d60759f93751237af3299d7faa947023e493821a001deabfa1581c9a5e0d55cdf4ce4e19c8acbff7b4dafc890af67a594a4c46d7dd1c0fa14001",
        "82825820a04996d5ef87fdece0c74625f02ee5c1497a06e0e476c5095a6b0626b295074a00825839001772f234940519e71318bb9c5c8ad6eacfe8fd91a509050624e3855e6c8e2cfd5a9478355fa1d60759f93751237af3299d7faa947023e4931a0016e360"
    ];
    let output = TransactionOutput::new(&Address::from_bech32("addr_test1qppkqaf5044y2t46g2y6udz636c4uultszte5l5p0kvgl3tv3ck06k550q64lgwkqavljd63yda0x2va074fguprujfsjre4xh").unwrap(), &Value::new(&BigNum::from_str("969750").unwrap()));
    tx_builder.add_output(&output);
    let mut utxos = TransactionUnspentOutputs::new();
    for hex_utxo in hex_utxos {
        utxos.add(&TransactionUnspentOutput::from_hex(hex_utxo).unwrap());
    }
    let change_config = ChangeConfig::new(&Address::from_bech32("addr_test1qqzf7fhgm0gf370ngxgpskg5c3kgp2g0u4ltxlrmsvumaztv3ck06k550q64lgwkqavljd63yda0x2va074fguprujfs43mc83").unwrap());
    assert!(&tx_builder.add_inputs_from_and_change(&utxos, CoinSelectionStrategyCIP2::LargestFirstMultiAsset, &change_config).is_ok());
    let build_res = tx_builder.build_tx();
    assert!(&build_res.is_ok());
}

#[test]
fn utxo_selection_with_collateral_return_test() {
    let mut tx_builder = create_reallistic_tx_builder();
    let hex_utxos = [
        "82825820731224c9d2bc3528578009fec9f9e34a67110aca2bd4dde0f050845a2daf660d0082583900436075347d6a452eba4289ae345a8eb15e73eb80979a7e817d988fc56c8e2cfd5a9478355fa1d60759f93751237af3299d7faa947023e493821a001deabfa1581c9a5e0d55cdf4ce4e19c8acbff7b4dafc890af67a594a4c46d7dd1c0fa14001",
        "82825820a04996d5ef87fdece0c74625f02ee5c1497a06e0e476c5095a6b0626b295074a00825839001772f234940519e71318bb9c5c8ad6eacfe8fd91a509050624e3855e6c8e2cfd5a9478355fa1d60759f93751237af3299d7faa947023e4931a0016e360"
    ];

    let collateral_percent = BigNum(150);
    let output = TransactionOutput::new(&Address::from_bech32("addr_test1qppkqaf5044y2t46g2y6udz636c4uultszte5l5p0kvgl3tv3ck06k550q64lgwkqavljd63yda0x2va074fguprujfsjre4xh").unwrap(), &Value::new(&BigNum::from_str("969750").unwrap()));
    tx_builder.add_output(&output).unwrap();
    let mut utxos = TransactionUnspentOutputs::new();
    for hex_utxo in hex_utxos {
        utxos.add(&TransactionUnspentOutput::from_hex(hex_utxo).unwrap());
    }
    let mut collateral_builder = TxInputsBuilder::new();
    let collateral_input = TransactionUnspentOutput::from_hex(hex_utxos[1]).unwrap();
    collateral_builder.add_regular_input(&collateral_input.output.address, &collateral_input.input, &collateral_input.output.amount).unwrap();
    tx_builder.set_collateral(&collateral_builder);

    let change_config = ChangeConfig::new(&Address::from_bech32("addr_test1qqzf7fhgm0gf370ngxgpskg5c3kgp2g0u4ltxlrmsvumaztv3ck06k550q64lgwkqavljd63yda0x2va074fguprujfs43mc83").unwrap());
    assert!(&tx_builder.add_inputs_from_and_change_with_collateral_return(&utxos, CoinSelectionStrategyCIP2::LargestFirstMultiAsset, &change_config, &collateral_percent).is_ok());

    let build_res = tx_builder.build_tx();
    assert!(&build_res.is_ok());

    let tx = build_res.unwrap();
    assert!(&tx.body.collateral_return().is_some());

    let mut vkey_witneses = Vkeywitnesses::new();
    vkey_witneses.add(&fake_vkey_witness(1));
    vkey_witneses.add(&fake_vkey_witness(2));
    let mut wit_set = tx.witness_set();
    wit_set.set_vkeys(&vkey_witneses);

    let tx_with_vkeys = Transaction::new(&tx.body(), &wit_set, tx.auxiliary_data());
    let tx_size = tx_with_vkeys.to_bytes().len();
    let fee = tx.body().fee();
    let min_fee = min_fee_for_size(tx_size, &create_linear_fee(44, 155381)).unwrap();
    assert!(fee >= min_fee);

    let collateral_amount = tx.body.total_collateral.unwrap();
    let calculated_collateral = fee.
        checked_mul(&collateral_percent).unwrap()
        .div_floor(&BigNum(100))
        .checked_add(&BigNum(1)).unwrap();

    assert_eq!(collateral_amount, calculated_collateral);
}

#[test]
fn utxo_selection_with_collateral_return_error() {
    let mut tx_builder = create_reallistic_tx_builder();
    let hex_utxos = [
        "82825820731224c9d2bc3528578009fec9f9e34a67110aca2bd4dde0f050845a2daf660d0082583900436075347d6a452eba4289ae345a8eb15e73eb80979a7e817d988fc56c8e2cfd5a9478355fa1d60759f93751237af3299d7faa947023e493821a001deabfa1581c9a5e0d55cdf4ce4e19c8acbff7b4dafc890af67a594a4c46d7dd1c0fa14001",
        "82825820a04996d5ef87fdece0c74625f02ee5c1497a06e0e476c5095a6b0626b295074a00825839001772f234940519e71318bb9c5c8ad6eacfe8fd91a509050624e3855e6c8e2cfd5a9478355fa1d60759f93751237af3299d7faa947023e4931a0016e360"
    ];

    //we use big percentage to lead not enough collaterals to cover the collateral fee
    let collateral_percent = BigNum(15000);
    let output = TransactionOutput::new(&Address::from_bech32("addr_test1qppkqaf5044y2t46g2y6udz636c4uultszte5l5p0kvgl3tv3ck06k550q64lgwkqavljd63yda0x2va074fguprujfsjre4xh").unwrap(), &Value::new(&BigNum::from_str("969750").unwrap()));
    tx_builder.add_output(&output).unwrap();
    let mut utxos = TransactionUnspentOutputs::new();
    for hex_utxo in hex_utxos {
        utxos.add(&TransactionUnspentOutput::from_hex(hex_utxo).unwrap());
    }
    let mut collateral_builder = TxInputsBuilder::new();
    let collateral_input = TransactionUnspentOutput::from_hex(hex_utxos[1]).unwrap();
    collateral_builder.add_regular_input(&collateral_input.output.address, &collateral_input.input, &collateral_input.output.amount).unwrap();
    tx_builder.set_collateral(&collateral_builder);

    let change_config = ChangeConfig::new(&Address::from_bech32("addr_test1qqzf7fhgm0gf370ngxgpskg5c3kgp2g0u4ltxlrmsvumaztv3ck06k550q64lgwkqavljd63yda0x2va074fguprujfs43mc83").unwrap());
    let change_res = tx_builder.add_inputs_from_and_change_with_collateral_return(&utxos, CoinSelectionStrategyCIP2::LargestFirstMultiAsset, &change_config, &collateral_percent);
    assert!(change_res.is_err());
}
