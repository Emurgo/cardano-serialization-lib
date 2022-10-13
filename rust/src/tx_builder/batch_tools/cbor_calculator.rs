use crate::fees::{LinearFee, min_fee_for_size};
use crate::serialization_tools::map_names::{TxBodyNames, WitnessSetNames};
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

    pub(super) fn get_coin_size(coin: &Coin) -> usize {
        Self::get_struct_size(coin.clone().into())
    }

    pub(super) fn get_address_size(address: &Address) -> usize {
        address.to_bytes().len()
    }

    pub(super) fn get_fake_vkey_size() -> usize {
        //precalculater fake vkey size
        //TODO: try to add const calculation
        101
    }

    pub(super) fn get_boostrap_witness_size(address: &ByronAddress) -> usize {
        //TODO: add precalculated boostrap witness size
        let witness = make_icarus_bootstrap_witness(
            &TransactionHash::from([0u8; TransactionHash::BYTE_COUNT]),
            address,
            &fake_private_key(),
        );
        witness.to_bytes().len()
    }

    pub(super) fn get_output_size(address: &Address) -> usize {
        //pre babbage output size is array of 2 elements address and value
        let legacy_output_size = CborCalculator::get_struct_size(2);
        let address_size = CborCalculator::get_address_size(address);
        let address_struct_size = CborCalculator::get_struct_size(address_size as u64);
        return legacy_output_size + address_size + address_struct_size
    }

    pub(super) fn get_value_struct_size(ada_only: bool) -> usize {
        if ada_only {
            //only ada value is encoded as coin without struct overhead
            0
        } else {
            //value with assets and ada is array of 2 elements
            CborCalculator::get_struct_size(2)
        }

    }

    pub(super) fn get_bare_tx_body_size(body_fields: &HashSet<TxBodyNames>) -> usize {
        let mut size = CborCalculator::get_struct_size(body_fields.len() as u64);
        for field in body_fields {
            size += CborCalculator::get_struct_size(field.to_number());
        }
        size
    }

    pub(super) fn get_wintnesses_set_struct_size(witnesses_fields: &HashSet<WitnessSetNames>) -> usize {
        let mut size = CborCalculator::get_struct_size(witnesses_fields.len() as u64);
        for field in witnesses_fields {
            size += CborCalculator::get_struct_size(field.to_number());
        }
        size
    }

    pub(super) fn get_bare_tx_size(has_auxiliary: bool) -> usize {
        //tx is array of 4 elements, tx_body, witnesses, is_valid and auxiliary
        let mut size = CborCalculator::get_struct_size(4);
        size += 1; //1 byte for bool is_valid
        if !has_auxiliary {
            size += 1; //1 byte for None auxiliary
        }
        size
    }


    //TODO: extract iterative logic from estimate_output_cost and estimate_fee to separate function
    pub(super) fn estimate_output_cost(used_coins: &Coin,
                                       output_size: usize,
                                       data_cost: &DataCost) -> Result<(Coin, usize), JsError> {
        let mut current_cost = MinOutputAdaCalculator::calc_size_cost(data_cost, output_size)?;
        if current_cost <= *used_coins {
            return Ok((current_cost, output_size));
        }

        let size_without_coin = output_size - CborCalculator::get_coin_size(used_coins);
        let mut last_size = size_without_coin + CborCalculator::get_coin_size(&current_cost);
        for _ in 0..3 {
            current_cost = MinOutputAdaCalculator::calc_size_cost(data_cost, last_size)?;
            let new_size = size_without_coin + CborCalculator::get_coin_size(&current_cost);
            if new_size == last_size {
                return Ok((current_cost, last_size));
            } else {
                last_size = new_size;
            }
        }

        let max_size = output_size + CborCalculator::get_coin_size(&Coin::max_value());
        let pessimistic_cost = MinOutputAdaCalculator::calc_size_cost(data_cost, max_size)?;
        Ok((pessimistic_cost, max_size))
    }


    pub(super) fn estimate_fee(tx_size_without_fee: usize,
                               min_dependable_amount: Option<Coin>,
                               dependable_amount: Option<Coin>,
                               fee_algo: &LinearFee) -> Result<(Coin, usize), JsError> {
        let mut current_cost = min_fee_for_size(tx_size_without_fee, fee_algo)?;
        let mut last_size = tx_size_without_fee + CborCalculator::get_coin_size(&current_cost);

        last_size = Self::recalc_size_with_dependable_value(last_size, &current_cost, min_dependable_amount, dependable_amount)?;

        for _ in 0..3 {
            current_cost = min_fee_for_size(last_size, fee_algo)?;
            let mut new_size = tx_size_without_fee + CborCalculator::get_coin_size(&current_cost);
            new_size = Self::recalc_size_with_dependable_value(new_size, &current_cost, min_dependable_amount, dependable_amount)?;

            if new_size == last_size {
                return Ok((current_cost, last_size));
            } else {
                last_size = new_size;
            }
        }

        let max_size = tx_size_without_fee + CborCalculator::get_coin_size(&Coin::max_value());
        let pessimistic_cost = min_fee_for_size(max_size, fee_algo)?;
        Ok((pessimistic_cost, max_size))
    }

    //if we get ada from somewhere for fee, that means that we reduce size of it can be reduced
    //by this logic we try to track this
    fn recalc_size_with_dependable_value(size: usize,
                                         current_cost: &Coin,
                                         min_dependable_amount: Option<Coin>,
                                         dependable_amount: Option<Coin>, ) -> Result<usize, JsError> {
        if let Some(dependable_amount) = dependable_amount {
            let mut remain_ada = dependable_amount.checked_sub(current_cost).unwrap_or(Coin::zero());
            if let Some(min_dependable_amount) = min_dependable_amount {
                if remain_ada < min_dependable_amount {
                    remain_ada = min_dependable_amount;
                }
            }
            return Ok(size + CborCalculator::get_coin_size(&remain_ada));
        }

        Ok(size)
    }
}