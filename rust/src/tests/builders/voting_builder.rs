use crate::fakes::{fake_key_hash, fake_script_hash, fake_tx_hash, fake_vkey};
use crate::fees::min_fee_for_size;
use crate::tests::mock_objects::{
    create_change_address, create_linear_fee, create_plutus_script, create_rich_tx_builder,
};
use crate::*;

#[test]
fn voting_builder_key_signers_test() {
    let mut builder = VotingBuilder::new();
    let key_hash_1 = fake_key_hash(1);
    let key_hash_2 = fake_key_hash(2);
    let key_hash_3 = fake_key_hash(3);
    let action_id_1 = GovernanceActionId::new(&fake_tx_hash(1), 1);
    let action_id_2 = GovernanceActionId::new(&fake_tx_hash(2), 2);
    let action_id_3 = GovernanceActionId::new(&fake_tx_hash(3), 3);
    let vote = VotingProcedure::new(VoteKind::No);
    let voter_1 =
        Voter::new_constitutional_committee_hot_key(&Credential::from_keyhash(&key_hash_1));
    let voter_2 = Voter::new_drep(&Credential::from_keyhash(&key_hash_2));
    let voter_3 = Voter::new_staking_pool(&key_hash_3);
    builder.add(&voter_1, &action_id_1, &vote).unwrap();
    builder.add(&voter_1, &action_id_2, &vote).unwrap();
    builder.add(&voter_2, &action_id_2, &vote).unwrap();
    builder.add(&voter_3, &action_id_3, &vote).unwrap();

    let req_signers = builder.get_required_signers();
    assert_eq!(req_signers.len(), 3);
    assert!(req_signers.contains(&key_hash_1));
    assert!(req_signers.contains(&key_hash_2));
    assert!(req_signers.contains(&key_hash_3));
    assert_eq!(builder.has_plutus_scripts(), false);

    let mut tx_builder = create_rich_tx_builder(false);
    tx_builder.set_voting_builder(&builder);
    tx_builder
        .add_change_if_needed(&create_change_address())
        .unwrap();

    let tx = tx_builder.build_tx().unwrap();
    let tx_len = tx.to_bytes().len();
    let vkey_size = fake_vkey().to_bytes().len();
    let rough_tx_size = tx_len + (vkey_size * 3);

    let fee_algo = create_linear_fee(44, 155381);
    let approx_fee_with_wit = min_fee_for_size(rough_tx_size, &fee_algo).unwrap();
    assert!(approx_fee_with_wit.less_than(&tx.body().fee()));

    let voting_procedures = tx.body().voting_procedures().unwrap();
    let voters = voting_procedures.get_voters();
    assert_eq!(voters.len(), 3);
    assert!(voters.0.contains(&voter_1));
    assert!(voters.0.contains(&voter_2));
    assert!(voters.0.contains(&voter_3));

    let action_ids_1 = voting_procedures.get_governance_action_ids_by_voter(&voter_1);
    assert_eq!(action_ids_1.len(), 2);
    assert!(action_ids_1.0.contains(&action_id_1));
    assert!(action_ids_1.0.contains(&action_id_2));

    let action_ids_2 = voting_procedures.get_governance_action_ids_by_voter(&voter_2);
    assert_eq!(action_ids_2.len(), 1);
    assert!(action_ids_2.0.contains(&action_id_2));

    let action_ids_3 = voting_procedures.get_governance_action_ids_by_voter(&voter_3);
    assert_eq!(action_ids_3.len(), 1);
    assert!(action_ids_3.0.contains(&action_id_3));

    let vote_1 = voting_procedures.get(&voter_1, &action_id_1).unwrap();
    assert_eq!(vote_1, vote);

    let vote_2 = voting_procedures.get(&voter_1, &action_id_2).unwrap();
    assert_eq!(vote_2, vote);

    let vote_3 = voting_procedures.get(&voter_2, &action_id_2).unwrap();
    assert_eq!(vote_3, vote);

    let vote_4 = voting_procedures.get(&voter_3, &action_id_3).unwrap();
    assert_eq!(vote_4, vote);

    let vote_5 = voting_procedures.get(&voter_3, &action_id_1);
    assert_eq!(vote_5, None);
}

#[test]
fn voting_builder_plutus_witness() {
    let mut builder = VotingBuilder::new();
    let script = create_plutus_script(1, &Language::new_plutus_v2());
    let script_hash = script.hash();
    let redeemer = Redeemer::new(
        &RedeemerTag::new_cert(),
        &BigNum::zero(),
        &PlutusData::new_empty_constr_plutus_data(&BigNum::zero()),
        &ExUnits::new(&BigNum::zero(), &BigNum::zero()),
    );
    let expected_redeemer =
        redeemer.clone_with_index_and_tag(&BigNum::zero(), &RedeemerTag::new_vote());
    let voter = Voter::new_drep(&Credential::from_scripthash(&script_hash));
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 1);
    let vote = VotingProcedure::new(VoteKind::No);

    let witness = PlutusWitness::new_without_datum(&script, &redeemer);
    builder
        .add_with_plutus_witness(&voter, &action_id, &vote, &witness)
        .unwrap();

    let req_signers = builder.get_required_signers();
    assert_eq!(req_signers.len(), 0);

    let witnesses = builder.get_plutus_witnesses();
    assert_eq!(witnesses.len(), 1);
    let witness_from_voting_builder = witnesses.get(0);
    assert_eq!(witness_from_voting_builder.datum(), None);
    assert_eq!(witness_from_voting_builder.script(), Some(script.clone()));
    assert_eq!(
        witness_from_voting_builder.redeemer(),
        expected_redeemer.clone()
    );

    let ref_inputs = builder.get_ref_inputs();
    assert_eq!(ref_inputs.len(), 0);

    assert_eq!(builder.has_plutus_scripts(), true);

    let langs = builder.get_used_plutus_lang_versions();
    assert_eq!(langs.len(), 1);
    assert!(langs.contains(&Language::new_plutus_v2()));

    let mut tx_builder = create_rich_tx_builder(true);
    tx_builder.set_voting_builder(&builder);
    tx_builder
        .add_change_if_needed(&create_change_address())
        .unwrap();

    let mut cost_models = TxBuilderConstants::plutus_default_cost_models();
    cost_models = cost_models.retain_language_versions(&Languages(vec![Language::new_plutus_v2()]));

    tx_builder.calc_script_data_hash(&cost_models).unwrap();

    let tx = tx_builder.build_tx().unwrap();

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

    let voting_procedures = tx.body().voting_procedures().unwrap();
    let voters = voting_procedures.get_voters();
    assert_eq!(voters.len(), 1);
    assert!(voters.0.contains(&voter));

    let action_ids = voting_procedures.get_governance_action_ids_by_voter(&voter);
    assert_eq!(action_ids.len(), 1);
    assert!(action_ids.0.contains(&action_id));

    let vote_from_tx = voting_procedures.get(&voter, &action_id).unwrap();
    assert_eq!(vote_from_tx, vote);
}

#[test]
fn voting_builder_plutus_ref_witness() {
    let mut builder = VotingBuilder::new();
    let script_hash = fake_script_hash(1);
    let redeemer = Redeemer::new(
        &RedeemerTag::new_cert(),
        &BigNum::zero(),
        &PlutusData::new_empty_constr_plutus_data(&BigNum::zero()),
        &ExUnits::new(&BigNum::zero(), &BigNum::zero()),
    );

    let ref_input = TransactionInput::new(&fake_tx_hash(5), 0);
    let expected_redeemer =
        redeemer.clone_with_index_and_tag(&BigNum::zero(), &RedeemerTag::new_vote());
    let voter = Voter::new_drep(&Credential::from_scripthash(&script_hash));
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 1);
    let vote = VotingProcedure::new(VoteKind::No);

    let script_source = PlutusScriptSource::new_ref_input(
        &script_hash,
        &ref_input,
        &Language::new_plutus_v2(),
    );
    let witness = PlutusWitness::new_with_ref_without_datum(&script_source, &redeemer);
    builder
        .add_with_plutus_witness(&voter, &action_id, &vote, &witness)
        .unwrap();

    let req_signers = builder.get_required_signers();
    assert_eq!(req_signers.len(), 0);

    let witnesses = builder.get_plutus_witnesses();
    assert_eq!(witnesses.len(), 1);
    let witness_from_voting_builder = witnesses.get(0);
    assert_eq!(witness_from_voting_builder.datum(), None);
    assert_eq!(witness_from_voting_builder.script(), None);
    assert_eq!(
        witness_from_voting_builder.redeemer(),
        expected_redeemer.clone()
    );

    let ref_inputs = builder.get_ref_inputs();
    assert_eq!(ref_inputs.len(), 1);
    assert_eq!(ref_inputs.get(0), ref_input);

    assert_eq!(builder.has_plutus_scripts(), true);

    let langs = builder.get_used_plutus_lang_versions();
    assert_eq!(langs.len(), 1);
    assert!(langs.contains(&Language::new_plutus_v2()));

    let mut tx_builder = create_rich_tx_builder(true);
    tx_builder.set_voting_builder(&builder);
    tx_builder
        .add_change_if_needed(&create_change_address())
        .unwrap();

    let mut cost_models = TxBuilderConstants::plutus_default_cost_models();
    cost_models = cost_models.retain_language_versions(&Languages(vec![Language::new_plutus_v2()]));

    tx_builder.calc_script_data_hash(&cost_models).unwrap();

    let tx = tx_builder.build_tx().unwrap();

    let tx_witnesses = tx.witness_set();
    assert_eq!(tx_witnesses.plutus_scripts().map_or(0, |x| x.len()), 0);

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

#[test]
fn voting_builder_native_script_witness() {
    let mut builder = VotingBuilder::new();
    let key_hash = fake_key_hash(10);
    let native_script = NativeScript::new_script_pubkey(&ScriptPubkey::new(&key_hash));
    let script_hash = native_script.hash();

    let voter = Voter::new_drep(&Credential::from_scripthash(&script_hash));
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 1);
    let vote = VotingProcedure::new(VoteKind::No);

    let script_source = NativeScriptSource::new(&native_script);
    builder
        .add_with_native_script(&voter, &action_id, &vote, &script_source)
        .unwrap();

    let req_signers = builder.get_required_signers();
    assert_eq!(req_signers.len(), 1);
    assert!(req_signers.contains(&key_hash));

    let native_scripts = builder.get_native_scripts();
    assert_eq!(native_scripts.len(), 1);
    assert_eq!(native_scripts.get(0), native_script);

    let witnesses = builder.get_plutus_witnesses();
    assert_eq!(witnesses.len(), 0);

    let ref_inputs = builder.get_ref_inputs();
    assert_eq!(ref_inputs.len(), 0);

    assert_eq!(builder.has_plutus_scripts(), false);

    let langs = builder.get_used_plutus_lang_versions();
    assert_eq!(langs.len(), 0);

    let mut tx_builder = create_rich_tx_builder(false);
    tx_builder.set_voting_builder(&builder);
    tx_builder
        .add_change_if_needed(&create_change_address())
        .unwrap();

    let tx = tx_builder.build_tx().unwrap();

    let tx_witnesses = tx.witness_set();
    assert_eq!(tx_witnesses.plutus_scripts(), None);

    let tx_redeemers = tx_witnesses.redeemers();
    assert_eq!(tx_redeemers, None);
    assert_eq!(tx_witnesses.plutus_data(), None);
    assert_eq!(tx.body().reference_inputs(), None);
    assert_eq!(tx.body().script_data_hash(), None);

    let native_scripts = tx_witnesses.native_scripts().unwrap();
    assert_eq!(native_scripts.len(), 1);
    assert_eq!(native_scripts.get(0), native_script);

    let voting_procedures = tx.body().voting_procedures().unwrap();
    let voters = voting_procedures.get_voters();
    assert_eq!(voters.len(), 1);
    assert!(voters.0.contains(&voter));

    let action_ids = voting_procedures.get_governance_action_ids_by_voter(&voter);
    assert_eq!(action_ids.len(), 1);
    assert!(action_ids.0.contains(&action_id));

    let vote_from_tx = voting_procedures.get(&voter, &action_id).unwrap();
    assert_eq!(vote_from_tx, vote);
}

#[test]
fn voting_builder_native_script_ref_witness() {
    let mut builder = VotingBuilder::new();
    let key_hash = fake_key_hash(10);
    let script_hash = fake_script_hash(1);

    let voter = Voter::new_drep(&Credential::from_scripthash(&script_hash));
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 1);
    let vote = VotingProcedure::new(VoteKind::No);

    let mut script_signers = RequiredSigners::new();
    script_signers.add(&key_hash);

    let ref_input = TransactionInput::new(&fake_tx_hash(5), 0);
    let script_source =
        NativeScriptSource::new_ref_input(&script_hash, &ref_input, &script_signers);
    builder
        .add_with_native_script(&voter, &action_id, &vote, &script_source)
        .unwrap();

    let req_signers = builder.get_required_signers();
    assert_eq!(req_signers.len(), 1);
    assert!(req_signers.contains(&key_hash));

    let native_scripts = builder.get_native_scripts();
    assert_eq!(native_scripts.len(), 0);

    let witnesses = builder.get_plutus_witnesses();
    assert_eq!(witnesses.len(), 0);

    let ref_inputs = builder.get_ref_inputs();
    assert_eq!(ref_inputs.len(), 1);
    assert_eq!(ref_inputs.get(0), ref_input.clone());

    assert_eq!(builder.has_plutus_scripts(), false);

    let langs = builder.get_used_plutus_lang_versions();
    assert_eq!(langs.len(), 0);

    let mut tx_builder = create_rich_tx_builder(false);
    tx_builder.set_voting_builder(&builder);
    tx_builder
        .add_change_if_needed(&create_change_address())
        .unwrap();

    let tx = tx_builder.build_tx().unwrap();

    let tx_witnesses = tx.witness_set();
    assert_eq!(tx_witnesses.plutus_scripts(), None);

    let tx_redeemers = tx_witnesses.redeemers();
    assert_eq!(tx_redeemers, None);
    assert_eq!(tx_witnesses.plutus_data(), None);

    let ref_inputs = tx.body().reference_inputs().unwrap();
    assert_eq!(ref_inputs.len(), 1);
    assert_eq!(ref_inputs.get(0), ref_input);

    assert_eq!(tx.body().script_data_hash(), None);

    assert_eq!(tx_witnesses.native_scripts(), None);

    let voting_procedures = tx.body().voting_procedures().unwrap();
    let voters = voting_procedures.get_voters();
    assert_eq!(voters.len(), 1);
    assert!(voters.0.contains(&voter));

    let action_ids = voting_procedures.get_governance_action_ids_by_voter(&voter);
    assert_eq!(action_ids.len(), 1);
    assert!(action_ids.0.contains(&action_id));

    let vote_from_tx = voting_procedures.get(&voter, &action_id).unwrap();
    assert_eq!(vote_from_tx, vote);
}

#[test]
fn voting_builder_non_script_voter_error() {
    let mut builder = VotingBuilder::new();
    let key_hash = fake_key_hash(10);
    let voter = Voter::new_drep(&Credential::from_keyhash(&key_hash));
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 1);
    let vote = VotingProcedure::new(VoteKind::No);

    let script_source = NativeScriptSource::new(&NativeScript::new_script_pubkey(
        &ScriptPubkey::new(&key_hash),
    ));
    let result_native = builder.add_with_native_script(&voter, &action_id, &vote, &script_source);
    assert!(result_native.is_err());

    let plutus_witness = PlutusWitness::new_without_datum(
        &create_plutus_script(1, &Language::new_plutus_v2()),
        &Redeemer::new(
            &RedeemerTag::new_cert(),
            &BigNum::zero(),
            &PlutusData::new_empty_constr_plutus_data(&BigNum::zero()),
            &ExUnits::new(&BigNum::zero(), &BigNum::zero()),
        ),
    );
    let result_plutus = builder.add_with_plutus_witness(&voter, &action_id, &vote, &plutus_witness);
    assert!(result_plutus.is_err());
}

#[test]
fn voting_builder_key_hash_error() {
    let mut builder = VotingBuilder::new();
    let voter = Voter::new_drep(&Credential::from_scripthash(&fake_script_hash(1)));
    let action_id = GovernanceActionId::new(&fake_tx_hash(1), 1);
    let vote = VotingProcedure::new(VoteKind::No);

    let result = builder.add(&voter, &action_id, &vote);
    assert!(result.is_err());
}
