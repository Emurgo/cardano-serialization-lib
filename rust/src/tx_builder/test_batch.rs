#[cfg(test)]
mod test {
    use crate::*;
    use crate::fakes::fake_policy_id;
    use crate::fees::{LinearFee, min_fee};
    use crate::tx_builder::{TransactionBuilderConfig, TransactionBuilderConfigBuilder};
    use crate::tx_builder::tx_batch_builder::{create_send_all, TransactionBatchList};

    fn root_key() -> Bip32PrivateKey {
        // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
        let entropy = [
            0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22,
            0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12,
        ];
        Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
    }

    fn harden(index: u32) -> u32 {
        index | 0x80_00_00_00
    }

    fn generate_address(index: u32) -> Address {
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
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        );
        addr.to_address()
    }

    fn generate_assets(from_policy: usize,
                       from_asset: usize,
                       policies_count: usize,
                       assets_count: usize,
                       asset_name_size: usize,
                       amount_per_asset: Coin) -> Option<MultiAsset> {
        let mut assets = MultiAsset::new();
        for i in from_policy..(policies_count + from_policy) {
            let policy_id = fake_policy_id(i as u8);
            for j in from_asset..(assets_count + from_asset) {
                let asset_name = AssetName::new(vec![j as u8; asset_name_size]).unwrap();
                assets.set_asset(&policy_id, &asset_name, amount_per_asset);
            }
        }

        if assets.0.is_empty() {
            None
        } else {
            Some(assets)
        }
    }

    fn generate_utxo(address: &Address,
                     index: u32,
                     from_policy: usize,
                     from_asset: usize,
                     policies: usize,
                     assets: usize,
                     asset_name_size: usize,
                     amount_per_asset: Coin,
                     ada: Coin) -> TransactionUnspentOutput {
        let input = TransactionInput::new(
            &TransactionHash::from_bytes(
                hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7").unwrap())
                .unwrap(), index);
        let assets = generate_assets(from_policy, from_asset, policies, assets, asset_name_size, amount_per_asset);
        let value = match assets {
            Some(assets) => Value::new_with_assets(&ada, &assets),
            None => Value::new(&ada),
        };
        let output = TransactionOutput::new(&address, &value);
        TransactionUnspentOutput::new(&input, &output)
    }

    fn generate_big_ada_utoxs_bacth() -> TransactionUnspentOutputs {
        let mut utxos = Vec::new();
        let address = generate_address(1);
        let address_2 = generate_address(2);
        for i in 0..10 {
            let utxo = generate_utxo(
                &address,
                i,
                (i / 2) as usize,
                (i / 2) as usize,
                5,
                5,
                20,
                Coin::from(500u64),
                Coin::from(5000u64));
            utxos.push(utxo);
        }

        for i in 0..10000 {
            let utxo = generate_utxo(
                &address_2,
                i + 1000,
                0,
                0,
                0,
                0,
                20,
                Coin::zero(),
                Coin::from(5000000u64));
            utxos.push(utxo);
        }
        TransactionUnspentOutputs(utxos)
    }

    fn generate_big_utoxs_bacth() -> TransactionUnspentOutputs {
        let mut utxos = Vec::new();
        let address = generate_address(1);
        let address_2 = generate_address(2);
        for i in 0..200 {
            let utxo = generate_utxo(
                &address,
                i,
                (i / 2) as usize,
                (i / 2) as usize,
                5,
                5,
                20,
                Coin::from(500u64),
                Coin::from(5000u64));
            utxos.push(utxo);
        }

        for i in 0..10 {
            let utxo = generate_utxo(
                &address_2,
                i + 1000,
                0,
                0,
                0,
                0,
                20,
                Coin::zero(),
                Coin::from(50000000u64));
            utxos.push(utxo);
        }
        TransactionUnspentOutputs(utxos)
    }

    fn get_utxos_total(utxos: &TransactionUnspentOutputs) -> Value {
        let mut total_value = Value::zero();
        for utxo in utxos {
            total_value = total_value.checked_add(&utxo.output.amount).unwrap();
        }

        total_value
    }

    fn get_batch_total(batches: &TransactionBatchList) -> Value {
        let mut total_value = Value::zero();
        for batch in batches {
            for tx in batch {
                for output in &tx.body.outputs {
                    total_value = total_value.checked_add(&output.amount).unwrap();
                }
                let fee = Value::new(&tx.body.fee);
                total_value = total_value.checked_add(&fee).unwrap();
            }
        }

        total_value
    }

    fn get_tx_fee(tx: &Transaction, cfg: &TransactionBuilderConfig) -> Coin {
        min_fee(tx, &cfg.fee_algo).unwrap()
    }

    fn get_ada_for_output(output: &TransactionOutput, cfg: &TransactionBuilderConfig) -> Coin {
        min_ada_for_output(output, &cfg.data_cost).unwrap()
    }

    fn check_balance(utxos: &TransactionUnspentOutputs, batches: &TransactionBatchList) {
        let utxos_total = get_utxos_total(utxos);
        let batches_total = get_batch_total(batches);
        assert_eq!(utxos_total, batches_total);
    }

    fn check_min_adas(batches: &TransactionBatchList, cfg: &TransactionBuilderConfig) {
        for batch in batches {
            for tx in batch {
                for output in &tx.body.outputs {
                    let min_ada = get_ada_for_output(output, cfg);
                    assert!(output.amount.coin >= min_ada);
                }
            }
        }
    }

    fn check_fees(batches: &TransactionBatchList, cfg: &TransactionBuilderConfig) {
        for batch in batches {
            for tx in batch {
                let fee = get_tx_fee(tx, cfg);
                assert_eq!(fee, tx.body.fee);
            }
        }
    }

    fn check_value_size_limit(batches: &TransactionBatchList, cfg: &TransactionBuilderConfig) {
        for batch in batches {
            for tx in batch {
                for output in &tx.body.outputs {
                    let value_size = output.amount().to_bytes().len();
                    assert!(value_size <= cfg.max_value_size as usize);
                }
            }
        }
    }

    fn check_tx_size_limit(batches: &TransactionBatchList, cfg: &TransactionBuilderConfig) {
        for batch in batches {
            for tx in batch {
                let tx_size = tx.to_bytes().len();
                assert!(tx_size <= cfg.max_tx_size as usize);
            }
        }
    }

    #[test]
    pub fn test_big_utoxs_batch() {
        let utxos = generate_big_utoxs_bacth();
        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(4000)
            .max_tx_size(8000)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_ok());

        let batches = res.unwrap();
        check_balance(&utxos, &batches);
        check_min_adas(&batches, &cfg);
        check_fees(&batches, &cfg);
        check_value_size_limit(&batches, &cfg);
        check_tx_size_limit(&batches, &cfg);
    }

    #[test]
    pub fn test_big_utoxs_ada_batch() {
        let utxos = generate_big_ada_utoxs_bacth();
        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(4000)
            .max_tx_size(8000)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_ok());

        let batches = res.unwrap();
        check_balance(&utxos, &batches);
        check_min_adas(&batches, &cfg);
        check_fees(&batches, &cfg);
        check_value_size_limit(&batches, &cfg);
        check_tx_size_limit(&batches, &cfg);
    }

    #[test]
    pub fn test_one_utxo() {
        let address = generate_address(1);
        let utxo = generate_utxo(
                &address,
                1,
                0,
                0,
                3,
                2,
                20,
                Coin::from(500u64),
                Coin::from(5000000u64));

        let utxos = TransactionUnspentOutputs(vec![utxo]);

        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(4000)
            .max_tx_size(8000)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_ok());

        let batches = res.unwrap();
        check_balance(&utxos, &batches);
        check_min_adas(&batches, &cfg);
        check_fees(&batches, &cfg);
        check_value_size_limit(&batches, &cfg);
        check_tx_size_limit(&batches, &cfg);
    }

    #[test]
    pub fn test_one_utxo_one_asset_per_output() {
        let address = generate_address(1);
        let utxo_1 = generate_utxo(
            &address,
            1,
            0,
            0,
            3,
            1,
            20,
            Coin::from(500u64),
            Coin::from(5000000u64));

        let utxo_2 = generate_utxo(
            &address,
            2,
            1,
            0,
            3,
            1,
            20,
            Coin::from(500u64),
            Coin::from(5000000u64));

        let utxo_3 = generate_utxo(
            &address,
            3,
            2,
            0,
            3,
            1,
            20,
            Coin::from(500u64),
            Coin::from(5000000u64));

        let utxos = TransactionUnspentOutputs(vec![utxo_1, utxo_2, utxo_3]);

        let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(0));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(80)
            .max_tx_size(8000)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_ok());

        let batches = res.unwrap();

        for batch in &batches {
            for tx in batch {
                for output in &tx.body.outputs() {
                    assert_eq!(output.amount().multiasset.unwrap().0.len(), 1);
                    for asset in output.amount().multiasset.unwrap().0.values() {
                        assert_eq!(asset.len(), 1);
                    }
                }
            }
        }

        check_balance(&utxos, &batches);
        check_min_adas(&batches, &cfg);
        check_fees(&batches, &cfg);
        check_value_size_limit(&batches, &cfg);
        check_tx_size_limit(&batches, &cfg);
    }

    #[test]
    pub fn test_one_utxo_one_asset_per_tx() {
        let address = generate_address(1);
        let utxo_1 = generate_utxo(
            &address,
            1,
            0,
            0,
            1,
            1,
            20,
            Coin::from(500u64),
            Coin::from(5000000u64));

        let utxo_2 = generate_utxo(
            &address,
            2,
            1,
            0,
            1,
            1,
            20,
            Coin::from(500u64),
            Coin::from(5000000u64));

        let utxo_3 = generate_utxo(
            &address,
            3,
            2,
            0,
            1,
            1,
            20,
            Coin::from(500u64),
            Coin::from(5000000u64));

        let utxos = TransactionUnspentOutputs(vec![utxo_1, utxo_2, utxo_3]);

        let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(0));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(80)
            .max_tx_size(300)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_ok());

        let batches = res.unwrap();

        for batch in &batches {
            for tx in batch {
                assert_eq!(tx.body.outputs().len(), 1);
                for output in &tx.body.outputs() {
                    assert_eq!(output.amount().multiasset.unwrap().0.len(), 1);
                    for asset in output.amount().multiasset.unwrap().0.values() {
                        assert_eq!(asset.len(), 1);
                    }
                }
            }
        }

        check_balance(&utxos, &batches);
        check_min_adas(&batches, &cfg);
        check_fees(&batches, &cfg);
        check_value_size_limit(&batches, &cfg);
        check_tx_size_limit(&batches, &cfg);
    }

    #[test]
    pub fn test_only_ada_utxo() {
        let address = generate_address(1);
        let utxo = generate_utxo(
            &address,
            1,
            0,
            0,
            0,
            0,
            20,
            Coin::zero(),
            Coin::from(5000000u64));

        let utxos = TransactionUnspentOutputs(vec![utxo]);

        let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(0));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(4000)
            .max_tx_size(8000)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_ok());

        let batches = res.unwrap();
        check_balance(&utxos, &batches);
        check_min_adas(&batches, &cfg);
        check_fees(&batches, &cfg);
        check_value_size_limit(&batches, &cfg);
        check_tx_size_limit(&batches, &cfg);
    }

    #[test]
    pub fn test_not_enough_ada() {
        let address = generate_address(1);
        let utxo = generate_utxo(
            &address,
            1,
            0,
            0,
            0,
            0,
            20,
            Coin::zero(),
            Coin::from(1u64));

        let utxos = TransactionUnspentOutputs(vec![utxo]);

        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(4000)
            .max_tx_size(8000)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_err());
    }

    #[test]
    pub fn test_value_limit_error() {
        let address = generate_address(1);
        let utxo = generate_utxo(
            &address,
            1,
            0,
            0,
            1,
            1,
            20,
            Coin::from(1000000u64),
            Coin::from(500000u64));

        let utxos = TransactionUnspentOutputs(vec![utxo]);

        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(10)
            .max_tx_size(8000000)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_err());
    }

    #[test]
    pub fn test_tx_limit_error() {
        let address = generate_address(1);
        let utxo = generate_utxo(
            &address,
            1,
            0,
            0,
            10,
            10,
            20,
            Coin::from(1000000u64),
            Coin::from(50000000u64));

        let utxos = TransactionUnspentOutputs(vec![utxo]);

        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(100)
            .max_tx_size(2000)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_err());
    }

    #[test]
    pub fn test_no_utxos() {
        let utxos = TransactionUnspentOutputs(vec!());

        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(10)
            .max_tx_size(8000000)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_ok());

        let batches = res.unwrap();
        let total_txs = batches.into_iter().fold(0, |acc, batch| acc + batch.len());
        assert_eq!(total_txs, 0);

    }

    #[test]
    pub fn test_script_input_error() {
        let address = Address::from_hex("10798c8ce251c36c15f8bccf3306feae1218fce7503b331e6d92e666aa00efb5788e8713c844dfd32b2e91de1e309fefffd555f827cc9ee164").unwrap();
        let utxo = generate_utxo(
            &address,
            1,
            0,
            0,
            0,
            0,
            20,
            Coin::zero(),
            Coin::from(1u64));
        let utxos = TransactionUnspentOutputs(vec![utxo]);

        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(10)
            .max_tx_size(8000000)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_err());
    }

    #[test]
    pub fn test_two_asset_utxo_one_ada_utxo() {
        let address = generate_address(1);
        let asset_utxo_1 = generate_utxo(
            &address,
            1,
            0,
            0,
            1,
            1,
            8,
            Coin::from(1u64),
            Coin::from(1344798u64));

        let asset_utxo_2 = generate_utxo(
            &address,
            2,
            1,
            1,
            1,
            1,
            8,
            Coin::from(1u64),
            Coin::from(1344798u64));

        let ada_utxo = generate_utxo(
            &address,
            3,
            0,
            0,
            0,
            0,
            20,
            Coin::from(1u64),
            Coin::from(9967920528u64));

        let utxos = TransactionUnspentOutputs(vec![asset_utxo_1, asset_utxo_2, ada_utxo]);

        let linear_fee = LinearFee::new(&to_bignum(44), &to_bignum(155381));
        let cfg = TransactionBuilderConfigBuilder::new()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(500000000))
            .key_deposit(&to_bignum(2000000))
            .max_value_size(4000)
            .max_tx_size(8000)
            .coins_per_utxo_word(&to_bignum(34_482))
            .ex_unit_prices(&ExUnitPrices::new(
                &SubCoin::new(&to_bignum(577), &to_bignum(10000)),
                &SubCoin::new(&to_bignum(721), &to_bignum(10000000)),
            ))
            .build()
            .unwrap();

        let res = create_send_all(&generate_address(10000), &utxos, &cfg);
        assert!(res.is_ok());

        let batches = res.unwrap();
        check_balance(&utxos, &batches);
        check_min_adas(&batches, &cfg);
        check_fees(&batches, &cfg);
        check_value_size_limit(&batches, &cfg);
        check_tx_size_limit(&batches, &cfg);
    }
}