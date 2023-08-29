use crate::*;

pub(crate) fn crate_full_protocol_param_update() -> ProtocolParamUpdate {
    ProtocolParamUpdate {
        minfee_a: Some(Coin::from(44_444u32)),
        minfee_b: Some(Coin::from(44_444u32)),
        max_block_body_size: Some(44_444u32),
        max_tx_size: Some(44_444u32),
        max_block_header_size: Some(44_444u32),
        key_deposit: Some(Coin::from(44_444u32)),
        pool_deposit: Some(Coin::from(44_444u32)),
        max_epoch: Some(44_444u32),
        n_opt: Some(44_444u32),
        pool_pledge_influence: Some(Rational::new(
            &BigNum::from(44_444u32),
            &BigNum::from(44_444u32),
        )),
        expansion_rate: Some(UnitInterval::new(
            &BigNum::from(44_444u32),
            &BigNum::from(44_444u32),
        )),
        treasury_growth_rate: Some(UnitInterval::new(
            &BigNum::from(44_444u32),
            &BigNum::from(44_444u32),
        )),
        d: Some(UnitInterval::new(
            &BigNum::from(44_444u32),
            &BigNum::from(44_444u32),
        )),
        extra_entropy: Some(Nonce::new_identity()),
        protocol_version: Some(ProtocolVersion::new(1, 2)),
        min_pool_cost: Some(Coin::from(44_444u32)),
        ada_per_utxo_byte: Some(Coin::from(44_444u32)),
        cost_models: Some(create_cost_models()),
        execution_costs: Some(ExUnitPrices::new(
            &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
            &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
        )),
        max_tx_ex_units: Some(ExUnits::new(&to_bignum(842996), &to_bignum(246100241))),
        max_block_ex_units: Some(ExUnits::new(&to_bignum(842996), &to_bignum(246100241))),
        max_value_size: Some(44_444u32),
        collateral_percentage: Some(44_444u32),
        max_collateral_inputs: Some(44_444u32),
    }
}

pub(crate) fn create_cost_models() -> Costmdls {
    let mut res = Costmdls::new();
    res.insert(
        &Language::new_plutus_v1(),
        &CostModel::from(vec![
            197209, 0, 1, 1, 396231, 621, 0, 1, 150000, 1000, 0, 1, 150000, 32, 2477736, 29175, 4,
            29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 100, 100,
            29773, 100, 150000, 32, 150000, 32, 150000, 32, 150000, 1000, 0, 1, 150000, 32, 150000,
            1000, 0, 8, 148000, 425507, 118, 0, 1, 1, 150000, 1000, 0, 8, 150000, 112536, 247, 1,
            150000, 10000, 1, 136542, 1326, 1, 1000, 150000, 1000, 1, 150000, 32, 150000, 32,
            150000, 32, 1, 1, 150000, 1, 150000, 4, 103599, 248, 1, 103599, 248, 1, 145276, 1366,
            1, 179690, 497, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000,
            32, 148000, 425507, 118, 0, 1, 1, 61516, 11218, 0, 1, 150000, 32, 148000, 425507, 118,
            0, 1, 1, 148000, 425507, 118, 0, 1, 1, 2477736, 29175, 4, 0, 82363, 4, 150000, 5000, 0,
            1, 150000, 32, 197209, 0, 1, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000,
            32, 150000, 32, 150000, 32, 3345831, 1, 1,
        ]),
    );
    res
}
