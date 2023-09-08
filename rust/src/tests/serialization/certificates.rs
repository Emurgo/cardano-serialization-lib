use crate::fakes::{
    fake_anchor_data_hash, fake_genesis_delegate_hash, fake_genesis_hash, fake_key_hash,
    fake_pool_metadata_hash, fake_script_hash, fake_vrf_key_hash,
};
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
fn committee_hot_key_deregistration_key_hash_ser_round_trip() {
    let cert = CommitteeHotKeyDeregistration::new(&Credential::from_keyhash(&fake_key_hash(1)));
    let cert_wrapped = Certificate::new_committee_hot_key_deregistration(&cert);
    to_from_test!(CommitteeHotKeyDeregistration, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped.as_committee_hot_key_deregistration().unwrap()
    );
}

#[test]
fn committee_hot_key_deregistration_script_hash_ser_round_trip() {
    let cert =
        CommitteeHotKeyDeregistration::new(&Credential::from_scripthash(&fake_script_hash(1)));
    let cert_wrapped = Certificate::new_committee_hot_key_deregistration(&cert);
    to_from_test!(CommitteeHotKeyDeregistration, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped.as_committee_hot_key_deregistration().unwrap()
    );
}

#[test]
fn committee_hot_key_registration_ser_round_trip() {
    let cert = CommitteeHotKeyRegistration::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Credential::from_keyhash(&fake_key_hash(2)),
    );
    let cert_wrapped = Certificate::new_committee_hot_key_registration(&cert);
    to_from_test!(CommitteeHotKeyRegistration, cert, cert_wrapped);
    assert_eq!(
        cert,
        cert_wrapped.as_committee_hot_key_registration().unwrap()
    );
}

#[test]
fn drep_registration_ser_round_trip() {
    let cert = DrepRegistration::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_drep_registration(&cert);
    to_from_test!(DrepRegistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_drep_registration().unwrap());
}

#[test]
fn drep_registration_with_anchor_ser_round_trip() {
    let url = URL::new("https://iohk.io".to_string()).unwrap();
    let anchor = Anchor::new(&url, &fake_anchor_data_hash(255));

    let cert = DrepRegistration::new_with_anchor(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
        &anchor,
    );
    let cert_wrapped = Certificate::new_drep_registration(&cert);
    to_from_test!(DrepRegistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_drep_registration().unwrap());
}

#[test]
fn drep_deregistration_ser_round_trip() {
    let cert = DrepDeregistration::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_drep_deregistration(&cert);
    to_from_test!(DrepDeregistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_drep_deregistration().unwrap());
}

#[test]
fn drep_update_ser_round_trip() {
    let cert = DrepUpdate::new(&Credential::from_keyhash(&fake_key_hash(1)));
    let cert_wrapped = Certificate::new_drep_update(&cert);
    to_from_test!(DrepUpdate, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_drep_update().unwrap());
}

#[test]
fn drep_update_with_anchor_ser_round_trip() {
    let url = URL::new("https://iohk.io".to_string()).unwrap();
    let anchor = Anchor::new(&url, &fake_anchor_data_hash(255));
    let cert = DrepUpdate::new_with_anchor(&Credential::from_keyhash(&fake_key_hash(1)), &anchor);
    let cert_wrapped = Certificate::new_drep_update(&cert);
    to_from_test!(DrepUpdate, cert, cert_wrapped);
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
    let reward_address = RewardAddress::new(NetworkInfo::testnet().network_id(), &staking_cred);
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
    let cert = StakeDeregistration::new_with_coin(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_stake_deregistration(&cert);
    to_from_test!(StakeDeregistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_deregistration().unwrap());
}

#[test]
fn stake_registration_ser_round_trip() {
    let cert = StakeRegistration::new(&Credential::from_keyhash(&fake_key_hash(1)));
    let cert_wrapped = Certificate::new_stake_registration(&cert);
    to_from_test!(StakeRegistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_registration().unwrap());
}

#[test]
fn stake_registration_with_coin_ser_round_trip() {
    let cert = StakeRegistration::new_with_coin(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &Coin::from(100u64),
    );
    let cert_wrapped = Certificate::new_stake_registration(&cert);
    to_from_test!(StakeRegistration, cert, cert_wrapped);
    assert_eq!(cert, cert_wrapped.as_stake_registration().unwrap());
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
