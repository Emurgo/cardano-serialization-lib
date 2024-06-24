use crate::tests::mock_objects::{create_anchor, fake_anchor_data_hash, fake_key_hash, fake_script_hash, fake_tx_hash};
use crate::*;

#[test]
fn drep_abstain_setters_getters_test() {
    let drep = DRep::new_always_abstain();
    assert_eq!(drep.kind(), DRepKind::AlwaysAbstain);
    assert_eq!(drep.to_key_hash(), None);
    assert_eq!(drep.to_script_hash(), None);
}

#[test]
fn drep_no_confidence_setters_getters_test() {
    let drep = DRep::new_always_no_confidence();
    assert_eq!(drep.kind(), DRepKind::AlwaysNoConfidence);
    assert_eq!(drep.to_key_hash(), None);
    assert_eq!(drep.to_script_hash(), None);
}

#[test]
fn drep_key_hash_setters_getters_test() {
    let key_hash = fake_key_hash(1);
    let drep = DRep::new_key_hash(&key_hash);
    assert_eq!(drep.kind(), DRepKind::KeyHash);
    assert_eq!(drep.to_key_hash(), Some(key_hash));
    assert_eq!(drep.to_script_hash(), None);
}

#[test]
fn drep_script_hash_setters_getters_test() {
    let script_hash = fake_script_hash(1);
    let drep = DRep::new_script_hash(&script_hash);
    assert_eq!(drep.kind(), DRepKind::ScriptHash);
    assert_eq!(drep.to_key_hash(), None);
    assert_eq!(drep.to_script_hash(), Some(script_hash));
}

#[test]
fn anchor_setters_getters_test() {
    let data_hash = fake_anchor_data_hash(1);
    let url = URL::new("https://example.com".to_string()).unwrap();
    let anchor = Anchor::new(&url, &data_hash);
    assert_eq!(anchor.url(), url);
    assert_eq!(anchor.anchor_data_hash(), data_hash);
}

#[test]
fn governance_action_id_setters_getters_test() {
    let index = 1;
    let tx_hash = fake_tx_hash(1);
    let governance_action_id = GovernanceActionId::new(&tx_hash, index);
    assert_eq!(governance_action_id.transaction_id(), tx_hash);
    assert_eq!(governance_action_id.index(), index);
}

#[test]
fn governance_action_ids_setters_getters_test() {
    let index_1 = 1;
    let tx_hash_1 = fake_tx_hash(1);
    let index_2 = 2;
    let tx_hash_2 = fake_tx_hash(2);
    let governance_action_id_1 = GovernanceActionId::new(&tx_hash_1, index_1);
    let governance_action_id_2 = GovernanceActionId::new(&tx_hash_2, index_2);
    let mut governance_action_ids = GovernanceActionIds::new();
    governance_action_ids.add(&governance_action_id_1);
    governance_action_ids.add(&governance_action_id_2);
    assert_eq!(governance_action_ids.len(), 2);
    assert_eq!(governance_action_ids.get(0), Some(governance_action_id_1));
    assert_eq!(governance_action_ids.get(1), Some(governance_action_id_2));
    assert_eq!(governance_action_ids.get(2), None);
}

#[test]
fn voter_drep_key_hash_setters_getters_test() {
    let key_hash = fake_key_hash(1);
    let voter = Voter::new_drep(&Credential::from_keyhash(&key_hash));
    assert_eq!(voter.kind(), VoterKind::DRepKeyHash);
    assert_eq!(
        voter.to_drep_cred(),
        Some(Credential::from_keyhash(&key_hash))
    );
    assert_eq!(voter.to_staking_pool_key_hash(), None);
    assert_eq!(voter.to_constitutional_committee_hot_cred(), None);
    assert_eq!(voter.has_script_credentials(), false);
    assert_eq!(voter.to_key_hash(), Some(key_hash));
}

#[test]
fn voter_drep_script_hash_setters_getters_test() {
    let script_hash = fake_script_hash(1);
    let voter = Voter::new_drep(&Credential::from_scripthash(&script_hash));
    assert_eq!(voter.kind(), VoterKind::DRepScriptHash);
    assert_eq!(
        voter.to_drep_cred(),
        Some(Credential::from_scripthash(&script_hash))
    );
    assert_eq!(voter.to_staking_pool_key_hash(), None);
    assert_eq!(voter.to_constitutional_committee_hot_cred(), None);
    assert_eq!(voter.has_script_credentials(), true);
    assert_eq!(voter.to_key_hash(), None);
}

#[test]
fn voter_constitutional_committee_hot_key_hash_setters_getters_test() {
    let key_hash = fake_key_hash(1);
    let voter = Voter::new_constitutional_committee_hot_key(&Credential::from_keyhash(&key_hash));
    assert_eq!(voter.kind(), VoterKind::ConstitutionalCommitteeHotKeyHash);
    assert_eq!(
        voter.to_constitutional_committee_hot_cred(),
        Some(Credential::from_keyhash(&key_hash))
    );
    assert_eq!(voter.to_staking_pool_key_hash(), None);
    assert_eq!(voter.to_drep_cred(), None);
    assert_eq!(voter.has_script_credentials(), false);
    assert_eq!(voter.to_key_hash(), Some(key_hash));
}

#[test]
fn voter_constitutional_committee_hot_script_hash_setters_getters_test() {
    let script_hash = fake_script_hash(1);
    let voter =
        Voter::new_constitutional_committee_hot_key(&Credential::from_scripthash(&script_hash));
    assert_eq!(
        voter.kind(),
        VoterKind::ConstitutionalCommitteeHotScriptHash
    );
    assert_eq!(
        voter.to_constitutional_committee_hot_cred(),
        Some(Credential::from_scripthash(&script_hash))
    );
    assert_eq!(voter.to_staking_pool_key_hash(), None);
    assert_eq!(voter.to_drep_cred(), None);
    assert_eq!(voter.has_script_credentials(), true);
    assert_eq!(voter.to_key_hash(), None);
}

#[test]
fn voter_staking_pool_key_hash_setters_getters_test() {
    let key_hash = fake_key_hash(1);
    let voter = Voter::new_staking_pool(&key_hash);
    assert_eq!(voter.kind(), VoterKind::StakingPoolKeyHash);
    assert_eq!(voter.to_staking_pool_key_hash(), Some(key_hash.clone()));
    assert_eq!(voter.to_constitutional_committee_hot_cred(), None);
    assert_eq!(voter.to_drep_cred(), None);
    assert_eq!(voter.has_script_credentials(), false);
    assert_eq!(voter.to_key_hash(), Some(key_hash));
}

#[test]
fn voters_setters_getters_test() {
    let key_hash_1 = fake_key_hash(1);
    let voter_1 = Voter::new_staking_pool(&key_hash_1);
    let key_hash_2 = fake_key_hash(2);
    let voter_2 = Voter::new_staking_pool(&key_hash_2);
    let mut voters = Voters::new();
    voters.add(&voter_1);
    voters.add(&voter_2);
    assert_eq!(voters.len(), 2);
    assert_eq!(voters.get(0), Some(voter_1));
    assert_eq!(voters.get(1), Some(voter_2));
    assert_eq!(voters.get(2), None);
}

#[test]
fn voting_procedure_setters_getters_test() {
    let yes_procedure = VotingProcedure::new(VoteKind::Yes);
    assert_eq!(yes_procedure.vote_kind(), VoteKind::Yes);
    assert_eq!(yes_procedure.anchor(), None);

    let no_procedure = VotingProcedure::new(VoteKind::No);
    assert_eq!(no_procedure.vote_kind(), VoteKind::No);
    assert_eq!(no_procedure.anchor(), None);

    let abstain_procedure = VotingProcedure::new(VoteKind::Abstain);
    assert_eq!(abstain_procedure.vote_kind(), VoteKind::Abstain);
    assert_eq!(abstain_procedure.anchor(), None);
}

#[test]
fn voting_procedure_with_anchor_setters_getters_test() {
    let anchor = create_anchor();
    let yes_procedure = VotingProcedure::new_with_anchor(VoteKind::Yes, &anchor);
    assert_eq!(yes_procedure.vote_kind(), VoteKind::Yes);
    assert_eq!(yes_procedure.anchor(), Some(anchor.clone()));

    let no_procedure = VotingProcedure::new_with_anchor(VoteKind::No, &anchor);
    assert_eq!(no_procedure.vote_kind(), VoteKind::No);
    assert_eq!(no_procedure.anchor(), Some(anchor.clone()));

    let abstain_procedure = VotingProcedure::new_with_anchor(VoteKind::Abstain, &anchor);
    assert_eq!(abstain_procedure.vote_kind(), VoteKind::Abstain);
    assert_eq!(abstain_procedure.anchor(), Some(anchor));
}

#[test]
fn voting_procedures_setters_getters_test() {
    let key_hash_1 = fake_key_hash(1);
    let voter_1 = Voter::new_staking_pool(&key_hash_1);
    let key_hash_2 = fake_key_hash(2);
    let voter_2 = Voter::new_staking_pool(&key_hash_2);
    let governance_action_id_1 = GovernanceActionId::new(&fake_tx_hash(1), 1);
    let governance_action_id_2 = GovernanceActionId::new(&fake_tx_hash(2), 2);
    let governance_action_id_3 = GovernanceActionId::new(&fake_tx_hash(3), 3);
    let voting_procedure_1 = VotingProcedure::new(VoteKind::Yes);
    let voting_procedure_2 = VotingProcedure::new(VoteKind::No);
    let voting_procedure_3 = VotingProcedure::new(VoteKind::Abstain);
    let mut voting_procedures = VotingProcedures::new();
    voting_procedures.insert(&voter_1, &governance_action_id_1, &voting_procedure_1);
    voting_procedures.insert(&voter_2, &governance_action_id_2, &voting_procedure_2);
    voting_procedures.insert(&voter_2, &governance_action_id_3, &voting_procedure_3);

    assert_eq!(
        voting_procedures.get(&voter_1, &governance_action_id_1),
        Some(voting_procedure_1)
    );
    assert_eq!(
        voting_procedures.get(&voter_2, &governance_action_id_2),
        Some(voting_procedure_2)
    );
    assert_eq!(
        voting_procedures.get(&voter_2, &governance_action_id_3),
        Some(voting_procedure_3)
    );
    assert_eq!(
        voting_procedures.get(&voter_1, &governance_action_id_2),
        None
    );
    assert_eq!(
        voting_procedures.get(&voter_1, &governance_action_id_3),
        None
    );
    assert_eq!(
        voting_procedures.get(&voter_2, &governance_action_id_1),
        None
    );

    let voters = voting_procedures.get_voters();
    assert_eq!(voters.len(), 2);
    assert!(voters.0.contains(&voter_1));
    assert!(voters.0.contains(&voter_2));

    let governance_action_ids_1 = voting_procedures.get_governance_action_ids_by_voter(&voter_1);
    assert_eq!(governance_action_ids_1.len(), 1);
    assert!(governance_action_ids_1.0.contains(&governance_action_id_1));

    let governance_action_ids_2 = voting_procedures.get_governance_action_ids_by_voter(&voter_2);
    assert_eq!(governance_action_ids_2.len(), 2);
    assert!(governance_action_ids_2.0.contains(&governance_action_id_2));
    assert!(governance_action_ids_2.0.contains(&governance_action_id_3));
}
