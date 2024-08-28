use crate::tests::fakes::{fake_anchor, fake_anchor_data_hash, fake_genesis_delegate_hash, fake_genesis_hash, fake_key_hash, fake_pool_metadata_hash, fake_script_hash, fake_vrf_key_hash};
use crate::*;

macro_rules! to_from_test {
    ($cert_type: ty, $variable_name: ident,  $variable_wrapped_name: ident) => {
        let json = $variable_name.to_json().unwrap();
        let cbor = $variable_name.to_bytes();
        let hex_cbor = $variable_name.to_hex();

        assert_eq!($variable_name, <$cert_type>::from_json(&json).unwrap());
        assert_eq!($variable_name, <$cert_type>::from_bytes(cbor).unwrap());
        assert_eq!($variable_name, <$cert_type>::from_hex(&hex_cbor).unwrap());

        let json_wrapped = $variable_wrapped_name.to_json().unwrap();
        let cbor_wrapped = $variable_wrapped_name.to_bytes();
        let hex_cbor_wrapped = $variable_wrapped_name.to_hex();

        assert_eq!(
            $variable_wrapped_name,
            Certificate::from_json(&json_wrapped).unwrap()
        );
        assert_eq!(
            $variable_wrapped_name,
            Certificate::from_bytes(cbor_wrapped).unwrap()
        );
        assert_eq!(
            $variable_wrapped_name,
            Certificate::from_hex(&hex_cbor_wrapped).unwrap()
        );
    };
}

#[test]
fn committee_cold_resign_key_hash_ser_round_trip() {
    let cert = CommitteeColdResign::new(&Credential::from_keyhash(&fake_key_hash(1)));
    let cert_wrapped = Certificate::new_committee_cold_resign(&cert);
    to_from_test!(CommitteeColdResign, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped.as_committee_cold_resign().unwrap()
    );
}

#[test]
fn committee_cold_resign_with_anchor_ser_round_trip() {
    let anchor = fake_anchor();
    let cert =
        CommitteeColdResign::new_with_anchor(&Credential::from_keyhash(&fake_key_hash(1)), &anchor);
    let cert_wrapped = Certificate::new_committee_cold_resign(&cert);
    to_from_test!(CommitteeColdResign, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped.as_committee_cold_resign().unwrap()
    );
}

#[test]
fn committee_cold_resign_script_hash_ser_round_trip() {
    let cert = CommitteeColdResign::new(&Credential::from_scripthash(&fake_script_hash(1)));
    let cert_wrapped = Certificate::new_committee_cold_resign(&cert);
    to_from_test!(CommitteeColdResign, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped.as_committee_cold_resign().unwrap()
    );
}

#[test]
fn committee_hot_auth_ser_round_trip() {
    let cert = CommitteeHotAuth::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Credential::from_keyhash(&fake_key_hash(2)),
    );
    let cert_wrapped = Certificate::new_committee_hot_auth(&cert);
    to_from_test!(CommitteeHotAuth, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped.as_committee_hot_auth().unwrap()
    );
}

#[test]
fn drep_registration_ser_round_trip() {
    let cert = DRepRegistration::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_drep_registration(&cert);
    to_from_test!(DRepRegistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_drep_registration().unwrap());
}

#[test]
fn drep_registration_with_anchor_ser_round_trip() {
    let url = URL::new("https://iohk.io".to_string()).unwrap();
    let anchor = Anchor::new(&url, &fake_anchor_data_hash(255));

    let cert = DRepRegistration::new_with_anchor(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
        &anchor,
    );
    let cert_wrapped = Certificate::new_drep_registration(&cert);
    to_from_test!(DRepRegistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_drep_registration().unwrap());
}

#[test]
fn drep_deregistration_ser_round_trip() {
    let cert = DRepDeregistration::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_drep_deregistration(&cert);
    to_from_test!(DRepDeregistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_drep_deregistration().unwrap());
}

#[test]
fn drep_update_ser_round_trip() {
    let cert = DRepUpdate::new(&Credential::from_keyhash(&fake_key_hash(1)));
    let cert_wrapped = Certificate::new_drep_update(&cert);
    to_from_test!(DRepUpdate, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_drep_update().unwrap());
}

#[test]
fn drep_update_with_anchor_ser_round_trip() {
    let url = URL::new("https://iohk.io".to_string()).unwrap();
    let anchor = Anchor::new(&url, &fake_anchor_data_hash(255));
    let cert = DRepUpdate::new_with_anchor(&Credential::from_keyhash(&fake_key_hash(1)), &anchor);
    let cert_wrapped = Certificate::new_drep_update(&cert);
    to_from_test!(DRepUpdate, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_drep_update().unwrap());
}

#[test]
fn genesis_key_delegation_ser_round_trip() {
    let cert = GenesisKeyDelegation::new(
        &fake_genesis_hash(1),
        &fake_genesis_delegate_hash(2),
        &fake_vrf_key_hash(3),
    );
    let cert_wrapped = Certificate::new_genesis_key_delegation(&cert);
    to_from_test!(GenesisKeyDelegation, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_genesis_key_delegation().unwrap());
}

#[test]
fn move_instantaneous_reward_to_pot_ser_round_trip() {
    let cert = MoveInstantaneousReward::new_to_other_pot(MIRPot::Reserves, &Coin::from(100u64));
    let cert_wrapped =
        Certificate::new_move_instantaneous_rewards_cert(&MoveInstantaneousRewardsCert::new(&cert));
    to_from_test!(MoveInstantaneousReward, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped
            .as_move_instantaneous_rewards_cert()
            .unwrap()
            .move_instantaneous_reward
    );
}

#[test]
fn move_instantaneous_reward_to_stake_creds_ser_round_trip() {
    let mut amounts = MIRToStakeCredentials::new();
    amounts.insert(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &DeltaCoin::new(&BigNum::from(100u64)),
    );
    let mut amounts = MIRToStakeCredentials::new();
    amounts.insert(
        &Credential::from_keyhash(&fake_key_hash(2)),
        &DeltaCoin::new(&BigNum::from(1200u64)),
    );
    let cert = MoveInstantaneousReward::new_to_stake_creds(MIRPot::Treasury, &amounts);
    let cert_wrapped =
        Certificate::new_move_instantaneous_rewards_cert(&MoveInstantaneousRewardsCert::new(&cert));
    to_from_test!(MoveInstantaneousReward, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped
            .as_move_instantaneous_rewards_cert()
            .unwrap()
            .move_instantaneous_reward
    );
}

#[test]
fn pool_registration_ser_round_trip() {
    let staking_cred = Credential::from_keyhash(&fake_key_hash(1));
    let reward_address = RewardAddress::new(NetworkInfo::testnet_preprod().network_id(), &staking_cred);
    let mut owners = Ed25519KeyHashes::new();
    owners.add(&fake_key_hash(2));
    owners.add(&fake_key_hash(3));
    let mut relays = Relays::new();
    relays.add(&Relay::new_single_host_addr(&SingleHostAddr::new(
        Some(123),
        Some(Ipv4::new([127u8, 0, 0, 1].to_vec()).unwrap()),
        Some(Ipv6::new([127u8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1].to_vec()).unwrap()),
    )));
    relays.add(&Relay::new_multi_host_name(&MultiHostName::new(
        &DNSRecordSRV::new("hi there".to_string()).unwrap(),
    )));
    relays.add(&Relay::new_single_host_name(&SingleHostName::new(
        Some(123),
        &DNSRecordAorAAAA::new("hi there".to_string()).unwrap(),
    )));
    let matadata = PoolMetadata::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_pool_metadata_hash(5),
    );

    let params = PoolParams::new(
        &fake_key_hash(1),
        &fake_vrf_key_hash(2),
        &Coin::from(100u64),
        &Coin::from(200u64),
        &UnitInterval::new(&BigNum::from(110u64), &BigNum::from(220u64)),
        &reward_address,
        &owners,
        &relays,
        Some(matadata),
    );

    let cert = PoolRegistration::new(&params);
    let cert_wrapped = Certificate::new_pool_registration(&cert);
    to_from_test!(PoolRegistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_pool_registration().unwrap());
}

#[test]
fn pool_retirement_ser_round_trip() {
    let cert = PoolRetirement::new(&fake_key_hash(1), Epoch::from(100u32));
    let cert_wrapped = Certificate::new_pool_retirement(&cert);
    to_from_test!(PoolRetirement, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_pool_retirement().unwrap());
}

#[test]
fn stake_and_vote_delegation_ser_round_trip() {
    let drep = DRep::new_key_hash(&fake_key_hash(3));

    let cert = StakeAndVoteDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &fake_key_hash(2),
        &drep,
    );
    let cert_wrapped = Certificate::new_stake_and_vote_delegation(&cert);
    to_from_test!(StakeAndVoteDelegation, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_and_vote_delegation().unwrap());
}

#[test]
fn stake_delegation_ser_round_trip() {
    let cert = StakeDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &fake_key_hash(2),
    );
    let cert_wrapped = Certificate::new_stake_delegation(&cert);
    to_from_test!(StakeDelegation, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_delegation().unwrap());
}

#[test]
fn stake_deregistration_ser_round_trip() {
    let cert = StakeDeregistration::new(&Credential::from_keyhash(&fake_key_hash(1)));
    let cert_wrapped = Certificate::new_stake_deregistration(&cert);
    to_from_test!(StakeDeregistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_deregistration().unwrap());
}

#[test]
fn stake_deregistration_with_coin_ser_round_trip() {
    let cert = StakeDeregistration::new_with_explicit_refund(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_stake_deregistration(&cert);
    to_from_test!(StakeDeregistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_deregistration().unwrap());
}

#[test]
fn stake_deregistration_getter_test() {
    let cert = StakeDeregistration::new(
        &Credential::from_keyhash(&fake_key_hash(1))
    );
    let cert_wrapped = Certificate::new_stake_deregistration(&cert);
    to_from_test!(StakeDeregistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_deregistration().unwrap());
    assert_eq!(None, cert_wrapped.as_unreg_cert());
}

#[test]
fn unreg_cert_getter_test() {
    let cert = StakeDeregistration::new_with_explicit_refund(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_unreg_cert(&cert).unwrap();
    to_from_test!(StakeDeregistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_deregistration().unwrap());
    assert_eq!(cert, cert_wrapped.as_unreg_cert().unwrap());
}

#[test]
fn unreg_cert_error_test() {
    let cert = StakeDeregistration::new(
        &Credential::from_keyhash(&fake_key_hash(1))
    );
    let res = Certificate::new_unreg_cert(&cert);
    assert!(res.is_err());
}

#[test]
fn stake_registration_ser_round_trip() {
    let cert = StakeRegistration::new(&Credential::from_keyhash(&fake_key_hash(1)));
    let cert_wrapped = Certificate::new_stake_registration(&cert);
    to_from_test!(StakeRegistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_registration().unwrap());
    assert_eq!(None, cert_wrapped.as_reg_cert())
}

#[test]
fn stake_registration_with_coin_ser_round_trip() {
    let cert = StakeRegistration::new_with_explicit_deposit(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_stake_registration(&cert);
    to_from_test!(StakeRegistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_registration().unwrap());
}

#[test]
fn reg_cert_getter_test() {
    let cert = StakeRegistration::new_with_explicit_deposit(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_reg_cert(&cert).unwrap();
    to_from_test!(StakeRegistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_registration().unwrap());
    assert_eq!(cert, cert_wrapped.as_reg_cert().unwrap());
}

#[test]
fn reg_cert_error_test() {
    let cert = StakeRegistration::new(&Credential::from_keyhash(&fake_key_hash(1)));
    let res = Certificate::new_reg_cert(&cert);
    assert!(res.is_err());
}

#[test]
fn stake_registration_and_delegation_ser_round_trip() {
    let cert = StakeRegistrationAndDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &fake_key_hash(2),
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_stake_registration_and_delegation(&cert);
    to_from_test!(StakeRegistrationAndDelegation, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped.as_stake_registration_and_delegation().unwrap()
    );
}

#[test]
fn stake_vote_registration_and_delegation_ser_round_trip() {
    let drep = DRep::new_key_hash(&fake_key_hash(3));
    let cert = StakeVoteRegistrationAndDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &fake_key_hash(2),
        &drep,
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_stake_vote_registration_and_delegation(&cert);
    to_from_test!(StakeVoteRegistrationAndDelegation, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped
            .as_stake_vote_registration_and_delegation()
            .unwrap()
    );
}

#[test]
fn vote_delegation_ser_round_trip() {
    let drep = DRep::new_key_hash(&fake_key_hash(3));
    let cert = VoteDelegation::new(&Credential::from_keyhash(&fake_key_hash(1)), &drep);
    let cert_wrapped = Certificate::new_vote_delegation(&cert);
    to_from_test!(VoteDelegation, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_vote_delegation().unwrap());
}

#[test]
fn vote_registration_and_delegation_ser_round_trip() {
    let drep = DRep::new_key_hash(&fake_key_hash(3));
    let cert = VoteRegistrationAndDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &drep,
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_vote_registration_and_delegation(&cert);
    to_from_test!(VoteRegistrationAndDelegation, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped.as_vote_registration_and_delegation().unwrap()
    );
}

#[test]
fn tx_with_drep_reg_deser_test() {
    let cbor = "84a4008182582038e88b8b95dc13639c2c0adc6a159316bd795da6672d4025f5f2bc50f122438f010181a20058390013ca2480e9651a5c504b36eda271ec171cdd404cfe349097524a48bd8bee57ce33c7c1f711bc5801986d89dd68078f5922b83812cc86f65f011b0000000253f7736e021a00029d59048184108200581c1033bbc7db733c057fed63fa085113dfb570566eb708d548d2f7cce800f6a1008182582072fe72c3f2506a2b88cf9c6388535d98f90d481aa734e0e3553792cb9984ffcc5840509a64b3e450f8b338ba3f759e8cf91273493d425a027a7373071c166de6ab83ed3af6b98415c6372906aeaba9269ecf1c40dccbebf8050b4e9ad5e2a5346503f5f6";
    let tx_deser = Transaction::from_hex(cbor);
    assert!(tx_deser.is_ok());
    let cert = tx_deser.unwrap().body().certs().unwrap().get(0);
    assert!(cert.as_drep_registration().is_some());
}

#[test]
fn tx_with_drep_reg_deleg_test() {
    let cbor = "84a400818258201e3f301eee4c02377c137eff0260a33b67ea421e3524ce8818e4c5184fa440d2000181a2005839002d745f050a8f7e263f4d0749a82284ed9cc065018c1f4f6a7c1b764882293a49e3ef29a4f9c32e4c18f202f5324182db7790f48dccf7a6dd011b0000000253e3e5ad021a0002a281048183098200581c82293a49e3ef29a4f9c32e4c18f202f5324182db7790f48dccf7a6dd8200581c1033bbc7db733c057fed63fa085113dfb570566eb708d548d2f7cce8a0f5f6";
    let tx_deser = Transaction::from_hex(cbor);
    assert!(tx_deser.is_ok());
    let cert = tx_deser.unwrap().body().certs().unwrap().get(0);
    assert!(cert.as_vote_delegation().is_some());
}

#[test]
fn block_with_tx_with_certs_under_tag_set() {
    let cbor = "85828a1a00093ff71a00b90a7e582030cf3798ec016ed63988b2e413fdadf4bda64e5b78587b74dec3e8807b3fd28058204e6f414dc8f402a977ef31062cae0e1251db537980f84c0e8623696975083fc15820f2768101e877acd0e08764765552a36c0f1a25e86d461c277bc19d0e476253fd825840622a847f3c0e77b608aa2aa1cff5c389348e80db4aa0d63b42d89753d5630cb0fcabbfd7293ee65a6b795c52cb4bd6b338f9d11fd007406fcbe89d06bb34f1145850d2c8f8179a8009b0979762ac10c6a514d8d0bc6b6d0d4413a0edbd5dbe888a8e72ba874d0f692ec940e58513c5b8ccb5072839ea1fa00776b4dcb493be8131b1a0b21bf5b5be5ff264e209fef448a30419016c5820388135462552cc6045399bd62876b28b768a760dd9a64adedbf7499bdb7cd1be8458202495446005fecca3a7ede95738dad5fd87393e81c815bde94a405be5779368c30218425840979096c8f12db5dc84a483626c966b64c10e16453b14f33b9648c250b096e391f6e9e6773017134a39c080d77f132950f43522015e9fa265695ee939625f89078209005901c0681c0f99e6f0d09b66b3e8e6eaed6a92649b635225f7d374a92af1a7ba2771880d14719c229892943bb85ba51699ae50bf7ae174e2e9869af7e289355aab000e741588d3b8c82efa6063c83fe326b72eb93f93bea07c5b5b9a720e9f9ecc20e47598b3ce56f370b268a2e2e075f24942e547d29182cecefdb9e7e45108e2261dd3d006ab778cba4cf0bced84f41fc61afcd79591d988eb401e2f870122ae590aec3467f464cebca50c5434d2491f631ebb3d835f43682244bcc839bd83e1c48950bcc73cfe5feaaa211d964bd9bdb4f9acd23fde11f469f6e0fb8bcc9aa4130a54ccab7381968e67ad1291bcdb8528228bbbb9fe15f72cf125b4de1cfdf3dd2b0d9189347a6964f17ce5063b75df8dd20f0fceeefb0d2f5781d34a03f14361ad4b9acb6c40c33ae366906f69dd422e5f2e00afd6bbecba078aebc53a69c567a864548da0205ed92937b4efbb12ab49273e598e3ec5f55abfcaae36c856024c6de779e8f2e28d997b94e116a7b6438cc04fb25b1dbb494b32e1eb97139e6d62e6a70fb2b480dc356225977a6b6b0a5bb9822d2dd5e0012c5de011219b8adc70af87304ab2c98c457078e0b859f50f69bd5eaaaf44e62d34776c8a6bcffd3c1bae81a40081825820b45589419cdc8218dabf8805fd09dded29c795939588fd2f1c397fcd29207307000181825839002844d663085837e4620b9114498f2a4577f2942d84573f95255cdea32134ccb579707f289dc83bc0fb386199a9cea4f1b0177d8384adbb1d1b0000000253eac253021a00029d2d04d901028182008200581c2134ccb579707f289dc83bc0fb386199a9cea4f1b0177d8384adbb1d81a100828258206f92479dc8d89cae74346be2e31997f8c04c977fabb3c806fd1740e7af20874b5840c3d442ba9e0c0915917eb64045603e6b90e0c2cf11c796b7c327f79a4c6e971149bcbbeae79339db8b557345c08de1f103e11c2032348825de9bc5e44150d1018258207be0e76fe15de98ecc3f02e95d4ec171ef884a6ca22b7623c75f66a07f16f3f458409644a34175257eec09a2b0ab52cc36b5bcfeea590d1ae7ead57604ce30be1ad79ecea7c07eadb7973c0c3fd99d63303b47f156fb767a4aa3180c4ed436233f05a080";
    let block = Block::from_hex(cbor);
    assert!(block.is_ok());

    let certs = block.unwrap().transaction_bodies.0[0].certs().unwrap();
    assert_eq!(certs.len(), 1);
}

#[test]
fn certificates_collection_ser_round_trip() {
    let mut certs = Certificates::new();
    let cert_1 = StakeRegistration::new(&Credential::from_keyhash(&fake_key_hash(1)));
    certs.add(&Certificate::new_stake_registration(&cert_1));
    let cert_2 = StakeDeregistration::new(&Credential::from_keyhash(&fake_key_hash(2)));
    certs.add(&Certificate::new_stake_deregistration(&cert_2));
    let cert_3 = StakeDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(3)),
        &fake_key_hash(4),
    );
    certs.add(&Certificate::new_stake_delegation(&cert_3));

    assert_eq!(certs.len(), 3);

    let json = certs.to_json().unwrap();
    let cbor = certs.to_bytes();
    let hex_cbor = certs.to_hex();

    assert_eq!(certs, Certificates::from_json(&json).unwrap());
    assert_eq!(certs, Certificates::from_bytes(cbor).unwrap());
    assert_eq!(certs, Certificates::from_hex(&hex_cbor).unwrap());
}
