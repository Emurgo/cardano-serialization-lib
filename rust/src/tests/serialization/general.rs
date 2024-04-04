use crate::{
    to_bignum, Address, BigInt, BigNum, Block, BlockHash, CborContainerType, Coin, Credential,
    DataHash, ExUnits, HeaderBody, HeaderLeaderCertEnum, Int, KESVKey, MIRPot,
    MIRToStakeCredentials, MoveInstantaneousReward, NativeScript, OperationalCert, PlutusData,
    PlutusList, PlutusScript, PlutusScripts, ProtocolVersion, Redeemer, RedeemerTag, Redeemers,
    ScriptHash, ScriptRef, TimelockStart, TransactionBody, TransactionInputs, TransactionOutput,
    TransactionOutputs, TransactionWitnessSet, VRFCert, VRFVKey, Value, Vkeywitness, Vkeywitnesses,
};

use crate::fakes::{
    fake_base_address, fake_bytes_32, fake_data_hash, fake_signature, fake_tx_input,
    fake_tx_output, fake_value, fake_value2, fake_vkey,
};

#[test]
fn tx_output_deser_lagacy() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    txo_dh.set_data_hash(&DataHash::from([47u8; DataHash::BYTE_COUNT]));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let bytes = txos.to_bytes();
    let txos_deser = TransactionOutputs::from_bytes(bytes.clone()).unwrap();
    let bytes_deser = txos_deser.to_bytes();
    assert_eq!(bytes, bytes_deser);
}

#[test]
fn tx_output_deser_post_alonzo_with_plutus_script_and_datum() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    txo_dh.set_plutus_data(&PlutusData::new_bytes(fake_bytes_32(11)));
    txo_dh.set_script_ref(&ScriptRef::new_plutus_script(&PlutusScript::new(
        [61u8; 29].to_vec(),
    )));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let bytes = txos.to_bytes();
    let txos_deser = TransactionOutputs::from_bytes(bytes.clone()).unwrap();
    let bytes_deser = txos_deser.to_bytes();
    assert_eq!(bytes, bytes_deser);
}

#[test]
fn tx_output_deser_post_alonzo_with_plutus_script() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    txo_dh.set_script_ref(&ScriptRef::new_plutus_script(&PlutusScript::new(
        [61u8; 29].to_vec(),
    )));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let bytes = txos.to_bytes();
    let txos_deser = TransactionOutputs::from_bytes(bytes.clone()).unwrap();
    let bytes_deser = txos_deser.to_bytes();
    assert_eq!(bytes, bytes_deser);
}

#[test]
fn tx_output_deser_post_alonzo_with_datum() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    txo_dh.set_plutus_data(&PlutusData::new_bytes(fake_bytes_32(11)));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let bytes = txos.to_bytes();
    let txos_deser = TransactionOutputs::from_bytes(bytes.clone()).unwrap();
    let bytes_deser = txos_deser.to_bytes();
    assert_eq!(bytes, bytes_deser);
}

#[test]
fn tx_output_deser_post_alonzo_with_native_script_and_datum() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    let native_script = NativeScript::new_timelock_start(&TimelockStart::new(20));
    txo_dh.set_script_ref(&ScriptRef::new_native_script(&native_script));
    txo_dh.set_plutus_data(&PlutusData::new_bytes(fake_bytes_32(11)));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let bytes = txos.to_bytes();
    let txos_deser = TransactionOutputs::from_bytes(bytes.clone()).unwrap();
    let bytes_deser = txos_deser.to_bytes();
    assert_eq!(bytes, bytes_deser);
}

#[test]
fn tx_output_deser_post_alonzo_with_native_script() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    let native_script = NativeScript::new_timelock_start(&TimelockStart::new(20));
    txo_dh.set_script_ref(&ScriptRef::new_native_script(&native_script));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let bytes = txos.to_bytes();
    let txos_deser = TransactionOutputs::from_bytes(bytes.clone()).unwrap();
    let bytes_deser = txos_deser.to_bytes();
    assert_eq!(bytes, bytes_deser);
}

#[test]
fn tx_output_deser_post_alonzo_with_native_script_and_data_hash() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    let native_script = NativeScript::new_timelock_start(&TimelockStart::new(20));
    let data_hash = DataHash::from_bytes(vec![
        201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218,
        219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232,
    ])
    .unwrap();
    txo_dh.set_data_hash(&data_hash);
    txo_dh.set_script_ref(&ScriptRef::new_native_script(&native_script));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let bytes = txos.to_bytes();
    let txos_deser = TransactionOutputs::from_bytes(bytes.clone()).unwrap();
    let bytes_deser = txos_deser.to_bytes();
    assert_eq!(bytes, bytes_deser);
}

#[test]
fn tx_output_deser_lagacy_json() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    txo_dh.set_data_hash(&DataHash::from([47u8; DataHash::BYTE_COUNT]));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let json_txos = txos.to_json().unwrap();
    let deser_txos = TransactionOutputs::from_json(json_txos.as_str()).unwrap();

    assert_eq!(deser_txos.to_bytes(), txos.to_bytes());
    assert_eq!(deser_txos.to_json().unwrap(), txos.to_json().unwrap());
}

#[test]
fn tx_output_deser_post_alonzo_with_plutus_script_and_datum_json() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    txo_dh.set_plutus_data(&PlutusData::new_bytes(fake_bytes_32(11)));
    txo_dh.set_script_ref(&ScriptRef::new_plutus_script(&PlutusScript::new(
        [61u8; 29].to_vec(),
    )));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let json_txos = txos.to_json().unwrap();
    let deser_txos = TransactionOutputs::from_json(json_txos.as_str()).unwrap();

    assert_eq!(deser_txos.to_bytes(), txos.to_bytes());
    assert_eq!(deser_txos.to_json().unwrap(), txos.to_json().unwrap());
}

#[test]
fn tx_output_deser_post_alonzo_with_plutus_script_json() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    txo_dh.set_script_ref(&ScriptRef::new_plutus_script(&PlutusScript::new(
        [61u8; 29].to_vec(),
    )));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let json_txos = txos.to_json().unwrap();
    let deser_txos = TransactionOutputs::from_json(json_txos.as_str()).unwrap();

    assert_eq!(deser_txos.to_bytes(), txos.to_bytes());
    assert_eq!(deser_txos.to_json().unwrap(), txos.to_json().unwrap());
}

#[test]
fn tx_output_deser_post_alonzo_with_datum_json() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    txo_dh.set_plutus_data(&PlutusData::new_bytes(fake_bytes_32(11)));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let json_txos = txos.to_json().unwrap();
    let deser_txos = TransactionOutputs::from_json(json_txos.as_str()).unwrap();

    assert_eq!(deser_txos.to_bytes(), txos.to_bytes());
    assert_eq!(deser_txos.to_json().unwrap(), txos.to_json().unwrap());
}

#[test]
fn tx_output_deser_post_alonzo_with_native_script_and_datum_json() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    let native_script = NativeScript::new_timelock_start(&TimelockStart::new(20));
    txo_dh.set_script_ref(&ScriptRef::new_native_script(&native_script));
    txo_dh.set_plutus_data(&PlutusData::new_bytes(fake_bytes_32(11)));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let json_txos = txos.to_json().unwrap();
    let deser_txos = TransactionOutputs::from_json(json_txos.as_str()).unwrap();

    assert_eq!(deser_txos.to_bytes(), txos.to_bytes());
    assert_eq!(deser_txos.to_json().unwrap(), txos.to_json().unwrap());
}

#[test]
fn tx_output_deser_post_alonzo_with_native_script_json() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    let native_script = NativeScript::new_timelock_start(&TimelockStart::new(20));
    txo_dh.set_script_ref(&ScriptRef::new_native_script(&native_script));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let json_txos = txos.to_json().unwrap();
    let deser_txos = TransactionOutputs::from_json(json_txos.as_str()).unwrap();

    assert_eq!(deser_txos.to_bytes(), txos.to_bytes());
    assert_eq!(deser_txos.to_json().unwrap(), txos.to_json().unwrap());
}

#[test]
fn tx_output_deser_post_alonzo_with_native_script_and_data_hash_json() {
    let mut txos = TransactionOutputs::new();
    let addr = Address::from_bech32("addr1qyxwnq9kylzrtqprmyu35qt8gwylk3eemq53kqd38m9kyduv2q928esxmrz4y5e78cvp0nffhxklfxsqy3vdjn3nty9s8zygkm").unwrap();
    let val = &Value::new(&BigNum::from_str("435464757").unwrap());
    let txo = TransactionOutput {
        address: addr.clone(),
        amount: val.clone(),
        plutus_data: None,
        script_ref: None,
        serialization_format: None,
    };
    let mut txo_dh = txo.clone();
    let native_script = NativeScript::new_timelock_start(&TimelockStart::new(20));
    let data_hash = DataHash::from_bytes(vec![
        201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218,
        219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232,
    ])
    .unwrap();
    txo_dh.set_data_hash(&data_hash);
    txo_dh.set_script_ref(&ScriptRef::new_native_script(&native_script));
    txos.add(&txo);
    txos.add(&txo_dh);
    txos.add(&txo_dh);
    txos.add(&txo);
    txos.add(&txo);
    txos.add(&txo_dh);
    let json_txos = txos.to_json().unwrap();
    let deser_txos = TransactionOutputs::from_json(json_txos.as_str()).unwrap();

    assert_eq!(deser_txos.to_bytes(), txos.to_bytes());
    assert_eq!(deser_txos.to_json().unwrap(), txos.to_json().unwrap());
}

#[test]
fn mir_deser() {
    let reserves_to_pot = MoveInstantaneousReward::new_to_other_pot(
        MIRPot::Treasury,
        &Coin::from_str("143546464").unwrap(),
    );
    let reserves_to_pot_deser =
        MoveInstantaneousReward::from_bytes(reserves_to_pot.to_bytes()).unwrap();
    assert_eq!(reserves_to_pot.to_bytes(), reserves_to_pot_deser.to_bytes());
    let treasury_to_pot =
        MoveInstantaneousReward::new_to_other_pot(MIRPot::Treasury, &Coin::from_str("0").unwrap());
    let treasury_to_pot_deser =
        MoveInstantaneousReward::from_bytes(treasury_to_pot.to_bytes()).unwrap();
    assert_eq!(treasury_to_pot.to_bytes(), treasury_to_pot_deser.to_bytes());
    let mut stake_creds = MIRToStakeCredentials::new();
    stake_creds.insert(
        &Credential::from_scripthash(&ScriptHash([54u8; ScriptHash::BYTE_COUNT])),
        &Int::new_i32(-314159265),
    );
    let to_stake_creds =
        MoveInstantaneousReward::new_to_stake_creds(MIRPot::Treasury, &stake_creds);
    let to_stake_creds_deser =
        MoveInstantaneousReward::from_bytes(to_stake_creds.to_bytes()).unwrap();
    assert_eq!(to_stake_creds.to_bytes(), to_stake_creds_deser.to_bytes());
}

#[test]
#[ignore]
fn alonzo_block() {
    // this test for some reason has 2-byte pool metadata hashes so don't run this without changing that
    let bytes = hex::decode("85828f03095820bb30a42c1e62f0afda5f0a4e8a562f7a13a24cea00ee81917b86b89e801314aa58208a88e3dd7409f195fd52db2d3cba5d72ca6709bf1d94121bf3748801b40f6f5c58208a88e3dd7409f195fd52db2d3cba5d72ca6709bf1d94121bf3748801b40f6f5c8258404fefc7c718693b57c87170ceba220382afbdd148c0a53b4a009ca63ad1f101483a6170c83a77f23d362a68dcb502802df7f98fa4f7a78b4082b211530e1234305850f770f6769ae9871d42b970fc6254bb927c2181fff45897f241bd72221d86d33c8df64c0a3c8cbb9aa52fef191d7202465c52df8d33727a38c7dc5d40864d753348a340f8afcbb3bb05d4a03f16b1080d825840fe682775f0fa232e909ddc9ec3210ea7a0ee6514cd8b0815190a08f7cef3985463152e10dfad9ed6c09b641b6c1824498e77814a7c12e03096a63cd62056446358500951ed3ef2065e4196d008b50a63bb3e2bdc9a64df67eff4e230b35429291def476684114e074357a5c834bf79eacf583b6fe9fcd1d17f3719a31de97aa4da5e4630b05421359e0b6e4a9bd76c6b920b190929582010c865acec05c84c2c0d0b889f7dbe9bf3b5561f8552da1eb286eac4ccdabc5e5820d298da3803eb9958f01c02e73f2410f2f9bb2ecbc346526b1b76772e1bdd7db500005840940f8a3696847e4a238705bdd27a345086282067b9bc5cb7b69847ca8756085844d576f59ab056c169a504320cc1eab4c11fd529482b3c57da6fa96d44635b0802005901c0a1b2ee63b357fe0b19c6bb8dc3fc865c0608a89626505c5f9aff3b74a0809ca2635e0c4235c247306987c7fd76a4a06210ebf74178e72a1faa78fb8865a69005cc6a5ab5c9b40f817c715df558af7d07b6186f0ccf31715ec2fb00980730ac166af657e6670608afe1bf651d496e01b1c7ff1eb44614d8cfd1b7e32b2c2939349236cc0ada145d8d8d7ad919ef1e60c8bbad31dbedf9f395849705a00c14a8785106aae31f55abc5b1f2089cbef16d9401f158704c1e4f740f7125cfc700a99d97d0332eacb33e4bbc8dab2872ec2b3df9e113addaebd156bfc64fdfc732614d2aedd10a58a34993b7b08c822af3aa615b6bbb9b267bc902e4f1075e194aed084ca18f8bcde1a6b094bf3f5295a0d454c0a083ed5b74f7092fc0a7346c03979a30eeea76d686e512ba48d21544ba874886cdd166cbf275b11f1f3881f4c4277c09a24b88fc6168f4578267bdc9d62cb9b78b8dfc888ccce226a177725f39e7d50930552861d1e88b7898971c780dc3b773321ba1854422b5cecead7d50e77783050eeae2cd9595b9cd91681c72e5d53bb7d12f28dec9b2847ee70a3d7781fb1133aea3b169f536ff5945ec0a76950e51beded0627bb78120617a2f0842e50e3981ae0081825820ee155ace9c40292074cb6aff8c9ccdd273c81648ff1149ef36bcea6ebb8a3e25000d81825820bb30a42c1e62f0afda5f0a4e8a562f7a13a24cea00ee81917b86b89e801314aa01018183583900cb9358529df4729c3246a2a033cb9821abbfd16de4888005904abc410d6a577e9441ad8ed9663931906e4d43ece8f82c712b1d0235affb06821864a1581ca646474b8f5431261506b6c273d307c7569a4eb6c96b42dd4a29520aa14a636f75747473436f696e1903e85820ee155ace9c40292074cb6aff8c9ccdd273c81648ff1149ef36bcea6ebb8a3e25021903e70304048382008200581c0d6a577e9441ad8ed9663931906e4d43ece8f82c712b1d0235affb068a03581c0d6a577e9441ad8ed9663931906e4d43ece8f82c712b1d0235affb065820c5e21ab1c9f6022d81c3b25e3436cb7f1df77f9652ae3e1310c28e621dd87b4c0105d81e82010a581de00d6a577e9441ad8ed9663931906e4d43ece8f82c712b1d0235affb0681581c0d6a577e9441ad8ed9663931906e4d43ece8f82c712b1d0235affb0680826e636f6e73656e7375732e706f6f6c427b7d82068200a18200581c008b47844d92812fc30d1f0ac9b6fbf38778ccba9db8312ad9079079186e05a1581de00d6a577e9441ad8ed9663931906e4d43ece8f82c712b1d0235affb0618640682a1581ce0a714319812c3f773ba04ec5d6b3ffcd5aad85006805b047b082541a104190fa00008020e81581cf81ce66e0f52da5ca48193386e7511fde5b030a307b4c3681736c6f009a1581cb16b56f5ec064be6ac3cab6035efae86b366cc3dc4a0d571603d70e5a14a636f75747473436f696e1903e80b58209e1199a988ba72ffd6e9c269cadb3b53b5f360ff99f112d9b2ee30c4d74ad88b0758209e1199a988ba72ffd6e9c269cadb3b53b5f360ff99f112d9b2ee30c4d74ad88b0f0181a400818258203b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da295840815671b581b4b02a30108a799a85c7f2e5487fb667e748e8fde59e466ab987ce133ecb77ffa0dc53c5804e6706e26b94e17803235da28112bc747de48ccbd70903814c4b0100002002002002000011048118bf058184000019039782191388191388a100d90103a300a40166737472696e67024562797465730382010204a10341620181820180028148470100002002006180").unwrap();
    let block = Block::from_bytes(bytes).unwrap();
    let block2 = Block::from_bytes(block.to_bytes()).unwrap();
    assert_eq!(block.to_bytes(), block2.to_bytes());
}

#[test]
fn test_tx_body_roundtrip() {
    let mut txb = TransactionBody::new(
        &TransactionInputs(vec![fake_tx_input(0)]),
        &TransactionOutputs(vec![fake_tx_output(1)]),
        &to_bignum(1234567),
        Some(12345678),
    );

    txb.set_collateral_return(&fake_tx_output(2));
    txb.set_total_collateral(&to_bignum(1234));

    let txb2 = TransactionBody::from_bytes(txb.to_bytes()).unwrap();
    assert_eq!(txb, txb2);
}

#[test]
fn test_header_body_roundtrip() {
    fn fake_header_body(leader_cert: HeaderLeaderCertEnum) -> HeaderBody {
        HeaderBody {
            block_number: 123,
            slot: to_bignum(123),
            prev_hash: Some(BlockHash::from_bytes(fake_bytes_32(1)).unwrap()),
            issuer_vkey: fake_vkey(),
            vrf_vkey: VRFVKey::from_bytes(fake_bytes_32(2)).unwrap(),
            leader_cert,
            block_body_size: 123456,
            block_body_hash: BlockHash::from_bytes(fake_bytes_32(4)).unwrap(),
            operational_cert: OperationalCert::new(
                &KESVKey::from_bytes(fake_bytes_32(5)).unwrap(),
                123,
                456,
                &fake_signature(6),
            ),
            protocol_version: ProtocolVersion::new(12, 13),
        }
    }

    let hbody1 = fake_header_body(HeaderLeaderCertEnum::VrfResult(
        VRFCert::new(fake_bytes_32(3), [0; 80].to_vec()).unwrap(),
    ));

    assert_eq!(hbody1, HeaderBody::from_bytes(hbody1.to_bytes()).unwrap());

    let hbody2 = fake_header_body(HeaderLeaderCertEnum::NonceAndLeader(
        VRFCert::new(fake_bytes_32(4), [1; 80].to_vec()).unwrap(),
        VRFCert::new(fake_bytes_32(5), [2; 80].to_vec()).unwrap(),
    ));

    assert_eq!(hbody2, HeaderBody::from_bytes(hbody2.to_bytes()).unwrap());
}

#[test]
fn test_witness_set_roundtrip() {
    fn witness_set_roundtrip(plutus_scripts: &PlutusScripts) {
        let mut ws = TransactionWitnessSet::new();
        ws.set_vkeys(&Vkeywitnesses::from_vec(vec![Vkeywitness::new(
            &fake_vkey(),
            &fake_signature(1),
        )]));
        ws.set_redeemers(&Redeemers::from(vec![Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(12),
            &PlutusData::new_integer(&BigInt::one()),
            &ExUnits::new(&to_bignum(123), &to_bignum(456)),
        )]));
        ws.set_plutus_data(&PlutusList::from(vec![PlutusData::new_integer(
            &BigInt::one(),
        )]));
        ws.set_plutus_scripts(plutus_scripts);

        assert_eq!(
            TransactionWitnessSet::from_bytes(ws.to_bytes()).unwrap(),
            ws
        );
    }

    let bytes = hex::decode("4e4d01000033222220051200120011").unwrap();
    let script_v1 = PlutusScript::from_bytes(bytes.clone()).unwrap();
    let script_v2 = PlutusScript::from_bytes_v2(bytes.clone()).unwrap();
    let script_v3 = PlutusScript::from_bytes_v3(bytes.clone()).unwrap();

    witness_set_roundtrip(&PlutusScripts(vec![]));
    witness_set_roundtrip(&PlutusScripts(vec![script_v1.clone()]));
    witness_set_roundtrip(&PlutusScripts(vec![script_v2.clone()]));
    witness_set_roundtrip(&PlutusScripts(vec![script_v3.clone()]));
    witness_set_roundtrip(&PlutusScripts(vec![script_v1.clone(), script_v2.clone()]));
    witness_set_roundtrip(&PlutusScripts(vec![
        script_v1.clone(),
        script_v2.clone(),
        script_v3.clone(),
    ]));
}

#[test]
fn test_script_ref_roundtrip() {
    let ref0 = ScriptRef::new_native_script(&NativeScript::new_timelock_start(
        &TimelockStart::new(123456),
    ));
    assert_eq!(ScriptRef::from_bytes(ref0.to_bytes()).unwrap(), ref0);

    let bytes = hex::decode("4e4d01000033222220051200120011").unwrap();
    let script_v1 = PlutusScript::from_bytes(bytes.clone()).unwrap();
    let script_v2 = PlutusScript::from_bytes_v2(bytes.clone()).unwrap();
    let script_v3 = PlutusScript::from_bytes_v3(bytes.clone()).unwrap();

    let ref1 = ScriptRef::new_plutus_script(&script_v1);
    assert_eq!(ScriptRef::from_bytes(ref1.to_bytes()).unwrap(), ref1);

    let ref2 = ScriptRef::new_plutus_script(&script_v2);
    assert_eq!(ScriptRef::from_bytes(ref2.to_bytes()).unwrap(), ref2);

    let ref3 = ScriptRef::new_plutus_script(&script_v3);
    assert_eq!(ScriptRef::from_bytes(ref3.to_bytes()).unwrap(), ref3);
}

#[test]
fn legacy_output_roundtrip() {
    let o1 = TransactionOutput::new(&fake_base_address(0), &fake_value());
    let mut o2 = TransactionOutput::new(&fake_base_address(1), &fake_value());
    o2.set_data_hash(&fake_data_hash(2));

    assert_eq!(TransactionOutput::from_bytes(o1.to_bytes()).unwrap(), o1);
    assert_eq!(TransactionOutput::from_bytes(o2.to_bytes()).unwrap(), o2);
}

#[test]
fn babbage_output_roundtrip() {
    let mut o1 = TransactionOutput::new(&fake_base_address(0), &fake_value2(234567));
    o1.set_plutus_data(&PlutusData::new_empty_constr_plutus_data(&to_bignum(42)));
    assert_eq!(TransactionOutput::from_bytes(o1.to_bytes()).unwrap(), o1);

    let mut o2 = TransactionOutput::new(&fake_base_address(1), &fake_value2(234568));
    o2.set_script_ref(&ScriptRef::new_native_script(
        &NativeScript::new_timelock_start(&TimelockStart::new(123456)),
    ));
    assert_eq!(TransactionOutput::from_bytes(o2.to_bytes()).unwrap(), o2);

    let bytes = hex::decode("4e4d01000033222220051200120011").unwrap();
    let script_v1 = PlutusScript::from_bytes(bytes.clone()).unwrap();
    let script_v2 = PlutusScript::from_bytes_v2(bytes.clone()).unwrap();
    let script_v3 = PlutusScript::from_bytes_v3(bytes.clone()).unwrap();

    let mut o3 = TransactionOutput::new(&fake_base_address(2), &fake_value2(234569));
    o3.set_script_ref(&ScriptRef::new_plutus_script(&script_v1));
    assert_eq!(TransactionOutput::from_bytes(o3.to_bytes()).unwrap(), o3);

    let mut o4 = TransactionOutput::new(&fake_base_address(3), &fake_value2(234570));
    o4.set_script_ref(&ScriptRef::new_plutus_script(&script_v2));
    assert_eq!(TransactionOutput::from_bytes(o4.to_bytes()).unwrap(), o4);

    let mut o5 = TransactionOutput::new(&fake_base_address(4), &fake_value2(234571));
    o5.set_plutus_data(&PlutusData::new_empty_constr_plutus_data(&to_bignum(43)));
    o5.set_script_ref(&ScriptRef::new_plutus_script(&script_v2));
    assert_eq!(TransactionOutput::from_bytes(o5.to_bytes()).unwrap(), o5);

    let mut o6 = TransactionOutput::new(&fake_base_address(5), &fake_value2(234572));
    o6.set_data_hash(&fake_data_hash(222));
    o6.set_script_ref(&ScriptRef::new_plutus_script(&script_v2));
    assert_eq!(TransactionOutput::from_bytes(o6.to_bytes()).unwrap(), o6);

    let mut o7 = TransactionOutput::new(&fake_base_address(6), &fake_value2(234573));
    o7.set_script_ref(&ScriptRef::new_plutus_script(&script_v3));
    assert_eq!(TransactionOutput::from_bytes(o7.to_bytes()).unwrap(), o7);
}

#[test]
fn pre_alonzo_block() {
    let bytes = hex::decode("84828f1a002072a81a00ca44f0582070d6f38b4569ba062c09632127db13474f22c534e6d8097895403c431e57f12358204f4d7523e41e058a6cbdefb5538654ffc2a53416a7f5bb99f7eac699d42d5c1f58205e3d96cb8ef0291d2f1df6aa7b5a4496ac8de1dcce100c31274325625102796d82584065417914ca323d842c5861407a638e146e6af55f59aff95f1451839de2aa709151237e24e6db7bf94db97293da9c1e61e68d60c8e2b10a116d3c71067247458b5850dc36a5a88f09f0b7a0b5d5d52d87c7c3e3c20752176a426d182255df3d026392f407990f09e5858de6432263fc167bc890a97d07d2371cd5bb26b12242c1ff6fda184ec78d15493a38a3e0df1494f800825840df4e07d3bca43341e4297e2914ea38363ecea1c17ce9145294c4631e0f09f706cb23a5f27c6b71ae9ac46a7ca25af4d7c156f15444fa41814f7d6a0b6a4e57525850d6073f277ded1ef9e8bfe9f6325858c142fbbbbff4395c45d82f0861a6ef6116204965f807e8650fa4e9ac4aa04aeb03984ea66abb129155a78931d39bbcb7ad64afef3f4f55cfa4eb6c97698e88f1051905db5820c1b1fbd809dc06e0e2dc544312aae2a46c059249f86c24ea0689a0b0944a75f558207ce5ce3992b23cb2bf566c48aba8bfc39eb24c9b43354de0129b81bf9f1414b307186058403ac64d720227c18139132b499046a168eb1c5bdd3983385e3518f33fc7f52fd0be348fe3e14d17e1ba606708c30bda061cf23ea3294b0089d3e1e1d58a7aa50702005901c074d3c2c0b5e17b12ba829017186daa1f7f365bbe5b0e0c038cb0cc05e849f702afd349234353ee3cc8878fa31299e85562f04d3cdd74e1bc73591be48d2fbc0d043f6b41fa527b2f9fb3f77605eee528926e76cc18a1638283e5591170f7073462441d40d7cc2e13a38e7d247928cb15d2e5b2e74a12d07f858f7e922bbff8a91c16e9bb8f5ea101c50d96627fb48a03d8191b5035b5de00b9824867fdffb5a2493799e94676bf685db85517dd8a87a0ba2589b3a8a69d529ae8052680c520c5577adbb91cf931e906b1629e621d5bd5c30eaee77f35c5f0a714827b48afaa4e549c1756e94291f4b083aad9c375caf9a67aeac08f32c91cd0572192267960cd74a85148b5e99d0053804dcfb44785417725c56e0fc5caf2ae50fbf25b92c7b7ebe17aa9e289470041a06fd8986f6f9ebdb12e87a970f1d388963929367013e17513e83cab8c98460cab703d5fdd26eeb079e4db701996f73c694365080236901289c5fc96471e91fb75e0e58560f5d073c3ef79a8f5dd4b45ff7abf9c7d7564232f7897ca3d85ac7bb9ecaa75b7c062f27de8b20f301e5607563b2c904e3c7f113b1eeba8a4d1c82fc1a747c920bac6af9a9f4dae1744847232ea03289e25e482a50082825820478ad95cafe9b1660809d618870c86dda1295764e113886e2b8a1de2de5af17201825820f84508cc7674b663db84ceb9f0790f5527f3c70f2a05e4d7f783cd9890463b4e01018182583900ff7f04abbd3050c0b138c8fa3005d48aaf8b9700d4565758e91a95385667fab107f848cfd4b73a7407a7661600cf68f0efc969ece37665ae1a000f4240021a000f4240031a00ca60f1075820e845fe9180ac36cc0102f892a839ad1ed2ea9a52c605fb8e4e1c2774ef0bb65ba50081825820c4b5ad6873b8581c75b8ee52f58a3eded29acbbb92d874a64228a1ca4e68956700018182581d60daad04ed2b7f69e2a9be582e37091739fa036a14c1c22f88061d43c71b004aca96b58fd90c021a000f4240031a00d986900682a7581c0d06d2547ed371fdf95fb5c4c735eecdd53e6a5bb831561bd0fcfd3da10e820300581c2f56e87d67b8e5216582cfeb95dbdc9083110a3ef68faaa51bef3a80a10e820300581c2fca486b4d8f1a0432f5bf18ef473ee4294c795a1a32e3132bc6b90fa10e820300581c4ee98623920698b77c1c7f77288cbdac5f9011ff8970b1f507567d0da10e820300581c514e81afb082fce01678809eebd90eda4f7918354ec7d0433ad16274a10e820300581c581e23030b6038bae716e5d64b9e053db10541b12e6b0b4eff485454a10e820300581ce5f27655371b54aed91cc916b2569060978be80056768fee2cc5ce1ba10e820300186582a1008182582028364596385174f5eabc763031b8d54b18ed5d06967ff44b3abbdbaca9cb58a75840de49197fed8dd13716c88e68452fb314d418a24fee9cc194308bd47b057d161ae40cd8f49bf6b378e7343ee5d3a7b9bdb1f2e9efeef896adaa9eb7373fbb8502a1008882582032a954b521c0b19514408965831ef6839637de7a1a6168bcf8455c504ba93b9c5840ab2d59239499807e25dc8025940a70cb890a52e8f47f35004cfec623036ca9f5c3e925b32bd23a7d1d044cef915913e853dbb57438f9c92a5d5f9581caa67d098258207ec249d890d0aaf9a81207960c163ae2d6ac5e715ca6b96d5860e50d9f2b2b2a5840f2d8031ac5d79777076dd1176cb7ed91690fcfb6be498320e5de9afbf6ea8e8ced23bff69230d050523a4a7e03c2b0599e18e93b31959063249fb50274a02a068258204f4d7523e41e058a6cbdefb5538654ffc2a53416a7f5bb99f7eac699d42d5c1f5840c5844b849865fed81f67842a4697c3090cf4ecb50510f1e6b379b7c63b78417ca28ea653c016d2e733877e1605e8a1712c42404ca0686f67455c620431d54b07825820e764b0340d7b353f5f745891033774e4beab6aa1458a54ff29a1324c05bb9876584026c35f8ec2102ec8fcc3bd0a1a0760486952e147f44236a35c7d818a7024590e1395f097a0d046085ded24ec8c585008d3ffc0321ad040649ce08eb33614760e82582073ae41eca2be37fc15c55a50d668c8647e10bf222172c2d58abfa6e9310e596258402c3f197360294781841f0669822b0449515a5e0b77b23185652a1b0ea8354537b3e9335577a87fa19e9fe47f1039fa286aaa11859d631f3ff74564c6da14c806825820234fb2b8530114b461c6ca8242c8b86a226c95c4c27479ca850d1aea4a52d2985840ba751817e70695a041a5f455c08947fa4e3d6ffc332adeb25691fac4927bbaafd4b3f5f9855946ad9681083aec277766c7f90da7543e912f46aeae07fdd5b90a825820dfb615a61568d6867f45a85c32227f27025180d738a8a3d7fd3c929f624d72395840cc1f728cce6ce2fec21d2648011c14d244c35ba3cbd553593655f6f07d86b8bdf103d52b61143bc1701319517d4a24b778c02e983e02a0f3fd0cd558d472f009825820e5bc21a83616bcccfe343ec36b9dc4c06c90e913df1d8a0b046008651f42caa95840f85bc5e753beed04b3f9072da7a6adadcdb87769528c59e16162e86782b6ce11feacbd5de97e352121e9509a809f613d5bcebf7413fd55f89776c5606e4a9408a100a119534da261638158220a201f79b4d15fd971297a842ac6a4e953b82886df66c0d9723f5870e5725da6380b617601").unwrap();
    let _block = Block::from_bytes(bytes).unwrap();
}

#[test]
fn tx_output_ser_type() {
    let array_tx_output = TransactionOutput::from_hex("8258390000efb5788e8713c844dfd32b2e91de1e309fefffd555f827cc9ee16400efb5788e8713c844dfd32b2e91de1e309fefffd555f827cc9ee1641a000f4240").unwrap();
    let map_tx_output = TransactionOutput::from_hex("a30058390000efb5788e8713c844dfd32b2e91de1e309fefffd555f827cc9ee16400efb5788e8713c844dfd32b2e91de1e309fefffd555f827cc9ee164011a00039447028201d81844d9052380").unwrap();
    assert_eq!(
        array_tx_output.serialization_format().unwrap(),
        CborContainerType::Array
    );
    assert_eq!(
        map_tx_output.serialization_format().unwrap(),
        CborContainerType::Map
    );
}

#[test]
fn oura_wrapped_block_test() {
    let hex ="820785828a1a00101e2c1a0143a1b35820cee15d6daecaeaf320a4ddb1f7c437846f798e4a9cd08d12fb7821b175c980115820e3c87f196ce9fc40a8d929f3365e247f8f71e1981bffaa7cbdb0aa3a83dc790d582054a580ddf99f67818e0312374cef1f7dcdd59450930898d4d2d10e606b963e49825840ca5d1f988222919982b6a20f4f54ce59626fece7d7c607487762129d5196c731bcd11dfefee94ce5a60a733478970631d41bfc0620769fa7b66ebc16c8a89e5c58502855f21ba12fb101d175c376e19496e464bf37c92ec21395e5bffb35e1ae8f433f2139de166161f2b2b26afe656d3d170acfd11a535a80fca6325479d2262e208b0a4b98a01f4845c45a58fb84cb58011952de5820f2e4c6554da5b773c3f7889944fdb5b1791f8552dcafe2916041a531860e912284582039b66a10f6b78c541ea5ed6ecec4c6dd385b869026ec16c4e48414cb39cac38b0018a258409ccd6cf71a5c337f71a41904c0ea0a889a2321c94374c3a8402d8a7dd25b222abe6cb325c6b39bd63bc99fa84c094fdac2523b72f1a22081903dd047be9be9078209005901c006b35937aba451d4738486ea3ba5644d9306651f09b2012de8acc5136771fc725164ad669dd716f2726dfe138137d09feddf9450b3c51a601577bff35d0d2202c887a260855dd8310fc9365f56a4757ea7d81103d409ea0a8ad51c6ae52fc7fcf4d3d456384b7566b70a2b7bd4e21010a1ad5df12bf5d332e82c1a4a5cca39740252e0ea163f206cacf193e59ebbd0e20d621fa9913c60efe1c035d8ebaa354fbe45768339d53a4e8e04fdea79d00b869a973cfa3eeba2e2668b1dee5fcd7d13762dceb4da804fd749e5fa977ead0003a9739837aa68b80bc5a32ee015f667574a7fbe03b4bf5b027c945fa4497c01efb4ec51f3da2fb2dda33ea7dc1dedcfd2ea2c0a4da5a1c553d033033f4986e2ef5c09bbe326a25e5082c1eec406aeec8105869a9d46a83689a2e026e6e31d4037e700ffeb2920bcab88d1a400976881d17cd84582521482db0be460fb43de88e40a4ee24745ac92ab8b40329bde1d855404478c9f59b05e6322f3640ad6f40d7a771fc6d58e94f8fd0006d54272e36a30034b14327c2e6ffb92ead2f8a4165a3e4a1c44de677829e8e797547b3c0bac4b5ea89cb86c01d5b1e67aee3ba36b8cf9617484db2e4d1bfc37fed1fabb73ce3c9fa600d901028182582088c310befd2e8c9b33b340a56f4ea8141689c16eddef5d9c606055ca35897bd600018182581d6052e63f22c5107ed776b70f7b92248b02552fd08f3e747bc745099441821b00000001f09cac72a1581c34250edd1e9836f5378702fbf9416b709bc140e04f668cc355208518a1494154414441636f696e1916d6021a00030739031a0145283409a1581c34250edd1e9836f5378702fbf9416b709bc140e04f668cc355208518a1494154414441636f696e01075820e2ea39e82586fa40304df3c2cfc753c6ba8aca62e780f01a0519c34c6d7c25f5a300818258200580612292c60a12689142d795c39d577aac4083c63a8b572fc10a69c0ae51fe185b0181a2005839007ce8986f5f3fb526a6d35d32edac0b6c8624daab6928df1964459c2723bcf2892e8182a68e3aac6f9f42ed3317d115ebad12a17232681175011b00000002540be400021a00030d40a300818258200580612292c60a12689142d795c39d577aac4083c63a8b572fc10a69c0ae51fe185c0181a200583900f7fa5ddf2c3c46ed4d913812d38dd43d585adfa884938adaa7a075dd1bf1e138f2f8beabc963c94cc28ee8ed4b41744601f2edaf50b21efd011b00000002540be400021a00030d40a300818258200580612292c60a12689142d795c39d577aac4083c63a8b572fc10a69c0ae51fe185d0181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a300818258200580612292c60a12689142d795c39d577aac4083c63a8b572fc10a69c0ae51fe18600181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a300818258200580612292c60a12689142d795c39d577aac4083c63a8b572fc10a69c0ae51fe18620181a200583900189f009d9536b1f52f0629bea3323f48df0eacdff68726f1a32edc49db89995ed3aa88dcfb43790e2e51761fcba7e8594bcc67684f52d524011b00000002540be400021a00030d40a300818258200580612292c60a12689142d795c39d577aac4083c63a8b572fc10a69c0ae51fe18630181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a30081825820a944bb37a59451f9a47d5c8888a8a1145527ffb5d45a17c1df40926a42ad08330001888258390038be1948e77426eaca9f967becc5455b11d3d40fb06b99dd3a817d5e75c7a5e1120727f24236cfb5981ec30fd50a2684a5aca866a123a1361a05f5e10082583900bb17dbac8d4a3452dbb6d0c664e804deb14a0b95ded5274007189c3641868c2b4e5289022a3a1f6f47f86823bc605c609d2c47a2db58e04a1a05f5e10082583900f8e61d5f13ab575771af475ac599ad88c7116339f82d2ea969b0e601d6d84c6a5b05cb8f89d24e9d46926975fa1dc08a58b3c26e96c06df71a05f5e10082583900693e466f25213254e061fdc95f8a5f07bf6ef0de0478adbf89a3308f7c4641296645e557c0a6426e140a09d4ba423d158aba1eae06aba7971a05f5e10082583900d93170064d82eab9dea2b3141bc88503ec80e93c8691fb6b223fe310877c17de5bd978526e288334114fada629f699c4e799394aa45c2aad1a05f5e1008258390093ab1cf6cececd048265573176355a322b7732299bbd624f655af2f674984fae4ca1715fa1f8759f9d871015ac87f449a85dea6cf9956da11a05f5e10082583900bc032a8614a84f5d9ee772f2788954e9d664b4264226cd36d0c4ddaeaa22f3a63400c1d96ad118b5cdd300cd039e83ae1957a35b764881941a05f5e10082583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a41b000000022a4fe9af021a0002d351a400818258206a7e3a926eafa74f72c0d6a721dfdee7a7202b1fac4eee12d8c6dd030217890b07018182583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a41b00000001eeb2890a021a000296a514d9010281841a3b9aca00581de0db1bc3c3f99ce68977ceaf27ab4dd917123ef9e73f85c304236eab238106827668747470733a2f2f6269742e6c792f337a434832484c58201111111111111111111111111111111111111111111111111111111111111111a300818258200580612292c60a12689142d795c39d577aac4083c63a8b572fc10a69c0ae51fe18640181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a30081825820a439638a9f8e0f52e153126e8b794b7514f3a0921b08b611f3866a1fc75b7a560001888258390038be1948e77426eaca9f967becc5455b11d3d40fb06b99dd3a817d5e75c7a5e1120727f24236cfb5981ec30fd50a2684a5aca866a123a1361a05f5e10082583900bb17dbac8d4a3452dbb6d0c664e804deb14a0b95ded5274007189c3641868c2b4e5289022a3a1f6f47f86823bc605c609d2c47a2db58e04a1a05f5e10082583900f8e61d5f13ab575771af475ac599ad88c7116339f82d2ea969b0e601d6d84c6a5b05cb8f89d24e9d46926975fa1dc08a58b3c26e96c06df71a05f5e10082583900693e466f25213254e061fdc95f8a5f07bf6ef0de0478adbf89a3308f7c4641296645e557c0a6426e140a09d4ba423d158aba1eae06aba7971a05f5e10082583900d93170064d82eab9dea2b3141bc88503ec80e93c8691fb6b223fe310877c17de5bd978526e288334114fada629f699c4e799394aa45c2aad1a05f5e1008258390093ab1cf6cececd048265573176355a322b7732299bbd624f655af2f674984fae4ca1715fa1f8759f9d871015ac87f449a85dea6cf9956da11a05f5e10082583900bc032a8614a84f5d9ee772f2788954e9d664b4264226cd36d0c4ddaeaa22f3a63400c1d96ad118b5cdd300cd039e83ae1957a35b764881941a05f5e10082583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a41b000000022a4fe9af021a0002d351a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4010181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4030181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4020181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4040181a2005839002b11f0e68a65cd6a243f1a5ec9d597ba972675a00bd3172a7ddc0293b1d312a60b3824d1820dec5ec769c4af7d7598387c16ca5ba6259f46011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4080181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a400818258203298fb7878ab004c1a4b369eae7fc89abca6342f06557cebf6c89f2d8c21aa9900018182583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a41b00000001b3152865021a000296a514d9010281841a3b9aca00581de0db1bc3c3f99ce68977ceaf27ab4dd917123ef9e73f85c304236eab238106827668747470733a2f2f6269742e6c792f337a434832484c58201111111111111111111111111111111111111111111111111111111111111111a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a40a0181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a40d0181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a40f0181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a40081825820dcdbb7a98286f5d48673c95b05f441bc40731b1e4c3429d192f0c6b7fc3749d100018182583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a41b00000002186e835b021a000296a514d9010281841a3b9aca00581de0db1bc3c3f99ce68977ceaf27ab4dd917123ef9e73f85c304236eab238106827668747470733a2f2f6269742e6c792f337a434832484c58201111111111111111111111111111111111111111111111111111111111111111a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4130181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a300818258207e4fddb60c2034bff37bb7069f9318735fcf4de03e01e9f92251c96dc59318750001888258390038be1948e77426eaca9f967becc5455b11d3d40fb06b99dd3a817d5e75c7a5e1120727f24236cfb5981ec30fd50a2684a5aca866a123a1361a05f5e10082583900bb17dbac8d4a3452dbb6d0c664e804deb14a0b95ded5274007189c3641868c2b4e5289022a3a1f6f47f86823bc605c609d2c47a2db58e04a1a05f5e10082583900f8e61d5f13ab575771af475ac599ad88c7116339f82d2ea969b0e601d6d84c6a5b05cb8f89d24e9d46926975fa1dc08a58b3c26e96c06df71a05f5e10082583900693e466f25213254e061fdc95f8a5f07bf6ef0de0478adbf89a3308f7c4641296645e557c0a6426e140a09d4ba423d158aba1eae06aba7971a05f5e10082583900d93170064d82eab9dea2b3141bc88503ec80e93c8691fb6b223fe310877c17de5bd978526e288334114fada629f699c4e799394aa45c2aad1a05f5e1008258390093ab1cf6cececd048265573176355a322b7732299bbd624f655af2f674984fae4ca1715fa1f8759f9d871015ac87f449a85dea6cf9956da11a05f5e10082583900bc032a8614a84f5d9ee772f2788954e9d664b4264226cd36d0c4ddaeaa22f3a63400c1d96ad118b5cdd300cd039e83ae1957a35b764881941a05f5e10082583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a41b000000022a4fe9af021a0002d351a40081825820705ab68071f9af1d314e74a053e39a52f3fdf96f9a1280dab30d45f04c05436d07018182583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a41b00000001eeb2890a021a000296a514d9010281841a3b9aca00581de0db1bc3c3f99ce68977ceaf27ab4dd917123ef9e73f85c304236eab238106827668747470733a2f2f6269742e6c792f337a434832484c58201111111111111111111111111111111111111111111111111111111111111111a400818258206fe0c3eae23f779b0694747ed28612f47271b45e84bb3d23c11c1ef2e90fa12100018182583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a41b00000001dcd122b6021a000296a514d9010281841a3b9aca00581de0db1bc3c3f99ce68977ceaf27ab4dd917123ef9e73f85c304236eab238106827668747470733a2f2f6269742e6c792f337a434832484c58201111111111111111111111111111111111111111111111111111111111111111a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4150181a200583900e698ee1c7d4361c6faf62716dca0d435eafd0b25e369a5d68455beaa0f5c16e3e747e7c5a9eb3ff189c0e330683665de9326d2ffe35d0631011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4160181a2005839005bed6070c549f1560cb89361564cd2be7b36536e8da868a218d514e5fd2e3e48dbc0278cc58e47ed50a1ba90cee61ab22c8f4a639c913d4b011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418180181a200583900ab3cd541317d83d072bcc38e1294166dea5d97ce453424b84c547cfc101c5bfa799985a6e96adbb5859e90cbe4a0e4edcbef408a3622558b011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4181a0181a2005839003b3ff2a2d98519fcf53c7abb15b6c4dfe76209c52e4c2065b33b97bc465f9e3a6c6c3a8eac01d39f519b9bf3bc031480936156b7cb2e45c8011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4181d0181a20058390050fc315c9c141b4da62b10525cf5049e8ab1bb8bd96903a6d87c5272bc616bee900ed3135eb065a11faf2100670f0182ae86827df52dba96011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4181c0181a2005839006087b2d29b8a424d7e3a756d08cb078ecb5215fa9344343ac2c6bfb02bdca5a48eca12260be94aecf81b9f21ca41871e06cdc4d12b5aa2e3011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418200181a2005839005deef04c1b44c606775db03444beae0f10c75f437c131628d264b17c439dc3dbc39b8bb91832384d44263001591fd806df73b413da861fd3011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418210181a2005839006087b2d29b8a424d7e3a756d08cb078ecb5215fa9344343ac2c6bfb02bdca5a48eca12260be94aecf81b9f21ca41871e06cdc4d12b5aa2e3011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418220181a2005839006087b2d29b8a424d7e3a756d08cb078ecb5215fa9344343ac2c6bfb02bdca5a48eca12260be94aecf81b9f21ca41871e06cdc4d12b5aa2e3011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418230181a2005839006087b2d29b8a424d7e3a756d08cb078ecb5215fa9344343ac2c6bfb02bdca5a48eca12260be94aecf81b9f21ca41871e06cdc4d12b5aa2e3011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418270181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418280181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418290181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4182d0181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418330181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418340181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418350181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418360181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418370181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4183a0181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4183c0181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418460181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418470181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418490181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4184a0181a200583900189f009d9536b1f52f0629bea3323f48df0eacdff68726f1a32edc49db89995ed3aa88dcfb43790e2e51761fcba7e8594bcc67684f52d524011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418520181a2005839005c85eb9c0aa544a6bb5d1577c7a588c39caca885c8a3a9fceb0933a2cd1a02667d16df1e109350555c325023dbfa31fd9a4a8b99ff904d96011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418530181a20058390030a33756d8cbf4d18ce8c9995feca1ea1fc70093943c17bd96d65fed0aed6caa1cfe93f03f6ef1d9701df8024494d0b3b8a53a1ee37c5ab2011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418540181a2005839001da17fce0b5ae3e7feae117785eb78c77b6272be34a3d381a2722154d29c294b138005ca78de7b329ed6d2763a74a3fa1710a403e18fcb4a011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418560181a2005839001da17fce0b5ae3e7feae117785eb78c77b6272be34a3d381a2722154d29c294b138005ca78de7b329ed6d2763a74a3fa1710a403e18fcb4a011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418570181a20058390098bebc68cf6f12a7aca6531cef75d83c1b6e323485146195ffdd727dd99bbe7f44fd382de2ca6d9e5e9cc26f940decdb1b12b1a98e343274011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4185a0181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a4185c0181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418610181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a30081825820058e5c03e1b0e08f8710cbbd59ea8589ef0cacf031727146d53e9c1067bf54a418620181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d0004564050181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d0004564070181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d00045640a0181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d00045640b0181a200583900003ad89c3e2ed8e98d2cdb207afda3a49cf73d7df70cff6f35d5a5afb7137b5e2a626ebed51ef0261692b21cfab50bf053e989f24d65f48f011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d0004564181a0181a2005839005746c1b032f826b5e5256357a713a7ca63988fe2ff862e0396993b97ef0cbd5199d0e460725b3e79d371deb42110d40b778d3bf162777d4c011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d0004564181b0181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d0004564181e0181a20058390020de866f290f45141315081d903f3eb3c06f3735e2a5b70f6a138462ada99823bc02291029853dc5338bc6e62b0540dbea54d9384f372639011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d000456418200181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d000456418210181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d000456418230181a200583900057fd21bf903c585ea95dd927dee373b4cc1febc61874c48571dfb88a0a307af2a3e6a55a238fe323f9e54be10c54a8a8b25939a4f9ab35a011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d000456418240181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a3008182582043c59e533d0934a6878a81403ec71a2225bb22d0764471cac8b5545120b475760001888258390038be1948e77426eaca9f967becc5455b11d3d40fb06b99dd3a817d5e75c7a5e1120727f24236cfb5981ec30fd50a2684a5aca866a123a1361a05f5e10082583900bb17dbac8d4a3452dbb6d0c664e804deb14a0b95ded5274007189c3641868c2b4e5289022a3a1f6f47f86823bc605c609d2c47a2db58e04a1a05f5e10082583900f8e61d5f13ab575771af475ac599ad88c7116339f82d2ea969b0e601d6d84c6a5b05cb8f89d24e9d46926975fa1dc08a58b3c26e96c06df71a05f5e10082583900693e466f25213254e061fdc95f8a5f07bf6ef0de0478adbf89a3308f7c4641296645e557c0a6426e140a09d4ba423d158aba1eae06aba7971a05f5e10082583900d93170064d82eab9dea2b3141bc88503ec80e93c8691fb6b223fe310877c17de5bd978526e288334114fada629f699c4e799394aa45c2aad1a05f5e1008258390093ab1cf6cececd048265573176355a322b7732299bbd624f655af2f674984fae4ca1715fa1f8759f9d871015ac87f449a85dea6cf9956da11a05f5e10082583900bc032a8614a84f5d9ee772f2788954e9d664b4264226cd36d0c4ddaeaa22f3a63400c1d96ad118b5cdd300cd039e83ae1957a35b764881941a05f5e10082583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a41b000000022a4fe9af021a0002d351a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d000456418250181a2005839008f221a3021b0c09c57336733b0411d9d664e5d5e259096033a9d4bbecbce4335fa28195472386e53f0c3ab74d5cd254797d1100e4b1a33b8011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d000456418290181a200583900d804dcdd0b0ec6ed0d8d2cd210a03b14f87c6849024930a8d6c91cf551a6a756817f0a3e1a1410730acf27202e7a9b63de26087e5cf466a5011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d0004564182b0181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a3008182582072af166d0cd8883c9abb1189ae93acb1fce482ca57cad9b14711bca9627b98d80001888258390038be1948e77426eaca9f967becc5455b11d3d40fb06b99dd3a817d5e75c7a5e1120727f24236cfb5981ec30fd50a2684a5aca866a123a1361a05f5e10082583900bb17dbac8d4a3452dbb6d0c664e804deb14a0b95ded5274007189c3641868c2b4e5289022a3a1f6f47f86823bc605c609d2c47a2db58e04a1a05f5e10082583900f8e61d5f13ab575771af475ac599ad88c7116339f82d2ea969b0e601d6d84c6a5b05cb8f89d24e9d46926975fa1dc08a58b3c26e96c06df71a05f5e10082583900693e466f25213254e061fdc95f8a5f07bf6ef0de0478adbf89a3308f7c4641296645e557c0a6426e140a09d4ba423d158aba1eae06aba7971a05f5e10082583900d93170064d82eab9dea2b3141bc88503ec80e93c8691fb6b223fe310877c17de5bd978526e288334114fada629f699c4e799394aa45c2aad1a05f5e1008258390093ab1cf6cececd048265573176355a322b7732299bbd624f655af2f674984fae4ca1715fa1f8759f9d871015ac87f449a85dea6cf9956da11a05f5e10082583900bc032a8614a84f5d9ee772f2788954e9d664b4264226cd36d0c4ddaeaa22f3a63400c1d96ad118b5cdd300cd039e83ae1957a35b764881941a05f5e10082583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a41b000000022a4fe9af021a0002d351a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d0004564182d0181a2005839001c4595c4f3180180c9e822f1ac0f2955dd329eeeb94752a84281ff5295558528c6e1f7f2e16d94b74d227b9fd709edd2aeb0ab1556db75fc011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d0004564182e0181a200583900c008dd489b67e0a774fe18d79ee8d1e280264933d3b31ba44cb37755dca94fb45aa2192ab26eff8409ea010fa2d4761efa92437e0d5b6b60011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d0004564182f0181a200581d60fc38cce3448bf3d2790ca85d6b09026f7c86f21095c31f9925cf49a0011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d000456418300181a200583900b5187cdefbc5b49ddc17b423c079f0717721a03882a3b265bd4c12e080f326af300273d19d5a541d45baa42ebc04265816735b026b5f34a4011b00000002540be400021a00030d40a3008182582005fb50ceafb7ec5392f24b830572c949bf5de8396ea862298285b7a4d000456418340181a20058390023a6fcbc8affc61518cff034c013aecf083dc64fe673ffc95cc9fd9e1fad7e0b1d0dd8820703a4f59c2488d148193a48d8fdc23a6dca8137011b00000002540be400021a00030d40ff9fa200d90102828258201287e9ce9e00a603d250b557146aa0581fc4edf277a244ce39d3b2f2ced5072f5840ae4cc1168265e2f60fec9ca9b644eaa42a77e65a39176e04aef29b01e25653a307d39ba61761f8d1ca44696e1d6bdf7a0679413ea3c448f76268e6eb02074102825820742d8af3543349b5b18f3cba28f23b2d6e465b9c136c42e1fae6b2390f5654275840112c95c93013e63fa73ee6a645fd522808d4dee019626e395a8042755c15fb1824e1503c17ea843a838809f55822366b05bce2e378d0b955e66d625c8e9acf0001d90102818200581c45d70e54f3b5e9c5a2b0cd417028197bd6f5fa5378c2f5eba896678da100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb82584005383334e98e9263b93ffeb3e5908dbd5657caa67d32f9964d7f91dbda76fff164cbeabb981beef470d0d3e1724b85847e6fbc1c039084a817110eabf9d29e08a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258406151f0d0cae4ef0ace62dc03300dcee276765c1b493ac61f770d0633f0f71fe0d642ef29b593c511034a847dd569c56a0278e063c46d6d060abad4e6baf4b705a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840cf518f30d291871a0277e367534557205cc015d3a0d78070e1aee298aeaae3e81d70b42c464a63fa741b5300b48c1699dfc39fdf07f96b8eb240c7d6d3267805a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb82584056185de4c1d012d322e7e82ae98f24887ed7264f262db53f019e5900b9110a439e5a462a75be036f9f04b0ddcf09decb0894c7b6b9ff17ab4cae8184072d690fa100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840ab78c606155b6aacbcf6692a18d97dd8773b4ae4c96684e4abb9cc59233023f67650ef7259069deddb65ba770ac3a1401d169ce33ec9483b8ebb9e83472e2c06a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840e21574b02002efcfe81382326aa98a0a971327ad4049690a04985400fcb14db7adc8149a0ce4dbfb5afa0d240ed9da23f15c1020d2826f50fc579a10a3662d0da10081825820b6a42d4ccc4d26adaec67e8578bf31f13b1b7e640527356248f2ec547f9de6e45840e64e3c19644edb6e788112ac75b4426ef94d535f1ffd9a34e86745777feaf083dc8e847a62634fef320a08b566c24ea26e8dc9e7b49fc456554215cedc0d3508a10081825820b6a42d4ccc4d26adaec67e8578bf31f13b1b7e640527356248f2ec547f9de6e45840f49fd8eeaa366873aeb2530b2bbcbf7c5970866162ae7250c4b913e19062de1396ed70d1e32a4605071bac11c2cde3fec1dc5b37044cbea073668fe5c478400ca100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840f0ddd023e0dbda32d296b359db809b0088246e512fd34c7c0cc4b5ae974361873e02330e955eaaf97117525bcb3cd014bb70810f8d0d62a28c3242c86d8c3a08a10081825820b6a42d4ccc4d26adaec67e8578bf31f13b1b7e640527356248f2ec547f9de6e45840615ee0444f039f02b26791872d6cd5562728cdc6dad02acc71475567b09f3d4b4655c601bf816ef6d11b2f3f81eeb6db09d800bf1bf4e2daf29493338c232901a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840d62bfd428359f4cd04950cc73f574f0ec1c613284fdff8028ed3d876b18b69335beee9792410c6dbdc1b196b4703f250fbaeb66569869ae97d7ee843b9053405a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840d813a836cc44c70647b2e7941fb72a4f720f16aca17e155a6c6a6f9bf175b1e49a3beff6edcfb0c442cc24790a12ee0b1d499a32fdbfc0a850f4846708ea340da100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840aae6ac20cd419eaa7f3a95144f9ccdb46401a0db295d544e920a54b5c24fb63197fde03f12174800c3cf5318a73d92ebc53c2ba97803766892add32fd9feb400a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb82584071d223fd255df1e912a9c0a8230ee9f0ac95d0aa325cd31e50701ac355dfb5f3fbb27983b372c1410156eeba9163aa0f8a9787dab8c44e7afd4e2d07459a4708a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840b7f821c0ff66fcbbe7c61f43725aa2234297d6765e002d0130301cba13465fe89f59a596549725542445c76d17cedc9c9cfea8b8862d41405646d725dabc7d08a10081825820b6a42d4ccc4d26adaec67e8578bf31f13b1b7e640527356248f2ec547f9de6e4584045830e5de108b9353b0c4c561af296e79eddb26b8ccfb18c5bd9fac1baf8d477691229c0bb9ea212ab56d9ae76c92de6ae50686fc0619510b8c35fb69c6b4402a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258400635ac784fe71d2f7927114edfc709dcb56013617df4edb9b6b770b067e7709e8abfd4cdcdd61512894fcf02f16e1d72bfe60fbfb86b815631d791bab132b909a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb82584032d67fe72feaf2175455815bbb624ee1b93f5efce905280158db94bbb2b5371d9eaff1bed6eddf9eafe8ff64b55f1d7213294bdb459e0b00c437edbcabf4cf07a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258404533de52e9e3f51e39951c9e197f6900081e98f38f3af5c4a7fe9219f8c311eaa43942b7a290ecbbbdd0bf4ef4ef1d11c39e6de4083c86892a6026c27bcf2509a10081825820b6a42d4ccc4d26adaec67e8578bf31f13b1b7e640527356248f2ec547f9de6e45840d46541920ce2c31ffaa00cb20e8f5f00f48b6b8aa5cda67d22ea4bf12fd318461a0d8c25ee04cd7446e18f0de59b0fd4f6631e29bc8207073f2404793ae5f108a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb82584041bcd52ae44a3ffaa1abe1cab6c8b21f8010e2f1aee1d8651b9f6e94aabf5b2dbcedb45dd154b78dce1c5b9679956dd25153a0d945d3eb83c1de3b713e721008a10081825820b6a42d4ccc4d26adaec67e8578bf31f13b1b7e640527356248f2ec547f9de6e45840027da3169c9c1fb9a67104061698bb0dadb2f58b660af4b461e7353fab1545a3d03157e077a388ec8556176239df3241255feb1f13b5e406bf7c3ad3af7d4202a10081825820b6a42d4ccc4d26adaec67e8578bf31f13b1b7e640527356248f2ec547f9de6e458401ae63dc54e511965f7010971de7fb99585afe492cb8084b395ee01555c1e5657ab08a24be0f70d4e9cd1bde2a6ae31815c5f64025c0415afe2d503b2cb5b3e0ca10081825820b6a42d4ccc4d26adaec67e8578bf31f13b1b7e640527356248f2ec547f9de6e45840d2211679ca8e9cc5de71b21bac2b58fd355f5cbd2b42ff31ec37af77b776fb77c64fa76a830f240c29a4b95ae6bff9c58fc6bc2b3d18b57a2af11710ae6e3006a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb82584084bb7bf64ecea446d6beca88bfa2c7de71d8899e96006920c4c18e52f042aa71e1d27e60bdb6d9d6b1aa2e3330f59ee95b2c001909ff8994ea1fe4e5cd7a760aa100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840c6f3d6194a2fdb2a50417f80dd020adf866c91a102f22eb6bc66f5131292a1a42e9a3550e18e06cb17bd153c08f55a6cce3a1c82052ec9197495900f3ca4f407a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258401a28cac7e80a657563e1433459f369bb0cb05e7e3ead78378dfc2ad15baa344e76e1ac0ca631c67848e81896fd6838b3821961928911635ca069f12c05772a08a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840d201ce4ca5572db792d1563ef3614f2e2b27072e4751327f4a8f75201183a189ac57cdd9399474850e87031c7545c896ebab3983434bb6005690b9ad8fd9d50aa100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258403f45e44aa7457c714599f0100702ec265e91493e30c57ba4f1f74d912858bae8fb71fdf2faddf865c816cb0218eda0db17b707c8f429290f1a1c02b6a4450a0ea100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840e16b1d8f5992fda268c9d7e5c0ab6c5d38b8abaa6b92ccae5b0d2f3079d098ab67ba9a15b27807746f3c7695091ec5bb74ba8772baae14d2786eb8a512d70201a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840628a5491c5d0b4e0202b7aae87a343afd642345b823252355f6d392d8398d2174c141622e3de167b4f82c3cb8b4e8105b341851005d2ec0c1e35c354006a910ba100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258408ad2ee919f520a903764e0322800f7c086f870374f063d2e62ad1dfbf54e71305d90371abe3a196132e123b9248281f2d676fb29442f80249f644ce1185dfc03a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840d53bf84fe8712171151bb6d5f988d76428292639737d817986b46d629aea6eac2a90675cbf0347ec004ce23f9ca0b2dcff5c6d1be91ab478634de8ba8ab96102a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258404702623a2a94b9efbc03dc7d506c4bd70c1e0fea8b21f3b76c592883f0c364ffc12215e59f9ea4d2eed2e786376e6128650b4c9c3f6ad9419f070fb458efa10ca100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb82584072bb89ee81a7bcc4a866ae498d3ca076d5d5a885547c7f5899b8b59b3077310f58df420e470bf36d4ed5beaaf30eb361f04ed578cdbd0ef04f7cb573f0c0770ca100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840e112bb46921840820f4d5b40ec45699bc1b818ca8fe77fcc222a6fa1edb2425487f32e2039e2cf6077ce1e8e2e0b0d0581c64fb866c1c183344af131ccb9390ba100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840039329e261d8386f81a28f5ef5062196a46b5d4389b06bde97e662f69e37812c3ee75352f392121f58e76e5c1e1649656632b01ea46f932ccedcee102d625001a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840c1dab5e2482101ad1bd04f4018425c7133171aaf1274573ed35305f4e37bddadb3636f0aa098d2c0b5f615e8eb629bb94afac5d4c4c0743dba16a847d898b905a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840abb898b3e1ae3c4c9d068a4517b83a070d4037f979b6365ea5eb368e7d43b3fd2152fb93a462fdc553f973d90ab136332057fb66ea529d4fbc53e7f082b6fe03a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258404a09ccfd9c09c6a453f46c721d280065081d89aa4b84fc809d75db1b923e78963bcbf36d64786d8c09c527e90da744e83116617b2e18d9145bac6cf66f876c08a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258408df2408adbd8b4c990a913b9ed2455c9de72d561ddb8f3ec0da5d1513f417a2fcee9ea9ace30cb840d37950c41455bd3655d12d534b70a6eac7034950f821108a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840534b54cc145722e3f7a62935d84c025e17e31c4b08d3f3fb16bb7673d37e9afb07fbdb5ffce5aeef743376bac161973e565e1c12db97bcd879cd7e9030c2a00ea100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840664fd5d8bc5d93509d02104f39e7a22c6cd894f49935cac9e662a9202b9a64baa9f55cd8aa07d3d1e095e9b974c59c0a8c50d14b0d077d70e236ad5cf52ac104a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840469cdadb48349224303d14980cab5c2ae5dacd0f6e36714d8dcb9ca85fa4eb688bd7b1334e30f8718178f7f36d8c5b204e0f9cdce5f88762fc2cbe2cb28c1d03a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840f330820de229a476e1b3f84dfcf9ad98264070212e6e2d61d8b05afb1e12a1426cfd7cb0b284db237d5882cecd6e8f1fe2cf9ddc06783242812178bcb053a105a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb82584066b89001a28583bed59dbd07d359333207eb74d39ee092c0bf1da4351da64d38c9938a3682bb52a4253dc76074767b4cc2bc1eb2a31bbe6be3c45a5c52cbdf04a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840f365b299297ade5117d72156050cb81a76ba0b859cb46d0f2326c4071110440108b20390f878ba082d41217b2a8fd5f1435b9ba48d176ad5dcb6faff54976b0da100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258407fb0b1f28d6ca64a29c6c7a51a2ab3436809b5038c06b8204104e3b98afa915246f742e2f1bd569f5132d2bbdcaeae68215a0b0f17f6367ce4eea37ed951ec01a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258406e6d2263c440486fc1b3c2aa930e519f20b70f70c28cb532d031f63cefc55c56f4647b10dd6390aa0d0f2ba75bf6cbe3ce2fc6d928dc4db74388f1e5e3057b0ca100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840dbb299923c24a70ae10dc637d862b750b3e6548e64c590674d2ceb87b7535199ea8dfd024694d26ae1dbbca683b1a4ba90af7d1680a9b8e4819a2ee6229e2408a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258405b2397d031c48f56b43416daea99dd3d8bd1733cb83c2a688dbe8b5dd9bfe64d596280d71973d7d540e929262dafd79b14954b855635fe845642090241003503a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840482c9089f2d60eb069432bf7f7213178a6fe3d52d37a4fa5aec677875bccdac64de7a45c6eb0bd4996414412b12d3e887a1b391e775ef56c47d53f9c944d020ba100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258401972d18c1e9c62837121efafddc2ec778a3a8f9ec5f534c9922778905d8f809609d6c92e427852d8b5f822ad590fdeacf3877c8056f0768b44b025a2b79e7704a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258409b7141b69a493bc4d597a645ed488325492772ad4c3cd5c5c7805a5d951a4b6ed960ea27428d1add867fe7c209f4e65000bdfa878bd7a4357b223e9c55af450da100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258407ac2d7823887de83bca86216e424ccb63fe9f4fa1f106bffc6afa388e91845c97177c410f1a8ca5ecd9f2701c42f5f9dd2faeb0ecf2163a37521badc8a6c1b03a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840c49b6714ccbbdebebb9461a2efb39b9ac5e00a389aadfb2a1b4fe384733610c45e1f03825f65e182988da97659a71e378d49d17fb93d76b80d579b7d49399b06a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840b53ea3508cbd7da47fef05e98c0b31b13ec33de4596ba4061a8e04d91b1015c49f328da58084a6f573d93cdb7aa0972a1a1936a69ee7362adf65df3eae4e2400a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840b1c6b15b311423aa83dfaebe118d1a2a3ff006399f2a63fa82f0d0e0c12bc2b844ec78f5bc8baeef588d15b2237db48cbfa48361a6d630052c9b68e62e035601a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840bb544721cd95655497375390da19fbd87b3c63a4edf333688e2fee7935e96b6572f84b81d80fee5239062be6d3c6a75a5f0c50696d3c6663d26cecdfd8fdc808a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840a7018dfacec705459856bc706b7d05d04c56867fb64dfd0cf97fa980e881cc609e91cf6df204fb6906f860e5cf390e0290d302b6231664aad8c2b4cb30090609a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258403ddf27cce21fbf1a361233d2bcff92ecc9d3cce64c3d8186495e3509b843a0a326f528be8241b8557bf3cdac9c304fcf0fa8fd2a8e389d6acf9fc62b5626d705a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb82584013bd40ae7383feb674c2cd3357122aec3f6efe17b9b4f25c47cd3dfec194d0c4f20a52cc30fb92245c1a20a962772808f3dc6ee51261c86af16879a1fed2210ba100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840a1cf100aff45ee45c0f868214187640c8c29cb005c7aab7100ff86338d78f972733a611b4e0dae436fe9e1493d6ece69c35ada3cc6506e730ea1bae277468108a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840667d6d2987788454c41f2e86867fe98e1fd48aa789fbf9cf2208f927a8f9941d0384ebf3e3e45ee80c49934aad9b6ccaa13179b69f35b9acd21b55f56caff80da100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258401cd5c48fa46d8f0fb07c7737b7719d1fba5729478be3eef3e2e19942c4d6d54b01a569eb34d4f4be84a2f6961832ec17bade0511cbc01f5db5749a09bb4d8808a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258403bb5ddd91b5f5d7366b41330bf5bbbf7cf7d703bd50376ac21b07c6da696562266361678c06247a57111c63bc1fe58463a8c125e0d117bdf05cd4fe57bb8b90aa100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840397067045ffe0cf7126a5f73f228e1bc8721c617ebb7c41c1bc2f7b6c8cc50bf2370bc1ee679bcb0581e11da1e186504f2e3f3322fddf40a1863067ffc5e2903a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840831f5ea5dd354f5a522e044b53aa1966d036871d5a3b7d2323e404996907a33aff5aabb9db22301064033045cbf14c91d29de84b8fbbb75683ff1cb51fd0920aa100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258401e8cc65db39cb5e9d54dac50cda84c55fd2f989f667a11dc46521768ac2f46a27a70173a92e849ee621ebe3025d87088528e7450b8312d678b6249f5a124f70fa10081825820b6a42d4ccc4d26adaec67e8578bf31f13b1b7e640527356248f2ec547f9de6e45840f4835bbcf5459db0826e27ea95d3ac31f7bea56c0253716212ef421995c7546a963ac89dc6cffad75692a149372cbdeaa19a9dcd171ac423711e8d71c495d703a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258408039411659145a9fb3863b2ae2f3890009bf004332f58daa6662368a7378d215cc7f40569284d5b86c5a7be210cdcb5551633762b5a7d3f8ad2095a220fec609a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb82584002c9ec1d95d0464eba5f4a656d72b55a92a80078e83f2f47682729af0fc782a68f3f31db7c64018ae8fbd36c5ac72d5573357a7578359056b9d3f9a29467d80ca100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258406f176d0fdb8768aad902c8fc115eb797719ef62a93938d7f09bbb213a88d61294143ec1d508a2a450f0e16ab3e2ffb7e0ed4cd7f75b3f2ff9f70cfaa77764506a10081825820b6a42d4ccc4d26adaec67e8578bf31f13b1b7e640527356248f2ec547f9de6e45840f4262eeb9183eec1e896b9be61984ed9d4192b38825ba1b560ea23fe6e3224d3c94f4cc64174c1d9166e665e1a1ff8f0c84024bb8b9068b853fe4ce683d96906a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840df64e327056e335f484582fa0e4188e62620968e955431fc3576da2c1935b15ec605bb5d738f5000dcdc022646f9545d6932c2c79611dccab116295ca03c2b04a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840132cce5cea5d8bf7e9b802e4477eff041ebe1c12a8b8658a42ae91727cbf4f39b0d23831c70923a68ad7a023092bcecb61ac6253fdd17be00cecc37a71301300a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840842ecaf1b907cb34799d78d6f35eb349a986559a396840aeba5f6e8dc7e4172c16bddcb1f926a21175531478773046e9484aeb4ca83c1cbaf25f5a4813afdd0ca100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb825840a417cdf9e02755e53d6061ce0c93cacb7de24ce33e3fda9ac3a85414a7bf62676446822875972867d5647e46c22e21099e5dd8e4114949b30b86146b0dba1b05a100818258206ca065df8b220ae79a96e871f92e53b7e816200b789749ab5f38e105a436eb8258401f2e86831349fa458c3e2403e2caacbf046fae3c513575e8ce2d34037d34337f7e58cc98cadf034e8bce930335285220624945b316fe6af71e8ef0d12ef05001ffa100d90103a100a11902a2a1636d73678f78264175746f2d4c6f6f702d5472616e73616374696f6e202336313138303020627920415441444160783c4c6976652045706f6368203234352c207765206861766520303132682032386d20323773206c65667420756e74696c20746865206e657874206f6e6578374974277320446f6e6e657273746167202d20313520466562727561722032303234202d2031333a30313a333320696e20417573747269616060607820412072616e646f6d205a656e2d51756f746520666f7220796f753a20f09f998f783b4265206b696e642c20666f722065766572796f6e6520796f75206d656574206973206669676874696e6720612068617264657220626174746c652e68202d20506c61746f6078374e6f64652d5265766973696f6e3a203462623230343864623737643632336565366533363738363138633264386236633436373633333360782953616e63686f4e657420697320617765736f6d652c206861766520736f6d652066756e2120f09f988d7819204265737420726567617264732c204d617274696e203a2d2980";
    let bytes = hex::decode(hex).unwrap();
    let block = Block::from_wrapped_bytes(bytes);
    assert!(block.is_ok());
}

#[test]
fn redeemers_default_array_round_trip() {
    let mut redeemers = Redeemers::from(vec![
        Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(12),
            &PlutusData::new_integer(&BigInt::one()),
            &ExUnits::new(&to_bignum(123), &to_bignum(456)),
        ),
        Redeemer::new(
            &RedeemerTag::new_cert(),
            &to_bignum(2),
            &PlutusData::new_integer(&BigInt::from(22)),
            &ExUnits::new(&to_bignum(23), &to_bignum(45)),
        )
    ]);

    let bytes = redeemers.to_bytes();
    let new_redeemers = Redeemers::from_bytes(bytes.clone()).unwrap();

    assert_eq!(new_redeemers.serialization_format, Some(CborContainerType::Array));
    assert_eq!(redeemers.serialization_format, None);
    assert_eq!(redeemers, new_redeemers);
    assert_eq!(bytes, new_redeemers.to_bytes())
}

#[test]
fn redeemers_array_round_trip() {
    let redeemers_vec = vec![
        Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(12),
            &PlutusData::new_integer(&BigInt::one()),
            &ExUnits::new(&to_bignum(123), &to_bignum(456)),
        ),
        Redeemer::new(
            &RedeemerTag::new_cert(),
            &to_bignum(2),
            &PlutusData::new_integer(&BigInt::from(22)),
            &ExUnits::new(&to_bignum(23), &to_bignum(45)),
        )
    ];

    let redeemers = Redeemers::new_with_serialization_format(redeemers_vec, CborContainerType::Array);

    let bytes = redeemers.to_bytes();
    let new_redeemers = Redeemers::from_bytes(bytes.clone()).unwrap();

    assert_eq!(new_redeemers.serialization_format, Some(CborContainerType::Array));
    assert_eq!(redeemers, new_redeemers);
    assert_eq!(bytes, new_redeemers.to_bytes())
}

#[test]
fn redeemers_map_round_trip() {
    let redeemers_vec = vec![
        Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(12),
            &PlutusData::new_integer(&BigInt::one()),
            &ExUnits::new(&to_bignum(123), &to_bignum(456)),
        ),
        Redeemer::new(
            &RedeemerTag::new_cert(),
            &to_bignum(2),
            &PlutusData::new_integer(&BigInt::from(22)),
            &ExUnits::new(&to_bignum(23), &to_bignum(45)),
        )
    ];

    let redeemers = Redeemers::new_with_serialization_format(redeemers_vec, CborContainerType::Map);

    let bytes = redeemers.to_bytes();
    let new_redeemers = Redeemers::from_bytes(bytes.clone()).unwrap();

    assert_eq!(new_redeemers.serialization_format, Some(CborContainerType::Map));
    assert_eq!(redeemers, new_redeemers);
    assert_eq!(bytes, new_redeemers.to_bytes())
}

#[test]
fn redeemers_map_array_round_trip() {
    let redeemers_vec = vec![
        Redeemer::new(
            &RedeemerTag::new_spend(),
            &to_bignum(12),
            &PlutusData::new_integer(&BigInt::one()),
            &ExUnits::new(&to_bignum(123), &to_bignum(456)),
        ),
        Redeemer::new(
            &RedeemerTag::new_cert(),
            &to_bignum(2),
            &PlutusData::new_integer(&BigInt::from(22)),
            &ExUnits::new(&to_bignum(23), &to_bignum(45)),
        )
    ];

    let redeemers_array = Redeemers::new_with_serialization_format(redeemers_vec.clone(), CborContainerType::Array);
    let redeemers_map = Redeemers::new_with_serialization_format(redeemers_vec, CborContainerType::Map);

    let bytes_array = redeemers_array.to_bytes();
    let new_redeemers_array = Redeemers::from_bytes(bytes_array.clone()).unwrap();

    let bytes_map = redeemers_map.to_bytes();
    let new_redeemers_map = Redeemers::from_bytes(bytes_map.clone()).unwrap();

    assert_eq!(new_redeemers_array.serialization_format, Some(CborContainerType::Array));
    assert_eq!(redeemers_array, new_redeemers_array);
    assert_eq!(bytes_array, new_redeemers_array.to_bytes());

    assert_eq!(new_redeemers_map.serialization_format, Some(CborContainerType::Map));
    assert_eq!(redeemers_map, new_redeemers_map);
    assert_eq!(bytes_map, new_redeemers_map.to_bytes());

    assert_eq!(new_redeemers_map, new_redeemers_array);
    assert_ne!(bytes_array, bytes_map)
}