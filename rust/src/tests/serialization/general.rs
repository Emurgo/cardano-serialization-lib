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
        ws.set_redeemers(&Redeemers(vec![Redeemer::new(
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
