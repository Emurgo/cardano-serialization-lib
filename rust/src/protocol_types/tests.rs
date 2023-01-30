#[cfg(test)]
mod test {
    use crate::protocol_types::fixed_tx::FixedTransaction;
    use crate::Transaction;
    use hex;

    #[test]
    fn simple_round_trip() {
        let original_tx = FixedTransaction::from_hex("84a700818258208b9c96823c19f2047f32210a330434b3d163e194ea17b2b702c0667f6fea7a7a000d80018182581d6138fe1dd1d91221a199ff0dacf41fdd5b87506b533d00e70fae8dae8f1abfbac06a021a0002b645031a03962de305a1581de1b3cabd3914ef99169ace1e8b545b635f809caa35f8b6c8bc69ae48061abf4009040e80a100828258207dc05ac55cdfb9cc24571d491d3a3bdbd7d48489a916d27fce3ffe5c9af1b7f55840d7eda8457f1814fe3333b7b1916e3b034e6d480f97f4f286b1443ef72383279718a3a3fddf127dae0505b01a48fd9ffe0f52d9d8c46d02bcb85d1d106c13aa048258201b3d6e1236891a921abf1a3f90a9fb1b2568b1096b6cd6d3eaaeb0ef0ee0802f58401ce4658303c3eb0f2b9705992ccd62de30423ade90219e2c4cfc9eb488c892ea28ba3110f0c062298447f4f6365499d97d31207075f9815c3fe530bd9a927402f5f6").unwrap();
        let body = hex::decode("a700818258208b9c96823c19f2047f32210a330434b3d163e194ea17b2b702c0667f6fea7a7a000d80018182581d6138fe1dd1d91221a199ff0dacf41fdd5b87506b533d00e70fae8dae8f1abfbac06a021a0002b645031a03962de305a1581de1b3cabd3914ef99169ace1e8b545b635f809caa35f8b6c8bc69ae48061abf4009040e80").unwrap();
        let wit_set = hex::decode("a100828258207dc05ac55cdfb9cc24571d491d3a3bdbd7d48489a916d27fce3ffe5c9af1b7f55840d7eda8457f1814fe3333b7b1916e3b034e6d480f97f4f286b1443ef72383279718a3a3fddf127dae0505b01a48fd9ffe0f52d9d8c46d02bcb85d1d106c13aa048258201b3d6e1236891a921abf1a3f90a9fb1b2568b1096b6cd6d3eaaeb0ef0ee0802f58401ce4658303c3eb0f2b9705992ccd62de30423ade90219e2c4cfc9eb488c892ea28ba3110f0c062298447f4f6365499d97d31207075f9815c3fe530bd9a927402").unwrap();
        let tx = FixedTransaction::new(&body, &wit_set, true)
        .unwrap();
        let tx2 = FixedTransaction::from_bytes(tx.to_bytes()).unwrap();

        assert_eq!(body, tx2.raw_body());
        assert_eq!(wit_set, tx2.raw_witness_set());
        assert_eq!(tx.raw_body(), tx2.raw_body());
        assert_eq!(tx.raw_witness_set(), tx2.raw_witness_set());
        assert_eq!(tx.is_valid(), tx2.is_valid());
        assert_eq!(tx.to_bytes(), tx2.to_bytes());
        assert_eq!(tx.raw_body(), original_tx.raw_body());
        assert_eq!(tx.raw_witness_set(), original_tx.raw_witness_set());
        assert_eq!(tx.is_valid(), original_tx.is_valid());
        assert_eq!(tx.to_bytes(), original_tx.to_bytes());
    }

    #[test]
    fn round_trip_via_tx() {
        let casual_tx = Transaction::from_hex("84a700818258208b9c96823c19f2047f32210a330434b3d163e194ea17b2b702c0667f6fea7a7a000d80018182581d6138fe1dd1d91221a199ff0dacf41fdd5b87506b533d00e70fae8dae8f1abfbac06a021a0002b645031a03962de305a1581de1b3cabd3914ef99169ace1e8b545b635f809caa35f8b6c8bc69ae48061abf4009040e80a100828258207dc05ac55cdfb9cc24571d491d3a3bdbd7d48489a916d27fce3ffe5c9af1b7f55840d7eda8457f1814fe3333b7b1916e3b034e6d480f97f4f286b1443ef72383279718a3a3fddf127dae0505b01a48fd9ffe0f52d9d8c46d02bcb85d1d106c13aa048258201b3d6e1236891a921abf1a3f90a9fb1b2568b1096b6cd6d3eaaeb0ef0ee0802f58401ce4658303c3eb0f2b9705992ccd62de30423ade90219e2c4cfc9eb488c892ea28ba3110f0c062298447f4f6365499d97d31207075f9815c3fe530bd9a927402f5f6").unwrap();
        let body = hex::decode("a700818258208b9c96823c19f2047f32210a330434b3d163e194ea17b2b702c0667f6fea7a7a000d80018182581d6138fe1dd1d91221a199ff0dacf41fdd5b87506b533d00e70fae8dae8f1abfbac06a021a0002b645031a03962de305a1581de1b3cabd3914ef99169ace1e8b545b635f809caa35f8b6c8bc69ae48061abf4009040e80").unwrap();
        let wit_set = hex::decode("a100828258207dc05ac55cdfb9cc24571d491d3a3bdbd7d48489a916d27fce3ffe5c9af1b7f55840d7eda8457f1814fe3333b7b1916e3b034e6d480f97f4f286b1443ef72383279718a3a3fddf127dae0505b01a48fd9ffe0f52d9d8c46d02bcb85d1d106c13aa048258201b3d6e1236891a921abf1a3f90a9fb1b2568b1096b6cd6d3eaaeb0ef0ee0802f58401ce4658303c3eb0f2b9705992ccd62de30423ade90219e2c4cfc9eb488c892ea28ba3110f0c062298447f4f6365499d97d31207075f9815c3fe530bd9a927402").unwrap();
        let tx = FixedTransaction::new(&body, &wit_set, true)
            .unwrap();
        let tx2 = Transaction::from_bytes(tx.to_bytes()).unwrap();

        assert_eq!(casual_tx.body(), tx.body());
        assert_eq!(casual_tx.witness_set(), tx.witness_set());
        assert_eq!(casual_tx.is_valid(), tx.is_valid());
        assert_eq!(casual_tx, tx2);
    }

    #[test]
    fn round_trip_nonstandart_body() {
        let original_tx = FixedTransaction::from_hex("84a7009F8258208b9c96823c19f2047f32210a330434b3d163e194ea17b2b702c0667f6fea7a7a00FF0d80018182581d6138fe1dd1d91221a199ff0dacf41fdd5b87506b533d00e70fae8dae8f1abfbac06a021a0002b645031a03962de305a1581de1b3cabd3914ef99169ace1e8b545b635f809caa35f8b6c8bc69ae48061abf4009040e80a100828258207dc05ac55cdfb9cc24571d491d3a3bdbd7d48489a916d27fce3ffe5c9af1b7f55840d7eda8457f1814fe3333b7b1916e3b034e6d480f97f4f286b1443ef72383279718a3a3fddf127dae0505b01a48fd9ffe0f52d9d8c46d02bcb85d1d106c13aa048258201b3d6e1236891a921abf1a3f90a9fb1b2568b1096b6cd6d3eaaeb0ef0ee0802f58401ce4658303c3eb0f2b9705992ccd62de30423ade90219e2c4cfc9eb488c892ea28ba3110f0c062298447f4f6365499d97d31207075f9815c3fe530bd9a927402f5f6").unwrap();
        let casual_tx = Transaction::from_hex("84a7009F8258208b9c96823c19f2047f32210a330434b3d163e194ea17b2b702c0667f6fea7a7a00FF0d80018182581d6138fe1dd1d91221a199ff0dacf41fdd5b87506b533d00e70fae8dae8f1abfbac06a021a0002b645031a03962de305a1581de1b3cabd3914ef99169ace1e8b545b635f809caa35f8b6c8bc69ae48061abf4009040e80a100828258207dc05ac55cdfb9cc24571d491d3a3bdbd7d48489a916d27fce3ffe5c9af1b7f55840d7eda8457f1814fe3333b7b1916e3b034e6d480f97f4f286b1443ef72383279718a3a3fddf127dae0505b01a48fd9ffe0f52d9d8c46d02bcb85d1d106c13aa048258201b3d6e1236891a921abf1a3f90a9fb1b2568b1096b6cd6d3eaaeb0ef0ee0802f58401ce4658303c3eb0f2b9705992ccd62de30423ade90219e2c4cfc9eb488c892ea28ba3110f0c062298447f4f6365499d97d31207075f9815c3fe530bd9a927402f5f6").unwrap();
        let body = hex::decode("a7009F8258208b9c96823c19f2047f32210a330434b3d163e194ea17b2b702c0667f6fea7a7a00FF0d80018182581d6138fe1dd1d91221a199ff0dacf41fdd5b87506b533d00e70fae8dae8f1abfbac06a021a0002b645031a03962de305a1581de1b3cabd3914ef99169ace1e8b545b635f809caa35f8b6c8bc69ae48061abf4009040e80").unwrap();
        let wit_set = hex::decode("a100828258207dc05ac55cdfb9cc24571d491d3a3bdbd7d48489a916d27fce3ffe5c9af1b7f55840d7eda8457f1814fe3333b7b1916e3b034e6d480f97f4f286b1443ef72383279718a3a3fddf127dae0505b01a48fd9ffe0f52d9d8c46d02bcb85d1d106c13aa048258201b3d6e1236891a921abf1a3f90a9fb1b2568b1096b6cd6d3eaaeb0ef0ee0802f58401ce4658303c3eb0f2b9705992ccd62de30423ade90219e2c4cfc9eb488c892ea28ba3110f0c062298447f4f6365499d97d31207075f9815c3fe530bd9a927402").unwrap();
        let tx = FixedTransaction::new(&body, &wit_set, true)
            .unwrap();
        let tx2 = Transaction::from_bytes(tx.to_bytes()).unwrap();
        let tx3 = FixedTransaction::from_bytes(tx.to_bytes()).unwrap();

        assert_eq!(casual_tx.body(), tx.body());
        assert_eq!(casual_tx.witness_set(), tx.witness_set());
        assert_eq!(casual_tx.is_valid(), tx.is_valid());

        assert_eq!(body, tx3.raw_body());
        assert_eq!(wit_set, tx3.raw_witness_set());
        assert_eq!(tx.raw_body(), tx3.raw_body());
        assert_eq!(tx.raw_witness_set(), tx3.raw_witness_set());
        assert_eq!(tx.is_valid(), tx3.is_valid());
        assert_eq!(tx.to_bytes(), tx3.to_bytes());
        assert_eq!(tx.raw_body(), original_tx.raw_body());
        assert_eq!(tx.raw_witness_set(), original_tx.raw_witness_set());
        assert_eq!(tx.is_valid(), original_tx.is_valid());
        assert_eq!(tx.to_bytes(), original_tx.to_bytes());

        assert_eq!(casual_tx, tx2);

        assert_ne!(tx2.to_bytes(), original_tx.to_bytes());
    }
}