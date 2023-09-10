use crate::fakes::{fake_anchor_data_hash, fake_key_hash, fake_script_hash, fake_tx_hash};
use crate::tests::mock_objects::crate_full_protocol_param_update;
use crate::*;

macro_rules! to_from_test {
    ($proposal_type: ty, $variable_name: ident,  $variable_wrapped_name: ident) => {
        let json = $variable_name.to_json().unwrap();
        let cbor = $variable_name.to_bytes();
        let hex_cbor = $variable_name.to_hex();

        assert_eq!($variable_name, <$proposal_type>::from_json(&json).unwrap());
        assert_eq!($variable_name, <$proposal_type>::from_bytes(cbor).unwrap());
        assert_eq!(
            $variable_name,
            <$proposal_type>::from_hex(&hex_cbor).unwrap()
        );

        let json_wrapped = $variable_wrapped_name.to_json().unwrap();
        let cbor_wrapped = $variable_wrapped_name.to_bytes();
        let hex_cbor_wrapped = $variable_wrapped_name.to_hex();

        assert_eq!(
            $variable_wrapped_name,
            VotingProposal::from_json(&json_wrapped).unwrap()
        );
        assert_eq!(
            $variable_wrapped_name,
            VotingProposal::from_bytes(cbor_wrapped).unwrap()
        );
        assert_eq!(
            $variable_wrapped_name,
            VotingProposal::from_hex(&hex_cbor_wrapped).unwrap()
        );
    };
}

#[test]
fn committee_ser_round_trip() {
    let mut committee =
        Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));
    committee.add_member(&Credential::from_keyhash(&fake_key_hash(1)), 1);
    committee.add_member(&Credential::from_scripthash(&fake_script_hash(2)), 2);

    let cbor = committee.to_bytes();
    let cbor_hex = committee.to_hex();
    let json = committee.to_json().unwrap();

    assert_eq!(committee, Committee::from_bytes(cbor).unwrap());
    assert_eq!(committee, Committee::from_hex(&cbor_hex).unwrap());
    assert_eq!(committee, Committee::from_json(&json).unwrap());
}

#[test]
fn committee_empty_ser_round_trip() {
    let committee = Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));

    let cbor = committee.to_bytes();
    let cbor_hex = committee.to_hex();
    let json = committee.to_json().unwrap();

    assert_eq!(committee, Committee::from_bytes(cbor).unwrap());
    assert_eq!(committee, Committee::from_hex(&cbor_hex).unwrap());
    assert_eq!(committee, Committee::from_json(&json).unwrap());
}

#[test]
fn constitution_ser_round_trip() {
    let anchor = Anchor::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_anchor_data_hash(1),
    );

    let constitution = Constitution::new(&anchor);

    let cbor = constitution.to_bytes();
    let cbor_hex = constitution.to_hex();
    let json = constitution.to_json().unwrap();

    assert_eq!(constitution, Constitution::from_bytes(cbor).unwrap());
    assert_eq!(constitution, Constitution::from_hex(&cbor_hex).unwrap());
    assert_eq!(constitution, Constitution::from_json(&json).unwrap());
}

#[test]
fn constitution_with_script_hash_ser_round_trip() {
    let anchor = Anchor::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_anchor_data_hash(1),
    );

    let constitution = Constitution::new_with_script_hash(&anchor, &fake_script_hash(1));

    let cbor = constitution.to_bytes();
    let cbor_hex = constitution.to_hex();
    let json = constitution.to_json().unwrap();

    assert_eq!(constitution, Constitution::from_bytes(cbor).unwrap());
    assert_eq!(constitution, Constitution::from_hex(&cbor_hex).unwrap());
    assert_eq!(constitution, Constitution::from_json(&json).unwrap());
}

#[test]
fn hard_fork_initiation_proposal_ser_round_trip() {
    let proposal = HardForkInitiationProposal::new(&ProtocolVersion::new(1, 2));

    let proposal_wrapped = VotingProposal::new_hard_fork_initiation_proposal(&proposal);

    to_from_test!(HardForkInitiationProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_hard_fork_initiation_proposal().unwrap()
    );
}

#[test]
fn hard_fork_initiation_proposal_with_action_id_ser_round_trip() {
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let proposal =
        HardForkInitiationProposal::new_with_action_id(&action_id, &ProtocolVersion::new(1, 2));

    let proposal_wrapped = VotingProposal::new_hard_fork_initiation_proposal(&proposal);

    to_from_test!(HardForkInitiationProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_hard_fork_initiation_proposal().unwrap()
    );
}

#[test]
fn new_committee_proposal_ser_round_trip() {
    let mut committee =
        Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));
    committee.add_member(&Credential::from_keyhash(&fake_key_hash(1)), 1);
    committee.add_member(&Credential::from_scripthash(&fake_script_hash(2)), 2);

    let mut members_to_remove = Credentials::new();
    members_to_remove.add(&Credential::from_keyhash(&fake_key_hash(1)));
    members_to_remove.add(&Credential::from_scripthash(&fake_script_hash(2)));

    let proposal = NewCommitteeProposal::new(&committee, &members_to_remove);

    let proposal_wrapped = VotingProposal::new_new_committee_proposal(&proposal);

    to_from_test!(NewCommitteeProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_new_committee_proposal().unwrap()
    );
}

#[test]
fn new_committee_proposal_with_action_id_ser_round_trip() {
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let mut committee =
        Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));
    committee.add_member(&Credential::from_keyhash(&fake_key_hash(1)), 1);
    committee.add_member(&Credential::from_scripthash(&fake_script_hash(2)), 2);

    let mut members_to_remove = Credentials::new();
    members_to_remove.add(&Credential::from_keyhash(&fake_key_hash(1)));
    members_to_remove.add(&Credential::from_scripthash(&fake_script_hash(2)));

    let proposal =
        NewCommitteeProposal::new_with_action_id(&action_id, &committee, &members_to_remove);

    let proposal_wrapped = VotingProposal::new_new_committee_proposal(&proposal);

    to_from_test!(NewCommitteeProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_new_committee_proposal().unwrap()
    );
}

#[test]
fn new_committee_proposal_with_empty_ser_round_trip() {
    let committee = Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));
    let members_to_remove = Credentials::new();
    let proposal = NewCommitteeProposal::new(&committee, &members_to_remove);

    let proposal_wrapped = VotingProposal::new_new_committee_proposal(&proposal);

    to_from_test!(NewCommitteeProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_new_committee_proposal().unwrap()
    );
}

#[test]
fn new_constitution_proposal_ser_round_trip() {
    let anchor = Anchor::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_anchor_data_hash(1),
    );

    let constitution = Constitution::new(&anchor);
    let proposal = NewConstitutionProposal::new(&constitution);

    let proposal_wrapped = VotingProposal::new_new_constitution_proposal(&proposal);

    to_from_test!(NewConstitutionProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_new_constitution_proposal().unwrap()
    );
}

#[test]
fn new_constitution_proposal_with_action_id_ser_round_trip() {
    let anchor = Anchor::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_anchor_data_hash(1),
    );

    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let constitution = Constitution::new(&anchor);
    let proposal = NewConstitutionProposal::new_with_action_id(&action_id, &constitution);

    let proposal_wrapped = VotingProposal::new_new_constitution_proposal(&proposal);

    to_from_test!(NewConstitutionProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_new_constitution_proposal().unwrap()
    );
}

#[test]
fn no_confidence_proposal_ser_round_trip() {
    let proposal = NoConfidenceProposal::new();

    let proposal_wrapped = VotingProposal::new_no_confidence_proposal(&proposal);

    to_from_test!(NoConfidenceProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_no_confidence_proposal().unwrap()
    );
}

#[test]
fn no_confidence_proposal_with_action_id_ser_round_trip() {
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let proposal = NoConfidenceProposal::new_with_action_id(&action_id);

    let proposal_wrapped = VotingProposal::new_no_confidence_proposal(&proposal);

    to_from_test!(NoConfidenceProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_no_confidence_proposal().unwrap()
    );
}

#[test]
fn parameter_change_proposal_ser_round_trip() {
    let parameters_update = crate_full_protocol_param_update();
    let proposal = ParameterChangeProposal::new(&parameters_update);
    let proposal_wrapped = VotingProposal::new_parameter_change_proposal(&proposal);
    to_from_test!(ParameterChangeProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_parameter_change_proposal().unwrap()
    );
}

#[test]
fn parameter_change_proposal_with_action_id_ser_round_trip() {
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let parameters_update = crate_full_protocol_param_update();
    let proposal = ParameterChangeProposal::new_with_action_id(&action_id, &parameters_update);
    let proposal_wrapped = VotingProposal::new_parameter_change_proposal(&proposal);
    to_from_test!(ParameterChangeProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_parameter_change_proposal().unwrap()
    );
}

#[test]
fn treasury_withdrawals_ser_round_trip() {
    let mut withdrawals = TreasuryWithdrawals::new();
    let addr1 = RewardAddress::new(1, &Credential::from_keyhash(&fake_key_hash(1)));
    let addr2 = RewardAddress::new(2, &Credential::from_keyhash(&fake_key_hash(2)));
    withdrawals.insert(&addr1, &Coin::from(1u32));
    withdrawals.insert(&addr2, &Coin::from(2u32));

    let json = withdrawals.to_json().unwrap();

    assert_eq!(withdrawals, TreasuryWithdrawals::from_json(&json).unwrap());
}

#[test]
fn treasury_withdrawals_proposal_ser_round_trip() {
    let mut withdrawals = TreasuryWithdrawals::new();
    let addr1 = RewardAddress::new(1, &Credential::from_keyhash(&fake_key_hash(1)));
    let addr2 = RewardAddress::new(2, &Credential::from_keyhash(&fake_key_hash(2)));
    withdrawals.insert(&addr1, &Coin::from(1u32));
    withdrawals.insert(&addr2, &Coin::from(2u32));

    let proposal = TreasuryWithdrawalsProposal::new(&withdrawals);

    let proposal_wrapped = VotingProposal::new_treasury_withdrawals_proposal(&proposal);

    to_from_test!(TreasuryWithdrawalsProposal, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_treasury_withdrawals_proposal().unwrap()
    );
}

#[test]
fn voting_proposals_ser_round_trip() {
    let mut proposals = VotingProposals::new();
    let mut withdrawals = TreasuryWithdrawals::new();
    let addr1 = RewardAddress::new(1, &Credential::from_keyhash(&fake_key_hash(1)));
    let addr2 = RewardAddress::new(2, &Credential::from_keyhash(&fake_key_hash(2)));
    withdrawals.insert(&addr1, &Coin::from(1u32));
    withdrawals.insert(&addr2, &Coin::from(2u32));

    let proposal1 = TreasuryWithdrawalsProposal::new(&withdrawals);
    let proposal2 = NoConfidenceProposal::new();
    let proposal3 = InfoProposal::new();

    proposals.add(&VotingProposal::new_treasury_withdrawals_proposal(
        &proposal1,
    ));
    proposals.add(&VotingProposal::new_no_confidence_proposal(&proposal2));
    proposals.add(&VotingProposal::new_info_proposal(&proposal3));

    let cbor = proposals.to_bytes();
    let cbor_hex = proposals.to_hex();
    let json = proposals.to_json().unwrap();

    assert_eq!(proposals, VotingProposals::from_bytes(cbor).unwrap());
    assert_eq!(proposals, VotingProposals::from_hex(&cbor_hex).unwrap());
    assert_eq!(proposals, VotingProposals::from_json(&json).unwrap());
}
