use crate::*;
use crate::tests::fakes::{fake_cost_models, fake_drep_voting_thresholds, fake_pool_voting_thresholds};

#[test]
fn protocol_param_update_ser_round_trip() {
    let pp_update= ProtocolParamUpdate {
        minfee_a: Some(Coin::from(1_444u32)),
        minfee_b: Some(Coin::from(2_444u32)),
        max_block_body_size: Some(3_444u32),
        max_tx_size: Some(4_444u32),
        max_block_header_size: Some(5_444u32),
        key_deposit: Some(Coin::from(6_444u32)),
        pool_deposit: Some(Coin::from(7_444u32)),
        max_epoch: Some(8_444u32),
        n_opt: Some(9_444u32),
        pool_pledge_influence: Some(UnitInterval::new(
            &BigNum::from(10_444u32),
            &BigNum::from(11_444u32),
        )),
        expansion_rate: Some(UnitInterval::new(
            &BigNum::from(12_444u32),
            &BigNum::from(13_444u32),
        )),
        treasury_growth_rate: Some(UnitInterval::new(
            &BigNum::from(14_444u32),
            &BigNum::from(15_444u32),
        )),
        d: Some(UnitInterval::new(
            &BigNum::from(16_444u32),
            &BigNum::from(17_444u32),
        )),
        extra_entropy: Some(Nonce::new_identity()),
        protocol_version: Some(ProtocolVersion::new(1, 2)),
        min_pool_cost: Some(Coin::from(18_444u32)),
        ada_per_utxo_byte: Some(Coin::from(19_444u32)),
        cost_models: Some(fake_cost_models()),
        execution_costs: Some(ExUnitPrices::new(
            &SubCoin::new(&BigNum(577), &BigNum(10000)),
            &SubCoin::new(&BigNum(721), &BigNum(10000000)),
        )),
        max_tx_ex_units: Some(ExUnits::new(&BigNum(842996), &BigNum(246100241))),
        max_block_ex_units: Some(ExUnits::new(&BigNum(842996), &BigNum(246100241))),
        max_value_size: Some(20_444u32),
        collateral_percentage: Some(21_444u32),
        max_collateral_inputs: Some(22_444u32),
        pool_voting_thresholds: Some(fake_pool_voting_thresholds()),
        drep_voting_thresholds: Some(fake_drep_voting_thresholds()),
        min_committee_size: Some(23_444u32),
        committee_term_limit: Some(24_444u32),
        governance_action_validity_period: Some(25_444u32),
        governance_action_deposit: Some(Coin::from(26_444u32)),
        drep_deposit: Some(Coin::from(27_444u32)),
        drep_inactivity_period: Some(28_444u32),
        ref_script_coins_per_byte: Some(UnitInterval::new(
            &BigNum::from(29_444u32),
            &BigNum::from(30_444u32),
        )),
    };

    let cbor = pp_update.to_bytes();
    let hex = pp_update.to_hex();
    let json = pp_update.to_json().unwrap();

    let pp_update_from_cbor = ProtocolParamUpdate::from_bytes(cbor).unwrap();
    let pp_update_from_hex = ProtocolParamUpdate::from_hex(&hex).unwrap();
    let pp_update_from_json = ProtocolParamUpdate::from_json(&json).unwrap();

    assert_eq!(pp_update, pp_update_from_cbor);
    assert_eq!(pp_update, pp_update_from_hex);
    assert_eq!(pp_update, pp_update_from_json);
}

#[test]
fn pool_voting_thresholds_ser_round_trip() {
   let thresholds = PoolVotingThresholds::new(
        &UnitInterval::new(&BigNum::from(44_401u32), &BigNum::from(44_402u32)),
        &UnitInterval::new(&BigNum::from(44_403u32), &BigNum::from(44_404u32)),
        &UnitInterval::new(&BigNum::from(44_405u32), &BigNum::from(44_406u32)),
        &UnitInterval::new(&BigNum::from(44_406u32), &BigNum::from(44_407u32)),
        &UnitInterval::new(&BigNum::from(44_408u32), &BigNum::from(44_409u32)),
    );

    let cbor = thresholds.to_bytes();
    let hex = thresholds.to_hex();
    let json = thresholds.to_json().unwrap();

    let thresholds_from_cbor = PoolVotingThresholds::from_bytes(cbor).unwrap();
    let thresholds_from_hex = PoolVotingThresholds::from_hex(&hex).unwrap();
    let thresholds_from_json = PoolVotingThresholds::from_json(&json).unwrap();

    assert_eq!(thresholds, thresholds_from_cbor);
    assert_eq!(thresholds, thresholds_from_hex);
    assert_eq!(thresholds, thresholds_from_json);
}

#[test]
fn drep_voting_thresholds_ser_round_trip() {
    let thresholds = DrepVotingThresholds::new(
        &UnitInterval::new(&BigNum::from(44_401u32), &BigNum::from(44_402u32)),
        &UnitInterval::new(&BigNum::from(44_403u32), &BigNum::from(44_404u32)),
        &UnitInterval::new(&BigNum::from(44_405u32), &BigNum::from(44_406u32)),
        &UnitInterval::new(&BigNum::from(44_406u32), &BigNum::from(44_407u32)),
        &UnitInterval::new(&BigNum::from(44_408u32), &BigNum::from(44_409u32)),
        &UnitInterval::new(&BigNum::from(44_410u32), &BigNum::from(44_411u32)),
        &UnitInterval::new(&BigNum::from(44_412u32), &BigNum::from(44_412u32)),
        &UnitInterval::new(&BigNum::from(44_414u32), &BigNum::from(44_415u32)),
        &UnitInterval::new(&BigNum::from(44_416u32), &BigNum::from(44_417u32)),
        &UnitInterval::new(&BigNum::from(44_418u32), &BigNum::from(44_419u32)),
    );

    let cbor = thresholds.to_bytes();
    let hex = thresholds.to_hex();
    let json = thresholds.to_json().unwrap();

    let thresholds_from_cbor = DrepVotingThresholds::from_bytes(cbor).unwrap();
    let thresholds_from_hex = DrepVotingThresholds::from_hex(&hex).unwrap();
    let thresholds_from_json = DrepVotingThresholds::from_json(&json).unwrap();

    assert_eq!(thresholds, thresholds_from_cbor);
    assert_eq!(thresholds, thresholds_from_hex);
    assert_eq!(thresholds, thresholds_from_json);
}