use super::*;
use crate::rational::Rational;

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
pub fn min_ref_script_fee(
    total_ref_scripts_size: usize,
    ref_script_coins_per_byte: &UnitInterval,
) -> Result<Coin, JsError> {
    let multiplier = Rational::new(BigInt::from(12), BigInt::from(10)); // 1.2
    let size_increment: usize = 25_600; // 25KiB
    let ref_multiplier: Rational = ref_script_coins_per_byte.into();
    let total_fee = tier_ref_script_fee(
        multiplier,
        size_increment,
        ref_multiplier,
        total_ref_scripts_size,
    )?;

    Ok(total_fee)
}

fn tier_ref_script_fee(
    multiplier: Rational,
    size_increment: usize,
    base_fee: Rational,
    total_size: usize,
) -> Result<BigNum, JsError> {
    if multiplier.is_negative_or_zero() || size_increment == 0 {
        return Err(JsError::from_str(
            "Size increment and multiplier must be positive",
        ));
    }

    let full_tiers = (total_size / size_increment) as u32;
    let partial_tier_size = total_size % size_increment;
    let tier_price = base_fee.mul_usize(size_increment);

    let mut acc = Rational::zero();

    if full_tiers > 0 {
        let progression_enumerator = Rational::one().sub(&multiplier.pow(full_tiers));
        let progression_denominator = Rational::one().sub(&multiplier);
        let tier_progression_sum = progression_enumerator.div_ratio(&progression_denominator);
        acc = acc.add(&tier_price.mul_ratio(&tier_progression_sum));
    }

    // Add the partial tier
    if partial_tier_size > 0 {
        let last_tier_price = base_fee.mul_ratio(&multiplier.pow(full_tiers));
        let partial_tier_fee = last_tier_price.mul_usize(partial_tier_size);
        acc = acc.add(&partial_tier_fee);
    }

    acc.to_bignum_floor()
}
