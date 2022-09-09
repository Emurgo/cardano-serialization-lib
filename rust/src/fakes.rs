#![allow(dead_code)]
use crate::{to_bignum, Address, BaseAddress, Bip32PrivateKey, DataHash, Ed25519KeyHash, Ed25519Signature, NetworkInfo, StakeCredential, TransactionHash, TransactionIndex, TransactionInput, TransactionOutput, Value, Vkey, PolicyID};

pub(crate) fn fake_bytes_32(x: u8) -> Vec<u8> {
    vec![
        x, 239, 181, 120, 142, 135, 19, 200, 68, 223, 211, 43, 46, 145, 222, 30, 48, 159, 239, 255,
        213, 85, 248, 39, 204, 158, 225, 100, 1, 2, 3, 4,
    ]
}

pub(crate) fn fake_data_hash(x: u8) -> DataHash {
    DataHash::from_bytes(fake_bytes_32(x)).unwrap()
}

pub(crate) fn fake_key_hash(x: u8) -> Ed25519KeyHash {
    Ed25519KeyHash::from_bytes((&fake_bytes_32(x)[0..28]).to_vec()).unwrap()
}

pub(crate) fn fake_base_address(x: u8) -> Address {
    BaseAddress::new(
        NetworkInfo::testnet().network_id(),
        &StakeCredential::from_keyhash(&fake_key_hash(x)),
        &StakeCredential::from_keyhash(&fake_key_hash(0)),
    )
    .to_address()
}

pub(crate) fn fake_tx_hash(input_hash_byte: u8) -> TransactionHash {
    TransactionHash::from([input_hash_byte; 32])
}

pub(crate) fn fake_tx_input(input_hash_byte: u8) -> TransactionInput {
    fake_tx_input2(input_hash_byte, 0)
}

pub(crate) fn fake_tx_input2(input_hash_byte: u8, idx: TransactionIndex) -> TransactionInput {
    TransactionInput::new(&fake_tx_hash(input_hash_byte), idx)
}

pub(crate) fn fake_value() -> Value {
    fake_value2(1_000_000)
}

pub(crate) fn fake_value2(v: u64) -> Value {
    Value::new(&to_bignum(v))
}

pub(crate) fn fake_tx_output(input_hash_byte: u8) -> TransactionOutput {
    TransactionOutput::new(&fake_base_address(input_hash_byte), &fake_value())
}

pub(crate) fn fake_tx_output2(input_hash_byte: u8, val: u64) -> TransactionOutput {
    TransactionOutput::new(&fake_base_address(input_hash_byte), &fake_value2(val))
}

pub(crate) fn fake_vkey() -> Vkey {
    Vkey::new(
        &Bip32PrivateKey::generate_ed25519_bip32()
            .unwrap()
            .to_public()
            .to_raw_key(),
    )
}

pub(crate) fn fake_signature(x: u8) -> Ed25519Signature {
    Ed25519Signature::from_bytes([x; 64].to_vec()).unwrap()
}

pub(crate) fn fake_policy_id(x: u8) -> PolicyID {
    PolicyID::from([x; 28])
}
