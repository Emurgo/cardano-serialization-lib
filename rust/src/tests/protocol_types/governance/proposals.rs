use crate::fakes::{fake_key_hash, fake_script_hash};
use crate::tests::mock_objects::{
    crate_full_protocol_param_update, create_action_id, create_anchor,
};
use crate::*;
use itertools::Itertools;

#[test]
fn committee_setters_getters_test() {
    let threshold = UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32));
    let mut committee = Committee::new(&threshold);
    let cred_1 = Credential::from_keyhash(&fake_key_hash(1));
    let epoch_1 = Epoch::from(100u32);
    let cred_2 = Credential::from_scripthash(&fake_script_hash(2));
    let epoch_2 = Epoch::from(200u32);
    let cred_3 = Credential::from_scripthash(&fake_script_hash(3));

    committee.add_member(&cred_1, epoch_1);
    committee.add_member(&cred_2, epoch_2);

    let keys = committee.members_keys();
    assert_eq!(committee.quorum_threshold(), threshold);
    assert_eq!(keys.len(), 2);
    assert!(keys.0.iter().contains(&cred_1));
    assert!(keys.0.iter().contains(&cred_2));
    assert_eq!(committee.get_member_epoch(&cred_1), Some(epoch_1));
    assert_eq!(committee.get_member_epoch(&cred_2), Some(epoch_2));
    assert_eq!(committee.get_member_epoch(&cred_3), None);
}

#[test]
fn constitution_setters_getters_test() {
    let anchor = create_anchor();
    let constitution = Constitution::new(&anchor);
    assert_eq!(constitution.anchor(), anchor);
    assert_eq!(constitution.script_hash(), None);

    let script_hash = fake_script_hash(1);
    let constitution = Constitution::new_with_script_hash(&anchor, &script_hash);
    assert_eq!(constitution.anchor(), anchor);
    assert_eq!(constitution.script_hash(), Some(script_hash));
}

#[test]
fn hard_fork_initiation_proposal_setters_getters_test() {
    let protocol_version = ProtocolVersion::new(1, 2);
    let proposal = HardForkInitiationProposal::new(&protocol_version);
    let action_id = create_action_id();
    let proposal_with_action_id =
        HardForkInitiationProposal::new_with_action_id(&action_id, &protocol_version);
    assert_eq!(proposal.gov_action_id(), None);
    assert_eq!(proposal.protocol_version(), protocol_version);
    assert_eq!(proposal_with_action_id.gov_action_id(), Some(action_id));
    assert_eq!(proposal_with_action_id.protocol_version(), protocol_version);
}

#[test]
fn new_committee_proposal_setters_getters_test() {
    let action_id = create_action_id();
    let committee = Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));
    let members_to_remove = Credentials(
        vec![
            Credential::from_keyhash(&fake_key_hash(1)),
            Credential::from_keyhash(&fake_key_hash(2)),
        ]
        .into_iter()
        .collect(),
    );

    let proposal = NewCommitteeProposal::new(&committee, &members_to_remove);
    let proposal_with_action_id =
        NewCommitteeProposal::new_with_action_id(&action_id, &committee, &members_to_remove);
    assert_eq!(proposal.gov_action_id(), None);
    assert_eq!(proposal.committee(), committee);
    assert_eq!(proposal.members_to_remove(), members_to_remove);
    assert_eq!(proposal_with_action_id.gov_action_id(), Some(action_id));
    assert_eq!(proposal_with_action_id.committee(), committee);
    assert_eq!(
        proposal_with_action_id.members_to_remove(),
        members_to_remove
    );
}

#[test]
fn new_constitution_proposal_setters_getters_test() {
    let action_id = create_action_id();
    let constitution = Constitution::new(&create_anchor());
    let proposal = NewConstitutionProposal::new(&constitution);
    let proposal_with_action_id =
        NewConstitutionProposal::new_with_action_id(&action_id, &constitution);
    assert_eq!(proposal.gov_action_id(), None);
    assert_eq!(proposal.constitution(), constitution);
    assert_eq!(proposal_with_action_id.gov_action_id(), Some(action_id));
    assert_eq!(proposal_with_action_id.constitution(), constitution);
}

#[test]
fn no_confidence_proposal_setters_getters_test() {
    let action_id = create_action_id();
    let proposal = NoConfidenceProposal::new();
    let proposal_with_action_id = NoConfidenceProposal::new_with_action_id(&action_id);
    assert_eq!(proposal.gov_action_id(), None);
    assert_eq!(proposal_with_action_id.gov_action_id(), Some(action_id));
}

#[test]
fn parameter_change_proposal_setters_getters_test() {
    let protocol_params = crate_full_protocol_param_update();
    let action_id = create_action_id();
    let proposal = ParameterChangeProposal::new(&protocol_params);
    let proposal_with_action_id =
        ParameterChangeProposal::new_with_action_id(&action_id, &protocol_params);
    assert_eq!(proposal.gov_action_id(), None);
    assert_eq!(proposal.protocol_param_updates(), protocol_params);
    assert_eq!(proposal_with_action_id.gov_action_id(), Some(action_id));
    assert_eq!(
        proposal_with_action_id.protocol_param_updates(),
        protocol_params
    );
}

#[test]
fn treasury_withdrawals_setters_getters_test() {
    let mut withdrawals = TreasuryWithdrawals::new();
    let addr1 = RewardAddress::new(1, &Credential::from_keyhash(&fake_key_hash(1)));
    let addr2 = RewardAddress::new(1, &Credential::from_keyhash(&fake_key_hash(2)));
    let coin1 = Coin::from(100u32);
    let coin2 = Coin::from(200u32);
    withdrawals.insert(&addr1, &coin1);
    withdrawals.insert(&addr2, &coin2);

    let keys = withdrawals.keys();
    assert_eq!(keys.len(), 2);
    assert!(keys.0.iter().contains(&addr1));
    assert!(keys.0.iter().contains(&addr2));
    assert_eq!(withdrawals.get(&addr1), Some(coin1));
    assert_eq!(withdrawals.get(&addr2), Some(coin2));
}

#[test]
fn treasury_withdrawals_proposal() {
    let mut withdrawals = TreasuryWithdrawals::new();
    let addr = RewardAddress::new(1, &Credential::from_keyhash(&fake_key_hash(1)));
    let coin = Coin::from(100u32);
    withdrawals.insert(&addr, &coin);
    let proposal = TreasuryWithdrawalsProposal::new(&withdrawals);
    assert_eq!(proposal.withdrawals(), withdrawals);
}

#[test]
fn voting_proposals_setters_getters_test() {
    let mut proposals = VotingProposals::new();
    let no_confidence_proposal = NoConfidenceProposal::new();
    let parameter_change_proposal =
        ParameterChangeProposal::new(&crate_full_protocol_param_update());
    proposals.add(&VotingProposal::new_no_confidence_proposal(
        &no_confidence_proposal,
    ));
    proposals.add(&VotingProposal::new_parameter_change_proposal(
        &parameter_change_proposal,
    ));
    assert_eq!(proposals.len(), 2);
    assert_eq!(
        proposals.get(0),
        VotingProposal::new_no_confidence_proposal(&no_confidence_proposal)
    );
    assert_eq!(
        proposals.get(1),
        VotingProposal::new_parameter_change_proposal(&parameter_change_proposal)
    );
}