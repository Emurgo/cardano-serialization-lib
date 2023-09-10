use crate::fakes::{
    fake_base_address, fake_bytes_32, fake_data_hash, fake_key_hash, fake_policy_id,
    fake_script_hash, fake_tx_hash, fake_tx_input, fake_tx_input2, fake_value, fake_value2,
};
use crate::tests::helpers::harden;
use crate::tests::mock_objects::{byron_address, create_change_address, create_default_tx_builder, create_linear_fee, create_reallistic_tx_builder, create_rich_tx_builder, create_tx_builder_with_amount, create_tx_builder_with_fee, create_tx_builder_with_fee_and_pure_change, create_tx_builder_with_fee_and_val_size, create_tx_builder_with_key_deposit};
use crate::*;

use fees::*;
use std::collections::{BTreeMap, HashMap, HashSet};

const MAX_TX_SIZE: u32 = 8000;

fn genesis_id() -> TransactionHash {
    TransactionHash::from([0u8; TransactionHash::BYTE_COUNT])
}

fn root_key_15() -> Bip32PrivateKey {
    // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
    let entropy = [
        0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4,
        0xd8, 0x09, 0x7c, 0x64, 0x12,
    ];
    Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
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
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&to_bignum(222))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
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
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&to_bignum(222))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);

    let datum_hash = fake_data_hash(20);
    let data_option = OutputDatum::new_data_hash(&datum_hash);
    let (_, script_hash) = plutus_script_and_hash(15);
    let change_cred = Credential::from_scripthash(&script_hash);
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
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
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&to_bignum(880_000))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
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
        &Value::new(&to_bignum(5_000_000)),
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
        NetworkInfo::testnet().network_id(),
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
        &Value::new(&to_bignum(222)),
    );
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
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
                .with_coin(&to_bignum(222))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(0);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
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
        &Value::new(&to_bignum(700)),
    );
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
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
                .with_coin(&to_bignum(222))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(0);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
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
        &Value::new(&to_bignum(5)),
    );
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
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
                .with_coin(&to_bignum(5))
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
        NetworkInfo::testnet().network_id(),
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
                    &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred)
                        .to_address(),
                    &TransactionInput::new(&genesis_id(), 0),
                    &Value::new(&to_bignum(1_000_000))
                )
                .unwrap()
                .to_str(),
            "69500"
        );
        tx_builder.add_input(
            &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
    }
    tx_builder.add_input(
        &BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address(),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder.add_input(
        &PointerAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &Pointer::new_pointer(&to_bignum(0), &to_bignum(0), &to_bignum(0)),
        )
        .to_address(),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder.add_input(
        &ByronAddress::icarus_from_key(&spend, NetworkInfo::testnet().protocol_magic())
            .to_address(),
        &TransactionInput::new(&genesis_id(), 3),
        &Value::new(&to_bignum(1_000_000)),
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

    tx_builder.add_input(
        &PointerAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &Pointer::new_pointer(&to_bignum(0), &to_bignum(0), &to_bignum(0)),
        )
        .to_address(),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&to_bignum(1_000_000)),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let output_amount = Value::new(&to_bignum(500));

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
        NetworkInfo::testnet().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(added_change, true);
    let final_tx = tx_builder.build().unwrap();
    assert_eq!(final_tx.outputs().len(), 2);
    assert_eq!(final_tx.reference_inputs().unwrap().len(), 4);
    assert_eq!(final_tx.outputs().get(1).amount().coin(), to_bignum(999499));
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

    tx_builder.add_input(
        &PointerAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &Pointer::new_pointer(&to_bignum(0), &to_bignum(0), &to_bignum(0)),
        )
        .to_address(),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&to_bignum(1_000_000)),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let output_amount = Value::new(&to_bignum(500));

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
        NetworkInfo::testnet().network_id(),
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

    tx_builder.add_input(
        &PointerAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &Pointer::new_pointer(&to_bignum(0), &to_bignum(0), &to_bignum(0)),
        )
        .to_address(),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&to_bignum(1_000_000)),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let output_amount = Value::new(&to_bignum(500));

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
        NetworkInfo::testnet().network_id(),
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
    tx_builder.add_input(
        &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(500)),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let (min_script, policy_id) = mint_script_and_policy(0);
    let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
    let amount = to_bignum(1234);

    // Adding mint of the asset - which should work as an input
    tx_builder.add_mint_asset(&min_script, &name, &Int::new(&amount));

    let mut ass = Assets::new();
    ass.insert(&name, &amount);
    let mut mass = MultiAsset::new();
    mass.insert(&policy_id, &ass);

    // One coin and the minted asset goes into the output
    let mut output_amount = Value::new(&to_bignum(264));
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
        NetworkInfo::testnet().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert!(added_change);
    assert_eq!(tx_builder.outputs.len(), 2);

    // Change must be one remaining coin because fee is one constant coin
    let change = tx_builder.outputs.get(1).amount();
    assert_eq!(change.coin(), to_bignum(235));
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
    tx_builder.add_input(
        &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(600)),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let (min_script, policy_id) = mint_script_and_policy(0);
    let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

    let amount_minted = to_bignum(1000);
    let amount_sent = to_bignum(500);

    // Adding mint of the asset - which should work as an input
    tx_builder.add_mint_asset(&min_script, &name, &Int::new(&amount_minted));

    let mut ass = Assets::new();
    ass.insert(&name, &amount_sent);
    let mut mass = MultiAsset::new();
    mass.insert(&policy_id, &ass);

    // One coin and the minted asset goes into the output
    let mut output_amount = Value::new(&to_bignum(300));
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
        NetworkInfo::testnet().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert!(added_change);
    assert_eq!(tx_builder.outputs.len(), 2);

    // Change must be one remaining coin because fee is one constant coin
    let change = tx_builder.outputs.get(1).amount();
    assert_eq!(change.coin(), to_bignum(299));
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

    let amount_minted = to_bignum(1000);
    let amount_sent = to_bignum(500);
    let amount_input_amount = to_bignum(600);

    let mut asset_input = Assets::new();
    asset_input.insert(&asset_name, &amount_input_amount);
    let mut mass_input = MultiAsset::new();
    mass_input.insert(&policy_id, &asset_input);

    // Input with 600 coins
    tx_builder.add_input(
        &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(600)),
    );

    tx_builder.add_input(
        &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new_with_assets(&to_bignum(1), &mass_input),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
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
    let mut output_amount = Value::new(&to_bignum(400));
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
        NetworkInfo::testnet().network_id(),
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

    let amount_minted = to_bignum(1000);
    let amount_sent = to_bignum(100000);
    let amount_input_amount = to_bignum(600);

    let mut asset_input = Assets::new();
    asset_input.insert(&asset_name, &amount_input_amount);
    let mut mass_input = MultiAsset::new();
    mass_input.insert(&policy_id, &asset_input);

    // Input with 600 coins
    tx_builder.add_input(
        &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(100000)),
    );

    tx_builder.add_input(
        &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address(),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new_with_assets(&to_bignum(1), &mass_input),
    );

    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
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
    let mut output_amount = Value::new(&to_bignum(400));
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
        NetworkInfo::testnet().network_id(),
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
                assets.insert(&name, &to_bignum(*input));
                assets
            });
            multiasset
        })
        .collect::<Vec<MultiAsset>>();

    for (i, (multiasset, ada)) in multiassets
        .iter()
        .zip([100u64, 1000].iter().cloned().map(to_bignum))
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
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let mut output_amount = Value::new(&to_bignum(500));
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
        NetworkInfo::testnet().network_id(),
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
        to_bignum(ma_input1 + ma_input2 - ma_output1)
    );
    assert_eq!(final_tx.outputs().get(1).amount().coin(), to_bignum(599));
}

#[test]
fn build_tx_with_native_assets_change_and_purification() {
    let coin_per_utxo_word = to_bignum(8);
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
                assets.insert(&name, &to_bignum(*input));
                assets
            });
            multiasset
        })
        .collect::<Vec<MultiAsset>>();

    for (i, (multiasset, ada)) in multiassets
        .iter()
        .zip([100u64, 1000].iter().cloned().map(to_bignum))
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
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let mut output_amount = Value::new(&to_bignum(600));
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
        NetworkInfo::testnet().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(added_change, true);
    let final_tx = tx_builder.build().unwrap();
    assert_eq!(final_tx.outputs().len(), 3);
    assert_eq!(final_tx.outputs().get(0).amount().coin(), to_bignum(600));
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
        to_bignum(ma_input1 + ma_input2 - ma_output1)
    );
    // The first change output that contains all the tokens contain minimum required Coin
    let min_coin_for_dirty_change = min_ada_required(
        &final_tx.outputs().get(1).amount(),
        false,
        &coin_per_utxo_word,
    )
    .unwrap();
    assert_eq!(
        final_tx.outputs().get(1).amount().coin(),
        min_coin_for_dirty_change
    );
    assert_eq!(final_tx.outputs().get(2).amount().coin(), to_bignum(236));
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
                assets.insert(&name, &to_bignum(*input));
                assets
            });
            multiasset
        })
        .collect::<Vec<MultiAsset>>();

    for (i, (multiasset, ada)) in multiassets
        .iter()
        .zip([300u64, 900].iter().cloned().map(to_bignum))
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
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    let mut output_amount = Value::new(&to_bignum(300));
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
        NetworkInfo::testnet().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
    assert_eq!(added_change, true);
    let final_tx = tx_builder.build().unwrap();
    assert_eq!(final_tx.outputs().len(), 2);
    assert_eq!(final_tx.outputs().get(0).amount().coin(), to_bignum(300));
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
        to_bignum(ma_input1 + ma_input2 - ma_output1)
    );
    // The single change output contains more Coin then minimal utxo value
    // But not enough to cover the additional fee for a separate output
    assert_eq!(final_tx.outputs().get(1).amount().coin(), to_bignum(499));
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
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    // add an input that contains an asset not present in the output
    let policy_id = &PolicyID::from([0u8; 28]);
    let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
    let mut input_amount = Value::new(&to_bignum(1_000_000));
    let mut input_multiasset = MultiAsset::new();
    input_multiasset.insert(policy_id, &{
        let mut assets = Assets::new();
        assets.insert(&name, &to_bignum(100));
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
                .with_coin(&to_bignum(880_000))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
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
                .with_value(&Value::new(&to_bignum(2_000_000)))
                .build()
                .unwrap(),
        )
        .unwrap();

    tx_builder.add_input(
        &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(2_400_000)),
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
                .with_value(&Value::new(&to_bignum(2_000_000)))
                .build()
                .unwrap(),
        )
        .unwrap();

    let mut input_value = Value::new(&to_bignum(2_400_000));
    input_value.set_multiasset(&MultiAsset::new());
    tx_builder.add_input(
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
    let mut input_amount = Value::new(&to_bignum(5_000_000));
    let mut input_multiasset = MultiAsset::new();
    input_multiasset.insert(policy_id, &{
        let mut assets = Assets::new();
        assets.insert(&name, &to_bignum(100));
        assets
    });
    input_amount.set_multiasset(&input_multiasset);

    tx_builder.add_input(
        &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &input_amount,
    );

    // add an input that contains an asset & ADA
    let mut output_amount = Value::new(&to_bignum(2_000_000));
    let mut output_multiasset = MultiAsset::new();
    output_multiasset.insert(policy_id, &{
        let mut assets = Assets::new();
        assets.insert(&name, &to_bignum(100));
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
                assets.insert(&name, &to_bignum(500));
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

    let mut input_value = Value::new(&to_bignum(1000));
    input_value.set_multiasset(&multiasset);

    tx_builder.add_input(
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
    let output_amount = Value::new(&to_bignum(208));

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

    tx_builder.add_input(
        &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(500)),
    );

    let output_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b")
            .unwrap()
            .to_address();
    let mut output_amount = Value::new(&to_bignum(50));
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
                assets.insert(&name, &to_bignum(500));
                assets
            });
            acc
        },
    );

    let mut input_value = Value::new(&to_bignum(58));
    input_value.set_multiasset(&multiasset);

    tx_builder.add_input(
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
    let output_amount = Value::new(&to_bignum(208));

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
                .with_coin(&to_bignum(9000))
                .build()
                .unwrap(),
        )
        .unwrap();
    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(0u8, Value::new(&to_bignum(1200))));
    available_inputs.add(&make_input(1u8, Value::new(&to_bignum(1600))));
    available_inputs.add(&make_input(2u8, Value::new(&to_bignum(6400))));
    available_inputs.add(&make_input(3u8, Value::new(&to_bignum(2400))));
    available_inputs.add(&make_input(4u8, Value::new(&to_bignum(800))));
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
                .with_coin(&to_bignum(1200))
                .build()
                .unwrap(),
        )
        .unwrap();
    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(0u8, Value::new(&to_bignum(150))));
    available_inputs.add(&make_input(1u8, Value::new(&to_bignum(200))));
    available_inputs.add(&make_input(2u8, Value::new(&to_bignum(800))));
    available_inputs.add(&make_input(3u8, Value::new(&to_bignum(400))));
    available_inputs.add(&make_input(4u8, Value::new(&to_bignum(100))));
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

    let mut output_value = Value::new(&to_bignum(415));
    let mut output_ma = MultiAsset::new();
    output_ma.set_asset(&pid1, &asset_name1, to_bignum(5));
    output_ma.set_asset(&pid1, &asset_name2, to_bignum(1));
    output_ma.set_asset(&pid2, &asset_name2, to_bignum(2));
    output_ma.set_asset(&pid2, &asset_name3, to_bignum(4));
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
    available_inputs.add(&make_input(0u8, Value::new(&to_bignum(150))));

    // should not be taken
    let mut input1 = make_input(1u8, Value::new(&to_bignum(200)));
    let mut ma1 = MultiAsset::new();
    ma1.set_asset(&pid1, &asset_name1, to_bignum(10));
    ma1.set_asset(&pid1, &asset_name2, to_bignum(1));
    ma1.set_asset(&pid2, &asset_name2, to_bignum(2));
    input1.output.amount.set_multiasset(&ma1);
    available_inputs.add(&input1);

    // taken first to satisfy pid1:asset_name1 (but also satisfies pid2:asset_name3)
    let mut input2 = make_input(2u8, Value::new(&to_bignum(10)));
    let mut ma2 = MultiAsset::new();
    ma2.set_asset(&pid1, &asset_name1, to_bignum(20));
    ma2.set_asset(&pid2, &asset_name3, to_bignum(4));
    input2.output.amount.set_multiasset(&ma2);
    available_inputs.add(&input2);

    // taken second to satisfy pid1:asset_name2 (but also satisfies pid2:asset_name1)
    let mut input3 = make_input(3u8, Value::new(&to_bignum(50)));
    let mut ma3 = MultiAsset::new();
    ma3.set_asset(&pid2, &asset_name1, to_bignum(5));
    ma3.set_asset(&pid1, &asset_name2, to_bignum(15));
    input3.output.amount.multiasset = Some(ma3);
    available_inputs.add(&input3);

    // should not be taken either
    let mut input4 = make_input(4u8, Value::new(&to_bignum(10)));
    let mut ma4 = MultiAsset::new();
    ma4.set_asset(&pid1, &asset_name1, to_bignum(10));
    ma4.set_asset(&pid1, &asset_name2, to_bignum(10));
    input4.output.amount.multiasset = Some(ma4);
    available_inputs.add(&input4);

    // taken third to satisfy pid2:asset_name_2
    let mut input5 = make_input(5u8, Value::new(&to_bignum(10)));
    let mut ma5 = MultiAsset::new();
    ma5.set_asset(&pid1, &asset_name2, to_bignum(10));
    ma5.set_asset(&pid2, &asset_name2, to_bignum(3));
    input5.output.amount.multiasset = Some(ma5);
    available_inputs.add(&input5);

    // should be taken to get enough ADA
    let input6 = make_input(6u8, Value::new(&to_bignum(900)));
    available_inputs.add(&input6);

    // should not be taken
    available_inputs.add(&make_input(7u8, Value::new(&to_bignum(100))));
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
    assert_eq!(from_bignum(&change.coin), 555);
    let change_ma = change.multiasset().unwrap();
    assert_eq!(15, from_bignum(&change_ma.get_asset(&pid1, &asset_name1)));
    assert_eq!(24, from_bignum(&change_ma.get_asset(&pid1, &asset_name2)));
    assert_eq!(1, from_bignum(&change_ma.get_asset(&pid2, &asset_name2)));
    assert_eq!(0, from_bignum(&change_ma.get_asset(&pid2, &asset_name3)));
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

    let mut output_value = Value::new(&to_bignum(415));
    let mut output_ma = MultiAsset::new();
    output_ma.set_asset(&pid1, &asset_name1, to_bignum(5));
    output_ma.set_asset(&pid1, &asset_name2, to_bignum(1));
    output_ma.set_asset(&pid2, &asset_name2, to_bignum(2));
    output_ma.set_asset(&pid2, &asset_name3, to_bignum(4));
    output_value.set_multiasset(&output_ma);
    tx_builder
        .add_output(&TransactionOutput::new(
            &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z")
                .unwrap(),
            &output_value,
        ))
        .unwrap();

    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(0u8, Value::new(&to_bignum(150))));

    let mut input1 = make_input(1u8, Value::new(&to_bignum(200)));
    let mut ma1 = MultiAsset::new();
    ma1.set_asset(&pid1, &asset_name1, to_bignum(10));
    ma1.set_asset(&pid1, &asset_name2, to_bignum(1));
    ma1.set_asset(&pid2, &asset_name2, to_bignum(2));
    input1.output.amount.set_multiasset(&ma1);
    available_inputs.add(&input1);

    let mut input2 = make_input(2u8, Value::new(&to_bignum(10)));
    let mut ma2 = MultiAsset::new();
    ma2.set_asset(&pid1, &asset_name1, to_bignum(20));
    ma2.set_asset(&pid2, &asset_name3, to_bignum(4));
    input2.output.amount.set_multiasset(&ma2);
    available_inputs.add(&input2);

    let mut input3 = make_input(3u8, Value::new(&to_bignum(50)));
    let mut ma3 = MultiAsset::new();
    ma3.set_asset(&pid2, &asset_name1, to_bignum(5));
    ma3.set_asset(&pid1, &asset_name2, to_bignum(15));
    input3.output.amount.multiasset = Some(ma3);
    available_inputs.add(&input3);

    let mut input4 = make_input(4u8, Value::new(&to_bignum(10)));
    let mut ma4 = MultiAsset::new();
    ma4.set_asset(&pid1, &asset_name1, to_bignum(10));
    ma4.set_asset(&pid1, &asset_name2, to_bignum(10));
    input4.output.amount.multiasset = Some(ma4);
    available_inputs.add(&input4);

    let mut input5 = make_input(5u8, Value::new(&to_bignum(10)));
    let mut ma5 = MultiAsset::new();
    ma5.set_asset(&pid1, &asset_name2, to_bignum(10));
    ma5.set_asset(&pid2, &asset_name2, to_bignum(3));
    input5.output.amount.multiasset = Some(ma5);
    available_inputs.add(&input5);

    let input6 = make_input(6u8, Value::new(&to_bignum(1000)));
    available_inputs.add(&input6);
    available_inputs.add(&make_input(7u8, Value::new(&to_bignum(100))));

    let mut input8 = make_input(8u8, Value::new(&to_bignum(10)));
    let mut ma8 = MultiAsset::new();
    ma8.set_asset(&pid2, &asset_name2, to_bignum(10));
    input8.output.amount.multiasset = Some(ma8);
    available_inputs.add(&input8);

    let mut input9 = make_input(9u8, Value::new(&to_bignum(10)));
    let mut ma9 = MultiAsset::new();
    ma9.set_asset(&pid2, &asset_name3, to_bignum(10));
    input9.output.amount.multiasset = Some(ma9);
    available_inputs.add(&input9);

    tx_builder
        .add_inputs_from(
            &available_inputs,
            CoinSelectionStrategyCIP2::RandomImproveMultiAsset,
        )
        .unwrap();

    let input_for_cover_change = make_input(10u8, Value::new(&to_bignum(1000)));
    tx_builder.add_input(
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
                .with_coin(&to_bignum(COST))
                .build()
                .unwrap(),
        )
        .unwrap();
    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(0u8, Value::new(&to_bignum(1500))));
    available_inputs.add(&make_input(1u8, Value::new(&to_bignum(2000))));
    available_inputs.add(&make_input(2u8, Value::new(&to_bignum(8000))));
    available_inputs.add(&make_input(3u8, Value::new(&to_bignum(4000))));
    available_inputs.add(&make_input(4u8, Value::new(&to_bignum(1000))));
    available_inputs.add(&make_input(5u8, Value::new(&to_bignum(2000))));
    available_inputs.add(&make_input(6u8, Value::new(&to_bignum(1500))));
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
                    .checked_add(&to_bignum(COST))
                    .unwrap()
            )
    );
}

#[test]
fn tx_builder_cip2_random_improve_when_using_all_available_inputs() {
    // we have a = 1 to test increasing fees when more inputs are added
    let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(0));
    let cfg = TransactionBuilderConfigBuilder::new()
        .fee_algo(&linear_fee)
        .pool_deposit(&to_bignum(0))
        .key_deposit(&to_bignum(0))
        .voting_proposal_deposit(&to_bignum(500000000))
        .max_value_size(9999)
        .max_tx_size(9999)
        .coins_per_utxo_word(&Coin::zero())
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
                .with_coin(&to_bignum(COST))
                .build()
                .unwrap(),
        )
        .unwrap();
    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(1u8, Value::new(&to_bignum(800))));
    available_inputs.add(&make_input(2u8, Value::new(&to_bignum(800))));
    let add_inputs_res =
        tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::RandomImprove);
    assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
}

#[test]
fn tx_builder_cip2_random_improve_adds_enough_for_fees() {
    // we have a = 1 to test increasing fees when more inputs are added
    let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(0));
    let cfg = TransactionBuilderConfigBuilder::new()
        .fee_algo(&linear_fee)
        .pool_deposit(&to_bignum(0))
        .key_deposit(&to_bignum(0))
        .voting_proposal_deposit(&to_bignum(500000000))
        .max_value_size(9999)
        .max_tx_size(9999)
        .coins_per_utxo_word(&Coin::zero())
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
                .with_coin(&to_bignum(COST))
                .build()
                .unwrap(),
        )
        .unwrap();
    assert_eq!(tx_builder.min_fee().unwrap(), to_bignum(53));
    let mut available_inputs = TransactionUnspentOutputs::new();
    available_inputs.add(&make_input(1u8, Value::new(&to_bignum(150))));
    available_inputs.add(&make_input(2u8, Value::new(&to_bignum(150))));
    available_inputs.add(&make_input(3u8, Value::new(&to_bignum(150))));
    let add_inputs_res =
        tx_builder.add_inputs_from(&available_inputs, CoinSelectionStrategyCIP2::RandomImprove);
    assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
    assert_eq!(tx_builder.min_fee().unwrap(), to_bignum(264));
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
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();

    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&to_bignum(999_000))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);
    tx_builder.set_fee(&to_bignum(1_000));

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
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    let addr_output = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();

    tx_builder.add_input(
        &addr_multisig,
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );

    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_output)
                .next()
                .unwrap()
                .with_coin(&to_bignum(999_000))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);
    tx_builder.set_fee(&to_bignum(1_000));

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
        NetworkInfo::testnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder
        .add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next()
                .unwrap()
                .with_coin(&to_bignum(999_000))
                .build()
                .unwrap(),
        )
        .unwrap();
    tx_builder.set_ttl(1000);
    tx_builder.set_fee(&to_bignum(1_000));

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
    let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(1));
    let max_value_size = 100; // super low max output size to test with fewer assets
    let mut tx_builder = TransactionBuilder::new(
        &TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(0))
            .key_deposit(&to_bignum(0))
            .voting_proposal_deposit(&to_bignum(500000000))
            .max_value_size(max_value_size)
            .max_tx_size(MAX_TX_SIZE)
            .coins_per_utxo_word(&to_bignum(8))
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
        a.insert(&name, &to_bignum(500));
        a
    });
    let mut multiasset = MultiAsset::new();
    multiasset.insert(&policy_id, &assets);

    let mut input_value = Value::new(&to_bignum(1200));
    input_value.set_multiasset(&multiasset);

    tx_builder.add_input(
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
    let output_amount = Value::new(&to_bignum(208));

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

    assert_eq!(change1.amount.coin, to_bignum(288));
    assert_eq!(change2.amount.coin, to_bignum(293));
    assert_eq!(change3.amount.coin, to_bignum(410));

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
        assert_eq!(v1.or(v2).unwrap(), to_bignum(500));
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

    let num = to_bignum(42);
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

    let num1 = to_bignum(42);
    tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

    let num2 = to_bignum(84);
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

    let num = to_bignum(42);
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

    let num1 = to_bignum(42);
    tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

    let num2 = to_bignum(84);
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

    let num = to_bignum(42);
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

    let num1 = to_bignum(42);
    tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

    let num2 = to_bignum(84);
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
    assets.insert(&create_asset_name(), &to_bignum(1234));
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
        result_asset.get(&create_asset_name()).unwrap(),
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

fn plutus_script_and_hash(x: u8) -> (PlutusScript, ScriptHash) {
    let s = PlutusScript::new(fake_bytes_32(x));
    (s.clone(), s.hash())
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
    let mut value = Value::new(&to_bignum(249));
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
    let coin = to_bignum(208);
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
    let coin = to_bignum(249);

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
    assert_eq!(out.amount.coin, to_bignum(1146460));
}

#[test]
fn add_mint_asset_and_output() {
    let mut tx_builder = create_default_tx_builder();

    let (mint_script0, policy_id0) = mint_script_and_policy(0);
    let (mint_script1, policy_id1) = mint_script_and_policy(1);

    let name = create_asset_name();
    let amount = Int::new_i32(1234);

    let address = byron_address();
    let coin = to_bignum(249);

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
    assert_eq!(asset.get(&name).unwrap(), to_bignum(1234));
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
    assert_eq!(out.amount.coin, to_bignum(1146460));

    let multiasset = out.amount.multiasset.unwrap();

    // Only second mint entry was added to the output
    assert_eq!(multiasset.len(), 1);
    assert!(multiasset.get(&policy_id0).is_none());
    assert!(multiasset.get(&policy_id1).is_some());

    let asset = multiasset.get(&policy_id1).unwrap();
    assert_eq!(asset.len(), 1);
    assert_eq!(asset.get(&name).unwrap(), to_bignum(1234));
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
        &Value::new(&to_bignum(10_000_000)),
    );

    // One input from same address as mint
    tx_builder.add_key_input(
        &hash1,
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&to_bignum(10_000_000)),
    );

    // Original tx fee now assumes two VKey signatures for two inputs
    let original_tx_fee = tx_builder.min_fee().unwrap();
    assert_eq!(original_tx_fee, to_bignum(168361));

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
        .checked_mul(&to_bignum(mint_len as u64))
        .unwrap();

    let raw_mint_script_fee = fee_coefficient
        .checked_mul(&to_bignum(mint_scripts_len as u64))
        .unwrap();

    assert_eq!(raw_mint_fee, to_bignum(5544));
    assert_eq!(raw_mint_script_fee, to_bignum(4312));

    let new_tx_fee = tx_builder.min_fee().unwrap();

    let fee_diff_from_adding_mint = new_tx_fee.checked_sub(&original_tx_fee).unwrap();

    let witness_fee_increase = fee_diff_from_adding_mint
        .checked_sub(&raw_mint_fee)
        .unwrap()
        .checked_sub(&raw_mint_script_fee)
        .unwrap();

    assert_eq!(witness_fee_increase, to_bignum(8932));

    let fee_increase_bytes = from_bignum(&witness_fee_increase)
        .checked_div(from_bignum(&fee_coefficient))
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
                assets.insert(&name, &to_bignum(*input));
                assets
            });
            multiasset.insert(&policy_id2, &{
                let mut assets = Assets::new();
                assets.insert(&name, &to_bignum(*input));
                assets
            });
            multiasset
        })
        .collect::<Vec<MultiAsset>>();

    for (i, (multiasset, ada)) in multiassets
        .iter()
        .zip([100u64, 100, 100].iter().cloned().map(to_bignum))
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
                .with_coin(&to_bignum(208))
                .build()
                .unwrap(),
        )
        .unwrap();

    let total_input_before_mint = tx_builder.get_total_input().unwrap();
    let total_output_before_mint = tx_builder.get_total_output().unwrap();

    assert_eq!(total_input_before_mint.coin, to_bignum(300));
    assert_eq!(total_output_before_mint.coin, to_bignum(208));
    let ma1_input = total_input_before_mint.multiasset.unwrap();
    let ma1_output = total_output_before_mint.multiasset;
    assert_eq!(
        ma1_input.get(&policy_id1).unwrap().get(&name).unwrap(),
        to_bignum(360)
    );
    assert_eq!(
        ma1_input.get(&policy_id2).unwrap().get(&name).unwrap(),
        to_bignum(360)
    );
    assert!(ma1_output.is_none());

    // Adding mint
    tx_builder.add_mint_asset(&mint_script1, &name, &Int::new_i32(40));

    // Adding burn
    tx_builder.add_mint_asset(&mint_script2, &name, &Int::new_i32(-40));

    let total_input_after_mint = tx_builder.get_total_input().unwrap();
    let total_output_after_mint = tx_builder.get_total_output().unwrap();

    assert_eq!(total_input_after_mint.coin, to_bignum(300));
    assert_eq!(total_output_before_mint.coin, to_bignum(208));
    let ma2_input = total_input_after_mint.multiasset.unwrap();
    let ma2_output = total_output_after_mint.multiasset.unwrap();
    assert_eq!(
        ma2_input.get(&policy_id1).unwrap().get(&name).unwrap(),
        to_bignum(400)
    );
    assert_eq!(
        ma2_input.get(&policy_id2).unwrap().get(&name).unwrap(),
        to_bignum(360)
    );
    assert_eq!(
        ma2_output.get(&policy_id2).unwrap().get(&name).unwrap(),
        to_bignum(40)
    );
}

fn create_base_address_from_script_hash(sh: &ScriptHash) -> Address {
    BaseAddress::new(
        NetworkInfo::testnet().network_id(),
        &Credential::from_scripthash(sh),
        &Credential::from_keyhash(&fake_key_hash(0)),
    )
    .to_address()
}

#[test]
fn test_set_input_scripts() {
    let mut tx_builder = create_reallistic_tx_builder();
    let (script1, hash1) = mint_script_and_policy(0);
    let (script2, hash2) = mint_script_and_policy(1);
    let (script3, _hash3) = mint_script_and_policy(2);
    // Trying to set native scripts to the builder
    let rem0 = tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![
        script1.clone(),
        script2.clone(),
        script3.clone(),
    ]));
    assert_eq!(rem0, 0);
    let missing0 = tx_builder.count_missing_input_scripts();
    assert_eq!(missing0, 0);
    // Adding two script inputs using script1 and script2 hashes
    tx_builder.add_input(
        &create_base_address_from_script_hash(&hash1),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder.add_input(
        &create_base_address_from_script_hash(&hash2),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    // Setting a non-matching script will not change anything
    let rem1 =
        tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![script3.clone()]));
    assert_eq!(rem1, 2);
    let missing1 = tx_builder.count_missing_input_scripts();
    assert_eq!(missing1, 2);
    // Setting one of the required scripts leaves one to be required
    let rem2 = tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![
        script1.clone(),
        script3.clone(),
    ]));
    assert_eq!(rem2, 1);
    let missing2 = tx_builder.count_missing_input_scripts();
    assert_eq!(missing2, 1);
    // Setting one non-required script again does not change anything
    // But shows the state has changed
    let rem3 =
        tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![script3.clone()]));
    assert_eq!(rem3, 1);
    let missing3 = tx_builder.count_missing_input_scripts();
    assert_eq!(missing3, 1);
    // Setting two required scripts will show both of them added
    // And the remainder required is zero
    let rem4 = tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![
        script1.clone(),
        script2.clone(),
    ]));
    assert_eq!(rem4, 0);
    let missing4 = tx_builder.count_missing_input_scripts();
    assert_eq!(missing4, 0);
    // Setting empty scripts does not change anything
    // But shows the state has changed
    let rem5 = tx_builder.add_required_native_input_scripts(&NativeScripts::new());
    assert_eq!(rem5, 0);
}

#[test]
fn test_add_native_script_input() {
    let mut tx_builder = create_reallistic_tx_builder();
    let (script1, _hash1) = mint_script_and_policy(0);
    let (script2, _hash2) = mint_script_and_policy(1);
    let (script3, hash3) = mint_script_and_policy(2);
    // Adding two script inputs directly with their witness
    tx_builder.add_native_script_input(
        &script1,
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder.add_native_script_input(
        &script2,
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    // Adding one script input indirectly via hash3 address
    tx_builder.add_input(
        &create_base_address_from_script_hash(&hash3),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    // Checking missing input scripts shows one
    // Because first two inputs already have their witness
    let missing1 = tx_builder.count_missing_input_scripts();
    assert_eq!(missing1, 1);
    // Setting the required script leaves none to be required`
    let rem1 =
        tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![script3.clone()]));
    assert_eq!(rem1, 0);
    let missing2 = tx_builder.count_missing_input_scripts();
    assert_eq!(missing2, 0);
}

fn unsafe_tx_len(b: &TransactionBuilder) -> usize {
    b.build_tx_unsafe().unwrap().to_bytes().len()
}

#[test]
fn test_native_input_scripts_are_added_to_the_witnesses() {
    let mut tx_builder = create_reallistic_tx_builder();
    let (script1, _hash1) = mint_script_and_policy(0);
    let (script2, hash2) = mint_script_and_policy(1);
    tx_builder.set_fee(&to_bignum(42));
    tx_builder.add_native_script_input(
        &script1,
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    let tx_len_before_new_script_input = unsafe_tx_len(&tx_builder);
    tx_builder.add_input(
        &create_base_address_from_script_hash(&hash2),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&to_bignum(1_000_000)),
    );
    let tx_len_after_new_script_input = unsafe_tx_len(&tx_builder);
    // Tx size increased cuz input is added even without the witness
    assert!(tx_len_after_new_script_input > tx_len_before_new_script_input);
    tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![script2.clone()]));
    let tx_len_after_adding_script_witness = unsafe_tx_len(&tx_builder);
    // Tx size increased cuz the witness is added to the witnesses
    assert!(tx_len_after_adding_script_witness > tx_len_after_new_script_input);
    tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![
        script1.clone(),
        script2.clone(),
    ]));
    let tx_len_after_adding_script_witness_again = unsafe_tx_len(&tx_builder);
    // Tx size did not change because calling to add same witnesses again doesn't change anything
    assert!(tx_len_after_adding_script_witness == tx_len_after_adding_script_witness_again);
    let tx: Transaction = tx_builder.build_tx_unsafe().unwrap();
    assert!(tx.witness_set.native_scripts.is_some());
    let native_scripts = tx.witness_set.native_scripts.unwrap();
    assert_eq!(native_scripts.len(), 2);
    assert_eq!(native_scripts.get(0), script1);
    assert_eq!(native_scripts.get(1), script2);
}

#[test]
fn test_building_with_missing_witness_script_fails() {
    let mut tx_builder = create_reallistic_tx_builder();
    let (script1, _hash1) = mint_script_and_policy(0);
    let (script2, hash2) = mint_script_and_policy(1);
    tx_builder.set_fee(&to_bignum(42));
    // Ok to build before any inputs
    assert!(tx_builder.build_tx().is_ok());
    // Adding native script input which adds the witness right away
    tx_builder.add_native_script_input(
        &script1,
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    // Ok to build when witness is added along with the input
    assert!(tx_builder.build_tx().is_ok());
    // Adding script input without the witness
    tx_builder.add_input(
        &create_base_address_from_script_hash(&hash2),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    // Not ok to build when missing a witness
    assert!(tx_builder.build_tx().is_err());
    // Can force to build using unsafe
    assert!(tx_builder.build_tx_unsafe().is_ok());
    // Adding the missing witness script
    tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![script2.clone()]));
    // Ok to build when all witnesses are added
    assert!(tx_builder.build_tx().is_ok());
}

#[test]
fn test_adding_plutus_script_input() {
    let mut tx_builder = create_reallistic_tx_builder();
    let (script1, _) = plutus_script_and_hash(0);
    let datum = PlutusData::new_bytes(fake_bytes_32(1));
    let redeemer_datum = PlutusData::new_bytes(fake_bytes_32(2));
    let redeemer = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &redeemer_datum,
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum, &redeemer),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder.set_fee(&to_bignum(42));
    // There are no missing script witnesses
    assert_eq!(tx_builder.count_missing_input_scripts(), 0);
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
    tx_builder.set_fee(&to_bignum(42));
    let (script1, hash1) = plutus_script_and_hash(0);
    let (script2, hash2) = plutus_script_and_hash(1);
    let (script3, _hash3) = plutus_script_and_hash(3);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &PlutusData::new_bytes(fake_bytes_32(20)),
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(1),
        &PlutusData::new_bytes(fake_bytes_32(21)),
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );
    tx_builder.add_input(
        &create_base_address_from_script_hash(&hash1),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder.add_input(
        &create_base_address_from_script_hash(&hash2),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&to_bignum(1_000_000)),
    );
    // There are TWO missing script witnesses
    assert_eq!(tx_builder.count_missing_input_scripts(), 2);
    // Calling to add two plutus witnesses, one of which is irrelevant
    tx_builder.add_required_plutus_input_scripts(&PlutusWitnesses::from(vec![
        PlutusWitness::new(&script1, &datum1, &redeemer1),
        PlutusWitness::new(&script3, &datum2, &redeemer2),
    ]));
    // There is now ONE missing script witnesses
    assert_eq!(tx_builder.count_missing_input_scripts(), 1);
    // Calling to add the one remaining relevant plutus witness now
    tx_builder.add_required_plutus_input_scripts(&PlutusWitnesses::from(vec![PlutusWitness::new(
        &script2, &datum2, &redeemer2,
    )]));
    // There is now no missing script witnesses
    assert_eq!(tx_builder.count_missing_input_scripts(), 0);
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
    collateral_builder.add_input(
        &byron_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    collateral_builder
}

#[test]
fn test_existing_plutus_scripts_require_data_hash() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(42));
    tx_builder.set_collateral(&create_collateral());
    let (script1, _) = plutus_script_and_hash(0);
    let datum = PlutusData::new_bytes(fake_bytes_32(1));
    let redeemer_datum = PlutusData::new_bytes(fake_bytes_32(2));
    let redeemer = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &redeemer_datum,
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum, &redeemer),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
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
    tx_builder.set_fee(&to_bignum(42));
    tx_builder.set_collateral(&create_collateral());

    let (script1, _) = plutus_script_and_hash(0);
    let datum = PlutusData::new_bytes(fake_bytes_32(1));
    let redeemer_datum = PlutusData::new_bytes(fake_bytes_32(2));
    let redeemer = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &redeemer_datum,
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum, &redeemer),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
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
    tx_builder.set_fee(&to_bignum(42));
    tx_builder.set_collateral(&create_collateral());
    let (script1, _) = plutus_script_and_hash(0);
    let (script2, _) = plutus_script_and_hash(1);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));

    // Creating redeemers with indexes ZERO
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &PlutusData::new_bytes(fake_bytes_32(20)),
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &PlutusData::new_bytes(fake_bytes_32(21)),
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );

    // Add a regular NON-script input first
    tx_builder.add_input(
        &byron_address(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );

    // Adding two plutus inputs then
    // both have redeemers with index ZERO
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum1, &redeemer1),
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&to_bignum(1_000_000)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script2, &datum2, &redeemer2),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&to_bignum(1_000_000)),
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
    assert_eq!(redeems.get(0).index(), to_bignum(1));
    assert_eq!(redeems.get(1).index(), to_bignum(2));
}

#[test]
fn test_native_and_plutus_scripts_together() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(42));
    tx_builder.set_collateral(&create_collateral());
    let (pscript1, _) = plutus_script_and_hash(0);
    let (pscript2, phash2) = plutus_script_and_hash(1);
    let (nscript1, _) = mint_script_and_policy(0);
    let (nscript2, nhash2) = mint_script_and_policy(1);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
    // Creating redeemers with indexes ZERO
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &PlutusData::new_bytes(fake_bytes_32(20)),
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &PlutusData::new_bytes(fake_bytes_32(21)),
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );

    // Add one plutus input directly with witness
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    // Add one native input directly with witness
    tx_builder.add_native_script_input(
        &nscript1,
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&to_bignum(1_000_000)),
    );
    // Add one plutus input generically without witness
    tx_builder.add_input(
        &create_base_address_from_script_hash(&phash2),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&to_bignum(1_000_000)),
    );
    // Add one native input generically without witness
    tx_builder.add_input(
        &create_base_address_from_script_hash(&nhash2),
        &TransactionInput::new(&genesis_id(), 3),
        &Value::new(&to_bignum(1_000_000)),
    );

    // There are two missing script witnesses
    assert_eq!(tx_builder.count_missing_input_scripts(), 2);

    let remaining1 = tx_builder.add_required_plutus_input_scripts(&PlutusWitnesses::from(vec![
        PlutusWitness::new(&pscript2, &datum2, &redeemer2),
    ]));

    // There is one missing script witness now
    assert_eq!(remaining1, 1);
    assert_eq!(tx_builder.count_missing_input_scripts(), 1);

    let remaining2 =
        tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![nscript2.clone()]));

    // There are no missing script witnesses now
    assert_eq!(remaining2, 0);
    assert_eq!(tx_builder.count_missing_input_scripts(), 0);

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
    assert_eq!(redeems.get(1), redeemer2.clone_with_index(&to_bignum(2)));
}

#[test]
fn test_json_serialization_native_and_plutus_scripts_together() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(42));
    tx_builder.set_collateral(&create_collateral());
    let (pscript1, _) = plutus_script_and_hash(0);
    let (pscript2, phash2) = plutus_script_and_hash(1);
    let (nscript1, _) = mint_script_and_policy(0);
    let (nscript2, nhash2) = mint_script_and_policy(1);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
    // Creating redeemers with indexes ZERO
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &PlutusData::new_bytes(fake_bytes_32(20)),
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &PlutusData::new_bytes(fake_bytes_32(21)),
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );

    // Add one plutus input directly with witness
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    // Add one native input directly with witness
    tx_builder.add_native_script_input(
        &nscript1,
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&to_bignum(1_000_000)),
    );
    // Add one plutus input generically without witness
    tx_builder.add_input(
        &create_base_address_from_script_hash(&phash2),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&to_bignum(1_000_000)),
    );
    // Add one native input generically without witness
    tx_builder.add_input(
        &create_base_address_from_script_hash(&nhash2),
        &TransactionInput::new(&genesis_id(), 3),
        &Value::new(&to_bignum(1_000_000)),
    );

    // There are two missing script witnesses
    assert_eq!(tx_builder.count_missing_input_scripts(), 2);

    let remaining1 = tx_builder.add_required_plutus_input_scripts(&PlutusWitnesses::from(vec![
        PlutusWitness::new(&pscript2, &datum2, &redeemer2),
    ]));

    // There is one missing script witness now
    assert_eq!(remaining1, 1);
    assert_eq!(tx_builder.count_missing_input_scripts(), 1);

    let remaining2 =
        tx_builder.add_required_native_input_scripts(&NativeScripts::from(vec![nscript2.clone()]));

    // There are no missing script witnesses now
    assert_eq!(remaining2, 0);
    assert_eq!(tx_builder.count_missing_input_scripts(), 0);

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
    input_builder.add_input(
        &fake_base_address(0),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    collateral_builder.add_input(
        &fake_base_address(0),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );

    fn get_fake_vkeys_count(i: &TxInputsBuilder, c: &TxInputsBuilder) -> usize {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        tx_builder.set_inputs(i);
        tx_builder.set_collateral(c);
        let tx: Transaction = fake_full_tx(&tx_builder, tx_builder.build().unwrap()).unwrap();
        tx.witness_set.vkeys.unwrap().len()
    }

    // There's only one fake witness in the builder
    // because a regular and a collateral inputs both use the same keyhash
    assert_eq!(get_fake_vkeys_count(&input_builder, &collateral_builder), 1);

    // Add a new input of each kind with DIFFERENT keyhashes
    input_builder.add_input(
        &fake_base_address(1),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    collateral_builder.add_input(
        &fake_base_address(2),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );

    // There are now three fake witnesses in the builder
    // because all three unique keyhashes got combined
    assert_eq!(get_fake_vkeys_count(&input_builder, &collateral_builder), 3);
}

#[test]
fn test_regular_and_collateral_inputs_together() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(42));
    let (pscript1, _) = plutus_script_and_hash(0);
    let (pscript2, _) = plutus_script_and_hash(1);
    let (nscript1, _) = mint_script_and_policy(0);
    let (nscript2, _) = mint_script_and_policy(1);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
    // Creating redeemers with indexes ZERO
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &PlutusData::new_bytes(fake_bytes_32(20)),
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &PlutusData::new_bytes(fake_bytes_32(21)),
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );

    let mut input_builder = TxInputsBuilder::new();
    let mut collateral_builder = TxInputsBuilder::new();

    input_builder.add_native_script_input(
        &nscript1,
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
    );
    collateral_builder.add_native_script_input(
        &nscript2,
        &TransactionInput::new(&genesis_id(), 1),
        &Value::new(&to_bignum(1_000_000)),
    );

    input_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
        &TransactionInput::new(&genesis_id(), 2),
        &Value::new(&to_bignum(1_000_000)),
    );
    collateral_builder.add_plutus_script_input(
        &PlutusWitness::new(&pscript2, &datum2, &redeemer2),
        &TransactionInput::new(&genesis_id(), 3),
        &Value::new(&to_bignum(1_000_000)),
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
    assert_eq!(redeemers.get(0), redeemer1.clone_with_index(&to_bignum(1)));
    assert_eq!(redeemers.get(1), redeemer2.clone_with_index(&to_bignum(1)));
}

#[test]
fn test_ex_unit_costs_are_added_to_the_fees() {
    fn calc_fee_with_ex_units(mem: u64, step: u64) -> Coin {
        let mut input_builder = TxInputsBuilder::new();
        let mut collateral_builder = TxInputsBuilder::new();

        // Add a single input of both kinds with the SAME keyhash
        input_builder.add_input(
            &fake_base_address(0),
            &TransactionInput::new(&genesis_id(), 0),
            &Value::new(&to_bignum(1_000_000)),
        );
        collateral_builder.add_input(
            &fake_base_address(0),
            &TransactionInput::new(&genesis_id(), 1),
            &Value::new(&to_bignum(1_000_000)),
        );

        let (pscript1, _) = plutus_script_and_hash(0);
        let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
        let redeemer1 = Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(0),
            &PlutusData::new_bytes(fake_bytes_32(20)),
            &ExUnits::new(&to_bignum(mem), &to_bignum(step)),
        );
        input_builder.add_plutus_script_input(
            &PlutusWitness::new(&pscript1, &datum1, &redeemer1),
            &TransactionInput::new(&genesis_id(), 2),
            &Value::new(&to_bignum(1_000_000)),
        );

        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_inputs(&input_builder);
        tx_builder.set_collateral(&collateral_builder);

        tx_builder
            .add_change_if_needed(&fake_base_address(42))
            .unwrap();

        tx_builder.get_fee_if_set().unwrap()
    }

    assert_eq!(calc_fee_with_ex_units(0, 0), to_bignum(173509));
    assert_eq!(calc_fee_with_ex_units(10000, 0), to_bignum(174174));
    assert_eq!(calc_fee_with_ex_units(0, 10000000), to_bignum(174406));
    assert_eq!(calc_fee_with_ex_units(10000, 10000000), to_bignum(175071));
}

#[test]
fn test_script_inputs_ordering() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(42));
    let (nscript1, _) = mint_script_and_policy(0);
    let (pscript1, _) = plutus_script_and_hash(0);
    let (pscript2, _) = plutus_script_and_hash(1);
    let datum1 = PlutusData::new_bytes(fake_bytes_32(10));
    let datum2 = PlutusData::new_bytes(fake_bytes_32(11));
    // Creating redeemers with indexes ZERO
    let pdata1 = PlutusData::new_bytes(fake_bytes_32(20));
    let pdata2 = PlutusData::new_bytes(fake_bytes_32(21));
    let redeemer1 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &pdata1,
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    );
    let redeemer2 = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &pdata2,
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
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
    assert_eq!(r.get(0).index(), to_bignum(2));

    // Redeemer1 now has the index 1 even tho the input was added last
    assert_eq!(r.get(1).data(), pdata2);
    assert_eq!(r.get(1).index(), to_bignum(1));
}

#[test]
fn test_required_signers() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(42));
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

    let rs: RequiredSigners = tx1.required_signers.unwrap();
    assert_eq!(rs.len(), 3);
    assert_eq!(rs.get(0), s1);
    assert_eq!(rs.get(1), s3);
    assert_eq!(rs.get(2), s2);
}

#[test]
fn test_required_signers_are_added_to_the_witness_estimate() {
    fn count_fake_witnesses_with_required_signers(keys: &Ed25519KeyHashes) -> usize {
        let mut tx_builder = create_reallistic_tx_builder();
        tx_builder.set_fee(&to_bignum(42));
        tx_builder.add_input(
            &fake_base_address(0),
            &TransactionInput::new(&fake_tx_hash(0), 0),
            &Value::new(&to_bignum(10_000_000)),
        );

        keys.0.iter().for_each(|k| {
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
        count_fake_witnesses_with_required_signers(&Ed25519KeyHashes(vec![fake_key_hash(1)]),),
        2
    );

    assert_eq!(
        count_fake_witnesses_with_required_signers(&Ed25519KeyHashes(vec![
            fake_key_hash(1),
            fake_key_hash(2)
        ]),),
        3
    );

    // This case still produces only 3 fake signatures, because the same key is already used in the input address
    assert_eq!(
        count_fake_witnesses_with_required_signers(&Ed25519KeyHashes(vec![
            fake_key_hash(1),
            fake_key_hash(2),
            fake_key_hash(0)
        ]),),
        3
    );

    // When a different key is used - 4 fake witnesses are produced
    assert_eq!(
        count_fake_witnesses_with_required_signers(&Ed25519KeyHashes(vec![
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
    tx_builder.set_fee(&to_bignum(123456));

    let mut inp = TxInputsBuilder::new();
    inp.add_input(&fake_base_address(0), &fake_tx_input(0), &fake_value());

    tx_builder.set_inputs(&inp);
    tx_builder.set_collateral(&inp);

    let col_return = TransactionOutput::new(&fake_base_address(1), &fake_value2(123123));
    let col_total = to_bignum(234234);

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
        &to_bignum(amount),
    );
    let mut masset = MultiAsset::new();
    masset.insert(&policy_id, &assets);
    masset
}

#[test]
fn inputs_builder_total_value() {
    let mut b = TxInputsBuilder::new();
    assert_eq!(b.total_value().unwrap(), Value::zero());

    b.add_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &fake_value2(100_000),
    );
    assert_eq!(b.total_value().unwrap(), Value::new(&to_bignum(100_000)));

    b.add_input(
        &fake_base_address(1),
        &fake_tx_input(1),
        &fake_value2(200_000),
    );
    assert_eq!(b.total_value().unwrap(), Value::new(&to_bignum(300_000)));

    let masset = fake_multiasset(123);

    b.add_input(
        &fake_base_address(2),
        &fake_tx_input(2),
        &Value::new_with_assets(&to_bignum(300_000), &masset),
    );
    assert_eq!(
        b.total_value().unwrap(),
        Value::new_with_assets(&to_bignum(600_000), &masset)
    );
}

#[test]
fn test_auto_calc_total_collateral() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(123456));

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_input(
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
        to_bignum(collateral_input_value - collateral_return_value),
    );
}

#[test]
fn test_auto_calc_total_collateral_with_assets() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(123456));

    let masset = fake_multiasset(123);

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new_with_assets(&to_bignum(collateral_input_value.clone()), &masset),
    );

    tx_builder.set_collateral(&inp);

    let collateral_return_value = 1_345_678;
    let col_return = TransactionOutput::new(
        &fake_base_address(1),
        &Value::new_with_assets(&to_bignum(collateral_return_value.clone()), &masset),
    );

    tx_builder
        .set_collateral_return_and_total(&col_return)
        .unwrap();

    assert!(tx_builder.collateral_return.is_some());
    assert_eq!(tx_builder.collateral_return.unwrap(), col_return,);

    assert!(tx_builder.total_collateral.is_some());
    assert_eq!(
        tx_builder.total_collateral.unwrap(),
        to_bignum(collateral_input_value - collateral_return_value),
    );
}

#[test]
fn test_auto_calc_total_collateral_fails_with_assets() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(123456));

    let masset = fake_multiasset(123);

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new_with_assets(&to_bignum(collateral_input_value.clone()), &masset),
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
    tx_builder.set_fee(&to_bignum(123456));

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
    tx_builder.set_fee(&to_bignum(123456));

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new(&to_bignum(collateral_input_value.clone())),
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
    tx_builder.set_fee(&to_bignum(123456));

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &fake_value2(collateral_input_value.clone()),
    );

    tx_builder.set_collateral(&inp);

    let total_collateral_value = 234_567;
    let collateral_return_address = fake_base_address(1);

    tx_builder
        .set_total_collateral_and_return(
            &to_bignum(total_collateral_value.clone()),
            &collateral_return_address,
        )
        .unwrap();

    assert!(tx_builder.total_collateral.is_some());
    assert_eq!(
        tx_builder.total_collateral.unwrap(),
        to_bignum(total_collateral_value.clone()),
    );

    assert!(tx_builder.collateral_return.is_some());
    let col_return: TransactionOutput = tx_builder.collateral_return.unwrap();
    assert_eq!(col_return.address, collateral_return_address);
    assert_eq!(
        col_return.amount,
        Value::new(&to_bignum(collateral_input_value - total_collateral_value),)
    );
}

#[test]
fn test_auto_calc_collateral_return_with_assets() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(123456));

    let masset = fake_multiasset(123);

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new_with_assets(&to_bignum(collateral_input_value.clone()), &masset),
    );

    tx_builder.set_collateral(&inp);

    let total_collateral_value = 345_678;
    let collateral_return_address = fake_base_address(1);

    tx_builder
        .set_total_collateral_and_return(
            &to_bignum(total_collateral_value.clone()),
            &collateral_return_address,
        )
        .unwrap();

    assert!(tx_builder.total_collateral.is_some());
    assert_eq!(
        tx_builder.total_collateral.unwrap(),
        to_bignum(total_collateral_value.clone()),
    );

    assert!(tx_builder.collateral_return.is_some());
    let col_return: TransactionOutput = tx_builder.collateral_return.unwrap();
    assert_eq!(col_return.address, collateral_return_address);
    assert_eq!(
        col_return.amount,
        Value::new_with_assets(
            &to_bignum(collateral_input_value - total_collateral_value),
            &masset,
        )
    );
}

#[test]
fn test_add_collateral_return_succeed_with_border_amount() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(123456));

    let masset = fake_multiasset(123);

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new_with_assets(&to_bignum(collateral_input_value.clone()), &masset),
    );

    tx_builder.set_collateral(&inp);

    let collateral_return_address = fake_base_address(1);

    let possible_ret = Value::new_from_assets(&masset);
    let fake_out = TransactionOutput::new(&collateral_return_address, &possible_ret);
    let min_ada = min_ada_for_output(&fake_out, &tx_builder.config.utxo_cost()).unwrap();

    let total_collateral_value = to_bignum(collateral_input_value)
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
    tx_builder.set_fee(&to_bignum(123456));

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new(&to_bignum(collateral_input_value.clone())),
    );

    tx_builder.set_collateral(&inp);

    let collateral_return_address = fake_base_address(1);

    tx_builder
        .set_total_collateral_and_return(
            &to_bignum(collateral_input_value.clone()),
            &collateral_return_address,
        )
        .unwrap();

    assert!(tx_builder.total_collateral.is_some());
    assert!(tx_builder.collateral_return.is_none());
}

#[test]
fn test_add_collateral_return_fails_no_enough_ada() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(123456));

    let masset = fake_multiasset(123);

    let mut inp = TxInputsBuilder::new();
    let collateral_input_value = 2_000_000;
    inp.add_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new_with_assets(&to_bignum(collateral_input_value.clone()), &masset),
    );

    tx_builder.set_collateral(&inp);

    let collateral_return_address = fake_base_address(1);

    let possible_ret = Value::new_from_assets(&masset);
    let fake_out = TransactionOutput::new(&collateral_return_address, &possible_ret);
    let min_ada = min_ada_for_output(&fake_out, &tx_builder.config.utxo_cost()).unwrap();
    let mut total_collateral_value = to_bignum(collateral_input_value)
        .checked_sub(&min_ada)
        .unwrap();
    //make total collateral value bigger for make collateral return less then min ada
    total_collateral_value = total_collateral_value.checked_add(&to_bignum(1)).unwrap();

    let coll_add_res = tx_builder
        .set_total_collateral_and_return(&total_collateral_value, &collateral_return_address);

    assert!(coll_add_res.is_err());
    assert!(tx_builder.total_collateral.is_none());
    assert!(tx_builder.collateral_return.is_none());
}

#[test]
fn test_auto_calc_collateral_return_fails_on_no_collateral() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(123456));

    let res = tx_builder
        .set_total_collateral_and_return(&to_bignum(345_678.clone()), &fake_base_address(1));

    assert!(res.is_err());
    assert!(tx_builder.total_collateral.is_none());
    assert!(tx_builder.collateral_return.is_none());
}

#[test]
fn test_costmodel_retaining_for_v1() {
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(42));
    tx_builder.set_collateral(&create_collateral());

    let (script1, _) = plutus_script_and_hash(0);
    let datum = PlutusData::new_integer(&BigInt::from_str("42").unwrap());
    let redeemer = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &datum,
        &ExUnits::new(&to_bignum(1700), &to_bignum(368100)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum, &redeemer),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
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
    tx_builder.set_fee(&to_bignum(42));
    tx_builder.set_collateral(&create_collateral());

    let (script1, _) = plutus_script_and_hash(0);
    let datum = PlutusData::new_integer(&BigInt::from_str("42").unwrap());
    let redeemer = Redeemer::new(
        &RedeemerTag::new_spend(),
        &to_bignum(0),
        &datum,
        &ExUnits::new(&to_bignum(1700), &to_bignum(368100)),
    );
    tx_builder.add_plutus_script_input(
        &PlutusWitness::new(&script1, &datum, &redeemer),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(1_000_000)),
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
fn plutus_mint_test() {
    let mut tx_builder = create_reallistic_tx_builder();
    let colateral_adress = Address::from_bech32("addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w").unwrap();
    let colateral_input = TransactionInput::from_json("\
          {
            \"transaction_id\": \"69b0b867056a2d4fdc3827e23aa7069b125935e2def774941ca8cc7f9e0de774\",
            \"index\": 1
          }").unwrap();

    let tx_input = TransactionInput::from_json("\
          {
            \"transaction_id\": \"f58a5bc761b1efdcf4b5684f6ad5495854a0d64b866e2f0f525d134750d3511b\",
            \"index\": 1
          }").unwrap();
    let plutus_script = PlutusScript::from_hex("5907d2010000332323232323232323232323232323322323232323222232325335332201b3333573466e1cd55ce9baa0044800080608c98c8060cd5ce00c80c00b1999ab9a3370e6aae7540092000233221233001003002323232323232323232323232323333573466e1cd55cea8062400046666666666664444444444442466666666666600201a01801601401201000e00c00a00800600466a02a02c6ae854030cd4054058d5d0a80599a80a80b9aba1500a3335501975ca0306ae854024ccd54065d7280c1aba1500833501502035742a00e666aa032042eb4d5d0a8031919191999ab9a3370e6aae75400920002332212330010030023232323333573466e1cd55cea8012400046644246600200600466a056eb4d5d0a80118161aba135744a004464c6405c66ae700bc0b80b04d55cf280089baa00135742a0046464646666ae68cdc39aab9d5002480008cc8848cc00400c008cd40add69aba15002302c357426ae8940088c98c80b8cd5ce01781701609aab9e5001137540026ae84d5d1280111931901519ab9c02b02a028135573ca00226ea8004d5d0a80299a80abae35742a008666aa03203a40026ae85400cccd54065d710009aba15002301f357426ae8940088c98c8098cd5ce01381301209aba25001135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135744a00226aae7940044dd50009aba15002300f357426ae8940088c98c8060cd5ce00c80c00b080b89931900b99ab9c4910350543500017135573ca00226ea800448c88c008dd6000990009aa80a911999aab9f0012500a233500930043574200460066ae880080508c8c8cccd5cd19b8735573aa004900011991091980080180118061aba150023005357426ae8940088c98c8050cd5ce00a80a00909aab9e5001137540024646464646666ae68cdc39aab9d5004480008cccc888848cccc00401401000c008c8c8c8cccd5cd19b8735573aa0049000119910919800801801180a9aba1500233500f014357426ae8940088c98c8064cd5ce00d00c80b89aab9e5001137540026ae854010ccd54021d728039aba150033232323333573466e1d4005200423212223002004357426aae79400c8cccd5cd19b875002480088c84888c004010dd71aba135573ca00846666ae68cdc3a801a400042444006464c6403666ae7007006c06406005c4d55cea80089baa00135742a00466a016eb8d5d09aba2500223263201533573802c02a02626ae8940044d5d1280089aab9e500113754002266aa002eb9d6889119118011bab00132001355012223233335573e0044a010466a00e66442466002006004600c6aae754008c014d55cf280118021aba200301213574200222440042442446600200800624464646666ae68cdc3a800a40004642446004006600a6ae84d55cf280191999ab9a3370ea0049001109100091931900819ab9c01101000e00d135573aa00226ea80048c8c8cccd5cd19b875001480188c848888c010014c01cd5d09aab9e500323333573466e1d400920042321222230020053009357426aae7940108cccd5cd19b875003480088c848888c004014c01cd5d09aab9e500523333573466e1d40112000232122223003005375c6ae84d55cf280311931900819ab9c01101000e00d00c00b135573aa00226ea80048c8c8cccd5cd19b8735573aa004900011991091980080180118029aba15002375a6ae84d5d1280111931900619ab9c00d00c00a135573ca00226ea80048c8cccd5cd19b8735573aa002900011bae357426aae7940088c98c8028cd5ce00580500409baa001232323232323333573466e1d4005200c21222222200323333573466e1d4009200a21222222200423333573466e1d400d2008233221222222233001009008375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c4664424444444660040120106eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc8848888888cc018024020c030d5d0a8049bae357426ae8940248cccd5cd19b875006480088c848888888c01c020c034d5d09aab9e500b23333573466e1d401d2000232122222223005008300e357426aae7940308c98c804ccd5ce00a00980880800780700680600589aab9d5004135573ca00626aae7940084d55cf280089baa0012323232323333573466e1d400520022333222122333001005004003375a6ae854010dd69aba15003375a6ae84d5d1280191999ab9a3370ea0049000119091180100198041aba135573ca00c464c6401866ae700340300280244d55cea80189aba25001135573ca00226ea80048c8c8cccd5cd19b875001480088c8488c00400cdd71aba135573ca00646666ae68cdc3a8012400046424460040066eb8d5d09aab9e500423263200933573801401200e00c26aae7540044dd500089119191999ab9a3370ea00290021091100091999ab9a3370ea00490011190911180180218031aba135573ca00846666ae68cdc3a801a400042444004464c6401466ae7002c02802001c0184d55cea80089baa0012323333573466e1d40052002200723333573466e1d40092000212200123263200633573800e00c00800626aae74dd5000a4c24002920103505431001220021123230010012233003300200200133351222335122335004335500248811c2b194b7d10a3d2d3152c5f3a628ff50cb9fc11e59453e8ac7a1aea4500488104544e4654005005112212330010030021120011122002122122330010040031200101").unwrap();
    let redeemer = Redeemer::from_json(
        "\
         {
            \"tag\": \"Mint\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"1042996\",
              \"steps\": \"446100241\"
            }
          }",
    )
    .unwrap();
    let asset_name = AssetName::from_hex("44544e4654").unwrap();
    let mut mint_builder = MintBuilder::new();
    let plutus_script_source = PlutusScriptSource::new(&plutus_script);
    let mint_witnes = MintWitness::new_plutus_script(&plutus_script_source, &redeemer);
    mint_builder.add_asset(&mint_witnes, &asset_name, &Int::new(&BigNum::from(100u64)));

    let output_adress = Address::from_bech32("addr_test1qpm5njmgzf4t7225v6j34wl30xfrufzt3jtqtdzf3en9ahpmnhtmynpasyc8fq75zv0uaj86vzsr7g3g8q5ypgu5fwtqr9zsgj").unwrap();
    let mut output_assets = MultiAsset::new();
    let mut asset = Assets::new();
    asset.insert(&asset_name, &BigNum::from(100u64));
    output_assets.insert(&plutus_script.hash(), &asset);
    let output_value = Value::new_with_assets(&Coin::from(50000u64), &output_assets);
    let output = TransactionOutput::new(&output_adress, &output_value);

    let mut col_builder = TxInputsBuilder::new();
    col_builder.add_input(
        &colateral_adress,
        &colateral_input,
        &Value::new(&Coin::from(1000000000u64)),
    );
    tx_builder.set_collateral(&col_builder);
    tx_builder.add_output(&output);
    tx_builder.add_input(
        &output_adress,
        &tx_input,
        &Value::new(&BigNum::from(100000000000u64)),
    );
    tx_builder.set_mint_builder(&mint_builder);

    tx_builder
        .calc_script_data_hash(&TxBuilderConstants::plutus_vasil_cost_models())
        .unwrap();

    let change_res = tx_builder.add_change_if_needed(&output_adress);
    assert!(change_res.is_ok());

    let build_res = tx_builder.build_tx();
    assert!(build_res.is_ok());

    assert!(mint_builder.get_plutus_witnesses().len() == 1);

    let tx = build_res.unwrap();
    assert!(tx.body.mint.is_some());
    assert_eq!(
        tx.body.mint.unwrap().0.iter().next().unwrap().0,
        plutus_script.hash()
    );
}

#[test]
fn plutus_mint_with_script_ref_test() {
    let mut tx_builder = create_reallistic_tx_builder();
    let colateral_adress = Address::from_bech32("addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w").unwrap();
    let colateral_input = TransactionInput::from_json("\
          {
            \"transaction_id\": \"69b0b867056a2d4fdc3827e23aa7069b125935e2def774941ca8cc7f9e0de774\",
            \"index\": 1
          }").unwrap();

    let tx_input = TransactionInput::from_json("\
          {
            \"transaction_id\": \"f58a5bc761b1efdcf4b5684f6ad5495854a0d64b866e2f0f525d134750d3511b\",
            \"index\": 1
          }").unwrap();
    let tx_input_ref = TransactionInput::from_json("\
          {
            \"transaction_id\": \"f58a5bc7adaadadcf4b5684f6ad5495854a0d64b866e2f0f525d134750d3511b\",
            \"index\": 2
          }").unwrap();
    let plutus_script = PlutusScript::from_hex("5907d2010000332323232323232323232323232323322323232323222232325335332201b3333573466e1cd55ce9baa0044800080608c98c8060cd5ce00c80c00b1999ab9a3370e6aae7540092000233221233001003002323232323232323232323232323333573466e1cd55cea8062400046666666666664444444444442466666666666600201a01801601401201000e00c00a00800600466a02a02c6ae854030cd4054058d5d0a80599a80a80b9aba1500a3335501975ca0306ae854024ccd54065d7280c1aba1500833501502035742a00e666aa032042eb4d5d0a8031919191999ab9a3370e6aae75400920002332212330010030023232323333573466e1cd55cea8012400046644246600200600466a056eb4d5d0a80118161aba135744a004464c6405c66ae700bc0b80b04d55cf280089baa00135742a0046464646666ae68cdc39aab9d5002480008cc8848cc00400c008cd40add69aba15002302c357426ae8940088c98c80b8cd5ce01781701609aab9e5001137540026ae84d5d1280111931901519ab9c02b02a028135573ca00226ea8004d5d0a80299a80abae35742a008666aa03203a40026ae85400cccd54065d710009aba15002301f357426ae8940088c98c8098cd5ce01381301209aba25001135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135744a00226aae7940044dd50009aba15002300f357426ae8940088c98c8060cd5ce00c80c00b080b89931900b99ab9c4910350543500017135573ca00226ea800448c88c008dd6000990009aa80a911999aab9f0012500a233500930043574200460066ae880080508c8c8cccd5cd19b8735573aa004900011991091980080180118061aba150023005357426ae8940088c98c8050cd5ce00a80a00909aab9e5001137540024646464646666ae68cdc39aab9d5004480008cccc888848cccc00401401000c008c8c8c8cccd5cd19b8735573aa0049000119910919800801801180a9aba1500233500f014357426ae8940088c98c8064cd5ce00d00c80b89aab9e5001137540026ae854010ccd54021d728039aba150033232323333573466e1d4005200423212223002004357426aae79400c8cccd5cd19b875002480088c84888c004010dd71aba135573ca00846666ae68cdc3a801a400042444006464c6403666ae7007006c06406005c4d55cea80089baa00135742a00466a016eb8d5d09aba2500223263201533573802c02a02626ae8940044d5d1280089aab9e500113754002266aa002eb9d6889119118011bab00132001355012223233335573e0044a010466a00e66442466002006004600c6aae754008c014d55cf280118021aba200301213574200222440042442446600200800624464646666ae68cdc3a800a40004642446004006600a6ae84d55cf280191999ab9a3370ea0049001109100091931900819ab9c01101000e00d135573aa00226ea80048c8c8cccd5cd19b875001480188c848888c010014c01cd5d09aab9e500323333573466e1d400920042321222230020053009357426aae7940108cccd5cd19b875003480088c848888c004014c01cd5d09aab9e500523333573466e1d40112000232122223003005375c6ae84d55cf280311931900819ab9c01101000e00d00c00b135573aa00226ea80048c8c8cccd5cd19b8735573aa004900011991091980080180118029aba15002375a6ae84d5d1280111931900619ab9c00d00c00a135573ca00226ea80048c8cccd5cd19b8735573aa002900011bae357426aae7940088c98c8028cd5ce00580500409baa001232323232323333573466e1d4005200c21222222200323333573466e1d4009200a21222222200423333573466e1d400d2008233221222222233001009008375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c4664424444444660040120106eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc8848888888cc018024020c030d5d0a8049bae357426ae8940248cccd5cd19b875006480088c848888888c01c020c034d5d09aab9e500b23333573466e1d401d2000232122222223005008300e357426aae7940308c98c804ccd5ce00a00980880800780700680600589aab9d5004135573ca00626aae7940084d55cf280089baa0012323232323333573466e1d400520022333222122333001005004003375a6ae854010dd69aba15003375a6ae84d5d1280191999ab9a3370ea0049000119091180100198041aba135573ca00c464c6401866ae700340300280244d55cea80189aba25001135573ca00226ea80048c8c8cccd5cd19b875001480088c8488c00400cdd71aba135573ca00646666ae68cdc3a8012400046424460040066eb8d5d09aab9e500423263200933573801401200e00c26aae7540044dd500089119191999ab9a3370ea00290021091100091999ab9a3370ea00490011190911180180218031aba135573ca00846666ae68cdc3a801a400042444004464c6401466ae7002c02802001c0184d55cea80089baa0012323333573466e1d40052002200723333573466e1d40092000212200123263200633573800e00c00800626aae74dd5000a4c24002920103505431001220021123230010012233003300200200133351222335122335004335500248811c2b194b7d10a3d2d3152c5f3a628ff50cb9fc11e59453e8ac7a1aea4500488104544e4654005005112212330010030021120011122002122122330010040031200101").unwrap();
    let plutus_script2 = PlutusScript::from_hex("5907adaada00332323232323232323232323232323322323232323222232325335332201b3333573466e1cd55ce9baa0044800080608c98c8060cd5ce00c80c00b1999ab9a3370e6aae7540092000233221233001003002323232323232323232323232323333573466e1cd55cea8062400046666666666664444444444442466666666666600201a01801601401201000e00c00a00800600466a02a02c6ae854030cd4054058d5d0a80599a80a80b9aba1500a3335501975ca0306ae854024ccd54065d7280c1aba1500833501502035742a00e666aa032042eb4d5d0a8031919191999ab9a3370e6aae75400920002332212330010030023232323333573466e1cd55cea8012400046644246600200600466a056eb4d5d0a80118161aba135744a004464c6405c66ae700bc0b80b04d55cf280089baa00135742a0046464646666ae68cdc39aab9d5002480008cc8848cc00400c008cd40add69aba15002302c357426ae8940088c98c80b8cd5ce01781701609aab9e5001137540026ae84d5d1280111931901519ab9c02b02a028135573ca00226ea8004d5d0a80299a80abae35742a008666aa03203a40026ae85400cccd54065d710009aba15002301f357426ae8940088c98c8098cd5ce01381301209aba25001135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135744a00226aae7940044dd50009aba15002300f357426ae8940088c98c8060cd5ce00c80c00b080b89931900b99ab9c4910350543500017135573ca00226ea800448c88c008dd6000990009aa80a911999aab9f0012500a233500930043574200460066ae880080508c8c8cccd5cd19b8735573aa004900011991091980080180118061aba150023005357426ae8940088c98c8050cd5ce00a80a00909aab9e5001137540024646464646666ae68cdc39aab9d5004480008cccc888848cccc00401401000c008c8c8c8cccd5cd19b8735573aa0049000119910919800801801180a9aba1500233500f014357426ae8940088c98c8064cd5ce00d00c80b89aab9e5001137540026ae854010ccd54021d728039aba150033232323333573466e1d4005200423212223002004357426aae79400c8cccd5cd19b875002480088c84888c004010dd71aba135573ca00846666ae68cdc3a801a400042444006464c6403666ae7007006c06406005c4d55cea80089baa00135742a00466a016eb8d5d09aba2500223263201533573802c02a02626ae8940044d5d1280089aab9e500113754002266aa002eb9d6889119118011bab00132001355012223233335573e0044a010466a00e66442466002006004600c6aae754008c014d55cf280118021aba200301213574200222440042442446600200800624464646666ae68cdc3a800a40004642446004006600a6ae84d55cf280191999ab9a3370ea0049001109100091931900819ab9c01101000e00d135573aa00226ea80048c8c8cccd5cd19b875001480188c848888c010014c01cd5d09aab9e500323333573466e1d400920042321222230020053009357426aae7940108cccd5cd19b875003480088c848888c004014c01cd5d09aab9e500523333573466e1d40112000232122223003005375c6ae84d55cf280311931900819ab9c01101000e00d00c00b135573aa00226ea80048c8c8cccd5cd19b8735573aa004900011991091980080180118029aba15002375a6ae84d5d1280111931900619ab9c00d00c00a135573ca00226ea80048c8cccd5cd19b8735573aa002900011bae357426aae7940088c98c8028cd5ce00580500409baa001232323232323333573466e1d4005200c21222222200323333573466e1d4009200a21222222200423333573466e1d400d2008233221222222233001009008375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c4664424444444660040120106eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc8848888888cc018024020c030d5d0a8049bae357426ae8940248cccd5cd19b875006480088c848888888c01c020c034d5d09aab9e500b23333573466e1d401d2000232122222223005008300e357426aae7940308c98c804ccd5ce00a00980880800780700680600589aab9d5004135573ca00626aae7940084d55cf280089baa0012323232323333573466e1d400520022333222122333001005004003375a6ae854010dd69aba15003375a6ae84d5d1280191999ab9a3370ea0049000119091180100198041aba135573ca00c464c6401866ae700340300280244d55cea80189aba25001135573ca00226ea80048c8c8cccd5cd19b875001480088c8488c00400cdd71aba135573ca00646666ae68cdc3a8012400046424460040066eb8d5d09aab9e500423263200933573801401200e00c26aae7540044dd500089119191999ab9a3370ea00290021091100091999ab9a3370ea00490011190911180180218031aba135573ca00846666ae68cdc3a801a400042444004464c6401466ae7002c02802001c0184d55cea80089baa0012323333573466e1d40052002200723333573466e1d40092000212200123263200633573800e00c00800626aae74dd5000a4c24002920103505431001220021123230010012233003300200200133351222335122335004335500248811c2b194b7d10a3d2d3152c5f3a628ff50cb9fc11e59453e8ac7a1aea4500488104544e4654005005112212330010030021120011122002122122330010040031200101").unwrap();

    let redeemer = Redeemer::from_json(
        "\
         {
            \"tag\": \"Mint\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"1042996\",
              \"steps\": \"446100241\"
            }
          }",
    )
    .unwrap();

    let redeemer2 = Redeemer::from_json(
        "\
         {
            \"tag\": \"Mint\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"2929292\",
              \"steps\": \"446188888\"
            }
          }",
    )
    .unwrap();

    let asset_name = AssetName::from_hex("44544e4654").unwrap();
    let mut mint_builder = MintBuilder::new();
    let plutus_script_source = PlutusScriptSource::new(&plutus_script);
    let plutus_script_source_ref = PlutusScriptSource::new_ref_input_with_lang_ver(
        &plutus_script2.hash(),
        &tx_input_ref,
        &Language::new_plutus_v2(),
    );
    let mint_witnes = MintWitness::new_plutus_script(&plutus_script_source, &redeemer);
    let mint_witnes_ref = MintWitness::new_plutus_script(&plutus_script_source_ref, &redeemer2);
    mint_builder.add_asset(&mint_witnes, &asset_name, &Int::new(&BigNum::from(100u64)));
    mint_builder.add_asset(
        &mint_witnes_ref,
        &asset_name,
        &Int::new(&BigNum::from(100u64)),
    );

    let output_adress = Address::from_bech32("addr_test1qpm5njmgzf4t7225v6j34wl30xfrufzt3jtqtdzf3en9ahpmnhtmynpasyc8fq75zv0uaj86vzsr7g3g8q5ypgu5fwtqr9zsgj").unwrap();
    let mut output_assets = MultiAsset::new();
    let mut asset = Assets::new();
    asset.insert(&asset_name, &BigNum::from(100u64));
    output_assets.insert(&plutus_script.hash(), &asset);
    let output_value = Value::new_with_assets(&Coin::from(50000u64), &output_assets);
    let output = TransactionOutput::new(&output_adress, &output_value);

    let mut col_builder = TxInputsBuilder::new();
    col_builder.add_input(
        &colateral_adress,
        &colateral_input,
        &Value::new(&Coin::from(1000000000u64)),
    );
    tx_builder.set_collateral(&col_builder);
    tx_builder.add_output(&output);
    tx_builder.add_input(
        &output_adress,
        &tx_input,
        &Value::new(&BigNum::from(100000000000u64)),
    );
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
fn plutus_mint_defferent_redeemers_test() {
    let mut tx_builder = create_reallistic_tx_builder();
    let colateral_adress = Address::from_bech32("addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w").unwrap();
    let colateral_input = TransactionInput::from_json("\
          {
            \"transaction_id\": \"69b0b867056a2d4fdc3827e23aa7069b125935e2def774941ca8cc7f9e0de774\",
            \"index\": 1
          }").unwrap();

    let tx_input = TransactionInput::from_json("\
          {
            \"transaction_id\": \"f58a5bc761b1efdcf4b5684f6ad5495854a0d64b866e2f0f525d134750d3511b\",
            \"index\": 1
          }").unwrap();
    let plutus_script = PlutusScript::from_hex("5907d2010000332323232323232323232323232323322323232323222232325335332201b3333573466e1cd55ce9baa0044800080608c98c8060cd5ce00c80c00b1999ab9a3370e6aae7540092000233221233001003002323232323232323232323232323333573466e1cd55cea8062400046666666666664444444444442466666666666600201a01801601401201000e00c00a00800600466a02a02c6ae854030cd4054058d5d0a80599a80a80b9aba1500a3335501975ca0306ae854024ccd54065d7280c1aba1500833501502035742a00e666aa032042eb4d5d0a8031919191999ab9a3370e6aae75400920002332212330010030023232323333573466e1cd55cea8012400046644246600200600466a056eb4d5d0a80118161aba135744a004464c6405c66ae700bc0b80b04d55cf280089baa00135742a0046464646666ae68cdc39aab9d5002480008cc8848cc00400c008cd40add69aba15002302c357426ae8940088c98c80b8cd5ce01781701609aab9e5001137540026ae84d5d1280111931901519ab9c02b02a028135573ca00226ea8004d5d0a80299a80abae35742a008666aa03203a40026ae85400cccd54065d710009aba15002301f357426ae8940088c98c8098cd5ce01381301209aba25001135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135744a00226aae7940044dd50009aba15002300f357426ae8940088c98c8060cd5ce00c80c00b080b89931900b99ab9c4910350543500017135573ca00226ea800448c88c008dd6000990009aa80a911999aab9f0012500a233500930043574200460066ae880080508c8c8cccd5cd19b8735573aa004900011991091980080180118061aba150023005357426ae8940088c98c8050cd5ce00a80a00909aab9e5001137540024646464646666ae68cdc39aab9d5004480008cccc888848cccc00401401000c008c8c8c8cccd5cd19b8735573aa0049000119910919800801801180a9aba1500233500f014357426ae8940088c98c8064cd5ce00d00c80b89aab9e5001137540026ae854010ccd54021d728039aba150033232323333573466e1d4005200423212223002004357426aae79400c8cccd5cd19b875002480088c84888c004010dd71aba135573ca00846666ae68cdc3a801a400042444006464c6403666ae7007006c06406005c4d55cea80089baa00135742a00466a016eb8d5d09aba2500223263201533573802c02a02626ae8940044d5d1280089aab9e500113754002266aa002eb9d6889119118011bab00132001355012223233335573e0044a010466a00e66442466002006004600c6aae754008c014d55cf280118021aba200301213574200222440042442446600200800624464646666ae68cdc3a800a40004642446004006600a6ae84d55cf280191999ab9a3370ea0049001109100091931900819ab9c01101000e00d135573aa00226ea80048c8c8cccd5cd19b875001480188c848888c010014c01cd5d09aab9e500323333573466e1d400920042321222230020053009357426aae7940108cccd5cd19b875003480088c848888c004014c01cd5d09aab9e500523333573466e1d40112000232122223003005375c6ae84d55cf280311931900819ab9c01101000e00d00c00b135573aa00226ea80048c8c8cccd5cd19b8735573aa004900011991091980080180118029aba15002375a6ae84d5d1280111931900619ab9c00d00c00a135573ca00226ea80048c8cccd5cd19b8735573aa002900011bae357426aae7940088c98c8028cd5ce00580500409baa001232323232323333573466e1d4005200c21222222200323333573466e1d4009200a21222222200423333573466e1d400d2008233221222222233001009008375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c4664424444444660040120106eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc8848888888cc018024020c030d5d0a8049bae357426ae8940248cccd5cd19b875006480088c848888888c01c020c034d5d09aab9e500b23333573466e1d401d2000232122222223005008300e357426aae7940308c98c804ccd5ce00a00980880800780700680600589aab9d5004135573ca00626aae7940084d55cf280089baa0012323232323333573466e1d400520022333222122333001005004003375a6ae854010dd69aba15003375a6ae84d5d1280191999ab9a3370ea0049000119091180100198041aba135573ca00c464c6401866ae700340300280244d55cea80189aba25001135573ca00226ea80048c8c8cccd5cd19b875001480088c8488c00400cdd71aba135573ca00646666ae68cdc3a8012400046424460040066eb8d5d09aab9e500423263200933573801401200e00c26aae7540044dd500089119191999ab9a3370ea00290021091100091999ab9a3370ea00490011190911180180218031aba135573ca00846666ae68cdc3a801a400042444004464c6401466ae7002c02802001c0184d55cea80089baa0012323333573466e1d40052002200723333573466e1d40092000212200123263200633573800e00c00800626aae74dd5000a4c24002920103505431001220021123230010012233003300200200133351222335122335004335500248811c2b194b7d10a3d2d3152c5f3a628ff50cb9fc11e59453e8ac7a1aea4500488104544e4654005005112212330010030021120011122002122122330010040031200101").unwrap();

    let redeemer = Redeemer::from_json(
        "\
         {
            \"tag\": \"Mint\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"1042996\",
              \"steps\": \"446100241\"
            }
          }",
    )
    .unwrap();

    let redeemer2 = Redeemer::from_json(
        "\
         {
            \"tag\": \"Mint\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"2929292\",
              \"steps\": \"446188888\"
            }
          }",
    )
    .unwrap();

    let asset_name = AssetName::from_hex("44544e4654").unwrap();
    let mut mint_builder = MintBuilder::new();
    let plutus_script_source = PlutusScriptSource::new(&plutus_script);
    let mint_witnes = MintWitness::new_plutus_script(&plutus_script_source, &redeemer);
    let mint_witnes2 = MintWitness::new_plutus_script(&plutus_script_source, &redeemer2);
    mint_builder.add_asset(&mint_witnes, &asset_name, &Int::new(&BigNum::from(100u64)));
    mint_builder.add_asset(&mint_witnes2, &asset_name, &Int::new(&BigNum::from(100u64)));

    let output_adress = Address::from_bech32("addr_test1qpm5njmgzf4t7225v6j34wl30xfrufzt3jtqtdzf3en9ahpmnhtmynpasyc8fq75zv0uaj86vzsr7g3g8q5ypgu5fwtqr9zsgj").unwrap();
    let mut output_assets = MultiAsset::new();
    let mut asset = Assets::new();
    asset.insert(&asset_name, &BigNum::from(100u64));
    output_assets.insert(&plutus_script.hash(), &asset);
    let output_value = Value::new_with_assets(&Coin::from(1142150u64), &output_assets);
    let output = TransactionOutput::new(&output_adress, &output_value);

    let mut col_builder = TxInputsBuilder::new();
    col_builder.add_input(
        &colateral_adress,
        &colateral_input,
        &Value::new(&Coin::from(1000000000u64)),
    );
    tx_builder.set_collateral(&col_builder);
    tx_builder.add_output(&output).unwrap();
    tx_builder.add_input(
        &output_adress,
        &tx_input,
        &Value::new(&BigNum::from(100000000000u64)),
    );
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
    assert!(tx.body.reference_inputs.is_none());
    assert!(tx.body.mint.is_some());
    assert_eq!(tx.body.mint.unwrap().len(), 2usize);
}

#[test]
fn multiple_plutus_inputs_test() {
    let mut tx_builder = create_reallistic_tx_builder();
    let plutus_script = PlutusScript::from_hex("5907d2010000332323232323232323232323232323322323232323222232325335332201b3333573466e1cd55ce9baa0044800080608c98c8060cd5ce00c80c00b1999ab9a3370e6aae7540092000233221233001003002323232323232323232323232323333573466e1cd55cea8062400046666666666664444444444442466666666666600201a01801601401201000e00c00a00800600466a02a02c6ae854030cd4054058d5d0a80599a80a80b9aba1500a3335501975ca0306ae854024ccd54065d7280c1aba1500833501502035742a00e666aa032042eb4d5d0a8031919191999ab9a3370e6aae75400920002332212330010030023232323333573466e1cd55cea8012400046644246600200600466a056eb4d5d0a80118161aba135744a004464c6405c66ae700bc0b80b04d55cf280089baa00135742a0046464646666ae68cdc39aab9d5002480008cc8848cc00400c008cd40add69aba15002302c357426ae8940088c98c80b8cd5ce01781701609aab9e5001137540026ae84d5d1280111931901519ab9c02b02a028135573ca00226ea8004d5d0a80299a80abae35742a008666aa03203a40026ae85400cccd54065d710009aba15002301f357426ae8940088c98c8098cd5ce01381301209aba25001135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135744a00226aae7940044dd50009aba15002300f357426ae8940088c98c8060cd5ce00c80c00b080b89931900b99ab9c4910350543500017135573ca00226ea800448c88c008dd6000990009aa80a911999aab9f0012500a233500930043574200460066ae880080508c8c8cccd5cd19b8735573aa004900011991091980080180118061aba150023005357426ae8940088c98c8050cd5ce00a80a00909aab9e5001137540024646464646666ae68cdc39aab9d5004480008cccc888848cccc00401401000c008c8c8c8cccd5cd19b8735573aa0049000119910919800801801180a9aba1500233500f014357426ae8940088c98c8064cd5ce00d00c80b89aab9e5001137540026ae854010ccd54021d728039aba150033232323333573466e1d4005200423212223002004357426aae79400c8cccd5cd19b875002480088c84888c004010dd71aba135573ca00846666ae68cdc3a801a400042444006464c6403666ae7007006c06406005c4d55cea80089baa00135742a00466a016eb8d5d09aba2500223263201533573802c02a02626ae8940044d5d1280089aab9e500113754002266aa002eb9d6889119118011bab00132001355012223233335573e0044a010466a00e66442466002006004600c6aae754008c014d55cf280118021aba200301213574200222440042442446600200800624464646666ae68cdc3a800a40004642446004006600a6ae84d55cf280191999ab9a3370ea0049001109100091931900819ab9c01101000e00d135573aa00226ea80048c8c8cccd5cd19b875001480188c848888c010014c01cd5d09aab9e500323333573466e1d400920042321222230020053009357426aae7940108cccd5cd19b875003480088c848888c004014c01cd5d09aab9e500523333573466e1d40112000232122223003005375c6ae84d55cf280311931900819ab9c01101000e00d00c00b135573aa00226ea80048c8c8cccd5cd19b8735573aa004900011991091980080180118029aba15002375a6ae84d5d1280111931900619ab9c00d00c00a135573ca00226ea80048c8cccd5cd19b8735573aa002900011bae357426aae7940088c98c8028cd5ce00580500409baa001232323232323333573466e1d4005200c21222222200323333573466e1d4009200a21222222200423333573466e1d400d2008233221222222233001009008375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c4664424444444660040120106eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc8848888888cc018024020c030d5d0a8049bae357426ae8940248cccd5cd19b875006480088c848888888c01c020c034d5d09aab9e500b23333573466e1d401d2000232122222223005008300e357426aae7940308c98c804ccd5ce00a00980880800780700680600589aab9d5004135573ca00626aae7940084d55cf280089baa0012323232323333573466e1d400520022333222122333001005004003375a6ae854010dd69aba15003375a6ae84d5d1280191999ab9a3370ea0049000119091180100198041aba135573ca00c464c6401866ae700340300280244d55cea80189aba25001135573ca00226ea80048c8c8cccd5cd19b875001480088c8488c00400cdd71aba135573ca00646666ae68cdc3a8012400046424460040066eb8d5d09aab9e500423263200933573801401200e00c26aae7540044dd500089119191999ab9a3370ea00290021091100091999ab9a3370ea00490011190911180180218031aba135573ca00846666ae68cdc3a801a400042444004464c6401466ae7002c02802001c0184d55cea80089baa0012323333573466e1d40052002200723333573466e1d40092000212200123263200633573800e00c00800626aae74dd5000a4c24002920103505431001220021123230010012233003300200200133351222335122335004335500248811c2b194b7d10a3d2d3152c5f3a628ff50cb9fc11e59453e8ac7a1aea4500488104544e4654005005112212330010030021120011122002122122330010040031200101").unwrap();
    let redeemer1 = Redeemer::from_json(
        "\
         {
            \"tag\": \"Mint\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"1042996\",
              \"steps\": \"446100241\"
            }
          }",
    )
    .unwrap();

    let redeemer2 = Redeemer::from_json(
        "\
         {
            \"tag\": \"Mint\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"1042996\",
              \"steps\": \"446100241\"
            }
          }",
    )
    .unwrap();

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
    col_builder.add_input(
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
fn multiple_plutus_inputs_with_missed_wit_test() {
    let mut tx_builder = create_reallistic_tx_builder();
    let plutus_script = PlutusScript::from_hex("5907d2010000332323232323232323232323232323322323232323222232325335332201b3333573466e1cd55ce9baa0044800080608c98c8060cd5ce00c80c00b1999ab9a3370e6aae7540092000233221233001003002323232323232323232323232323333573466e1cd55cea8062400046666666666664444444444442466666666666600201a01801601401201000e00c00a00800600466a02a02c6ae854030cd4054058d5d0a80599a80a80b9aba1500a3335501975ca0306ae854024ccd54065d7280c1aba1500833501502035742a00e666aa032042eb4d5d0a8031919191999ab9a3370e6aae75400920002332212330010030023232323333573466e1cd55cea8012400046644246600200600466a056eb4d5d0a80118161aba135744a004464c6405c66ae700bc0b80b04d55cf280089baa00135742a0046464646666ae68cdc39aab9d5002480008cc8848cc00400c008cd40add69aba15002302c357426ae8940088c98c80b8cd5ce01781701609aab9e5001137540026ae84d5d1280111931901519ab9c02b02a028135573ca00226ea8004d5d0a80299a80abae35742a008666aa03203a40026ae85400cccd54065d710009aba15002301f357426ae8940088c98c8098cd5ce01381301209aba25001135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135744a00226aae7940044dd50009aba15002300f357426ae8940088c98c8060cd5ce00c80c00b080b89931900b99ab9c4910350543500017135573ca00226ea800448c88c008dd6000990009aa80a911999aab9f0012500a233500930043574200460066ae880080508c8c8cccd5cd19b8735573aa004900011991091980080180118061aba150023005357426ae8940088c98c8050cd5ce00a80a00909aab9e5001137540024646464646666ae68cdc39aab9d5004480008cccc888848cccc00401401000c008c8c8c8cccd5cd19b8735573aa0049000119910919800801801180a9aba1500233500f014357426ae8940088c98c8064cd5ce00d00c80b89aab9e5001137540026ae854010ccd54021d728039aba150033232323333573466e1d4005200423212223002004357426aae79400c8cccd5cd19b875002480088c84888c004010dd71aba135573ca00846666ae68cdc3a801a400042444006464c6403666ae7007006c06406005c4d55cea80089baa00135742a00466a016eb8d5d09aba2500223263201533573802c02a02626ae8940044d5d1280089aab9e500113754002266aa002eb9d6889119118011bab00132001355012223233335573e0044a010466a00e66442466002006004600c6aae754008c014d55cf280118021aba200301213574200222440042442446600200800624464646666ae68cdc3a800a40004642446004006600a6ae84d55cf280191999ab9a3370ea0049001109100091931900819ab9c01101000e00d135573aa00226ea80048c8c8cccd5cd19b875001480188c848888c010014c01cd5d09aab9e500323333573466e1d400920042321222230020053009357426aae7940108cccd5cd19b875003480088c848888c004014c01cd5d09aab9e500523333573466e1d40112000232122223003005375c6ae84d55cf280311931900819ab9c01101000e00d00c00b135573aa00226ea80048c8c8cccd5cd19b8735573aa004900011991091980080180118029aba15002375a6ae84d5d1280111931900619ab9c00d00c00a135573ca00226ea80048c8cccd5cd19b8735573aa002900011bae357426aae7940088c98c8028cd5ce00580500409baa001232323232323333573466e1d4005200c21222222200323333573466e1d4009200a21222222200423333573466e1d400d2008233221222222233001009008375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c4664424444444660040120106eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc8848888888cc018024020c030d5d0a8049bae357426ae8940248cccd5cd19b875006480088c848888888c01c020c034d5d09aab9e500b23333573466e1d401d2000232122222223005008300e357426aae7940308c98c804ccd5ce00a00980880800780700680600589aab9d5004135573ca00626aae7940084d55cf280089baa0012323232323333573466e1d400520022333222122333001005004003375a6ae854010dd69aba15003375a6ae84d5d1280191999ab9a3370ea0049000119091180100198041aba135573ca00c464c6401866ae700340300280244d55cea80189aba25001135573ca00226ea80048c8c8cccd5cd19b875001480088c8488c00400cdd71aba135573ca00646666ae68cdc3a8012400046424460040066eb8d5d09aab9e500423263200933573801401200e00c26aae7540044dd500089119191999ab9a3370ea00290021091100091999ab9a3370ea00490011190911180180218031aba135573ca00846666ae68cdc3a801a400042444004464c6401466ae7002c02802001c0184d55cea80089baa0012323333573466e1d40052002200723333573466e1d40092000212200123263200633573800e00c00800626aae74dd5000a4c24002920103505431001220021123230010012233003300200200133351222335122335004335500248811c2b194b7d10a3d2d3152c5f3a628ff50cb9fc11e59453e8ac7a1aea4500488104544e4654005005112212330010030021120011122002122122330010040031200101").unwrap();
    let redeemer1 = Redeemer::from_json(
        "\
         {
            \"tag\": \"Mint\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"1042996\",
              \"steps\": \"446100241\"
            }
          }",
    )
    .unwrap();

    let redeemer2 = Redeemer::from_json(
        "\
         {
            \"tag\": \"Mint\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"1042996\",
              \"steps\": \"446100241\"
            }
          }",
    )
    .unwrap();

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
    let output_value = Value::new(&Coin::from(5000000u64));
    let output = TransactionOutput::new(&output_adress, &output_value);

    tx_builder.add_output(&output).unwrap();
    let mut col_builder = TxInputsBuilder::new();
    col_builder.add_input(
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
    let script_addr = create_base_address_from_script_hash(&plutus_script.hash());
    in_builder.add_input(&script_addr, &input_2, &value);

    assert_eq!(in_builder.count_missing_input_scripts(), 1usize);
    let mut inputs_with_wit = InputsWithScriptWitness::new();
    let in_with_wit = InputWithScriptWitness::new_with_plutus_witness(&input_2, &plutus_wit2);
    inputs_with_wit.add(&in_with_wit);
    in_builder.add_required_script_input_witnesses(&inputs_with_wit);

    tx_builder.set_inputs(&in_builder);

    tx_builder
        .calc_script_data_hash(&TxBuilderConstants::plutus_vasil_cost_models())
        .unwrap();
    tx_builder.add_change_if_needed(&output_adress).unwrap();
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

    let redeemer_cert1 = Redeemer::from_json(
        "\
         {
            \"tag\": \"Spend\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"1\",
              \"steps\": \"1\"
            }
          }",
    )
    .unwrap();

    let redeemer_cert2 = Redeemer::from_json(
        "\
         {
            \"tag\": \"Spend\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"2\",
              \"steps\": \"2\"
            }
          }",
    )
    .unwrap();

    let redeemer_cert3 = Redeemer::from_json(
        "\
         {
            \"tag\": \"Spend\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"2\",
              \"steps\": \"2\"
            }
          }",
    )
    .unwrap();

    let redeemer_withdraw1 = Redeemer::from_json(
        "\
         {
            \"tag\": \"Spend\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"4\",
              \"steps\": \"4\"
            }
          }",
    )
    .unwrap();

    let redeemer_withdraw2 = Redeemer::from_json(
        "\
         {
            \"tag\": \"Spend\",
            \"index\": \"0\",
            \"data\": \"{\\\"constructor\\\":0,\\\"fields\\\":[]}\",
            \"ex_units\": {
              \"mem\": \"5\",
              \"steps\": \"5\"
            }
          }",
    )
    .unwrap();

    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    tx_builder.add_key_input(
        &spend.to_raw_key().hash(),
        &TransactionInput::new(&genesis_id(), 0),
        &Value::new(&to_bignum(5_000_000)),
    );
    tx_builder.set_ttl(1000);
    let (cert_script1, cert_script_hash1) = plutus_script_and_hash(1);
    let cert_script_cred1 = Credential::from_scripthash(&cert_script_hash1);

    let (cert_script2, cert_script_hash2) = plutus_script_and_hash(2);
    let cert_script_cred2 = Credential::from_scripthash(&cert_script_hash2);

    let cert_script_hash3 = fake_script_hash(3);
    let cert_script_cred3 = Credential::from_scripthash(&cert_script_hash3);

    let (withdraw_script1, withdraw_script_hash1) = plutus_script_and_hash(3);
    let withdraw_script_cred1 = Credential::from_scripthash(&withdraw_script_hash1);

    let withdraw_script_hash2 = fake_script_hash(3);
    let withdraw_script_cred2 = Credential::from_scripthash(&withdraw_script_hash2);

    let cert_witness_1 = PlutusWitness::new_without_datum(&cert_script1, &redeemer_cert1);
    let cert_witness_2 = PlutusWitness::new_without_datum(&cert_script2, &redeemer_cert2);

    let ref_cert_script_input_3 = fake_tx_input(1);
    let ref_cert_withdrawal_input_2 = fake_tx_input(2);
    let plutus_cert_source = PlutusScriptSource::new_ref_input_with_lang_ver(
        &cert_script_hash3,
        &ref_cert_script_input_3,
        &Language::new_plutus_v2(),
    );
    let plutus_withdrawal_source = PlutusScriptSource::new_ref_input_with_lang_ver(
        &withdraw_script_hash2,
        &ref_cert_withdrawal_input_2,
        &Language::new_plutus_v2(),
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
            &RewardAddress::new(NetworkInfo::testnet().network_id(), &reward_cred),
            &Coin::from(1u32),
        )
        .unwrap();
    withdrawals
        .add_with_plutus_witness(
            &RewardAddress::new(NetworkInfo::testnet().network_id(), &withdraw_script_cred1),
            &Coin::from(2u32),
            &withdraw_witness1,
        )
        .unwrap();
    withdrawals
        .add_with_plutus_witness(
            &RewardAddress::new(NetworkInfo::testnet().network_id(), &withdraw_script_cred2),
            &Coin::from(3u32),
            &withdraw_witness2,
        )
        .unwrap();
    tx_builder.set_withdrawals_builder(&withdrawals);

    let change_cred = Credential::from_keyhash(&change_key.to_raw_key().hash());
    let change_addr = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
        &change_cred,
        &stake_cred,
    )
    .to_address();
    let cost_models = TxBuilderConstants::plutus_default_cost_models();
    let collateral_input = fake_tx_input(1);
    let collateral_addr = BaseAddress::new(
        NetworkInfo::testnet().network_id(),
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Credential::from_keyhash(&fake_key_hash(2)),
    )
    .to_address();
    let mut collateral_builder = TxInputsBuilder::new();
    collateral_builder.add_input(
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
    for redeemer in &redeemers.0 {
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
    let mut tx_builder = create_reallistic_tx_builder();
    tx_builder.set_fee(&to_bignum(42));

    let datum = PlutusData::new_bytes(fake_bytes_32(1));
    tx_builder.add_extra_witness_datum(&datum);

    let mut inp = TxInputsBuilder::new();
    inp.add_input(
        &fake_base_address(0),
        &fake_tx_input(0),
        &Value::new(&to_bignum(1000000u64)),
    );

    tx_builder.set_inputs(&inp);
    tx_builder
        .calc_script_data_hash(&TxBuilderConstants::plutus_default_cost_models())
        .unwrap();

    let res = tx_builder.build_tx();
    assert!(res.is_ok());

    let tx = res.unwrap();

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
    builder.add_change_if_needed(&create_change_address()).unwrap();

    assert_eq!(builder.get_current_treasury_value().unwrap(), treasure_value);

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
    builder.add_change_if_needed(&create_change_address()).unwrap();

    assert_eq!(builder.get_donation().unwrap(), donation);

    let tx = builder.build_tx().unwrap();
    assert_eq!(tx.body().outputs().len(), 1);

    let mut total_out = tx.body().outputs().get(0).amount().coin();
    total_out = total_out.checked_add(&tx.body().fee()).unwrap();
    total_out = total_out.checked_add(&donation).unwrap();

    assert_eq!(total_out, Coin::from(input_amount));
}