use crate::tests::fakes::{fake_full_pool_params, fake_anchor, fake_genesis_delegate_hash, fake_genesis_hash, fake_key_hash, fake_script_hash, fake_vrf_key_hash};
use crate::*;

#[test]
fn committee_cold_resign_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let cred_script_hash = Credential::from_scripthash(&fake_script_hash(2));
    let committee_cold_resign_1 = CommitteeColdResign::new(&cred_key_hash);

    let committee_cold_resign_2 = CommitteeColdResign::new(&cred_script_hash);

    assert_eq!(
        committee_cold_resign_1.committee_cold_key(),
        cred_key_hash
    );
    assert!(!committee_cold_resign_1.has_script_credentials());
    assert_eq!(
        committee_cold_resign_2.committee_cold_key(),
        cred_script_hash
    );
    assert!(committee_cold_resign_2.has_script_credentials());
}

#[test]
fn committee_hot_auth_setters_getters_test() {
    let cold_cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let hot_cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let committee_hot_auth =
        CommitteeHotAuth::new(&cold_cred_key_hash, &hot_cred_key_hash);

    assert_eq!(
        committee_hot_auth.committee_cold_key(),
        cold_cred_key_hash
    );
    assert_eq!(
        committee_hot_auth.committee_hot_key(),
        hot_cred_key_hash
    );
    assert!(!committee_hot_auth.has_script_credentials());
}

#[test]
fn drep_deregistration_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let cred_script_hash = Credential::from_scripthash(&fake_script_hash(2));
    let coin = Coin::from(100u32);
    let drep_deregistration_1 = DrepDeregistration::new(&cred_key_hash, &coin);

    let drep_deregistration_2 = DrepDeregistration::new(&cred_script_hash, &coin);

    assert_eq!(drep_deregistration_1.voting_credential(), cred_key_hash);
    assert_eq!(drep_deregistration_1.coin(), coin);
    assert!(!drep_deregistration_1.has_script_credentials());
    assert_eq!(drep_deregistration_2.voting_credential(), cred_script_hash);
    assert_eq!(drep_deregistration_2.coin(), coin);
    assert!(drep_deregistration_2.has_script_credentials());
}

#[test]
fn drep_registration_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let cred_script_hash = Credential::from_scripthash(&fake_script_hash(2));
    let coin = Coin::from(100u32);
    let drep_registration_1 = DrepRegistration::new(&cred_key_hash, &coin);

    let anchor = fake_anchor();
    let drep_registration_2 = DrepRegistration::new_with_anchor(&cred_script_hash, &coin, &anchor);

    assert_eq!(drep_registration_1.voting_credential(), cred_key_hash);
    assert_eq!(drep_registration_1.coin(), coin);
    assert_eq!(drep_registration_2.voting_credential(), cred_script_hash);
    assert_eq!(drep_registration_2.coin(), coin);
}

#[test]
fn drep_update_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let cred_script_hash = Credential::from_scripthash(&fake_script_hash(2));
    let drep_update_1 = DrepUpdate::new(&cred_key_hash);

    let anchor = fake_anchor();
    let drep_update_2 = DrepUpdate::new_with_anchor(&cred_script_hash, &anchor);

    assert_eq!(drep_update_1.voting_credential(), cred_key_hash);
    assert_eq!(drep_update_1.anchor(), None);
    assert_eq!(drep_update_2.voting_credential(), cred_script_hash);
    assert_eq!(drep_update_2.anchor(), Some(anchor));
}

#[test]
fn genesis_key_delegation_setters_getters_test() {
    let genesishash = fake_genesis_hash(1);
    let genesis_delegate_hash = fake_genesis_delegate_hash(2);
    let vrf_keyhash = fake_vrf_key_hash(3);

    let genesis_key_delegation =
        GenesisKeyDelegation::new(&genesishash, &genesis_delegate_hash, &vrf_keyhash);

    assert_eq!(genesis_key_delegation.genesishash(), genesishash);
    assert_eq!(
        genesis_key_delegation.genesis_delegate_hash(),
        genesis_delegate_hash
    );
    assert_eq!(genesis_key_delegation.vrf_keyhash(), vrf_keyhash);
}

#[test]
fn move_instantaneous_rewards_setters_getters_test() {
    let mir_to_other_pot =
        MoveInstantaneousReward::new_to_other_pot(MIRPot::Reserves, &Coin::from(100u32));

    let mut rewards = MIRToStakeCredentials::new();
    rewards.insert(
        &Credential::from_keyhash(&fake_key_hash(1)),
        &DeltaCoin::new_i32(100),
    );
    rewards.insert(
        &Credential::from_keyhash(&fake_key_hash(2)),
        &DeltaCoin::new_i32(200),
    );

    let mir_to_stake_credentials =
        MoveInstantaneousReward::new_to_stake_creds(MIRPot::Treasury, &rewards);

    assert_eq!(mir_to_other_pot.kind(), MIRKind::ToOtherPot);
    assert_eq!(mir_to_other_pot.pot(), MIRPot::Reserves);
    assert_eq!(mir_to_other_pot.as_to_other_pot(), Some(Coin::from(100u32)));
    assert_eq!(mir_to_other_pot.as_to_stake_creds(), None);

    assert_eq!(mir_to_stake_credentials.kind(), MIRKind::ToStakeCredentials);
    assert_eq!(mir_to_stake_credentials.pot(), MIRPot::Treasury);
    assert_eq!(mir_to_stake_credentials.as_to_other_pot(), None);
    assert_eq!(mir_to_stake_credentials.as_to_stake_creds(), Some(rewards));
}

#[test]
fn pool_registration_setters_getters_test() {
    let pool_params = fake_full_pool_params();
    let pool_registration = PoolRegistration::new(&pool_params);

    assert_eq!(pool_registration.pool_params(), pool_params);
}

#[test]
fn pool_retirement_setters_getters_test() {
    let pool_key_hash = fake_key_hash(1);
    let epoch = Epoch::from(100u32);
    let pool_retirement = PoolRetirement::new(&pool_key_hash, epoch);

    assert_eq!(pool_retirement.pool_keyhash(), pool_key_hash);
    assert_eq!(pool_retirement.epoch(), epoch);
}

#[test]
fn stake_and_vote_delegation_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let pool_key_hash = fake_key_hash(2);
    let drep = DRep::new_always_no_confidence();
    let stake_and_vote_delegation =
        StakeAndVoteDelegation::new(&cred_key_hash, &pool_key_hash, &drep);

    assert_eq!(stake_and_vote_delegation.stake_credential(), cred_key_hash);
    assert_eq!(stake_and_vote_delegation.pool_keyhash(), pool_key_hash);
    assert_eq!(stake_and_vote_delegation.drep(), drep);
    assert_eq!(stake_and_vote_delegation.has_script_credentials(), false);
}

#[test]
fn stake_delegation_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let pool_key_hash = fake_key_hash(2);
    let stake_delegation = StakeDelegation::new(&cred_key_hash, &pool_key_hash);
    assert_eq!(stake_delegation.stake_credential(), cred_key_hash);
    assert_eq!(stake_delegation.pool_keyhash(), pool_key_hash);
    assert_eq!(stake_delegation.has_script_credentials(), false);
}

#[test]
fn stake_deregisration_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let stake_deregistration_1 = StakeDeregistration::new(&cred_key_hash);

    let coin = Coin::from(100u32);
    let stake_deregistration_2 = StakeDeregistration::new_with_explicit_refund(&cred_key_hash, &coin);

    assert_eq!(stake_deregistration_1.stake_credential(), cred_key_hash);
    assert_eq!(stake_deregistration_1.coin(), None);
    assert_eq!(stake_deregistration_1.has_script_credentials(), false);
    assert_eq!(stake_deregistration_2.stake_credential(), cred_key_hash);
    assert_eq!(stake_deregistration_2.coin(), Some(coin));
    assert_eq!(stake_deregistration_2.has_script_credentials(), false);
}

#[test]
fn stake_regisration_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let coin = Coin::from(100u32);
    let stake_registration_1 = StakeRegistration::new(&cred_key_hash);
    let stake_registration_2 = StakeRegistration::new_with_explicit_deposit(&cred_key_hash, &coin);

    assert_eq!(stake_registration_1.stake_credential(), cred_key_hash);
    assert_eq!(stake_registration_1.coin(), None);
    assert_eq!(stake_registration_2.stake_credential(), cred_key_hash);
    assert_eq!(stake_registration_2.coin(), Some(coin));
}

#[test]
fn stake_registration_and_delegation_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let pool_key_hash = fake_key_hash(2);
    let coin = Coin::from(100u32);
    let stake_registration_and_delegation =
        StakeRegistrationAndDelegation::new(&cred_key_hash, &pool_key_hash, &coin);

    assert_eq!(
        stake_registration_and_delegation.stake_credential(),
        cred_key_hash
    );
    assert_eq!(
        stake_registration_and_delegation.pool_keyhash(),
        pool_key_hash
    );
    assert_eq!(stake_registration_and_delegation.coin(), coin);
}

#[test]
fn stake_vote_registration_and_delegation_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let pool_key_hash = fake_key_hash(2);
    let drep = DRep::new_always_no_confidence();
    let coin = Coin::from(100u32);
    let stake_vote_registration_and_delegation =
        StakeVoteRegistrationAndDelegation::new(&cred_key_hash, &pool_key_hash, &drep, &coin);

    assert_eq!(
        stake_vote_registration_and_delegation.stake_credential(),
        cred_key_hash
    );
    assert_eq!(
        stake_vote_registration_and_delegation.pool_keyhash(),
        pool_key_hash
    );
    assert_eq!(stake_vote_registration_and_delegation.drep(), drep);
    assert_eq!(stake_vote_registration_and_delegation.coin(), coin);
    assert_eq!(
        stake_vote_registration_and_delegation.has_script_credentials(),
        false
    );
}

#[test]
fn vote_delegation_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let drep = DRep::new_always_no_confidence();
    let vote_delegation = VoteDelegation::new(&cred_key_hash, &drep);

    assert_eq!(vote_delegation.stake_credential(), cred_key_hash);
    assert_eq!(vote_delegation.drep(), drep);
    assert_eq!(vote_delegation.has_script_credentials(), false);
}

#[test]
fn vote_registration_and_delegation_setters_getters_test() {
    let cred_key_hash = Credential::from_keyhash(&fake_key_hash(1));
    let drep = DRep::new_always_no_confidence();
    let coin = Coin::from(100u32);
    let vote_registration_and_delegation =
        VoteRegistrationAndDelegation::new(&cred_key_hash, &drep, &coin);

    assert_eq!(
        vote_registration_and_delegation.stake_credential(),
        cred_key_hash
    );
    assert_eq!(vote_registration_and_delegation.drep(), drep);
    assert_eq!(vote_registration_and_delegation.coin(), coin);
    assert_eq!(
        vote_registration_and_delegation.has_script_credentials(),
        false
    );
}

#[test]
fn certificates_deduplication_test() {
    let mut certs = Certificates::new();
    let cert1 = Certificate::new_stake_registration(&StakeRegistration::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
    ));
    let cert2 = Certificate::new_stake_registration(&StakeRegistration::new(
        &Credential::from_keyhash(&fake_key_hash(2)),
    ));
    let cert3 = Certificate::new_stake_registration(&StakeRegistration::new(
        &Credential::from_keyhash(&fake_key_hash(1)),
    ));

    assert_eq!(certs.len(), 0);
    assert!(certs.add(&cert1));
    assert_eq!(certs.len(), 1);
    assert!(certs.add(&cert2));
    assert_eq!(certs.len(), 2);
    assert!(!certs.add(&cert3));
    assert_eq!(certs.len(), 2);
    assert_eq!(certs.get(0), cert1);
    assert_eq!(certs.get(1), cert2);

    let bytes = certs.to_bytes();
    let certs2 = Certificates::from_bytes(bytes).unwrap();
    assert_eq!(certs, certs2);
}
