use super::*;
use crate::rational::{Rational};

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LinearFee {
    constant: Coin,
    coefficient: Coin,
}

#[wasm_bindgen]
impl LinearFee {
    pub fn constant(&self) -> Coin {
        self.constant
    }

    pub fn coefficient(&self) -> Coin {
        self.coefficient
    }

    pub fn new(coefficient: &Coin, constant: &Coin) -> Self {
        Self {
            constant: constant.clone(),
            coefficient: coefficient.clone(),
        }
    }
}

#[wasm_bindgen]
pub fn min_fee(tx: &Transaction, linear_fee: &LinearFee) -> Result<Coin, JsError> {
    min_fee_for_size(tx.to_bytes().len(), linear_fee)
}

pub fn min_fee_for_size(size: usize, linear_fee: &LinearFee) -> Result<Coin, JsError> {
    BigNum::from(size)
        .checked_mul(&linear_fee.coefficient())?
        .checked_add(&linear_fee.constant())
}

#[wasm_bindgen]
pub fn calculate_ex_units_ceil_cost(
    ex_units: &ExUnits,
    ex_unit_prices: &ExUnitPrices,
) -> Result<Coin, JsError> {
    let mem_price: Rational = ex_unit_prices.mem_price().into();
    let steps_price: Rational = ex_unit_prices.step_price().into();
    let mem_ratio = mem_price.mul_bignum(&ex_units.mem())?;
    let steps_ratio = steps_price.mul_bignum(&ex_units.steps())?;
    let total = mem_ratio.add(&steps_ratio);
    total.to_bignum_ceil()
}

#[wasm_bindgen]
pub fn min_script_fee(tx: &Transaction, ex_unit_prices: &ExUnitPrices) -> Result<Coin, JsError> {
    if let Some(redeemers) = &tx.witness_set.redeemers {
        let total_ex_units: ExUnits = redeemers.total_ex_units()?;
        return calculate_ex_units_ceil_cost(&total_ex_units, ex_unit_prices);
    }
    Ok(Coin::zero())
}

#[wasm_bindgen]
pub fn min_ref_script_fee(total_ref_scripts_size: usize, ref_script_coins_per_byte: &UnitInterval) -> Result<Coin, JsError> {
    let ref_multiplier : Rational = ref_script_coins_per_byte.into();
    let total_fee = ref_multiplier.mul_usize(total_ref_scripts_size);
    total_fee.to_bignum_ceil()
}