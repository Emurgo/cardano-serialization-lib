use crate::fakes::{
    fake_genesis_delegate_hash, fake_genesis_hash, fake_key_hash, fake_pool_metadata_hash,
    fake_vrf_key_hash,
};
use crate::*;

#[test]
fn certificatess_builder_deposit_test() {
    let mut builder = CertificatesBuilder::new();
    let pool_deposit = 100u64;
    let key_deposit = 200u64;
    let key_deposit_form_args = 201u64;
    let drep_reg_deposit = 400u64;

    let committee_hot_key_dereg_cert =
        CommitteeColdResign::new(&Credential::from_keyhash(&fake_key_hash(1)));
    let committee_hot_key_dereg_cert_wrapped =
        Certificate::new_committee_cold_resign(&committee_hot_key_dereg_cert);

    let committee_hot_key_reg_cert = CommitteeHotAuth::new(
        &Credential::from_keyhash(&fake_key_hash(2)),
        &Credential::from_keyhash(&fake_key_hash(3)),
    );
    let committee_hot_key_reg_cert_wrapped =
        Certificate::new_committee_hot_auth(&committee_hot_key_reg_cert);

    let drep_reg_cert = DrepRegistration::new(
        &Credential::from_keyhash(&fake_key_hash(4)),
        &Coin::from(drep_reg_deposit),
    );
    let drep_reg_cert_wrapped = Certificate::new_drep_registration(&drep_reg_cert);

    let drep_dereg_cert = DrepDeregistration::new(
        &Credential::from_keyhash(&fake_key_hash(5)),
        &Coin::from(drep_reg_deposit),
    );
    let drep_dereg_cert_wrapped = Certificate::new_drep_deregistration(&drep_dereg_cert);

    let drep_update_cert = DrepUpdate::new(&Credential::from_keyhash(&fake_key_hash(6)));
    let cdrep_update_cert_wrapped = Certificate::new_drep_update(&drep_update_cert);

    let genesis_key_deleg_cert = GenesisKeyDelegation::new(
        &fake_genesis_hash(7),
        &fake_genesis_delegate_hash(8),
        &fake_vrf_key_hash(9),
    );
    let genesis_key_deleg_cert_wrapped =
        Certificate::new_genesis_key_delegation(&genesis_key_deleg_cert);

    let mir_cert = MoveInstantaneousReward::new_to_other_pot(MIRPot::Reserves, &Coin::from(100u64));
    let mir_cert_wrapped = Certificate::new_move_instantaneous_rewards_cert(
        &MoveInstantaneousRewardsCert::new(&mir_cert),
    );

    let staking_cred = Credential::from_keyhash(&fake_key_hash(10));
    let reward_address = RewardAddress::new(NetworkInfo::testnet().network_id(), &staking_cred);
    let mut owners = Ed25519KeyHashes::new();
    owners.add(&fake_key_hash(11));
    owners.add(&fake_key_hash(12));
    let relays = Relays::new();
    let matadata = PoolMetadata::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_pool_metadata_hash(5),
    );

    let params = PoolParams::new(
        &fake_key_hash(13),
        &fake_vrf_key_hash(15),
        &Coin::from(100u64),
        &Coin::from(200u64),
        &UnitInterval::new(&BigNum::from(110u64), &BigNum::from(220u64)),
        &reward_address,
        &owners,
        &relays,
        Some(matadata),
    );

    let pool_reg_cert = PoolRegistration::new(&params);
    let pool_reg_cert_wrapped = Certificate::new_pool_registration(&pool_reg_cert);

    let pool_ret_cert = PoolRetirement::new(&fake_key_hash(16), Epoch::from(100u32));
    let pool_ret_cert_wrapped = Certificate::new_pool_retirement(&pool_ret_cert);

    let drep = DRep::new_key_hash(&fake_key_hash(17));
    let stake_vote_deleg_cert = StakeAndVoteDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(18)),
        &fake_key_hash(19),
        &drep,
    );
    let stake_vote_deleg_cert_wrapped =
        Certificate::new_stake_and_vote_delegation(&stake_vote_deleg_cert);

    let stake_deleg_cert = StakeDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(20)),
        &fake_key_hash(21),
    );
    let stake_deleg_cert_wrapped = Certificate::new_stake_delegation(&stake_deleg_cert);

    let stake_dereg_cert = StakeDeregistration::new(&Credential::from_keyhash(&fake_key_hash(22)));
    let stake_dereg_cert_wrapped = Certificate::new_stake_deregistration(&stake_dereg_cert);

    let stake_dereg_with_coin_cert =
        StakeDeregistration::new_with_coin(
            &Credential::from_keyhash(&fake_key_hash(22)),
            &Coin::from(key_deposit_form_args),
        );
    let stake_dereg_with_coint_wrapped = Certificate::new_stake_deregistration(&stake_dereg_with_coin_cert);

    let stake_reg_cert = StakeRegistration::new(&Credential::from_keyhash(&fake_key_hash(23)));
    let stake_reg_cert_wrapped = Certificate::new_stake_registration(&stake_reg_cert);

    let stake_reg_with_coin_cert =
        StakeRegistration::new_with_coin(
            &Credential::from_keyhash(&fake_key_hash(23)),
            &Coin::from(key_deposit_form_args),
        );
    let stake_reg_with_coint_wrapped = Certificate::new_stake_registration(&stake_reg_with_coin_cert);

    let stake_reg_deleg_cert = StakeRegistrationAndDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(23)),
        &fake_key_hash(24),
        &Coin::from(key_deposit_form_args),
    );
    let stake_reg_deleg_cert_wrapped = Certificate::new_stake_registration_and_delegation(&stake_reg_deleg_cert);

    let drep = DRep::new_key_hash(&fake_key_hash(25));
    let stake_vote_reg_deleg_cert = StakeVoteRegistrationAndDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(26)),
        &fake_key_hash(27),
        &drep,
        &Coin::from(key_deposit_form_args),
    );
    let stake_vote_reg_deleg_cert_wrapped = Certificate::new_stake_vote_registration_and_delegation(&stake_vote_reg_deleg_cert);

    let drep = DRep::new_key_hash(&fake_key_hash(28));
    let vote_deleg_cert = VoteDelegation::new(&Credential::from_keyhash(&fake_key_hash(29)), &drep);
    let vote_deleg_cert_wrapped = Certificate::new_vote_delegation(&vote_deleg_cert);

    let drep = DRep::new_key_hash(&fake_key_hash(30));
    let vote_reg_deleg_cert = VoteRegistrationAndDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(31)),
        &drep,
        &Coin::from(key_deposit_form_args),
    );
    let vote_reg_deleg_cert_wrapped = Certificate::new_vote_registration_and_delegation(&vote_reg_deleg_cert);

    builder.add(&committee_hot_key_dereg_cert_wrapped).unwrap();
    builder.add(&committee_hot_key_reg_cert_wrapped).unwrap();
    builder.add(&drep_reg_cert_wrapped).unwrap();
    builder.add(&drep_dereg_cert_wrapped).unwrap();
    builder.add(&cdrep_update_cert_wrapped).unwrap();
    builder.add(&genesis_key_deleg_cert_wrapped).unwrap();
    builder.add(&mir_cert_wrapped).unwrap();
    builder.add(&pool_reg_cert_wrapped).unwrap();
    builder.add(&pool_ret_cert_wrapped).unwrap();
    builder.add(&stake_vote_deleg_cert_wrapped).unwrap();
    builder.add(&stake_deleg_cert_wrapped).unwrap();
    builder.add(&stake_dereg_cert_wrapped).unwrap();
    builder.add(&stake_dereg_with_coint_wrapped).unwrap();
    builder.add(&stake_reg_cert_wrapped).unwrap();
    builder.add(&stake_reg_with_coint_wrapped).unwrap();
    builder.add(&stake_reg_deleg_cert_wrapped).unwrap();
    builder.add(&stake_vote_reg_deleg_cert_wrapped).unwrap();
    builder.add(&vote_deleg_cert_wrapped).unwrap();
    builder.add(&vote_reg_deleg_cert_wrapped).unwrap();

    let builder_deposit = builder.get_certificates_deposit(
        &Coin::from(pool_deposit),
        &Coin::from(key_deposit),
    ).unwrap();

    let expected_deposit = Coin::from(
        pool_deposit
            + key_deposit
            + drep_reg_deposit
            + (key_deposit_form_args * 4));

    assert_eq!(builder_deposit, expected_deposit);
}

#[test]
fn certificatess_builder_no_deposit_test() {
    let mut builder = CertificatesBuilder::new();
    let pool_deposit = 100u64;
    let key_deposit = 200u64;
    let key_deposit_form_args = 201u64;
    let drep_reg_deposit = 400u64;

    let committee_hot_key_dereg_cert =
        CommitteeColdResign::new(&Credential::from_keyhash(&fake_key_hash(1)));
    let committee_hot_key_dereg_cert_wrapped =
        Certificate::new_committee_cold_resign(&committee_hot_key_dereg_cert);

    let committee_hot_key_reg_cert = CommitteeHotAuth::new(
        &Credential::from_keyhash(&fake_key_hash(2)),
        &Credential::from_keyhash(&fake_key_hash(3)),
    );
    let committee_hot_key_reg_cert_wrapped =
        Certificate::new_committee_hot_auth(&committee_hot_key_reg_cert);

    let drep_dereg_cert = DrepDeregistration::new(
        &Credential::from_keyhash(&fake_key_hash(5)),
        &Coin::from(drep_reg_deposit),
    );
    let drep_dereg_cert_wrapped = Certificate::new_drep_deregistration(&drep_dereg_cert);

    let drep_update_cert = DrepUpdate::new(&Credential::from_keyhash(&fake_key_hash(6)));
    let cdrep_update_cert_wrapped = Certificate::new_drep_update(&drep_update_cert);

    let genesis_key_deleg_cert = GenesisKeyDelegation::new(
        &fake_genesis_hash(7),
        &fake_genesis_delegate_hash(8),
        &fake_vrf_key_hash(9),
    );
    let genesis_key_deleg_cert_wrapped =
        Certificate::new_genesis_key_delegation(&genesis_key_deleg_cert);

    let mir_cert = MoveInstantaneousReward::new_to_other_pot(MIRPot::Reserves, &Coin::from(100u64));
    let mir_cert_wrapped = Certificate::new_move_instantaneous_rewards_cert(
        &MoveInstantaneousRewardsCert::new(&mir_cert),
    );

    let staking_cred = Credential::from_keyhash(&fake_key_hash(10));
    let reward_address = RewardAddress::new(NetworkInfo::testnet().network_id(), &staking_cred);
    let mut owners = Ed25519KeyHashes::new();
    owners.add(&fake_key_hash(11));
    owners.add(&fake_key_hash(12));
    let relays = Relays::new();
    let matadata = PoolMetadata::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_pool_metadata_hash(5),
    );

    let params = PoolParams::new(
        &fake_key_hash(13),
        &fake_vrf_key_hash(15),
        &Coin::from(100u64),
        &Coin::from(200u64),
        &UnitInterval::new(&BigNum::from(110u64), &BigNum::from(220u64)),
        &reward_address,
        &owners,
        &relays,
        Some(matadata),
    );

    let pool_ret_cert = PoolRetirement::new(&fake_key_hash(16), Epoch::from(100u32));
    let pool_ret_cert_wrapped = Certificate::new_pool_retirement(&pool_ret_cert);

    let drep = DRep::new_key_hash(&fake_key_hash(17));
    let stake_vote_deleg_cert = StakeAndVoteDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(18)),
        &fake_key_hash(19),
        &drep,
    );
    let stake_vote_deleg_cert_wrapped =
        Certificate::new_stake_and_vote_delegation(&stake_vote_deleg_cert);

    let stake_deleg_cert = StakeDelegation::new(
        &Credential::from_keyhash(&fake_key_hash(20)),
        &fake_key_hash(21),
    );
    let stake_deleg_cert_wrapped = Certificate::new_stake_delegation(&stake_deleg_cert);

    let stake_dereg_cert = StakeDeregistration::new(&Credential::from_keyhash(&fake_key_hash(22)));
    let stake_dereg_cert_wrapped = Certificate::new_stake_deregistration(&stake_dereg_cert);

    let stake_dereg_with_coin_cert =
        StakeDeregistration::new_with_coin(
            &Credential::from_keyhash(&fake_key_hash(22)),
            &Coin::from(key_deposit_form_args),
        );
    let stake_dereg_with_coint_wrapped = Certificate::new_stake_deregistration(&stake_dereg_with_coin_cert);

    let drep = DRep::new_key_hash(&fake_key_hash(28));
    let vote_deleg_cert = VoteDelegation::new(&Credential::from_keyhash(&fake_key_hash(29)), &drep);
    let vote_deleg_cert_wrapped = Certificate::new_vote_delegation(&vote_deleg_cert);

    builder.add(&committee_hot_key_dereg_cert_wrapped).unwrap();
    builder.add(&committee_hot_key_reg_cert_wrapped).unwrap();
    builder.add(&drep_dereg_cert_wrapped).unwrap();
    builder.add(&cdrep_update_cert_wrapped).unwrap();
    builder.add(&genesis_key_deleg_cert_wrapped).unwrap();
    builder.add(&mir_cert_wrapped).unwrap();
    builder.add(&pool_ret_cert_wrapped).unwrap();
    builder.add(&stake_vote_deleg_cert_wrapped).unwrap();
    builder.add(&stake_deleg_cert_wrapped).unwrap();
    builder.add(&stake_dereg_cert_wrapped).unwrap();
    builder.add(&stake_dereg_with_coint_wrapped).unwrap();
    builder.add(&vote_deleg_cert_wrapped).unwrap();

    let builder_deposit = builder.get_certificates_deposit(
        &Coin::from(pool_deposit),
        &Coin::from(key_deposit),
    ).unwrap();

    let expected_deposit = Coin::zero();

    assert_eq!(builder_deposit, expected_deposit);
}

#[test]
fn certificatess_builder_req_signers_test() {
    let mut builder = CertificatesBuilder::new();
    let pool_deposit = 100u64;
    let key_deposit = 200u64;
    let key_deposit_form_args = 201u64;
    let drep_reg_deposit = 400u64;

    let key_hash_1 = fake_key_hash(1);
    let key_hash_2 = fake_key_hash(2);
    let key_hash_3 = fake_key_hash(3);
    let key_hash_4 = fake_key_hash(4);
    let key_hash_5 = fake_key_hash(5);
    let key_hash_6 = fake_key_hash(6);
    let key_hash_8 = fake_key_hash(8);
    let key_hash_10 = fake_key_hash(10);
    let key_hash_11 = fake_key_hash(11);
    let key_hash_12 = fake_key_hash(12);
    let key_hash_13 = fake_key_hash(13);
    let key_hash_15 = fake_key_hash(15);
    let key_hash_16 = fake_key_hash(16);
    let key_hash_17 = fake_key_hash(17);
    let key_hash_18 = fake_key_hash(18);
    let key_hash_19 = fake_key_hash(19);
    let key_hash_20 = fake_key_hash(20);
    let key_hash_21 = fake_key_hash(21);
    let key_hash_22 = fake_key_hash(22);
    let key_hash_23 = fake_key_hash(23);
    let key_hash_24 = fake_key_hash(24);
    let key_hash_25 = fake_key_hash(25);
    let key_hash_26 = fake_key_hash(26);
    let key_hash_27 = fake_key_hash(27);
    let key_hash_28 = fake_key_hash(28);
    let key_hash_29 = fake_key_hash(29);
    let key_hash_30 = fake_key_hash(30);
    let key_hash_31 = fake_key_hash(31);
    let key_hash_32 = fake_key_hash(32);
    let key_hash_33 = fake_key_hash(33);

    let committee_hot_key_dereg_cert =
        CommitteeColdResign::new(&Credential::from_keyhash(&key_hash_1));
    let committee_hot_key_dereg_cert_wrapped =
        Certificate::new_committee_cold_resign(&committee_hot_key_dereg_cert);

    let committee_hot_key_reg_cert = CommitteeHotAuth::new(
        &Credential::from_keyhash(&key_hash_2),
        &Credential::from_keyhash(&key_hash_3),
    );
    let committee_hot_key_reg_cert_wrapped =
        Certificate::new_committee_hot_auth(&committee_hot_key_reg_cert);

    let drep_reg_cert = DrepRegistration::new(
        &Credential::from_keyhash(&key_hash_4),
        &Coin::from(drep_reg_deposit),
    );
    let drep_reg_cert_wrapped = Certificate::new_drep_registration(&drep_reg_cert);

    let drep_dereg_cert = DrepDeregistration::new(
        &Credential::from_keyhash(&key_hash_5),
        &Coin::from(drep_reg_deposit),
    );
    let drep_dereg_cert_wrapped = Certificate::new_drep_deregistration(&drep_dereg_cert);

    let drep_update_cert = DrepUpdate::new(&Credential::from_keyhash(&key_hash_6));
    let cdrep_update_cert_wrapped = Certificate::new_drep_update(&drep_update_cert);

    let genesis_key_deleg_cert = GenesisKeyDelegation::new(
        &fake_genesis_hash(7),
        &fake_genesis_delegate_hash(8),
        &fake_vrf_key_hash(9),
    );
    let genesis_key_deleg_cert_wrapped =
        Certificate::new_genesis_key_delegation(&genesis_key_deleg_cert);

    let mir_cert = MoveInstantaneousReward::new_to_other_pot(MIRPot::Reserves, &Coin::from(100u64));
    let mir_cert_wrapped = Certificate::new_move_instantaneous_rewards_cert(
        &MoveInstantaneousRewardsCert::new(&mir_cert),
    );

    let staking_cred = Credential::from_keyhash(&key_hash_10);
    let reward_address = RewardAddress::new(NetworkInfo::testnet().network_id(), &staking_cred);
    let mut owners = Ed25519KeyHashes::new();
    owners.add(&key_hash_11);
    owners.add(&key_hash_12);
    let relays = Relays::new();
    let matadata = PoolMetadata::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_pool_metadata_hash(5),
    );

    let params = PoolParams::new(
        &key_hash_13,
        &fake_vrf_key_hash(14),
        &Coin::from(100u64),
        &Coin::from(200u64),
        &UnitInterval::new(&BigNum::from(110u64), &BigNum::from(220u64)),
        &reward_address,
        &owners,
        &relays,
        Some(matadata),
    );

    let pool_reg_cert = PoolRegistration::new(&params);
    let pool_reg_cert_wrapped = Certificate::new_pool_registration(&pool_reg_cert);

    let pool_ret_cert = PoolRetirement::new(&key_hash_15, Epoch::from(100u32));
    let pool_ret_cert_wrapped = Certificate::new_pool_retirement(&pool_ret_cert);

    let drep = DRep::new_key_hash(&key_hash_16);
    let stake_vote_deleg_cert = StakeAndVoteDelegation::new(
        &Credential::from_keyhash(&key_hash_17),
        &key_hash_18,
        &drep,
    );
    let stake_vote_deleg_cert_wrapped =
        Certificate::new_stake_and_vote_delegation(&stake_vote_deleg_cert);

    let stake_deleg_cert = StakeDelegation::new(
        &Credential::from_keyhash(&key_hash_19),
        &key_hash_20,
    );
    let stake_deleg_cert_wrapped = Certificate::new_stake_delegation(&stake_deleg_cert);

    let stake_dereg_cert = StakeDeregistration::new(&Credential::from_keyhash(&key_hash_21));
    let stake_dereg_cert_wrapped = Certificate::new_stake_deregistration(&stake_dereg_cert);

    let stake_dereg_with_coin_cert =
        StakeDeregistration::new_with_coin(
            &Credential::from_keyhash(&key_hash_22),
            &Coin::from(key_deposit_form_args),
        );
    let stake_dereg_with_coint_wrapped = Certificate::new_stake_deregistration(&stake_dereg_with_coin_cert);

    let stake_reg_cert = StakeRegistration::new(&Credential::from_keyhash(&key_hash_23));
    let stake_reg_cert_wrapped = Certificate::new_stake_registration(&stake_reg_cert);

    let stake_reg_with_coin_cert =
        StakeRegistration::new_with_coin(
            &Credential::from_keyhash(&key_hash_24),
            &Coin::from(key_deposit_form_args),
        );
    let stake_reg_with_coin_wrapped = Certificate::new_stake_registration(&stake_reg_with_coin_cert);

    let stake_reg_deleg_cert = StakeRegistrationAndDelegation::new(
        &Credential::from_keyhash(&key_hash_25),
        &key_hash_26,
        &Coin::from(key_deposit_form_args),
    );
    let stake_reg_deleg_cert_wrapped = Certificate::new_stake_registration_and_delegation(&stake_reg_deleg_cert);

    let drep = DRep::new_key_hash(&key_hash_27);
    let stake_vote_reg_deleg_cert = StakeVoteRegistrationAndDelegation::new(
        &Credential::from_keyhash(&key_hash_28),
        &key_hash_29,
        &drep,
        &Coin::from(key_deposit_form_args),
    );
    let stake_vote_reg_deleg_cert_wrapped = Certificate::new_stake_vote_registration_and_delegation(&stake_vote_reg_deleg_cert);

    let drep = DRep::new_key_hash(&key_hash_30);
    let vote_deleg_cert = VoteDelegation::new(&Credential::from_keyhash(&key_hash_31), &drep);
    let vote_deleg_cert_wrapped = Certificate::new_vote_delegation(&vote_deleg_cert);

    let drep = DRep::new_key_hash(&key_hash_32);
    let vote_reg_deleg_cert = VoteRegistrationAndDelegation::new(
        &Credential::from_keyhash(&key_hash_33),
        &drep,
        &Coin::from(key_deposit_form_args),
    );
    let vote_reg_deleg_cert_wrapped = Certificate::new_vote_registration_and_delegation(&vote_reg_deleg_cert);

    builder.add(&committee_hot_key_dereg_cert_wrapped).unwrap();
    builder.add(&committee_hot_key_reg_cert_wrapped).unwrap();
    builder.add(&drep_reg_cert_wrapped).unwrap();
    builder.add(&drep_dereg_cert_wrapped).unwrap();
    builder.add(&cdrep_update_cert_wrapped).unwrap();
    builder.add(&genesis_key_deleg_cert_wrapped).unwrap();
    builder.add(&mir_cert_wrapped).unwrap();
    builder.add(&pool_reg_cert_wrapped).unwrap();
    builder.add(&pool_ret_cert_wrapped).unwrap();
    builder.add(&stake_vote_deleg_cert_wrapped).unwrap();
    builder.add(&stake_deleg_cert_wrapped).unwrap();
    builder.add(&stake_dereg_cert_wrapped).unwrap();
    builder.add(&stake_dereg_with_coint_wrapped).unwrap();
    builder.add(&stake_reg_cert_wrapped).unwrap();
    builder.add(&stake_reg_with_coin_wrapped).unwrap();
    builder.add(&stake_reg_deleg_cert_wrapped).unwrap();
    builder.add(&stake_vote_reg_deleg_cert_wrapped).unwrap();
    builder.add(&vote_deleg_cert_wrapped).unwrap();
    builder.add(&vote_reg_deleg_cert_wrapped).unwrap();

    let builder_deposit = builder.get_certificates_deposit(
        &Coin::from(pool_deposit),
        &Coin::from(key_deposit),
    ).unwrap();

    let req_signers = builder.get_required_signers();

    assert_eq!(req_signers.len(), 18);
    assert!(req_signers.contains(&key_hash_1));
    assert!(req_signers.contains(&key_hash_2));
    assert!(req_signers.contains(&key_hash_5));
    assert!(req_signers.contains(&key_hash_6));
    assert!(req_signers.contains(&key_hash_8));
    assert!(req_signers.contains(&key_hash_11));
    assert!(req_signers.contains(&key_hash_12));
    assert!(req_signers.contains(&key_hash_13));
    assert!(req_signers.contains(&key_hash_15));
    assert!(req_signers.contains(&key_hash_17));
    assert!(req_signers.contains(&key_hash_19));
    assert!(req_signers.contains(&key_hash_21));
    assert!(req_signers.contains(&key_hash_22));
    assert!(req_signers.contains(&key_hash_24));
    assert!(req_signers.contains(&key_hash_25));
    assert!(req_signers.contains(&key_hash_28));
    assert!(req_signers.contains(&key_hash_31));
    assert!(req_signers.contains(&key_hash_33));
}