use crate::fakes::{fake_anchor_data_hash, fake_key_hash, fake_pool_metadata_hash, fake_script_hash, fake_tx_hash, fake_vrf_key_hash};
use crate::fees::LinearFee;
use crate::tests::helpers::harden;
use crate::*;

const MAX_VALUE_SIZE: u32 = 4000;
const MAX_TX_SIZE: u32 = 8000; // might be out of date but suffices for our tests
                               // this is what is used in mainnet
static COINS_PER_UTXO_BYTE: u64 = 34_482 / 8;

pub(crate) fn root_key_15() -> Bip32PrivateKey {
    // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
    let entropy = [
        0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4,
        0xd8, 0x09, 0x7c, 0x64, 0x12,
    ];
    Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
}

pub(crate) fn root_key() -> Bip32PrivateKey {
    // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
    let entropy = [
        0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4,
        0xd8, 0x09, 0x7c, 0x64, 0x12,
    ];
    Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
}

pub(crate) fn generate_address(index: u32) -> Address {
    let spend = root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(index)
        .to_public();
    let stake = root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    );
    addr.to_address()
}

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
        pool_pledge_influence: Some(UnitInterval::new(
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
            &SubCoin::new(&BigNum(577), &BigNum(10000)),
            &SubCoin::new(&BigNum(721), &BigNum(10000000)),
        )),
        max_tx_ex_units: Some(ExUnits::new(&BigNum(842996), &BigNum(246100241))),
        max_block_ex_units: Some(ExUnits::new(&BigNum(842996), &BigNum(246100241))),
        max_value_size: Some(44_444u32),
        collateral_percentage: Some(44_444u32),
        max_collateral_inputs: Some(44_444u32),
        pool_voting_thresholds: Some(create_pool_voting_thresholds()),
        drep_voting_thresholds: Some(create_drep_voting_thresholds()),
        min_committee_size: Some(44_444u32),
        committee_term_limit: Some(44_444u32),
        governance_action_validity_period: Some(44_444u32),
        governance_action_deposit: Some(Coin::from(44_444u32)),
        drep_deposit: Some(Coin::from(44_444u32)),
        drep_inactivity_period: Some(44_444u32),
        ref_script_coins_per_byte: Some(UnitInterval::new(
            &BigNum::from(44_444u32),
            &BigNum::from(44_444u32),
        )),
    }
}

pub(crate) fn create_pool_voting_thresholds() -> PoolVotingThresholds {
    PoolVotingThresholds::new(
        &UnitInterval::new(&BigNum::from(44_401u32), &BigNum::from(44_402u32)),
        &UnitInterval::new(&BigNum::from(44_403u32), &BigNum::from(44_404u32)),
        &UnitInterval::new(&BigNum::from(44_405u32), &BigNum::from(44_406u32)),
        &UnitInterval::new(&BigNum::from(44_406u32), &BigNum::from(44_407u32)),
        &UnitInterval::new(&BigNum::from(44_408u32), &BigNum::from(44_409u32)),
    )
}

pub(crate) fn create_drep_voting_thresholds() -> DrepVotingThresholds {
    DrepVotingThresholds::new(
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
    )
}

pub(crate) fn crate_full_pool_params() -> PoolParams {
    PoolParams {
        operator: fake_key_hash(1),
        vrf_keyhash: fake_vrf_key_hash(2),
        pledge: Coin::from(44_444u32),
        cost: Coin::from(44_444u32),
        margin: UnitInterval::new(&BigNum::from(44_444u32), &BigNum::from(44_444u32)),
        reward_account: RewardAddress::new(2, &Credential::from_keyhash(&fake_key_hash(3))),
        pool_owners: Ed25519KeyHashes::from_vec(vec![fake_key_hash(4), fake_key_hash(5)].into_iter().collect()),
        relays: Relays(vec![Relay::new_multi_host_name(&MultiHostName::new(
            &DNSRecordSRV::new("iohk.io".to_string()).unwrap(),
        ))]),
        pool_metadata: Some(PoolMetadata::new(
            &URL::new("https://iohk.io".to_string()).unwrap(),
            &fake_pool_metadata_hash(6),
        )),
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

pub(crate) fn create_anchor() -> Anchor {
    Anchor::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_anchor_data_hash(1),
    )
}

pub(crate) fn create_action_id() -> GovernanceActionId {
    GovernanceActionId::new(&fake_tx_hash(1), 1)
}
pub(crate) fn byron_address() -> Address {
    ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
        .unwrap()
        .to_address()
}

pub(crate) fn create_linear_fee(coefficient: u64, constant: u64) -> LinearFee {
    LinearFee::new(&BigNum(coefficient), &BigNum(constant))
}

pub(crate) fn create_default_linear_fee() -> LinearFee {
    create_linear_fee(500, 2)
}

pub(crate) fn create_tx_builder_full(
    linear_fee: &LinearFee,
    pool_deposit: u64,
    key_deposit: u64,
    max_val_size: u32,
    coins_per_utxo_byte: u64,
) -> TransactionBuilder {
    let cfg = TransactionBuilderConfigBuilder::new()
        .fee_algo(linear_fee)
        .pool_deposit(&BigNum(pool_deposit))
        .key_deposit(&BigNum(key_deposit))
        .max_value_size(max_val_size)
        .max_tx_size(MAX_TX_SIZE)
        .coins_per_utxo_byte(&BigNum(coins_per_utxo_byte))
        .ex_unit_prices(&ExUnitPrices::new(
            &SubCoin::new(&BigNum(577), &BigNum(10000)),
            &SubCoin::new(&BigNum(721), &BigNum(10000000)),
        ))
        .ref_script_coins_per_byte(
            &UnitInterval::new(&BigNum(1), &BigNum(2)),
        )
        .build()
        .unwrap();
    TransactionBuilder::new(&cfg)
}

pub(crate) fn create_tx_builder(
    linear_fee: &LinearFee,
    coins_per_utxo_byte: u64,
    pool_deposit: u64,
    key_deposit: u64,
) -> TransactionBuilder {
    create_tx_builder_full(
        linear_fee,
        pool_deposit,
        key_deposit,
        MAX_VALUE_SIZE,
        coins_per_utxo_byte,
    )
}

pub(crate) fn create_reallistic_tx_builder() -> TransactionBuilder {
    create_tx_builder(
        &create_linear_fee(44, 155381),
        COINS_PER_UTXO_BYTE,
        500000000,
        2000000,
    )
}

pub(crate) fn create_tx_builder_with_fee_and_val_size(
    linear_fee: &LinearFee,
    max_val_size: u32,
) -> TransactionBuilder {
    create_tx_builder_full(linear_fee, 1, 1, max_val_size, 1)
}

pub(crate) fn create_tx_builder_with_fee(linear_fee: &LinearFee) -> TransactionBuilder {
    create_tx_builder(linear_fee, 1, 1, 1)
}

pub(crate) fn create_tx_builder_with_fee_and_pure_change(
    linear_fee: &LinearFee,
) -> TransactionBuilder {
    TransactionBuilder::new(
        &TransactionBuilderConfigBuilder::new()
            .fee_algo(linear_fee)
            .pool_deposit(&BigNum(1))
            .key_deposit(&BigNum(1))
            .max_value_size(MAX_VALUE_SIZE)
            .max_tx_size(MAX_TX_SIZE)
            .coins_per_utxo_byte(&BigNum(1))
            .prefer_pure_change(true)
            .build()
            .unwrap(),
    )
}

pub(crate) fn create_tx_builder_with_key_deposit(deposit: u64) -> TransactionBuilder {
    create_tx_builder(&create_default_linear_fee(), 8, 1, deposit)
}

pub(crate) fn create_default_tx_builder() -> TransactionBuilder {
    create_tx_builder_with_fee(&create_default_linear_fee())
}

pub(crate) fn create_change_address() -> Address {
    let spend = root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    );
    addr.to_address()
}

pub(crate) fn create_base_address(index: u32) -> Address {
    let spend = root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(index)
        .to_public();
    let stake = root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    );
    addr.to_address()
}

pub(crate) fn create_base_script_address(index: u8) -> Address {
    let stake = root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();
    let spend_cred = Credential::from_scripthash(&fake_script_hash(index));
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &stake_cred,
    );
    addr.to_address()
}

pub(crate) fn create_enterprise_address(index: u32) -> Address {
    let spend = root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(index)
        .to_public();
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let addr = EnterpriseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
    );
    addr.to_address()
}

pub(crate) fn create_enterprise_script_address(index: u8) -> Address {
    let spend_cred = Credential::from_scripthash(&fake_script_hash(index));
    let addr = EnterpriseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
    );
    addr.to_address()
}

pub(crate) fn create_pointer_address(index: u32) -> Address {
    let spend = root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(index)
        .to_public();
    let spend_cred = Credential::from_keyhash(&spend.to_raw_key().hash());
    let pointer = Pointer::new(1, 2, 3);
    let addr = PointerAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &pointer
    );
    addr.to_address()
}

pub(crate) fn create_pointer_script_address(index: u8) -> Address {
    let spend_cred = Credential::from_scripthash(&fake_script_hash(index));
    let pointer = Pointer::new(1, 2, 3);
    let addr = PointerAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &pointer
    );
    addr.to_address()
}

pub(crate) fn create_reward_address(index: u32) -> Address {
    let stake = root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(index)
        .to_public();
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr = RewardAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &stake_cred,
    );
    addr.to_address()
}

pub(crate) fn create_malformed_address() -> Address {
    MalformedAddress(vec![255, 255, 255, 255, 255, 255]).to_address()
}

pub(crate) fn create_rich_tx_builder(with_collateral: bool) -> TransactionBuilder {
    let mut tx_builder = create_reallistic_tx_builder();
    let input = TransactionInput::new(&fake_tx_hash(1), 0);
    let address = generate_address(1);
    let mut input_builder = TxInputsBuilder::new();
    input_builder.add_regular_input(&address, &input, &Value::new(&Coin::from(u64::MAX / 2)))
        .expect("should add input");
    tx_builder.set_inputs(&input_builder);
    if with_collateral {
        tx_builder.set_collateral(&input_builder);
    }

    tx_builder
}

pub(crate) fn create_tx_builder_with_amount(
    amount: u64,
    with_collateral: bool,
) -> TransactionBuilder {
    let mut tx_builder = create_reallistic_tx_builder();
    let input = TransactionInput::new(&fake_tx_hash(1), 0);
    let address = generate_address(1);
    let mut input_builder = TxInputsBuilder::new();
    input_builder.add_regular_input(&address, &input, &Value::new(&Coin::from(amount))).expect("should add input");
    tx_builder.set_inputs(&input_builder);
    if with_collateral {
        let col_input = TransactionInput::new(&fake_tx_hash(1), 0);
        let mut col_input_builder = TxInputsBuilder::new();
        col_input_builder.add_regular_input(&address, &col_input, &Value::new(&Coin::from(u64::MAX / 2))).expect("should add input");
        tx_builder.set_collateral(&col_input_builder);
    }

    tx_builder
}

pub(crate) fn create_tx_builder_with_amount_and_deposit_params(
    amount: u64,
    pool_deposit: u64,
    key_deposit: u64,
    with_collateral: bool,
) -> TransactionBuilder {
    let mut tx_builder = create_tx_builder(
        &create_linear_fee(44, 155381),
        COINS_PER_UTXO_BYTE,
        pool_deposit,
        key_deposit
    );
    let input = TransactionInput::new(&fake_tx_hash(1), 0);
    let address = generate_address(1);
    let mut input_builder = TxInputsBuilder::new();
    input_builder.add_regular_input(&address, &input, &Value::new(&Coin::from(amount))).expect("should add input");
    tx_builder.set_inputs(&input_builder);
    if with_collateral {
        let col_input = TransactionInput::new(&fake_tx_hash(1), 0);
        let mut col_input_builder = TxInputsBuilder::new();
        col_input_builder.add_regular_input(&address, &col_input, &Value::new(&Coin::from(u64::MAX / 2))).expect("should add input");
        tx_builder.set_collateral(&col_input_builder);
    }

    tx_builder
}

pub(crate) fn create_plutus_script(x: u8, lang: &Language) -> PlutusScript {
    let mut bytes = hex::decode("4e4d01000033222220051200120011").unwrap();
    let pos = bytes.len() - 1;
    bytes[pos] = x;
    PlutusScript::from_bytes_with_version(bytes, lang).unwrap()
}

pub(crate) fn create_redeemer(x: u8) -> Redeemer {
    Redeemer::new(
        &RedeemerTag::new_cert(),
        &BigNum::from(x as u64),
        &PlutusData::new_empty_constr_plutus_data(&BigNum::from(x)),
        &ExUnits::new(&BigNum::from(x), &BigNum::from(x)),
    )
}

pub(crate) fn create_redeemer_zero_cost(x: u8) -> Redeemer {
    Redeemer::new(
        &RedeemerTag::new_cert(),
        &BigNum::from(x as u64),
        &PlutusData::new_empty_constr_plutus_data(&BigNum::from(x)),
        &ExUnits::new(&BigNum::zero(), &BigNum::zero()),
    )
}