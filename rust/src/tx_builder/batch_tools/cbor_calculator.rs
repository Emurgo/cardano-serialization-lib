use super::super::*;

pub(super) struct CborCalculator();

const MAX_INLINE_ENCODING: u64 = 23;

impl CborCalculator {

    // According to the CBOR spec, the maximum size of a inlined CBOR value is 23 bytes.
    // Otherwise, the value is encoded as pair of type and value.
    pub(super) fn get_struct_size(items_count: u64) -> usize {
        if items_count <= MAX_INLINE_ENCODING {
            return 1;
        } else if items_count < 0x1_00 {
            return 2;
        } else if items_count < 0x1_00_00 {
            return 3;
        } else if items_count < 0x1_00_00_00_00 {
            return 5;
        } else {
            return 9;
        }
    }

    pub(super) fn get_coin_size(coin: &Coin)-> usize {
        Self::get_struct_size(coin.clone().into())
    }

    pub (super) fn get_address_size(address: &Address) -> usize {
        address.to_bytes().len()
    }

    pub (super) fn output_size(address: &Address, only_ada: bool) -> usize {
        //pre babbage output size is array of 2 elements address and value
        let legacy_output_size = CborCalculator::get_struct_size(2);
        let address_size = CborCalculator::get_address_size(address);
        if only_ada {
            legacy_output_size + address_size
        } else {
            //value with coin and assets is array of 2 elements
            legacy_output_size + CborCalculator::get_struct_size(2)
        }
    }

    pub (super) fn estimate_output_cost(output_size: usize,
                                        coins_per_byte: Coin,
                                        cost_estimator: fn(coins_per_byte: &Coin, size: usize) -> Result<Coin, JsError>)
    -> Result<Coin, JsError> {
        //todo add iteration logic
        cost_estimator(&coins_per_byte, output_size)
    }
}