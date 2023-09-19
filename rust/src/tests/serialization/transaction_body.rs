use crate::fakes::{
    fake_asset_name, fake_auxiliary_data_hash, fake_base_address, fake_key_hash, fake_policy_id,
    fake_script_data_hash, fake_tx_hash, fake_tx_input,
};
use crate::*;

#[test]
fn transaction_round_trip_test() {
    let input = fake_tx_input(1);
    let output = TransactionOutput::new(&fake_base_address(2), &Value::new(&to_bignum(1_000_001)));
    let inputs = TransactionInputs(vec![input]);
    let outputs = TransactionOutputs(vec![output]);
    let fee = Coin::from(1_000_002u64);
    let mut body = TransactionBody::new_tx_body(&inputs, &outputs, &fee);
    let mut mint = Mint::new();
    let mint_asset =
        MintAssets::new_from_entry(&fake_asset_name(4), &Int::new(&to_bignum(1_000_003u64)))
            .unwrap();
    mint.insert(&fake_policy_id(3), &mint_asset);

    let mut req_signers = RequiredSigners::new();
    req_signers.add(&fake_key_hash(5));

    let mut collateral_inputs = TransactionInputs::new();
    collateral_inputs.add(&fake_tx_input(6));

    let mut ref_inputs = TransactionInputs::new();
    ref_inputs.add(&fake_tx_input(7));

    let mut certs = Certificates::new();
    let stake_registration = StakeRegistration::new(&Credential::from_keyhash(&fake_key_hash(8)));
    certs.add(&Certificate::new_stake_registration(&stake_registration));

    let mut withdrawals = Withdrawals::new();
    withdrawals.insert(
        &RewardAddress::new(
            NetworkInfo::testnet().network_id(),
            &Credential::from_keyhash(&fake_key_hash(9)),
        ),
        &Coin::from(1_000_010u64),
    );

    let mut voting_procedures = VotingProcedures::new();
    let voter = Voter::new_drep(&Credential::from_keyhash(&fake_key_hash(1)));
    let gov_action_id = GovernanceActionId::new(&fake_tx_hash(2), 0);
    let procedure = VotingProcedure::new(VoteKind::Abstain);
    voting_procedures.insert(&voter, &gov_action_id, &procedure);

    let mut voting_proposals = VotingProposals::new();
    let info_proposal = InfoProposal::new();
    let proposal = VotingProposal::new_info_proposal(&info_proposal);
    voting_proposals.add(&proposal);

    body.set_ttl(&to_bignum(1_000_003u64));
    body.set_certs(&certs);
    body.set_withdrawals(&withdrawals);
    body.set_update(&Update::new(&ProposedProtocolParameterUpdates::new(), 1));
    body.set_auxiliary_data_hash(&fake_auxiliary_data_hash(2));
    body.set_validity_start_interval_bignum(&SlotBigNum::from(1_000_004u64));
    body.set_mint(&mint);
    body.set_reference_inputs(&ref_inputs);
    body.set_script_data_hash(&fake_script_data_hash(3));
    body.set_collateral(&collateral_inputs);
    body.set_required_signers(&req_signers);
    body.set_network_id(&NetworkId::testnet());
    body.set_collateral_return(&TransactionOutput::new(
        &fake_base_address(4),
        &Value::new(&to_bignum(1_000_005u64)),
    ));
    body.set_total_collateral(&Coin::from(1_000_006u64));
    body.set_voting_procedures(&voting_procedures);
    body.set_voting_proposals(&voting_proposals);
    body.set_donation(&Coin::from(1_000_007u64));
    body.set_current_treasury_value(&Coin::from(1_000_008u64));

    let body_cbor = body.to_bytes();
    let body_hex_cbor = body.to_hex();
    let body_json = body.to_json().unwrap();

    assert_eq!(TransactionBody::from_bytes(body_cbor).unwrap(), body);
    assert_eq!(TransactionBody::from_hex(&body_hex_cbor).unwrap(), body);
    assert_eq!(TransactionBody::from_json(&body_json).unwrap(), body);
}
