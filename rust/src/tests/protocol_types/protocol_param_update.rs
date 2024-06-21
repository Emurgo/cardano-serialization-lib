use crate::*;

#[test]
fn ppu_setters_getters_test() {
    let mut ppu = ProtocolParamUpdate::new();

    assert!(ppu.max_tx_size().is_none());
    let max_tx_size = 1234;
    ppu.set_max_tx_size(max_tx_size);
    assert_eq!(ppu.max_tx_size().unwrap(), max_tx_size);

    assert!(ppu.max_block_body_size().is_none());
    let max_block_body_size = 5678;
    ppu.set_max_block_body_size(max_block_body_size);
    assert_eq!(ppu.max_block_body_size().unwrap(), max_block_body_size);

    assert!(ppu.max_block_header_size().is_none());
    let max_block_header_size = 91011;
    ppu.set_max_block_header_size(max_block_header_size);
    assert_eq!(ppu.max_block_header_size().unwrap(), max_block_header_size);

    assert!(ppu.minfee_a().is_none());
    let minfee_a = Coin::from(1u32);
    ppu.set_minfee_a(&minfee_a);
    assert_eq!(ppu.minfee_a().unwrap(), minfee_a);

    assert!(ppu.minfee_b().is_none());
    let minfee_b = Coin::from(2u32);
    ppu.set_minfee_b(&minfee_b);
    assert_eq!(ppu.minfee_b().unwrap(), minfee_b);

    assert!(ppu.key_deposit().is_none());
    let key_deposit = Coin::from(3u32);
    ppu.set_key_deposit(&key_deposit);
    assert_eq!(ppu.key_deposit().unwrap(), key_deposit);

    assert!(ppu.pool_deposit().is_none());
    let pool_deposit = Coin::from(4u32);
    ppu.set_pool_deposit(&pool_deposit);
    assert_eq!(ppu.pool_deposit().unwrap(), pool_deposit);

    assert!(ppu.max_epoch().is_none());
    let max_epoch = 5;
    ppu.set_max_epoch(max_epoch);
    assert_eq!(ppu.max_epoch().unwrap(), max_epoch);

    assert!(ppu.n_opt().is_none());
    let n_opt = 6;
    ppu.set_n_opt(n_opt);
    assert_eq!(ppu.n_opt().unwrap(), n_opt);

    assert!(ppu.pool_pledge_influence().is_none());
    let pool_pledge_influence = UnitInterval::new(&BigNum::from(7u32), &BigNum::from(77u32));
    ppu.set_pool_pledge_influence(&pool_pledge_influence);
    assert_eq!(ppu.pool_pledge_influence().unwrap(), pool_pledge_influence);

    assert!(ppu.expansion_rate().is_none());
    let expansion_rate = UnitInterval::new(&BigNum::from(8u32), &BigNum::from(9u32));
    ppu.set_expansion_rate(&expansion_rate);
    assert_eq!(ppu.expansion_rate().unwrap(), expansion_rate);

    assert!(ppu.treasury_growth_rate().is_none());
    let treasury_growth_rate = UnitInterval::new(&BigNum::from(10u32), &BigNum::from(11u32));
    ppu.set_treasury_growth_rate(&treasury_growth_rate);
    assert_eq!(ppu.treasury_growth_rate().unwrap(), treasury_growth_rate);

    assert!(ppu.protocol_version().is_none());
    let protocol_version = ProtocolVersion::new(12u32, 13u32);
    ppu.set_protocol_version(&protocol_version);
    assert_eq!(ppu.protocol_version().unwrap(), protocol_version);

    assert!(ppu.min_pool_cost().is_none());
    let min_pool_cost = Coin::from(14u32);
    ppu.set_min_pool_cost(&min_pool_cost);
    assert_eq!(ppu.min_pool_cost().unwrap(), min_pool_cost);

    assert!(ppu.ada_per_utxo_byte().is_none());
    let ada_per_utxo_byte = Coin::from(15u32);
    ppu.set_ada_per_utxo_byte(&ada_per_utxo_byte);
    assert_eq!(ppu.ada_per_utxo_byte().unwrap(), ada_per_utxo_byte);

    assert!(ppu.cost_models().is_none());
    let cost_models = TxBuilderConstants::plutus_vasil_cost_models();
    ppu.set_cost_models(&cost_models);
    assert_eq!(ppu.cost_models().unwrap(), cost_models);

    assert!(ppu.execution_costs().is_none());
    let execution_costs = ExUnitPrices::new(
        &SubCoin::new(&BigNum::from(16u32), &BigNum::from(17u32)),
        &SubCoin::new(&BigNum::from(18u32), &BigNum::from(19u32)),
    );
    ppu.set_execution_costs(&execution_costs);
    assert_eq!(ppu.execution_costs().unwrap(), execution_costs);

    assert!(ppu.max_tx_ex_units().is_none());
    let max_tx_ex_units = ExUnits::new(&BigNum::from(20u32), &BigNum::from(21u32));
    ppu.set_max_tx_ex_units(&max_tx_ex_units);
    assert_eq!(ppu.max_tx_ex_units().unwrap(), max_tx_ex_units);

    assert!(ppu.max_block_ex_units().is_none());
    let max_block_ex_units = ExUnits::new(&BigNum::from(22u32), &BigNum::from(23u32));
    ppu.set_max_block_ex_units(&max_block_ex_units);
    assert_eq!(ppu.max_block_ex_units().unwrap(), max_block_ex_units);

    assert!(ppu.max_value_size().is_none());
    let max_value_size = 24;
    ppu.set_max_value_size(max_value_size);
    assert_eq!(ppu.max_value_size().unwrap(), max_value_size);

    assert!(ppu.collateral_percentage().is_none());
    let collateral_percentage = 25;
    ppu.set_collateral_percentage(collateral_percentage);
    assert_eq!(ppu.collateral_percentage().unwrap(), collateral_percentage);

    assert!(ppu.max_collateral_inputs().is_none());
    let max_collateral_inputs = 25;
    ppu.set_max_collateral_inputs(max_collateral_inputs);
    assert_eq!(ppu.max_collateral_inputs().unwrap(), max_collateral_inputs);

    assert!(ppu.pool_voting_thresholds().is_none());
    let pool_voting_thresholds = PoolVotingThresholds::new(
        &UnitInterval::new(&BigNum::from(26u32), &BigNum::from(27u32)),
        &UnitInterval::new(&BigNum::from(28u32), &BigNum::from(29u32)),
        &UnitInterval::new(&BigNum::from(30u32), &BigNum::from(31u32)),
        &UnitInterval::new(&BigNum::from(40u32), &BigNum::from(41u32)),
        &UnitInterval::new(&BigNum::from(50u32), &BigNum::from(51u32)),
    );
    ppu.set_pool_voting_thresholds(&pool_voting_thresholds);
    assert_eq!(ppu.pool_voting_thresholds().unwrap(), pool_voting_thresholds);

    assert!(ppu.drep_voting_thresholds().is_none());
    let drep_voting_thresholds = DrepVotingThresholds::new(
        &UnitInterval::new(&BigNum::from(26u32), &BigNum::from(27u32)),
        &UnitInterval::new(&BigNum::from(28u32), &BigNum::from(29u32)),
        &UnitInterval::new(&BigNum::from(30u32), &BigNum::from(31u32)),
        &UnitInterval::new(&BigNum::from(40u32), &BigNum::from(41u32)),
        &UnitInterval::new(&BigNum::from(50u32), &BigNum::from(51u32)),
        &UnitInterval::new(&BigNum::from(60u32), &BigNum::from(61u32)),
        &UnitInterval::new(&BigNum::from(66u32), &BigNum::from(65u32)),
        &UnitInterval::new(&BigNum::from(70u32), &BigNum::from(71u32)),
        &UnitInterval::new(&BigNum::from(77u32), &BigNum::from(75u32)),
        &UnitInterval::new(&BigNum::from(80u32), &BigNum::from(81u32)),
    );
    ppu.set_drep_voting_thresholds(&drep_voting_thresholds);
    assert_eq!(ppu.drep_voting_thresholds().unwrap(), drep_voting_thresholds);

    assert!(ppu.min_committee_size().is_none());
    let min_committee_size = 32;
    ppu.set_min_committee_size(min_committee_size);
    assert_eq!(ppu.min_committee_size().unwrap(), min_committee_size);

    assert!(ppu.committee_term_limit().is_none());
    let committee_term_limit = 33;
    ppu.set_committee_term_limit(committee_term_limit);
    assert_eq!(ppu.committee_term_limit().unwrap(), committee_term_limit);

    assert!(ppu.governance_action_validity_period().is_none());
    let governance_action_validity_period = 34;
    ppu.set_governance_action_validity_period(governance_action_validity_period);
    assert_eq!(ppu.governance_action_validity_period().unwrap(), governance_action_validity_period);

    assert!(ppu.governance_action_deposit().is_none());
    let governance_action_deposit = Coin::from(35u32);
    ppu.set_governance_action_deposit(&governance_action_deposit);
    assert_eq!(ppu.governance_action_deposit().unwrap(), governance_action_deposit);

    assert!(ppu.drep_deposit().is_none());
    let drep_deposit = Coin::from(36u32);
    ppu.set_drep_deposit(&drep_deposit);
    assert_eq!(ppu.drep_deposit().unwrap(), drep_deposit);

    assert!(ppu.drep_inactivity_period().is_none());
    let drep_inactivity_period = 37;
    ppu.set_drep_inactivity_period(drep_inactivity_period);
    assert_eq!(ppu.drep_inactivity_period().unwrap(), drep_inactivity_period);

    assert!(ppu.ref_script_coins_per_byte().is_none());
    let ref_script_coins_per_byte = UnitInterval::new(&BigNum::from(38u32), &BigNum::from(39u32));
    ppu.set_ref_script_coins_per_byte(&ref_script_coins_per_byte);
    assert_eq!(ppu.ref_script_coins_per_byte().unwrap(), ref_script_coins_per_byte);

    //since it is deprecated
    assert!(ppu.d().is_none());
}

#[test]
fn pool_voting_thresholds_test() {
    // Creating unit intervals for testing
    let motion_no_confidence = UnitInterval::new(&BigNum::from(1u32), &BigNum::from(100u32));
    let committee_normal = UnitInterval::new(&BigNum::from(2u32), &BigNum::from(100u32));
    let committee_no_confidence = UnitInterval::new(&BigNum::from(3u32), &BigNum::from(100u32));
    let hard_fork_initiation = UnitInterval::new(&BigNum::from(4u32), &BigNum::from(100u32));
    let security_relevant_threshold = UnitInterval::new(&BigNum::from(5u32), &BigNum::from(100u32));

    // Creating a new PoolVotingThresholds instance
    let pvt = PoolVotingThresholds::new(
        &motion_no_confidence,
        &committee_normal,
        &committee_no_confidence,
        &hard_fork_initiation,
        &security_relevant_threshold,
    );

    // Asserting that the getters return the expected values
    assert_eq!(pvt.motion_no_confidence(), motion_no_confidence);
    assert_eq!(pvt.committee_normal(), committee_normal);
    assert_eq!(pvt.committee_no_confidence(), committee_no_confidence);
    assert_eq!(pvt.hard_fork_initiation(), hard_fork_initiation);
    assert_eq!(pvt.security_relevant_threshold(), security_relevant_threshold);
}

#[test]
fn drep_voting_thresholds_test() {
    // Creating unit intervals for testing
    let motion_no_confidence = UnitInterval::new(&BigNum::from(1u32), &BigNum::from(100u32));
    let committee_normal = UnitInterval::new(&BigNum::from(2u32), &BigNum::from(100u32));
    let committee_no_confidence = UnitInterval::new(&BigNum::from(3u32), &BigNum::from(100u32));
    let update_constitution = UnitInterval::new(&BigNum::from(4u32), &BigNum::from(100u32));
    let hard_fork_initiation = UnitInterval::new(&BigNum::from(5u32), &BigNum::from(100u32));
    let pp_network_group = UnitInterval::new(&BigNum::from(6u32), &BigNum::from(100u32));
    let pp_economic_group = UnitInterval::new(&BigNum::from(7u32), &BigNum::from(100u32));
    let pp_technical_group = UnitInterval::new(&BigNum::from(8u32), &BigNum::from(100u32));
    let pp_governance_group = UnitInterval::new(&BigNum::from(9u32), &BigNum::from(100u32));
    let treasury_withdrawal = UnitInterval::new(&BigNum::from(10u32), &BigNum::from(100u32));

    // Creating a new DrepVotingThresholds instance
    let dvt = DrepVotingThresholds::new(
        &motion_no_confidence,
        &committee_normal,
        &committee_no_confidence,
        &update_constitution,
        &hard_fork_initiation,
        &pp_network_group,
        &pp_economic_group,
        &pp_technical_group,
        &pp_governance_group,
        &treasury_withdrawal,
    );

    // Asserting that the getters return the expected values
    assert_eq!(dvt.motion_no_confidence(), motion_no_confidence);
    assert_eq!(dvt.committee_normal(), committee_normal);
    assert_eq!(dvt.committee_no_confidence(), committee_no_confidence);
    assert_eq!(dvt.update_constitution(), update_constitution);
    assert_eq!(dvt.hard_fork_initiation(), hard_fork_initiation);
    assert_eq!(dvt.pp_network_group(), pp_network_group);
    assert_eq!(dvt.pp_economic_group(), pp_economic_group);
    assert_eq!(dvt.pp_technical_group(), pp_technical_group);
    assert_eq!(dvt.pp_governance_group(), pp_governance_group);
    assert_eq!(dvt.treasury_withdrawal(), treasury_withdrawal);
}