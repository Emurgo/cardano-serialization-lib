use crate::*;
use crate::fakes::{fake_key_hash, fake_script_hash, fake_tx_hash};
use crate::tests::mock_objects::{crate_full_protocol_param_update, create_anchor, create_change_address, create_plutus_script, create_tx_builder_with_amount_and_deposit_params};

fn total_tx_output_with_fee(tx: &Transaction) -> Coin {
    let mut total = Coin::zero();
    for output in &tx.body().outputs() {
        total = total.checked_add(&output.amount().coin()).unwrap();
    }

    total.checked_add(&tx.body().fee()).unwrap()
}

#[test]
fn voting_proposal_builder_one_proposal() {
    let proposal_deposit = Coin::from(1000u64);
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let proposal =
        HardForkInitiationProposal::new_with_action_id(&action_id, &ProtocolVersion::new(1, 2));
    let mut builder = VotingProposalBuilder::new();
    let wrapped_proposal = VotingProposal::new_hard_fork_initiation_proposal(&proposal);
    builder.add(&wrapped_proposal).unwrap();

    let witnesses = builder.get_plutus_witnesses();
    assert_eq!(witnesses.len(), 0);

    let inputs = builder.get_ref_inputs();
    assert_eq!(inputs.len(), 0);

    assert_eq!(builder.has_plutus_scripts(), false);
    assert_eq!(builder.get_total_deposit(&proposal_deposit).unwrap(), proposal_deposit.clone());

    let initial_amount = 1000000000u64;
    let mut tx_builder = create_tx_builder_with_amount_and_deposit_params(
        initial_amount,
        500,
        500,
        proposal_deposit.into(),
        false);

    tx_builder.set_voting_proposal_builder(&builder);
    tx_builder.add_change_if_needed(&create_change_address()).unwrap();

    let tx = tx_builder.build_tx().unwrap();

    let voting_proposals = tx.body().voting_proposals().unwrap();
    assert_eq!(voting_proposals.len(), 1);
    assert_eq!(voting_proposals.get(0), wrapped_proposal);

    let mut total_out = total_tx_output_with_fee(&tx);
    total_out = total_out.checked_add(&proposal_deposit).unwrap();
    assert_eq!(total_out, Coin::from(initial_amount));
}

#[test]
fn voting_proposal_builder_all_proposals() {
    let proposal_deposit = Coin::from(1000u64);
    let total_deposit = proposal_deposit.checked_mul(&Coin::from(7u64)).unwrap();

    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let hf_proposal =
        HardForkInitiationProposal::new_with_action_id(&action_id, &ProtocolVersion::new(1, 2));
    let mut builder = VotingProposalBuilder::new();
    let wrapped_hf_proposal = VotingProposal::new_hard_fork_initiation_proposal(&hf_proposal);
    builder.add(&wrapped_hf_proposal).unwrap();

    let mut committee =
        Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));
    committee.add_member(&Credential::from_keyhash(&fake_key_hash(1)), 1);
    let mut members_to_remove = StakeCredentials::new();
    members_to_remove.add(&Credential::from_keyhash(&fake_key_hash(1)));
    let committee_proposal = NewCommitteeProposal::new(&committee, &members_to_remove);
    let wrapped_committee_proposal = VotingProposal::new_new_committee_proposal(&committee_proposal);
    builder.add(&wrapped_committee_proposal).unwrap();

    let anchor = create_anchor();
    let constitution = Constitution::new(&anchor);
    let constitution_proposal = NewConstitutionProposal::new(&constitution);
    let wrapped_constitution_proposal = VotingProposal::new_new_constitution_proposal(&constitution_proposal);
    builder.add(&wrapped_constitution_proposal).unwrap();

    let no_conf_proposal = NoConfidenceProposal::new();
    let wrapped_no_conf_proposal = VotingProposal::new_no_confidence_proposal(&no_conf_proposal);
    builder.add(&wrapped_no_conf_proposal).unwrap();

    let parameters_update = crate_full_protocol_param_update();
    let pp_update_proposal = ParameterChangeProposal::new(&parameters_update);
    let wrapped_pp_update_proposal = VotingProposal::new_parameter_change_proposal(&pp_update_proposal);
    builder.add(&wrapped_pp_update_proposal).unwrap();

    let mut withdrawals = TreasuryWithdrawals::new();
    let addr1 = RewardAddress::new(1, &Credential::from_keyhash(&fake_key_hash(1)));
    withdrawals.insert(&addr1, &Coin::from(1u32));
    let withdrawal_proposal = TreasuryWithdrawalsProposal::new(&withdrawals);
    let wrapped_withdrawal_proposal = VotingProposal::new_treasury_withdrawals_proposal(&withdrawal_proposal);
    builder.add(&wrapped_withdrawal_proposal).unwrap();

    let info_proposal = InfoProposal::new();
    let wrapped_info_proposal = VotingProposal::new_info_proposal(&info_proposal);
    builder.add(&wrapped_info_proposal).unwrap();

    let witnesses = builder.get_plutus_witnesses();
    assert_eq!(witnesses.len(), 0);

    let inputs = builder.get_ref_inputs();
    assert_eq!(inputs.len(), 0);

    assert_eq!(builder.has_plutus_scripts(), false);
    assert_eq!(builder.get_total_deposit(&proposal_deposit).unwrap(), total_deposit.clone());

    let initial_amount = 1000000000u64;
    let mut tx_builder = create_tx_builder_with_amount_and_deposit_params(
        initial_amount,
        500,
        500,
        proposal_deposit.into(),
        false);

    tx_builder.set_voting_proposal_builder(&builder);
    tx_builder.add_change_if_needed(&create_change_address()).unwrap();

    let tx = tx_builder.build_tx().unwrap();

    let voting_proposals = tx.body().voting_proposals().unwrap();
    assert_eq!(voting_proposals.len(), 7);
    assert!(voting_proposals.0.contains(&wrapped_hf_proposal));
    assert!(voting_proposals.0.contains(&wrapped_committee_proposal));
    assert!(voting_proposals.0.contains(&wrapped_constitution_proposal));
    assert!(voting_proposals.0.contains(&wrapped_no_conf_proposal));
    assert!(voting_proposals.0.contains(&wrapped_pp_update_proposal));
    assert!(voting_proposals.0.contains(&wrapped_withdrawal_proposal));
    assert!(voting_proposals.0.contains(&wrapped_info_proposal));

    let mut total_out = total_tx_output_with_fee(&tx);
    total_out = total_out.checked_add(&total_deposit).unwrap();
    assert_eq!(total_out, Coin::from(initial_amount));
}

#[test]
fn voting_proposal_builder_with_plutus_script_witness() {
    let proposal_deposit = Coin::from(1000u64);
    let total_deposit = proposal_deposit.checked_mul(&Coin::from(2u64)).unwrap();

    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let hf_proposal =
        HardForkInitiationProposal::new_with_action_id(&action_id, &ProtocolVersion::new(1, 2));
    let mut builder = VotingProposalBuilder::new();
    let wrapped_hf_proposal = VotingProposal::new_hard_fork_initiation_proposal(&hf_proposal);
    builder.add(&wrapped_hf_proposal).unwrap();

    let script = create_plutus_script(1, &Language::new_plutus_v2());
    let redeemer = Redeemer::new(
        &RedeemerTag::new_cert(),
        &BigNum::from(100u32),
        &PlutusData::new_empty_constr_plutus_data(&BigNum::zero()),
        &ExUnits::new(&BigNum::zero(), &BigNum::zero()),
    );
    let expected_redeemer =
        redeemer.clone_with_index_and_tag(&BigNum::from(1u64), &RedeemerTag::new_voting_proposal());
    let plutus_witness = PlutusWitness::new_without_datum(
        &script,
        &redeemer,
    );

    let mut committee =
        Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));
    committee.add_member(&Credential::from_keyhash(&fake_key_hash(1)), 1);
    let mut members_to_remove = StakeCredentials::new();
    members_to_remove.add(&Credential::from_keyhash(&fake_key_hash(1)));
    let committee_proposal = NewCommitteeProposal::new(&committee, &members_to_remove);
    let wrapped_committee_proposal = VotingProposal::new_new_committee_proposal(&committee_proposal);
    builder.add_with_plutus_witness(&wrapped_committee_proposal, &plutus_witness).unwrap();

    let witnesses = builder.get_plutus_witnesses();
    assert_eq!(witnesses.len(), 1);

    let builder_witness = witnesses.get(0);
    assert_eq!(builder_witness.redeemer(), expected_redeemer.clone());
    assert_eq!(builder_witness.script(), Some(script.clone()));
    assert_eq!(builder_witness.datum(), None);

    let inputs = builder.get_ref_inputs();
    assert_eq!(inputs.len(), 0);

    assert_eq!(builder.has_plutus_scripts(), true);
    assert_eq!(builder.get_total_deposit(&proposal_deposit).unwrap(), total_deposit.clone());

    let initial_amount = 1000000000u64;
    let mut tx_builder = create_tx_builder_with_amount_and_deposit_params(
        initial_amount,
        500,
        500,
        proposal_deposit.into(),
        true);

    tx_builder.set_voting_proposal_builder(&builder);
    tx_builder.add_change_if_needed(&create_change_address()).unwrap();

    let mut cost_models = TxBuilderConstants::plutus_default_cost_models();
    cost_models = cost_models.retain_language_versions(&Languages(vec![Language::new_plutus_v2()]));

    tx_builder.calc_script_data_hash(&cost_models).unwrap();

    let tx = tx_builder.build_tx().unwrap();

    let voting_proposals = tx.body().voting_proposals().unwrap();
    assert_eq!(voting_proposals.len(), 2);
    assert!(voting_proposals.0.contains(&wrapped_hf_proposal));
    assert!(voting_proposals.0.contains(&wrapped_committee_proposal));

    let mut total_out = total_tx_output_with_fee(&tx);
    total_out = total_out.checked_add(&total_deposit).unwrap();
    assert_eq!(total_out, Coin::from(initial_amount));

    let tx_witnesses = tx.witness_set();
    let tx_script = tx_witnesses.plutus_scripts().unwrap();

    assert_eq!(tx_script.len(), 1);
    assert_eq!(tx_script.get(0), script);

    let tx_redeemers = tx_witnesses.redeemers().unwrap();
    assert_eq!(tx_redeemers.len(), 1);
    assert_eq!(tx_redeemers.get(0), expected_redeemer);

    assert_eq!(tx_witnesses.plutus_data(), None);

    assert_eq!(tx.body().reference_inputs(), None);

    let script_data_hash = hash_script_data(&tx_redeemers, &cost_models, None);

    assert_eq!(tx.body().script_data_hash(), Some(script_data_hash));
}

#[test]
fn voting_proposal_builder_with_ref_plutus_script_witness() {
    let proposal_deposit = Coin::from(1000u64);
    let total_deposit = proposal_deposit.checked_mul(&Coin::from(2u64)).unwrap();

    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 0);
    let hf_proposal =
        HardForkInitiationProposal::new_with_action_id(&action_id, &ProtocolVersion::new(1, 2));
    let mut builder = VotingProposalBuilder::new();
    let wrapped_hf_proposal = VotingProposal::new_hard_fork_initiation_proposal(&hf_proposal);
    builder.add(&wrapped_hf_proposal).unwrap();

    let script_hash = fake_script_hash(1);
    let ref_input = TransactionInput::new(&fake_tx_hash(5), 0);
    let redeemer = Redeemer::new(
        &RedeemerTag::new_cert(),
        &BigNum::from(100u32),
        &PlutusData::new_empty_constr_plutus_data(&BigNum::zero()),
        &ExUnits::new(&BigNum::zero(), &BigNum::zero()),
    );
    let expected_redeemer =
        redeemer.clone_with_index_and_tag(&BigNum::from(1u64), &RedeemerTag::new_voting_proposal());
    let plutus_source = PlutusScriptSource::new_ref_input_with_lang_ver(&script_hash, &ref_input, &Language::new_plutus_v2());
    let plutus_witness = PlutusWitness::new_with_ref_without_datum(
        &plutus_source,
        &redeemer,
    );

    let mut committee =
        Committee::new(&UnitInterval::new(&BigNum::from(1u32), &BigNum::from(2u32)));
    committee.add_member(&Credential::from_keyhash(&fake_key_hash(1)), 1);
    let mut members_to_remove = StakeCredentials::new();
    members_to_remove.add(&Credential::from_keyhash(&fake_key_hash(1)));
    let committee_proposal = NewCommitteeProposal::new(&committee, &members_to_remove);
    let wrapped_committee_proposal = VotingProposal::new_new_committee_proposal(&committee_proposal);
    builder.add_with_plutus_witness(&wrapped_committee_proposal, &plutus_witness).unwrap();

    let witnesses = builder.get_plutus_witnesses();
    assert_eq!(witnesses.len(), 1);

    let builder_witness = witnesses.get(0);
    assert_eq!(builder_witness.redeemer(), expected_redeemer.clone());
    assert_eq!(builder_witness.script(), None);
    assert_eq!(builder_witness.datum(), None);

    let builder_ref_inputs = builder.get_ref_inputs();
    assert_eq!(builder_ref_inputs.len(), 1);
    assert_eq!(builder_ref_inputs.get(0), ref_input);

    assert_eq!(builder.has_plutus_scripts(), true);
    assert_eq!(builder.get_total_deposit(&proposal_deposit).unwrap(), total_deposit.clone());

    let initial_amount = 1000000000u64;
    let mut tx_builder = create_tx_builder_with_amount_and_deposit_params(
        initial_amount,
        500,
        500,
        proposal_deposit.into(),
        true);

    tx_builder.set_voting_proposal_builder(&builder);
    tx_builder.add_change_if_needed(&create_change_address()).unwrap();

    let mut cost_models = TxBuilderConstants::plutus_default_cost_models();
    cost_models = cost_models.retain_language_versions(&Languages(vec![Language::new_plutus_v2()]));

    tx_builder.calc_script_data_hash(&cost_models).unwrap();

    let tx = tx_builder.build_tx().unwrap();

    let voting_proposals = tx.body().voting_proposals().unwrap();
    assert_eq!(voting_proposals.len(), 2);
    assert!(voting_proposals.0.contains(&wrapped_hf_proposal));
    assert!(voting_proposals.0.contains(&wrapped_committee_proposal));

    let mut total_out = total_tx_output_with_fee(&tx);
    total_out = total_out.checked_add(&total_deposit).unwrap();
    assert_eq!(total_out, Coin::from(initial_amount));

    let tx_witnesses = tx.witness_set();
    let tx_script = tx_witnesses.plutus_scripts().unwrap();
    assert_eq!(tx_script.len(), 0);

    let tx_redeemers = tx_witnesses.redeemers().unwrap();
    assert_eq!(tx_redeemers.len(), 1);
    assert_eq!(tx_redeemers.get(0), expected_redeemer);

    assert_eq!(tx_witnesses.plutus_data(), None);

    let tx_ref_inputs = tx.body().reference_inputs().unwrap();
    assert_eq!(tx_ref_inputs.len(), 1);
    assert_eq!(tx_ref_inputs.get(0), ref_input);

    let script_data_hash = hash_script_data(&tx_redeemers, &cost_models, None);

    assert_eq!(tx.body().script_data_hash(), Some(script_data_hash));
}