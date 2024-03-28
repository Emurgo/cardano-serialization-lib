use crate::*;

#[test]
fn nonce_identity() {
    let orig = Nonce::new_identity();
    let deser = Nonce::deserialize(&mut Deserializer::from(std::io::Cursor::new(
        orig.to_bytes(),
    )))
    .unwrap();
    assert_eq!(orig.to_bytes(), deser.to_bytes());
}

#[test]
fn nonce_hash() {
    let orig = Nonce::new_from_hash(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ])
    .unwrap();
    let deser = Nonce::deserialize(&mut Deserializer::from(std::io::Cursor::new(
        orig.to_bytes(),
    )))
    .unwrap();
    assert_eq!(orig.to_bytes(), deser.to_bytes());
}

#[test]
fn xprv_128_test() {
    // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
    let entropy = [
        0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4,
        0xd8, 0x09, 0x7c, 0x64, 0x12,
    ];
    let root_key = Bip32PrivateKey::from_bip39_entropy(&entropy, &[]);

    assert_eq!(hex::encode(&root_key.as_bytes()), "b8f2bece9bdfe2b0282f5bad705562ac996efb6af96b648f4445ec44f47ad95c10e3d72f26ed075422a36ed8585c745a0e1150bcceba2357d058636991f38a3791e248de509c070d812ab2fda57860ac876bc489192c1ef4ce253c197ee219a4");
    let xprv_128 = root_key.to_128_xprv();
    // test the 128 xprv is the right format
    assert_eq!(hex::encode(&xprv_128), "b8f2bece9bdfe2b0282f5bad705562ac996efb6af96b648f4445ec44f47ad95c10e3d72f26ed075422a36ed8585c745a0e1150bcceba2357d058636991f38a37cf76399a210de8720e9fa894e45e41e29ab525e30bc402801c076250d1585bcd91e248de509c070d812ab2fda57860ac876bc489192c1ef4ce253c197ee219a4");
    let root_key_copy = Bip32PrivateKey::from_128_xprv(&xprv_128).unwrap();

    // test converting to and back is equivalent to the identity function
    assert_eq!(root_key.to_bech32(), root_key_copy.to_bech32());
}

#[test]
fn chaincode_gen() {
    // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
    let entropy = [
        0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4,
        0xd8, 0x09, 0x7c, 0x64, 0x12,
    ];
    let root_key = Bip32PrivateKey::from_bip39_entropy(&entropy, &[]);

    let prv_chaincode = root_key.chaincode();
    assert_eq!(
        hex::encode(&prv_chaincode),
        "91e248de509c070d812ab2fda57860ac876bc489192c1ef4ce253c197ee219a4"
    );

    let pub_chaincode = root_key.to_public().chaincode();
    assert_eq!(
        hex::encode(&pub_chaincode),
        "91e248de509c070d812ab2fda57860ac876bc489192c1ef4ce253c197ee219a4"
    );
}

#[test]
fn private_key_from_bech32() {
    let pk = PrivateKey::generate_ed25519().unwrap();
    let pk_ext = PrivateKey::generate_ed25519extended().unwrap();

    assert_eq!(
        PrivateKey::from_bech32(&pk.to_bech32()).unwrap().as_bytes(),
        pk.as_bytes(),
    );
    assert_eq!(
        PrivateKey::from_bech32(&pk_ext.to_bech32())
            .unwrap()
            .as_bytes(),
        pk_ext.as_bytes(),
    );

    let er = PrivateKey::from_bech32("qwe");
    assert!(er.is_err());
}
