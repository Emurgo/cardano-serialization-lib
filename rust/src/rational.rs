use crate::{BigInt, BigNum, JsError, UnitInterval};

#[derive(Clone, Debug)]
pub(crate) struct Rational {
    numerator: BigInt,
    denominator: BigInt,
}

impl From<UnitInterval> for Rational {
    fn from(sc: UnitInterval) -> Self {
        Rational::new(BigInt::from(sc.numerator), BigInt::from(sc.denominator))
    }
}

impl From<&UnitInterval> for Rational {
    fn from(sc: &UnitInterval) -> Self {
        Rational::new(BigInt::from(sc.numerator), BigInt::from(sc.denominator))
    }
}

impl Rational {
    pub(crate) fn new(n: BigInt, d: BigInt) -> Self {
        Rational {
            numerator: n,
            denominator: d,
        }
    }

    pub(crate) fn numerator(&self) -> &BigInt {
        &self.numerator
    }

    pub(crate) fn denominator(&self) -> &BigInt {
        &self.denominator
    }

    pub(crate) fn mul_bignum(&self, x: &BigNum) -> Result<Rational, JsError> {
        let m: BigInt = x.into();
        Ok(Rational::new(self.numerator.mul(&m), self.denominator.clone()))
    }

    pub(crate) fn mul_usize(&self, x: usize) -> Rational {
        Rational::new(self.numerator.mul(&BigInt::from(x)), self.denominator.clone())
    }

    pub(crate) fn add(&self, x: &Rational) -> Rational {
        let a_num = &self.numerator;
        let a_denum = &self.denominator;
        let b_num = &x.numerator;
        let b_denum = &x.denominator;

        if a_num.is_zero() {
            return x.clone();
        }
        if b_num.is_zero() {
            return self.clone();
        }
        let a_num_fixed = &a_num.mul(b_denum);
        let b_num_fixed = &b_num.mul(a_denum);
        let a_b_num_sum = a_num_fixed.add(b_num_fixed);
        let common_denum = a_denum.mul(b_denum);
        Rational::new(a_b_num_sum, common_denum)
    }

    pub(crate) fn to_bignum_ceil(&self) -> Result<BigNum, JsError> {
        let num = self.numerator();
        let denum = self.denominator();
        if denum.is_zero() {
            return Err(JsError::from_str("Division by zero"));
        }
        let value = num.div_ceil(denum);
        match value.as_u64() {
            Some(coin) => Ok(coin),
            _ => Err(JsError::from_str(&format!(
                "Failed to calculate ceil from ratio {}/{}",
                num.to_str(),
                denum.to_str(),
            ))),
        }
    }

    pub(crate) fn to_bignum_floor(&self) -> Result<BigNum, JsError> {
        let num = self.numerator();
        let denum = self.denominator();
        if denum.is_zero() {
            return Err(JsError::from_str("Division by zero"));
        }
        let value = num.div_ceil(denum);
        match value.as_u64() {
            Some(coin) => Ok(coin),
            _ => Err(JsError::from_str(&format!(
                "Failed to calculate floor from ratio {}/{}",
                num.to_str(),
                denum.to_str(),
            ))),
        }
    }
}