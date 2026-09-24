use crate::*;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const DEPTH: usize = 10_000;

fn fake_keyhash() -> Ed25519KeyHash {
    Ed25519KeyHash::from([0u8; Ed25519KeyHash::BYTE_COUNT])
}

fn native_script_all_spine(depth: usize) -> NativeScript {
    let mut current =
        NativeScript::new_script_pubkey(&ScriptPubkey::new(&fake_keyhash()));
    for _ in 0..depth {
        let mut scripts = NativeScripts::new();
        scripts.push_move(current);
        current = NativeScript::from_script_all_moved(scripts);
    }
    current
}

fn plutus_list_spine(depth: usize) -> PlutusData {
    let mut current = PlutusData::new_integer(&BigInt::from(1i32));
    for _ in 0..depth {
        let mut list = PlutusList::new();
        list.push(current);
        current = PlutusData::from_list_moved(list);
    }
    current
}

fn plutus_constr_spine(depth: usize) -> PlutusData {
    let mut current = PlutusData::new_integer(&BigInt::from(1i32));
    for _ in 0..depth {
        let mut list = PlutusList::new();
        list.push(current);
        current = PlutusData::from_constr_moved(ConstrPlutusData::new_moved(
            BigNum::from(0u32),
            list,
        ));
    }
    current
}

fn metadatum_list_spine(depth: usize) -> TransactionMetadatum {
    let mut current = TransactionMetadatum::new_int(&Int::new_i32(1));
    for _ in 0..depth {
        let mut list = MetadataList::new();
        list.push_move(current);
        current = TransactionMetadatum::from_list_moved(list);
    }
    current
}

fn assert_hash_eq<T: Hash>(a: &T, b: &T) {
    let mut ha = DefaultHasher::new();
    let mut hb = DefaultHasher::new();
    a.hash(&mut ha);
    b.hash(&mut hb);
    assert_eq!(ha.finish(), hb.finish());
}

#[test]
fn native_script_spine_cbor_clone_eq_drop() {
    let script = native_script_all_spine(DEPTH);
    let bytes = script.to_bytes();
    let decoded = NativeScript::from_bytes(bytes.clone()).unwrap();
    assert_eq!(decoded.to_bytes(), bytes);
    let cloned = decoded.clone();
    assert_eq!(cloned, decoded);
    let _ = format!("{:?}", cloned);
    let _ = cloned.hash();
    let _ = cloned.get_required_signers();
}

// JSON input stays on serde_json, whose recursion limit (128 levels) turns pathological depth
// into an error. Only the output side is iterative, since chain data of arbitrary depth feeds it.
// A nested `ScriptAll` costs three JSON levels per script, DetailedSchema lists two.
const JSON_INPUT_SAFE_DEPTH: usize = 32;

#[test]
fn native_script_json_output_is_iterative_and_input_is_depth_limited() {
    let deep = native_script_all_spine(DEPTH);
    let json = deep.to_json().unwrap();
    assert!(NativeScript::from_json(&json).is_err());
    let mut scripts = NativeScripts::new();
    scripts.push_move(deep);
    let json = scripts.to_json().unwrap();
    assert!(NativeScripts::from_json(&json).is_err());

    let script = native_script_all_spine(JSON_INPUT_SAFE_DEPTH);
    let json = script.to_json().unwrap();
    assert_eq!(NativeScript::from_json(&json).unwrap(), script);
    let mut scripts = NativeScripts::new();
    scripts.push_move(script);
    let json = scripts.to_json().unwrap();
    assert_eq!(NativeScripts::from_json(&json).unwrap(), scripts);
}

#[test]
fn native_scripts_synthetic_818201_pattern() {
    let script = native_script_all_spine(64);
    let mut scripts = NativeScripts::new();
    scripts.push_move(script);
    let bytes = scripts.to_bytes();
    let hex = hex::encode(&bytes);
    assert!(
        hex.contains("818201"),
        "expected nested ScriptAll CBOR pattern 818201, got {}",
        &hex[..hex.len().min(32)]
    );
    let roundtrip = NativeScripts::from_bytes(bytes.clone()).unwrap();
    assert_eq!(roundtrip.to_bytes(), bytes);
}

#[test]
fn plutus_list_spine_cbor_clone_eq_drop() {
    let data = plutus_list_spine(DEPTH);
    let encoded = data.to_bytes();
    let decoded = PlutusData::from_bytes(encoded.clone()).unwrap();
    assert_eq!(decoded.to_bytes(), encoded);
    assert!(decoded.original_bytes.is_some());
    let cloned = decoded.clone();
    assert_eq!(cloned, decoded);
    assert_eq!(cloned.cmp(&decoded), Ordering::Equal);
    assert_hash_eq(&cloned, &decoded);
    let _ = format!("{:?}", cloned);
}

#[test]
fn plutus_constr_spine_hash_ord_original_bytes() {
    let constructed = plutus_constr_spine(DEPTH);
    assert!(constructed.original_bytes.is_none());
    let encoded = constructed.to_bytes();
    let decoded = PlutusData::from_bytes(encoded.clone()).unwrap();
    assert!(decoded.original_bytes.is_some());
    assert_eq!(decoded.to_bytes(), encoded);
    assert_eq!(constructed, decoded);
    let cloned = constructed.clone();
    assert_eq!(cloned.cmp(&constructed), Ordering::Equal);
    assert_hash_eq(&cloned, &constructed);
    let decoded_clone = decoded.clone();
    assert_hash_eq(&decoded, &decoded_clone);
}

#[test]
fn plutus_json_output_is_iterative_and_input_is_depth_limited() {
    let deep = plutus_list_spine(DEPTH);
    for schema in [PlutusDatumSchema::DetailedSchema, PlutusDatumSchema::BasicConversions] {
        let json = decode_plutus_datum_to_json_str(&deep, schema).unwrap();
        assert!(encode_json_str_to_plutus_datum(&json, schema).is_err());
    }

    let data = plutus_list_spine(JSON_INPUT_SAFE_DEPTH);
    for schema in [PlutusDatumSchema::DetailedSchema, PlutusDatumSchema::BasicConversions] {
        let json = decode_plutus_datum_to_json_str(&data, schema).unwrap();
        assert_eq!(encode_json_str_to_plutus_datum(&json, schema).unwrap(), data);
    }
}

#[test]
fn metadatum_list_spine_cbor_clone_eq_drop() {
    let data = metadatum_list_spine(DEPTH);
    let bytes = data.to_bytes();
    let decoded = TransactionMetadatum::from_bytes(bytes.clone()).unwrap();
    assert_eq!(decoded.to_bytes(), bytes);
    let cloned = decoded.clone();
    assert_eq!(cloned, decoded);
    assert_eq!(cloned.cmp(&decoded), Ordering::Equal);
    assert_hash_eq(&cloned, &decoded);
    let _ = format!("{:?}", cloned);
}

#[test]
fn metadatum_json_output_is_iterative_and_input_is_depth_limited() {
    let deep = metadatum_list_spine(DEPTH);
    for schema in [MetadataJsonSchema::NoConversions, MetadataJsonSchema::DetailedSchema] {
        let json = decode_metadatum_to_json_str(&deep, schema).unwrap();
        assert!(encode_json_str_to_metadatum(json, schema).is_err());
    }

    let data = metadatum_list_spine(JSON_INPUT_SAFE_DEPTH);
    for schema in [
        MetadataJsonSchema::NoConversions,
        MetadataJsonSchema::BasicConversions,
        MetadataJsonSchema::DetailedSchema,
    ] {
        let json = decode_metadatum_to_json_str(&data, schema).unwrap();
        assert_eq!(encode_json_str_to_metadatum(json, schema).unwrap(), data);
    }
}

#[test]
fn json_object_keys_follow_serde_json_map_semantics() {
    // JSON text is parsed through `serde_json::Value`, whose object is a `BTreeMap`: keys come
    // out sorted and a repeated key keeps its last value. The CBOR produced from JSON text
    // must not depend on the key order in the document, and must match the `Value` entry point.
    let text = r#"{"zeta":1,"alpha":2,"zeta":3}"#;

    let from_str =
        encode_json_str_to_metadatum(text.to_string(), MetadataJsonSchema::NoConversions).unwrap();
    let from_value = encode_json_value_to_metadatum(
        serde_json::from_str(text).unwrap(),
        MetadataJsonSchema::NoConversions,
    )
    .unwrap();
    assert_eq!(from_str.to_hex(), from_value.to_hex());
    assert_eq!(from_str.to_hex(), "a265616c70686102647a65746103");

    let from_str = encode_json_str_to_plutus_datum(text, PlutusDatumSchema::BasicConversions).unwrap();
    let from_value = encode_json_value_to_plutus_datum(
        serde_json::from_str(text).unwrap(),
        PlutusDatumSchema::BasicConversions,
    )
    .unwrap();
    assert_eq!(from_str.to_hex(), from_value.to_hex());
}

#[test]
fn json_output_map_keys_are_sorted_like_serde_value() {
    let mut map = MetadataMap::new();
    map.insert_str("zeta", &TransactionMetadatum::new_int(&Int::new_i32(1))).unwrap();
    map.insert_str("alpha", &TransactionMetadatum::new_int(&Int::new_i32(2))).unwrap();
    let md = TransactionMetadatum::new_map(&map);
    let text = decode_metadatum_to_json_str(&md, MetadataJsonSchema::BasicConversions).unwrap();
    assert_eq!(text, r#"{"alpha":2,"zeta":1}"#);
    let value = decode_metadatum_to_json_value(&md, MetadataJsonSchema::BasicConversions).unwrap();
    assert_eq!(value.to_string(), text);

    let mut pmap = PlutusMap::new();
    pmap.add_value(
        &PlutusData::new_bytes(b"zeta".to_vec()),
        &PlutusData::new_integer(&BigInt::from(1i32)),
    );
    pmap.add_value(
        &PlutusData::new_bytes(b"alpha".to_vec()),
        &PlutusData::new_integer(&BigInt::from(2i32)),
    );
    let pd = PlutusData::new_map(&pmap);
    let text = decode_plutus_datum_to_json_str(&pd, PlutusDatumSchema::BasicConversions).unwrap();
    assert_eq!(text, r#"{"alpha":2,"zeta":1}"#);
    let value = decode_plutus_datum_to_json_value(&pd, PlutusDatumSchema::BasicConversions).unwrap();
    assert_eq!(value.to_string(), text);
}

#[test]
fn plutus_general_form_constr_requires_two_element_array() {
    // tag 102 must wrap exactly `[alternative, fields]`
    let valid = hex::decode("d866820080").unwrap();
    let decoded = PlutusData::from_bytes(valid.clone()).unwrap();
    assert_eq!(decoded.to_bytes(), valid);
    assert_eq!(
        decoded.as_constr_plutus_data().unwrap().alternative(),
        BigNum::zero()
    );
    // `[alternative]` followed by a stray list used to be accepted
    assert!(PlutusData::from_bytes(hex::decode("d866810080").unwrap()).is_err());
    assert!(PlutusData::from_bytes(hex::decode("d86683008000").unwrap()).is_err());
}
