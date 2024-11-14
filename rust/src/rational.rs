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
        }.reduce_minuses()
    }


    pub (crate) fn one() -> Self {
        Rational {
            numerator: BigInt::one(),
            denominator: BigInt::one(),
        }
    }

    pub(crate) fn zero() -> Self {
        Rational {
            numerator: BigInt::zero(),
            denominator: BigInt::one(),
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

    pub(crate) fn mul_ratio(&self, x: &Rational) -> Rational {
        let a_num = &self.numerator;
        let a_denum = &self.denominator;
        let b_num = &x.numerator;
        let b_denum = &x.denominator;

        let a_num_fixed = a_num.mul(b_num);
        let a_denum_fixed = a_denum.mul(b_denum);
        Rational::new(a_num_fixed, a_denum_fixed)
    }

    pub(crate) fn div_ratio(&self, x: &Rational) -> Rational {
        let a_num = &self.numerator;
        let a_denum = &self.denominator;
        let b_num = &x.numerator;
        let b_denum = &x.denominator;

        let a_num_fixed = a_num.mul(b_denum);
        let a_denum_fixed = a_denum.mul(b_num);

        Rational::new(a_num_fixed, a_denum_fixed)
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

    pub(crate) fn sub(&self, x: &Rational) -> Rational {
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
        let a_b_num_diff = a_num_fixed.sub(b_num_fixed);
        let common_denum = a_denum.mul(b_denum);
        Rational::new(a_b_num_diff, common_denum)
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

    pub(crate) fn pow(&self, exp: u32) -> Rational {
        let num = self.numerator.pow(exp);
        let denum = self.denominator.pow(exp);
        Rational::new(num, denum).reduce_minuses()
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }

    pub(crate) fn is_negative(&self) -> bool {
        let is_num_negative = self.numerator.is_negative();
        let is_denum_negative = self.denominator.is_negative();
        is_num_negative ^ is_denum_negative
    }

    pub(crate) fn is_negative_or_zero(&self) -> bool {
        self.is_zero() || self.is_negative()
    }

    #[allow(dead_code)]
    pub(crate) fn to_bignum_floor(&self) -> Result<BigNum, JsError> {
        let num = self.numerator();
        let denum = self.denominator();
        if denum.is_zero() {
            return Err(JsError::from_str("Division by zero"));
        }
        let value = num.div_floor(denum);
        match value.as_u64() {
            Some(coin) => Ok(coin),
            _ => Err(JsError::from_str(&format!(
                "Failed to calculate floor from ratio {}/{}",
                num.to_str(),
                denum.to_str(),
            ))),
        }
    }

    fn reduce_minuses(mut self) -> Self{
        if self.numerator.is_negative() && self.denominator.is_negative() {
            self.numerator = self.numerator.abs();
            self.denominator = self.denominator.abs();
        }
        self
    }
}