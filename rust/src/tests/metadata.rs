use crate::*;

#[test]
fn binary_encoding() {
    let input_bytes = (0..1000).map(|x| x as u8).collect::<Vec<u8>>();
    let metadata = encode_arbitrary_bytes_as_metadatum(input_bytes.as_ref());
    let output_bytes = decode_arbitrary_bytes_from_metadatum(&metadata).expect("decode failed");
    assert_eq!(input_bytes, output_bytes);
}

#[test]
fn json_encoding_no_conversions() {
    let input_str = String::from("{\"receiver_id\": \"SJKdj34k3jjKFDKfjFUDfdjkfd\",\"sender_id\": \"jkfdsufjdk34h3Sdfjdhfduf873\",\"comment\": \"happy birthday\",\"tags\": [0, 264, -1024, 32]}");
    let metadata =
        encode_json_str_to_metadatum(input_str.clone(), MetadataJsonSchema::NoConversions)
            .expect("encode failed");
    let map = metadata.as_map().unwrap();
    assert_eq!(
        map.get_str("receiver_id").unwrap().as_text().unwrap(),
        "SJKdj34k3jjKFDKfjFUDfdjkfd"
    );
    assert_eq!(
        map.get_str("sender_id").unwrap().as_text().unwrap(),
        "jkfdsufjdk34h3Sdfjdhfduf873"
    );
    assert_eq!(
        map.get_str("comment").unwrap().as_text().unwrap(),
        "happy birthday"
    );
    let tags = map.get_str("tags").unwrap().as_list().unwrap();
    let tags_i32 = tags
        .0
        .iter()
        .map(|md| md.as_int().unwrap().as_i32_or_fail().unwrap())
        .collect::<Vec<i32>>();
    assert_eq!(tags_i32, vec![0, 264, -1024, 32]);
    let output_str = decode_metadatum_to_json_str(&metadata, MetadataJsonSchema::NoConversions)
        .expect("decode failed");
    let input_json: serde_json::Value = serde_json::from_str(&input_str).unwrap();
    let output_json: serde_json::Value = serde_json::from_str(&output_str).unwrap();
    assert_eq!(input_json, output_json);
}

#[test]
fn json_encoding_basic() {
    let input_str =
        String::from("{\"0x8badf00d\": \"0xdeadbeef\",\"9\": 5,\"obj\": {\"a\":[{\"5\": 2},{}]}}");
    let metadata =
        encode_json_str_to_metadatum(input_str.clone(), MetadataJsonSchema::BasicConversions)
            .expect("encode failed");
    json_encoding_check_example_metadatum(&metadata);
    let output_str = decode_metadatum_to_json_str(&metadata, MetadataJsonSchema::BasicConversions)
        .expect("decode failed");
    let input_json: serde_json::Value = serde_json::from_str(&input_str).unwrap();
    let output_json: serde_json::Value = serde_json::from_str(&output_str).unwrap();
    assert_eq!(input_json, output_json);
}

#[test]
fn json_encoding_detailed() {
    let input_str = String::from(
        "{\"map\":[
            {
                \"k\":{\"bytes\":\"8badf00d\"},
                \"v\":{\"bytes\":\"deadbeef\"}
            },
            {
                \"k\":{\"int\":9},
                \"v\":{\"int\":5}
            },
            {
                \"k\":{\"string\":\"obj\"},
                \"v\":{\"map\":[
                    {
                        \"k\":{\"string\":\"a\"},
                        \"v\":{\"list\":[
                        {\"map\":[
                            {
                                \"k\":{\"int\":5},
                                \"v\":{\"int\":2}
                            }
                            ]},
                            {\"map\":[
                            ]}
                        ]}
                    }
                ]}
            }
        ]}",
    );
    let metadata =
        encode_json_str_to_metadatum(input_str.clone(), MetadataJsonSchema::DetailedSchema)
            .expect("encode failed");
    json_encoding_check_example_metadatum(&metadata);
    let output_str = decode_metadatum_to_json_str(&metadata, MetadataJsonSchema::DetailedSchema)
        .expect("decode failed");
    let input_json: serde_json::Value = serde_json::from_str(&input_str).unwrap();
    let output_json: serde_json::Value = serde_json::from_str(&output_str).unwrap();
    assert_eq!(input_json, output_json);
}

fn json_encoding_check_example_metadatum(metadata: &TransactionMetadatum) {
    let map = metadata.as_map().unwrap();
    assert_eq!(
        map.get(&TransactionMetadatum::new_bytes(hex::decode("8badf00d").unwrap()).unwrap())
            .unwrap()
            .as_bytes()
            .unwrap(),
        hex::decode("deadbeef").unwrap()
    );
    assert_eq!(
        map.get_i32(9)
            .unwrap()
            .as_int()
            .unwrap()
            .as_i32_or_fail()
            .unwrap(),
        5
    );
    let inner_map = map.get_str("obj").unwrap().as_map().unwrap();
    let a = inner_map.get_str("a").unwrap().as_list().unwrap();
    let a1 = a.get(0).as_map().unwrap();
    assert_eq!(
        a1.get_i32(5)
            .unwrap()
            .as_int()
            .unwrap()
            .as_i32_or_fail()
            .unwrap(),
        2
    );
    let a2 = a.get(1).as_map().unwrap();
    assert_eq!(a2.keys().len(), 0);
}

#[test]
fn json_encoding_detailed_complex_key() {
    let input_str = String::from(
        "{\"map\":[
            {
            \"k\":{\"list\":[
                {\"map\": [
                    {
                        \"k\": {\"int\": 5},
                        \"v\": {\"int\": -7}
                    },
                    {
                        \"k\": {\"string\": \"hello\"},
                        \"v\": {\"string\": \"world\"}
                    }
                ]},
                {\"bytes\": \"ff00ff00\"}
            ]},
            \"v\":{\"int\":5}
            }
        ]}",
    );
    let metadata =
        encode_json_str_to_metadatum(input_str.clone(), MetadataJsonSchema::DetailedSchema)
            .expect("encode failed");

    let map = metadata.as_map().unwrap();
    let key = map.keys().get(0);
    assert_eq!(
        map.get(&key)
            .unwrap()
            .as_int()
            .unwrap()
            .as_i32_or_fail()
            .unwrap(),
        5
    );
    let key_list = key.as_list().unwrap();
    assert_eq!(key_list.len(), 2);
    let key_map = key_list.get(0).as_map().unwrap();
    assert_eq!(
        key_map
            .get_i32(5)
            .unwrap()
            .as_int()
            .unwrap()
            .as_i32_or_fail()
            .unwrap(),
        -7
    );
    assert_eq!(
        key_map.get_str("hello").unwrap().as_text().unwrap(),
        "world"
    );
    let key_bytes = key_list.get(1).as_bytes().unwrap();
    assert_eq!(key_bytes, hex::decode("ff00ff00").unwrap());

    let output_str = decode_metadatum_to_json_str(&metadata, MetadataJsonSchema::DetailedSchema)
        .expect("decode failed");
    let input_json: serde_json::Value = serde_json::from_str(&input_str).unwrap();
    let output_json: serde_json::Value = serde_json::from_str(&output_str).unwrap();
    assert_eq!(input_json, output_json);
}

#[test]
fn metadata_serialize() {
    let mut gmd = GeneralTransactionMetadata::new();
    let mdatum = TransactionMetadatum::new_text(String::from("string md")).unwrap();
    gmd.insert(&BigNum(100), &mdatum);
    let mut aux_data = AuxiliaryData::new();
    // alonzo (empty)
    let ad0_deser = AuxiliaryData::from_bytes(aux_data.to_bytes()).unwrap();
    assert_eq!(aux_data.to_bytes(), ad0_deser.to_bytes());
    // pre-mary shelley
    aux_data.set_metadata(&gmd);
    let ad1_deser = AuxiliaryData::from_bytes(aux_data.to_bytes()).unwrap();
    assert_eq!(aux_data.to_bytes(), ad1_deser.to_bytes());
    // mary shelley
    let mut native_scripts = NativeScripts::new();
    native_scripts.add(&NativeScript::new_timelock_start(&TimelockStart::new(20)));
    aux_data.set_native_scripts(&native_scripts);
    let ad2_deser = AuxiliaryData::from_bytes(aux_data.to_bytes()).unwrap();
    assert_eq!(aux_data.to_bytes(), ad2_deser.to_bytes());
    // alonzo
    let mut plutus_scripts = PlutusScripts::new();
    plutus_scripts.add(&PlutusScript::new([61u8; 29].to_vec()));
    aux_data.set_plutus_scripts(&plutus_scripts);
    let ad3_deser = AuxiliaryData::from_bytes(aux_data.to_bytes()).unwrap();
    assert_eq!(aux_data.to_bytes(), ad3_deser.to_bytes());
}

#[test]
fn alonzo_metadata_round_trip() {
    let bytes_alonzo = hex::decode("d90103a100a1186469737472696e67206d64").unwrap();
    let aux_alonzo = AuxiliaryData::from_bytes(bytes_alonzo.clone()).unwrap();
    assert!(aux_alonzo.prefer_alonzo_format);
    assert_eq!(aux_alonzo.to_bytes(), bytes_alonzo);

    let bytes_pre_alonzo = hex::decode("a1186469737472696e67206d64").unwrap();
    let aux_pre_alonzo = AuxiliaryData::from_bytes(bytes_pre_alonzo.clone()).unwrap();
    assert!(!aux_pre_alonzo.prefer_alonzo_format);
    assert_eq!(aux_pre_alonzo.to_bytes(), bytes_pre_alonzo);
}

#[test]
fn metadatum_map_duplicate_keys() {
    let bytes = hex::decode("a105a4781b232323232323232323232323232323232323232323232323232323827840232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323237840232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323236e232323232323232323232323232382a36f2323232323232323232323232323236a323030302d30312d303166232323232323784023232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323712323232323232323232323232323232323784023232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323a36f2323232323232323232323232323236a323030302d30312d303166232323232323784023232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323712323232323232323232323232323232323784023232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323752323232323232323232323232323232323232323236a323030302d30312d3031752323232323232323232323232323232323232323236a323030302d30312d3031").unwrap();
    TransactionMetadatum::from_bytes(bytes).unwrap();
}

#[test]
fn test_auxiliary_data_roundtrip() {
    fn auxiliary_data_roundtrip(plutus_scripts: &PlutusScripts) {
        let mut aux = AuxiliaryData::new();
        let mut metadata = GeneralTransactionMetadata::new();
        metadata.insert(
            &BigNum(42),
            &encode_json_str_to_metadatum(
                "{ \"test\": 148 }".to_string(),
                MetadataJsonSchema::BasicConversions,
            )
            .unwrap(),
        );
        aux.set_metadata(&metadata);
        aux.set_native_scripts(&NativeScripts::from(vec![
            NativeScript::new_timelock_start(&TimelockStart::new(1234556)),
        ]));
        aux.set_plutus_scripts(plutus_scripts);
        assert_eq!(AuxiliaryData::from_bytes(aux.to_bytes()).unwrap(), aux);
    }

    let bytes = hex::decode("4e4d01000033222220051200120011").unwrap();
    let script_v1 = PlutusScript::from_bytes(bytes.clone()).unwrap();
    let script_v2 = PlutusScript::from_bytes_v2(bytes.clone()).unwrap();
    let script_v3 = PlutusScript::from_bytes_v3(bytes.clone()).unwrap();

    auxiliary_data_roundtrip(&PlutusScripts(vec![]));
    auxiliary_data_roundtrip(&PlutusScripts(vec![script_v1.clone()]));
    auxiliary_data_roundtrip(&PlutusScripts(vec![script_v2.clone()]));
    auxiliary_data_roundtrip(&PlutusScripts(vec![script_v3.clone()]));
    auxiliary_data_roundtrip(&PlutusScripts(vec![
        script_v1.clone(),
        script_v2.clone(),
        script_v3.clone(),
    ]));
}
