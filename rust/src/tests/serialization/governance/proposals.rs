use crate::fakes::{
    fake_anchor_data_hash, fake_key_hash, fake_reward_address, fake_script_hash, fake_tx_hash,
};
use crate::tests::mock_objects::{crate_full_protocol_param_update, create_anchor};
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
            GovernanceAction::from_json(&json_wrapped).unwrap()
        );
        assert_eq!(
            $variable_wrapped_name,
            GovernanceAction::from_bytes(cbor_wrapped).unwrap()
        );
        assert_eq!(
            $variable_wrapped_name,
            GovernanceAction::from_hex(&hex_cbor_wrapped).unwrap()
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
fn hard_fork_initiation_action_ser_round_trip() {
    let proposal = HardForkInitiationAction::new(&ProtocolVersion::new(1, 2));

    let proposal_wrapped = GovernanceAction::new_hard_fork_initiation_action(&proposal);

    to_from_test!(HardForkInitiationAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_hard_fork_initiation_action().unwrap()
    );
}

#[test]
fn hard_fork_initiation_action_with_action_id_ser_round_trip() {
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let proposal =
        HardForkInitiationAction::new_with_action_id(&action_id, &ProtocolVersion::new(1, 2));

    let proposal_wrapped = GovernanceAction::new_hard_fork_initiation_action(&proposal);

    to_from_test!(HardForkInitiationAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_hard_fork_initiation_action().unwrap()
    );
}

#[test]
fn new_committee_action_ser_round_trip() {
    let mut committee =
        Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));
    committee.add_member(&Credential::from_keyhash(&fake_key_hash(1)), 1);
    committee.add_member(&Credential::from_scripthash(&fake_script_hash(2)), 2);

    let mut members_to_remove = Credentials::new();
    members_to_remove.add(&Credential::from_keyhash(&fake_key_hash(1)));
    members_to_remove.add(&Credential::from_scripthash(&fake_script_hash(2)));

    let proposal = UpdateCommitteeAction::new(&committee, &members_to_remove);

    let proposal_wrapped = GovernanceAction::new_new_committee_action(&proposal);

    to_from_test!(UpdateCommitteeAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_new_committee_action().unwrap()
    );
}

#[test]
fn new_committee_action_with_action_id_ser_round_trip() {
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let mut committee =
        Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));
    committee.add_member(&Credential::from_keyhash(&fake_key_hash(1)), 1);
    committee.add_member(&Credential::from_scripthash(&fake_script_hash(2)), 2);

    let mut members_to_remove = Credentials::new();
    members_to_remove.add(&Credential::from_keyhash(&fake_key_hash(1)));
    members_to_remove.add(&Credential::from_scripthash(&fake_script_hash(2)));

    let proposal =
        UpdateCommitteeAction::new_with_action_id(&action_id, &committee, &members_to_remove);

    let proposal_wrapped = GovernanceAction::new_new_committee_action(&proposal);

    to_from_test!(UpdateCommitteeAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_new_committee_action().unwrap()
    );
}

#[test]
fn new_committee_action_with_empty_ser_round_trip() {
    let committee = Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));
    let members_to_remove = Credentials::new();
    let proposal = UpdateCommitteeAction::new(&committee, &members_to_remove);

    let proposal_wrapped = GovernanceAction::new_new_committee_action(&proposal);

    to_from_test!(UpdateCommitteeAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_new_committee_action().unwrap()
    );
}

#[test]
fn new_constitution_action_ser_round_trip() {
    let anchor = Anchor::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_anchor_data_hash(1),
    );

    let constitution = Constitution::new(&anchor);
    let proposal = NewConstitutionAction::new(&constitution);

    let proposal_wrapped = GovernanceAction::new_new_constitution_action(&proposal);

    to_from_test!(NewConstitutionAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_new_constitution_action().unwrap()
    );
}

#[test]
fn new_constitution_action_with_action_id_ser_round_trip() {
    let anchor = Anchor::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_anchor_data_hash(1),
    );

    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let constitution = Constitution::new(&anchor);
    let proposal = NewConstitutionAction::new_with_action_id(&action_id, &constitution);

    let proposal_wrapped = GovernanceAction::new_new_constitution_action(&proposal);

    to_from_test!(NewConstitutionAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_new_constitution_action().unwrap()
    );
}

#[test]
fn no_confidence_action_ser_round_trip() {
    let proposal = NoConfidenceAction::new();

    let proposal_wrapped = GovernanceAction::new_no_confidence_action(&proposal);

    to_from_test!(NoConfidenceAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_no_confidence_action().unwrap()
    );
}

#[test]
fn no_confidence_action_with_action_id_ser_round_trip() {
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let proposal = NoConfidenceAction::new_with_action_id(&action_id);

    let proposal_wrapped = GovernanceAction::new_no_confidence_action(&proposal);

    to_from_test!(NoConfidenceAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_no_confidence_action().unwrap()
    );
}

#[test]
fn parameter_change_action_ser_round_trip() {
    let parameters_update = crate_full_protocol_param_update();
    let proposal = ParameterChangeAction::new(&parameters_update);
    let proposal_wrapped = GovernanceAction::new_parameter_change_action(&proposal);
    to_from_test!(ParameterChangeAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_parameter_change_action().unwrap()
    );
}

#[test]
fn parameter_change_action_with_action_id_ser_round_trip() {
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let parameters_update = crate_full_protocol_param_update();
    let proposal = ParameterChangeAction::new_with_action_id(&action_id, &parameters_update);
    let proposal_wrapped = GovernanceAction::new_parameter_change_action(&proposal);
    to_from_test!(ParameterChangeAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_parameter_change_action().unwrap()
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
fn treasury_withdrawals_action_ser_round_trip() {
    let mut withdrawals = TreasuryWithdrawals::new();
    let addr1 = RewardAddress::new(1, &Credential::from_keyhash(&fake_key_hash(1)));
    let addr2 = RewardAddress::new(2, &Credential::from_keyhash(&fake_key_hash(2)));
    withdrawals.insert(&addr1, &Coin::from(1u32));
    withdrawals.insert(&addr2, &Coin::from(2u32));

    let proposal = TreasuryWithdrawalsAction::new(&withdrawals);

    let proposal_wrapped = GovernanceAction::new_treasury_withdrawals_action(&proposal);

    to_from_test!(TreasuryWithdrawalsAction, proposal, proposal_wrapped);
    assert_eq!(
        proposal,
        proposal_wrapped.as_treasury_withdrawals_action().unwrap()
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

    let action1 = GovernanceAction::new_treasury_withdrawals_action(
        &TreasuryWithdrawalsAction::new(&withdrawals),
    );
    let action2 = GovernanceAction::new_no_confidence_action(&NoConfidenceAction::new());
    let action3 = GovernanceAction::new_info_action(&InfoAction::new());

    let proposal1 = VotingProposal::new(
        &action1,
        &create_anchor(),
        &fake_reward_address(1),
        &Coin::from(100u32),
    );
    let proposal2 = VotingProposal::new(
        &action2,
        &create_anchor(),
        &fake_reward_address(2),
        &Coin::from(200u32),
    );
    let proposal3 = VotingProposal::new(
        &action3,
        &create_anchor(),
        &fake_reward_address(3),
        &Coin::from(300u32),
    );

    proposals.add(&proposal1);
    proposals.add(&proposal2);
    proposals.add(&proposal3);

    let cbor = proposals.to_bytes();
    let cbor_hex = proposals.to_hex();
    let json = proposals.to_json().unwrap();

    assert_eq!(proposals, VotingProposals::from_bytes(cbor).unwrap());
    assert_eq!(proposals, VotingProposals::from_hex(&cbor_hex).unwrap());
    assert_eq!(proposals, VotingProposals::from_json(&json).unwrap());
}

#[test]
fn voting_proposal_round_trip_test()
{
    let mut withdrawals = TreasuryWithdrawals::new();
    let addr1 = RewardAddress::new(1, &Credential::from_keyhash(&fake_key_hash(1)));
    let addr2 = RewardAddress::new(2, &Credential::from_keyhash(&fake_key_hash(2)));
    withdrawals.insert(&addr1, &Coin::from(1u32));
    withdrawals.insert(&addr2, &Coin::from(2u32));

    let action1 = GovernanceAction::new_treasury_withdrawals_action(
        &TreasuryWithdrawalsAction::new(&withdrawals),
    );

    let proposal = VotingProposal::new(
        &action1,
        &create_anchor(),
        &fake_reward_address(1),
        &Coin::from(100u32),
    );

    let cbor = proposal.to_bytes();
    let cbor_hex = proposal.to_hex();
    let json = proposal.to_json().unwrap();

    assert_eq!(proposal, VotingProposal::from_bytes(cbor).unwrap());
    assert_eq!(proposal, VotingProposal::from_hex(&cbor_hex).unwrap());
    assert_eq!(proposal, VotingProposal::from_json(&json).unwrap());
}

#[test]
fn tx_with_voting_proposal_deser_test() {
    let cbor = "84a40081825820017b91576a79a3602a02a65b600665ab71037ad14aa162538a26e64b3f5069fc000181a2005839002d745f050a8f7e263f4d0749a82284ed9cc065018c1f4f6a7c1b764882293a49e3ef29a4f9c32e4c18f202f5324182db7790f48dccf7a6dd011b0000000253d1efbc021a0002b3b11481841a000f4240581de082293a49e3ef29a4f9c32e4c18f202f5324182db7790f48dccf7a6dd8305f68282781968747470733a2f2f73686f727475726c2e61742f6173494a365820ee90ece16c47bf812b88edb89a01539e6683d6549a80b15383a4fb218ab9412df682781968747470733a2f2f73686f727475726c2e61742f784d53313558206f890de0c6e418e6526e2b1aa821850cb87aee94a6d77dc2a2e440116abc8e09a0f5f6";
    let tx_deser = Transaction::from_hex(cbor);
    assert!(tx_deser.is_ok());

    let proposals = tx_deser.unwrap().body().voting_proposals();
    assert!(proposals.is_some());
    let proposal = proposals.unwrap().get(0);
    let expected_coin = Coin::from(1000000u32);
    assert_eq!(proposal.deposit(), expected_coin);
}

#[test]
fn tx_with_info_proposal_deser_test() {
    let cbor = "84a40081825820f83bdffcbc203eec54dc71208aa7974c538414898673cd7af900149e8c8e392b0001818258390030a33756d8cbf4d18ce8c9995feca1ea1fc70093943c17bd96d65fed0aed6caa1cfe93f03f6ef1d9701df8024494d0b3b8a53a1ee37c5ab21b0000000253cd778c021a0002a75114818400581de00aed6caa1cfe93f03f6ef1d9701df8024494d0b3b8a53a1ee37c5ab2810682781868747470733a2f2f73686f727475726c2e61742f7279616e582013b0234dab754774e4530a0918d8272491541a8d2f6cf8ab0a10abdaa81f2440a10081825820684cb4218cb7e943e5f728ec09ed7f9486b6c164f332312c095067e21db9592b5840a3294bdea8fd49c8e7bd965d02b37033285db1907d1fab13cce281686cae7b23ee7c8aa534f229aade6b0bacfd71a518a24aeb73d08d879aaaee14aa16abf30af5f6";
    let tx_deser = Transaction::from_hex(cbor);
    assert!(tx_deser.is_ok());

    let proposals = tx_deser.unwrap().body().voting_proposals();
    assert!(proposals.is_some());
    let proposal = proposals.unwrap().get(0);
    let expected_coin = Coin::zero();
    assert_eq!(proposal.deposit(), expected_coin);

    let info = proposal.governance_action().as_info_action();
    assert!(info.is_some());
}