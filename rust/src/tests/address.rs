use crate::*;
use crypto::*;

#[test]
fn variable_nat_encoding() {
    let cases = [0u64, 127u64, 128u64, 255u64, 256275757658493284u64];
    for case in cases.iter() {
        let encoded = variable_nat_encode(*case);
        let decoded = variable_nat_decode(&encoded).unwrap().0;
        assert_eq!(*case, decoded);
    }
}

#[test]
fn variable_nat_decode_too_big() {
    let too_big = [129, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127];
    assert_eq!(None, variable_nat_decode(&too_big));
}

#[test]
fn base_serialize_consistency() {
    let base = BaseAddress::new(
        5,
        &Credential::from_keyhash(&Ed25519KeyHash::from([23; Ed25519KeyHash::BYTE_COUNT])),
        &Credential::from_scripthash(&ScriptHash::from([42; ScriptHash::BYTE_COUNT])),
    );
    let addr = base.to_address();
    let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
    assert_eq!(addr.to_bytes(), addr2.to_bytes());
}

#[test]
fn ptr_serialize_consistency() {
    let ptr = PointerAddress::new(
        25,
        &Credential::from_keyhash(&Ed25519KeyHash::from([23; Ed25519KeyHash::BYTE_COUNT])),
        &Pointer::new_pointer(&BigNum(2354556573), &BigNum(127), &BigNum(0)),
    );
    let addr = ptr.to_address();
    let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
    assert_eq!(addr.to_bytes(), addr2.to_bytes());
}

#[test]
fn enterprise_serialize_consistency() {
    let enterprise = EnterpriseAddress::new(
        64,
        &Credential::from_keyhash(&Ed25519KeyHash::from([23; Ed25519KeyHash::BYTE_COUNT])),
    );
    let addr = enterprise.to_address();
    let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
    assert_eq!(addr.to_bytes(), addr2.to_bytes());
}

#[test]
fn reward_serialize_consistency() {
    let reward = RewardAddress::new(
        9,
        &Credential::from_scripthash(&ScriptHash::from([127; Ed25519KeyHash::BYTE_COUNT])),
    );
    let addr = reward.to_address();
    let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
    assert_eq!(addr.to_bytes(), addr2.to_bytes());
}

fn root_key_12() -> Bip32PrivateKey {
    // test walk nut penalty hip pave soap entry language right filter choice
    let entropy = [
        0xdf, 0x9e, 0xd2, 0x5e, 0xd1, 0x46, 0xbf, 0x43, 0x33, 0x6a, 0x5d, 0x7c, 0xf7, 0x39, 0x59,
        0x94,
    ];
    Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
}

fn root_key_15() -> Bip32PrivateKey {
    // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
    let entropy = [
        0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4,
        0xd8, 0x09, 0x7c, 0x64, 0x12,
    ];
    Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
}

fn root_key_24() -> Bip32PrivateKey {
    let entropy = [
        0x4e, 0x82, 0x8f, 0x9a, 0x67, 0xdd, 0xcf, 0xf0, 0xe6, 0x39, 0x1a, 0xd4, 0xf2, 0x6d, 0xdb,
        0x75, 0x79, 0xf5, 0x9b, 0xa1, 0x4b, 0x6d, 0xd4, 0xba, 0xf6, 0x3d, 0xcf, 0xdb, 0x9d, 0x24,
        0x20, 0xda,
    ];
    Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
}

fn harden(index: u32) -> u32 {
    index | 0x80_00_00_00
}

#[test]
fn bech32_parsing() {
    let addr =
        Address::from_bech32("addr1u8pcjgmx7962w6hey5hhsd502araxp26kdtgagakhaqtq8sxy9w7g").unwrap();
    assert_eq!(
        addr.to_bech32(Some("foobar".to_string())).unwrap(),
        "foobar1u8pcjgmx7962w6hey5hhsd502araxp26kdtgagakhaqtq8s92n4tm"
    );
}

#[test]
fn byron_magic_parsing() {
    // mainnet address w/ protocol magic omitted
    let addr =
        ByronAddress::from_base58("Ae2tdPwUPEZ4YjgvykNpoFeYUxoyhNj2kg8KfKWN2FizsSpLUPv68MpTVDo")
            .unwrap();
    assert_eq!(
        addr.byron_protocol_magic(),
        NetworkInfo::mainnet().protocol_magic()
    );
    assert_eq!(
        addr.network_id().unwrap(),
        NetworkInfo::mainnet().network_id()
    );
}

#[test]
fn bip32_12_base() {
    let spend = root_key_12()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let stake = root_key_12()
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
    assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1qz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3jcu5d8ps7zex2k2xt3uqxgjqnnj83ws8lhrn648jjxtwq2ytjqp");
    let addr_net_3 = BaseAddress::new(
        NetworkInfo::mainnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3jcu5d8ps7zex2k2xt3uqxgjqnnj83ws8lhrn648jjxtwqfjkjv7");
}

#[test]
fn bip32_12_enterprise() {
    let spend = root_key_12()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let addr_net_0 =
        EnterpriseAddress::new(NetworkInfo::testnet_preprod().network_id(), &spend_cred).to_address();
    assert_eq!(
        addr_net_0.to_bech32(None).unwrap(),
        "addr_test1vz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzerspjrlsz"
    );
    let addr_net_3 =
        EnterpriseAddress::new(NetworkInfo::mainnet().network_id(), &spend_cred).to_address();
    assert_eq!(
        addr_net_3.to_bech32(None).unwrap(),
        "addr1vx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzers66hrl8"
    );
}

#[test]
fn bip32_12_pointer() {
    let spend = root_key_12()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();

    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let addr_net_0 = PointerAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &Pointer::new_pointer(&BigNum(1), &BigNum(2), &BigNum(3)),
    )
    .to_address();
    assert_eq!(
        addr_net_0.to_bech32(None).unwrap(),
        "addr_test1gz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzerspqgpsqe70et"
    );
    let addr_net_3 = PointerAddress::new(
        NetworkInfo::mainnet().network_id(),
        &spend_cred,
        &Pointer::new_pointer(&BigNum(24157), &BigNum(177), &BigNum(42)),
    )
    .to_address();
    assert_eq!(
        addr_net_3.to_bech32(None).unwrap(),
        "addr1gx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer5ph3wczvf2w8lunk"
    );
}

#[test]
fn bip32_15_base() {
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
    assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w");
    let addr_net_3 = BaseAddress::new(
        NetworkInfo::mainnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1q9u5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qld6xc3");
}

#[test]
fn bip32_15_enterprise() {
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let addr_net_0 =
        EnterpriseAddress::new(NetworkInfo::testnet_preprod().network_id(), &spend_cred).to_address();
    assert_eq!(
        addr_net_0.to_bech32(None).unwrap(),
        "addr_test1vpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5eg57c2qv"
    );
    let addr_net_3 =
        EnterpriseAddress::new(NetworkInfo::mainnet().network_id(), &spend_cred).to_address();
    assert_eq!(
        addr_net_3.to_bech32(None).unwrap(),
        "addr1v9u5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5eg0kvk0f"
    );
}

#[test]
fn bip32_15_pointer() {
    let spend = root_key_15()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let addr_net_0 = PointerAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &Pointer::new_pointer(&BigNum(1), &BigNum(2), &BigNum(3)),
    )
    .to_address();
    assert_eq!(
        addr_net_0.to_bech32(None).unwrap(),
        "addr_test1gpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5egpqgpsdhdyc0"
    );
    let addr_net_3 = PointerAddress::new(
        NetworkInfo::mainnet().network_id(),
        &spend_cred,
        &Pointer::new_pointer(&BigNum(24157), &BigNum(177), &BigNum(42)),
    )
    .to_address();
    assert_eq!(
        addr_net_3.to_bech32(None).unwrap(),
        "addr1g9u5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5evph3wczvf2kd5vam"
    );
}

#[test]
fn parse_redeem_address() {
    assert!(ByronAddress::is_valid(
        "Ae2tdPwUPEZ3MHKkpT5Bpj549vrRH7nBqYjNXnCV8G2Bc2YxNcGHEa8ykDp"
    ));
    let byron_addr =
        ByronAddress::from_base58("Ae2tdPwUPEZ3MHKkpT5Bpj549vrRH7nBqYjNXnCV8G2Bc2YxNcGHEa8ykDp")
            .unwrap();
    assert_eq!(
        byron_addr.to_base58(),
        "Ae2tdPwUPEZ3MHKkpT5Bpj549vrRH7nBqYjNXnCV8G2Bc2YxNcGHEa8ykDp"
    );
    let byron_addr2 = ByronAddress::from_bytes(byron_addr.to_bytes()).unwrap();
    assert_eq!(
        byron_addr2.to_base58(),
        "Ae2tdPwUPEZ3MHKkpT5Bpj549vrRH7nBqYjNXnCV8G2Bc2YxNcGHEa8ykDp"
    );
}

#[test]
fn bip32_15_byron() {
    let byron_key = root_key_15()
        .derive(harden(44))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let byron_addr =
        ByronAddress::icarus_from_key(&byron_key, NetworkInfo::mainnet().protocol_magic());
    assert_eq!(
        byron_addr.to_base58(),
        "Ae2tdPwUPEZHtBmjZBF4YpMkK9tMSPTE2ADEZTPN97saNkhG78TvXdp3GDk"
    );
    assert!(ByronAddress::is_valid(
        "Ae2tdPwUPEZHtBmjZBF4YpMkK9tMSPTE2ADEZTPN97saNkhG78TvXdp3GDk"
    ));
    assert_eq!(byron_addr.network_id().unwrap(), 0b0001);

    let byron_addr_2 =
        ByronAddress::from_address(&Address::from_bytes(byron_addr.to_bytes()).unwrap()).unwrap();
    assert_eq!(byron_addr.to_base58(), byron_addr_2.to_base58());
}

#[test]
fn bip32_24_base() {
    let spend = root_key_24()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let stake = root_key_24()
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
    assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1qqy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmn8k8ttq8f3gag0h89aepvx3xf69g0l9pf80tqv7cve0l33sw96paj");
    let addr_net_3 = BaseAddress::new(
        NetworkInfo::mainnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1qyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmn8k8ttq8f3gag0h89aepvx3xf69g0l9pf80tqv7cve0l33sdn8p3d");
}

#[test]
fn bip32_24_enterprise() {
    let spend = root_key_24()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let addr_net_0 =
        EnterpriseAddress::new(NetworkInfo::testnet_preprod().network_id(), &spend_cred).to_address();
    assert_eq!(
        addr_net_0.to_bech32(None).unwrap(),
        "addr_test1vqy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqtjtf68"
    );
    let addr_net_3 =
        EnterpriseAddress::new(NetworkInfo::mainnet().network_id(), &spend_cred).to_address();
    assert_eq!(
        addr_net_3.to_bech32(None).unwrap(),
        "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z"
    );
}

#[test]
fn bip32_24_pointer() {
    let spend = root_key_24()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let addr_net_0 = PointerAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &Pointer::new_pointer(&BigNum(1), &BigNum(2), &BigNum(3)),
    )
    .to_address();
    assert_eq!(
        addr_net_0.to_bech32(None).unwrap(),
        "addr_test1gqy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqpqgps5mee0p"
    );
    let addr_net_3 = PointerAddress::new(
        NetworkInfo::mainnet().network_id(),
        &spend_cred,
        &Pointer::new_pointer(&BigNum(24157), &BigNum(177), &BigNum(42)),
    )
    .to_address();
    assert_eq!(
        addr_net_3.to_bech32(None).unwrap(),
        "addr1gyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnyph3wczvf2dqflgt"
    );
}

#[test]
fn bip32_12_reward() {
    let staking_key = root_key_12()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();
    let staking_cred = Credential::from_keyhash(&staking_key.to_raw_key().hash());
    let addr_net_0 =
        RewardAddress::new(NetworkInfo::testnet_preprod().network_id(), &staking_cred).to_address();
    assert_eq!(
        addr_net_0.to_bech32(None).unwrap(),
        "stake_test1uqevw2xnsc0pvn9t9r9c7qryfqfeerchgrlm3ea2nefr9hqp8n5xl"
    );
    let addr_net_3 =
        RewardAddress::new(NetworkInfo::mainnet().network_id(), &staking_cred).to_address();
    assert_eq!(
        addr_net_3.to_bech32(None).unwrap(),
        "stake1uyevw2xnsc0pvn9t9r9c7qryfqfeerchgrlm3ea2nefr9hqxdekzz"
    );
}

#[test]
fn bip32_24_base_multisig_hd_derivation() {
    let spend = root_key_24()
        .derive(harden(1854))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();
    let stake = root_key_24()
        .derive(harden(1854))
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
    assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1qz8fg2e9yn0ga6sav0760cxmx0antql96mfuhqgzcc5swugw2jqqlugnx9qjep9xvcx40z0zfyep55r2t3lav5smyjrs96cusg");
    let addr_net_3 = BaseAddress::new(
        NetworkInfo::mainnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1qx8fg2e9yn0ga6sav0760cxmx0antql96mfuhqgzcc5swugw2jqqlugnx9qjep9xvcx40z0zfyep55r2t3lav5smyjrsxv9uuh");
}

#[test]
fn multisig_from_script() {
    let spend = root_key_24()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0)
        .to_public();

    let mut pubkey_native_scripts = NativeScripts::new();

    let spending_hash = spend.to_raw_key().hash();
    pubkey_native_scripts.add(&NativeScript::new_script_pubkey(&ScriptPubkey::new(
        &spending_hash,
    )));
    let oneof_native_script =
        NativeScript::new_script_n_of_k(&ScriptNOfK::new(1, &pubkey_native_scripts));

    let script_hash = ScriptHash::from_bytes(oneof_native_script.hash().to_bytes()).unwrap();

    let spend_cred = Credential::from_scripthash(&script_hash);
    let stake_cred = Credential::from_scripthash(&script_hash);
    let addr_net_0 = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1xr0de0mz3m9xmgtlmqqzu06s0uvfsczskdec8k7v4jhr7077mjlk9rk2dkshlkqq9cl4qlccnps9pvmns0duet9w8uls8flvxc");
    let addr_net_3 = BaseAddress::new(
        NetworkInfo::mainnet().network_id(),
        &spend_cred,
        &stake_cred,
    )
    .to_address();
    assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1x80de0mz3m9xmgtlmqqzu06s0uvfsczskdec8k7v4jhr7077mjlk9rk2dkshlkqq9cl4qlccnps9pvmns0duet9w8ulsylzv28");
}

#[test]
fn pointer_address_big() {
    let addr = Address::from_bech32("addr_test1grqe6lg9ay8wkcu5k5e38lne63c80h3nq6xxhqfmhewf645pllllllllllll7lupllllllllllll7lupllllllllllll7lc9wayvj").unwrap();
    let ptr = PointerAddress::from_address(&addr).unwrap().stake;
    assert_eq!(u64::MAX, u64::from(ptr.slot));
    assert_eq!(u64::MAX, u64::from(ptr.tx_index));
    assert_eq!(u64::MAX, u64::from(ptr.cert_index));
}

#[test]
fn point_address_old() {
    let p1 = Pointer::new(10, 20, 30);
    let p2 = Pointer::new_pointer(&BigNum(10), &BigNum(20), &BigNum(30));
    assert_eq!(p1, p2);
}

#[test]
fn prepod_network_id_test() {
    let address = "KjgoiXJS2coTnqpCLHXFtd89Hv9ttjsE6yW4msyLXFNkykUpTsyBs85r2rDDia2uKrhdpGKCJnmFXwvPSWLe75564ixZWdTxRh7TnuaDLnHx";
    let network_id = ByronAddress::from_base58(address)
        .unwrap()
        .to_address()
        .network_id()
        .unwrap();
    assert_eq!(network_id, NetworkInfo::testnet_preprod().network_id());
}

#[test]
fn malformed_addres_deserialisation_errors() {
    let address_bech32 = "addr1q9d66zzs27kppmx8qc8h43q7m4hkxp5d39377lvxefvxd8j7eukjsdqc5c97t2zg5guqadepqqx6rc9m7wtnxy6tajjvk4a0kze4ljyuvvrpexg5up2sqxj33363v35gtew";
    let address = Address::from_bech32(address_bech32);
    assert!(address.is_err());
}

#[test]
fn malformed_addres_embedded() {
    let address = MalformedAddress(vec![5u8; 32]);
    let output = TransactionOutput::new(
        &address.to_address(),
        &Value::new(&Coin::from(100u64)),
    );
    let bytes = output.to_bytes();
    let output2 = TransactionOutput::from_bytes(bytes).unwrap();

    assert!(output2.address.is_malformed());
    assert_eq!(&address.to_address(), &output2.address);
}