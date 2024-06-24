#![allow(dead_code)]

use crate::*;
use crate::tests::helpers::harden;

const MAX_VALUE_SIZE: u32 = 4000;
const MAX_TX_SIZE: u32 = 8000; // might be out of date but suffices for our tests
                               // this is what is used in mainnet
static COINS_PER_UTXO_BYTE: u64 = 34_482 / 8;

pub(crate) fn fake_root_key_15() -> Bip32PrivateKey {
    // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
    let entropy = [
        0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4,
        0xd8, 0x09, 0x7c, 0x64, 0x12,
    ];
    Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
}

pub(crate) fn fake_root_key() -> Bip32PrivateKey {
    // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
    let entropy = [
        0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4,
        0xd8, 0x09, 0x7c, 0x64, 0x12,
    ];
    Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
}

pub(crate) fn fake_base_address_with_payment_cred(payment_cred: Credential) -> Address {
    let stake = fake_root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(2)
        .derive(0)
        .to_public();
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    let addr = BaseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &payment_cred,
        &stake_cred,
    );
    addr.to_address()
}

pub(crate) fn fake_base_address(index: u32) -> Address {
    let spend = fake_root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(index)
        .to_public();
    let stake = fake_root_key()
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

pub(crate) fn fake_full_protocol_param_update() -> ProtocolParamUpdate {
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
        cost_models: Some(fake_cost_models()),
        execution_costs: Some(ExUnitPrices::new(
            &SubCoin::new(&BigNum(577), &BigNum(10000)),
            &SubCoin::new(&BigNum(721), &BigNum(10000000)),
        )),
        max_tx_ex_units: Some(ExUnits::new(&BigNum(842996), &BigNum(246100241))),
        max_block_ex_units: Some(ExUnits::new(&BigNum(842996), &BigNum(246100241))),
        max_value_size: Some(44_444u32),
        collateral_percentage: Some(44_444u32),
        max_collateral_inputs: Some(44_444u32),
        pool_voting_thresholds: Some(fake_pool_voting_thresholds()),
        drep_voting_thresholds: Some(fake_drep_voting_thresholds()),
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

pub(crate) fn fake_pool_voting_thresholds() -> PoolVotingThresholds {
    PoolVotingThresholds::new(
        &UnitInterval::new(&BigNum::from(44_401u32), &BigNum::from(44_402u32)),
        &UnitInterval::new(&BigNum::from(44_403u32), &BigNum::from(44_404u32)),
        &UnitInterval::new(&BigNum::from(44_405u32), &BigNum::from(44_406u32)),
        &UnitInterval::new(&BigNum::from(44_406u32), &BigNum::from(44_407u32)),
        &UnitInterval::new(&BigNum::from(44_408u32), &BigNum::from(44_409u32)),
    )
}

pub(crate) fn fake_drep_voting_thresholds() -> DrepVotingThresholds {
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

pub(crate) fn fake_full_pool_params() -> PoolParams {
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

pub(crate) fn fake_cost_models() -> Costmdls {
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

pub(crate) fn fake_anchor() -> Anchor {
    Anchor::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_anchor_data_hash(1),
    )
}

pub(crate) fn fake_action_id() -> GovernanceActionId {
    GovernanceActionId::new(&fake_tx_hash(1), 1)
}
pub(crate) fn fake_byron_address() -> Address {
    ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
        .unwrap()
        .to_address()
}

pub(crate) fn fake_linear_fee(coefficient: u64, constant: u64) -> LinearFee {
    LinearFee::new(&BigNum(coefficient), &BigNum(constant))
}

pub(crate) fn fake_default_linear_fee() -> LinearFee {
    fake_linear_fee(500, 2)
}

pub(crate) fn fake_tx_builder_full(
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

pub(crate) fn fake_tx_builder(
    linear_fee: &LinearFee,
    coins_per_utxo_byte: u64,
    pool_deposit: u64,
    key_deposit: u64,
) -> TransactionBuilder {
    fake_tx_builder_full(
        linear_fee,
        pool_deposit,
        key_deposit,
        MAX_VALUE_SIZE,
        coins_per_utxo_byte,
    )
}

pub(crate) fn fake_reallistic_tx_builder() -> TransactionBuilder {
    fake_tx_builder(
        &fake_linear_fee(44, 155381),
        COINS_PER_UTXO_BYTE,
        500000000,
        2000000,
    )
}

pub(crate) fn fake_tx_builder_with_fee_and_val_size(
    linear_fee: &LinearFee,
    max_val_size: u32,
) -> TransactionBuilder {
    fake_tx_builder_full(linear_fee, 1, 1, max_val_size, 1)
}

pub(crate) fn fake_tx_builder_with_fee(linear_fee: &LinearFee) -> TransactionBuilder {
    fake_tx_builder(linear_fee, 1, 1, 1)
}

pub(crate) fn fake_tx_builder_with_fee_and_pure_change(
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

pub(crate) fn fake_tx_builder_with_key_deposit(deposit: u64) -> TransactionBuilder {
    fake_tx_builder(&fake_default_linear_fee(), 8, 1, deposit)
}

pub(crate) fn fake_default_tx_builder() -> TransactionBuilder {
    fake_tx_builder_with_fee(&fake_default_linear_fee())
}

pub(crate) fn fake_change_address() -> Address {
    let spend = fake_root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(0)
        .to_public();
    let stake = fake_root_key()
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

pub(crate) fn fake_base_script_address(index: u8) -> Address {
    let stake = fake_root_key()
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

pub(crate) fn fake_enterprise_address(index: u32) -> Address {
    let spend = fake_root_key()
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

pub(crate) fn fake_enterprise_script_address(index: u8) -> Address {
    let spend_cred = Credential::from_scripthash(&fake_script_hash(index));
    let addr = EnterpriseAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
    );
    addr.to_address()
}

pub(crate) fn fake_pointer_address(index: u32) -> Address {
    let spend = fake_root_key()
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

pub(crate) fn fake_pointer_script_address(index: u8) -> Address {
    let spend_cred = Credential::from_scripthash(&fake_script_hash(index));
    let pointer = Pointer::new(1, 2, 3);
    let addr = PointerAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &spend_cred,
        &pointer
    );
    addr.to_address()
}

pub(crate) fn fake_reward_address(index: u32) -> RewardAddress {
    let stake = fake_root_key()
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(1)
        .derive(index)
        .to_public();
    let stake_cred = Credential::from_keyhash(&stake.to_raw_key().hash());
    RewardAddress::new(
        NetworkInfo::testnet_preprod().network_id(),
        &stake_cred,
    )
}

pub(crate) fn fake_malformed_address() -> Address {
    MalformedAddress(vec![255, 255, 255, 255, 255, 255]).to_address()
}

pub(crate) fn fake_rich_tx_builder(with_collateral: bool) -> TransactionBuilder {
    let mut tx_builder = fake_reallistic_tx_builder();
    let input = TransactionInput::new(&fake_tx_hash(1), 0);
    let address = fake_base_address(1);
    let mut input_builder = TxInputsBuilder::new();
    input_builder.add_regular_input(&address, &input, &Value::new(&Coin::from(u64::MAX / 2)))
        .expect("should add input");
    tx_builder.set_inputs(&input_builder);
    if with_collateral {
        tx_builder.set_collateral(&input_builder);
    }

    tx_builder
}

pub(crate) fn fake_tx_builder_with_amount(
    amount: u64,
    with_collateral: bool,
) -> TransactionBuilder {
    let mut tx_builder = fake_reallistic_tx_builder();
    let input = TransactionInput::new(&fake_tx_hash(1), 0);
    let address = fake_base_address(1);
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

pub(crate) fn fake_tx_builder_with_amount_and_deposit_params(
    amount: u64,
    pool_deposit: u64,
    key_deposit: u64,
    with_collateral: bool,
) -> TransactionBuilder {
    let mut tx_builder = fake_tx_builder(
        &fake_linear_fee(44, 155381),
        COINS_PER_UTXO_BYTE,
        pool_deposit,
        key_deposit
    );
    let input = TransactionInput::new(&fake_tx_hash(1), 0);
    let address = fake_base_address(1);
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

pub(crate) fn fake_plutus_script(x: u8, lang: &Language) -> PlutusScript {
    let mut bytes = hex::decode("4e4d01000033222220051200120011").unwrap();
    let pos = bytes.len() - 1;
    bytes[pos] = x;
    PlutusScript::from_bytes_with_version(bytes, lang).unwrap()
}

pub(crate) fn fake_redeemer(x: u8) -> Redeemer {
    Redeemer::new(
        &RedeemerTag::new_cert(),
        &BigNum::from(x as u64),
        &PlutusData::new_empty_constr_plutus_data(&BigNum::from(x)),
        &ExUnits::new(&BigNum::from(x), &BigNum::from(x)),
    )
}

pub(crate) fn fake_redeemer_zero_cost(x: u8) -> Redeemer {
    Redeemer::new(
        &RedeemerTag::new_cert(),
        &BigNum::from(x as u64),
        &PlutusData::new_empty_constr_plutus_data(&BigNum::from(x)),
        &ExUnits::new(&BigNum::zero(), &BigNum::zero()),
    )
}


pub(crate) fn fake_bytes_32(x: u8) -> Vec<u8> {
    vec![
        x, 239, 181, 120, 142, 135, 19, 200, 68, 223, 211, 43, 46, 145, 222, 30, 48, 159, 239, 255,
        213, 85, 248, 39, 204, 158, 225, 100, 1, 2, 3, 4,
    ]
}

pub(crate) fn fake_data_hash(x: u8) -> DataHash {
    DataHash::from_bytes(fake_bytes_32(x)).unwrap()
}

pub(crate) fn fake_anchor_data_hash(x: u8) -> AnchorDataHash {
    AnchorDataHash::from_bytes(fake_bytes_32(x)).unwrap()
}

pub(crate) fn fake_auxiliary_data_hash(x: u8) -> AuxiliaryDataHash {
    AuxiliaryDataHash::from_bytes(fake_bytes_32(x)).unwrap()
}

pub(crate) fn fake_pool_metadata_hash(x: u8) -> PoolMetadataHash {
    PoolMetadataHash::from_bytes(fake_bytes_32(x)).unwrap()
}

pub(crate) fn fake_genesis_hash(x: u8) -> GenesisHash {
    GenesisHash::from_bytes((&fake_bytes_32(x)[0..28]).to_vec()).unwrap()
}

pub(crate) fn fake_genesis_delegate_hash(x: u8) -> GenesisDelegateHash {
    GenesisDelegateHash::from_bytes((&fake_bytes_32(x)[0..28]).to_vec()).unwrap()
}

pub(crate) fn fake_vrf_key_hash(x: u8) -> VRFKeyHash {
    VRFKeyHash::from_bytes(fake_bytes_32(x)).unwrap()
}

pub(crate) fn fake_key_hash(x: u8) -> Ed25519KeyHash {
    Ed25519KeyHash::from_bytes((&fake_bytes_32(x)[0..28]).to_vec()).unwrap()
}

pub(crate) fn fake_script_hash(x: u8) -> ScriptHash {
    ScriptHash::from_bytes((&fake_bytes_32(x)[0..28]).to_vec()).unwrap()
}

pub(crate) fn fake_script_data_hash(x: u8) -> ScriptDataHash {
    ScriptDataHash::from_bytes(fake_bytes_32(x)).unwrap()
}

pub(crate) fn fake_tx_hash(input_hash_byte: u8) -> TransactionHash {
    TransactionHash::from([input_hash_byte; 32])
}

pub(crate) fn fake_tx_input(input_hash_byte: u8) -> TransactionInput {
    fake_tx_input2(input_hash_byte, 0)
}

pub(crate) fn fake_tx_input2(input_hash_byte: u8, idx: TransactionIndex) -> TransactionInput {
    TransactionInput::new(&fake_tx_hash(input_hash_byte), idx)
}

pub(crate) fn fake_value() -> Value {
    fake_value2(1_000_000)
}

pub(crate) fn fake_value2(v: u64) -> Value {
    Value::new(&BigNum(v))
}

pub(crate) fn fake_tx_output(input_hash_byte: u8) -> TransactionOutput {
    TransactionOutput::new(&fake_base_address(input_hash_byte as u32), &fake_value())
}

pub(crate) fn fake_tx_output2(input_hash_byte: u8, val: u64) -> TransactionOutput {
    TransactionOutput::new(&fake_base_address(input_hash_byte as u32), &fake_value2(val))
}

pub(crate) fn fake_vkey() -> Vkey {
    Vkey::new(
        &Bip32PrivateKey::generate_ed25519_bip32()
            .unwrap()
            .to_public()
            .to_raw_key(),
    )
}

pub(crate) fn fake_vkey_numbered(x: u8) -> Vkey {
    Vkey::new(&PublicKey::from_bytes(&[x; 32]).unwrap())
}

pub(crate) fn fake_signature(x: u8) -> Ed25519Signature {
    Ed25519Signature::from_bytes([x; 64].to_vec()).unwrap()
}

pub(crate) fn fake_policy_id(x: u8) -> PolicyID {
    PolicyID::from([x; 28])
}

pub(crate) fn fake_asset_name(x: u8) -> AssetName {
    AssetName([x; 32].to_vec())
}

pub(crate) fn fake_vkey_witness(x: u8) -> Vkeywitness {
    Vkeywitness::new(&fake_vkey_numbered(x), &fake_signature(x))
}

pub(crate) fn fake_boostrap_witness(x: u8) -> BootstrapWitness {
    BootstrapWitness::new(
        &fake_vkey_numbered(x),
        &fake_signature(x),
        vec![x; 32],
        vec![x; 32],
    )
}

pub(crate) fn fake_plutus_script_and_hash(x: u8) -> (PlutusScript, ScriptHash) {
    let s = PlutusScript::new(fake_bytes_32(x));
    (s.clone(), s.hash())
}