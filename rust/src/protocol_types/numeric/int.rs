use crate::*;

// CBOR has int = uint / nint
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int(pub(crate) i128);

impl_to_from!(Int);

#[wasm_bindgen]
impl Int {
    pub fn new(x: &BigNum) -> Self {
        Self(x.0 as i128)
    }

    pub fn new_negative(x: &BigNum) -> Self {
        Self(-(x.0 as i128))
    }

    pub fn new_i32(x: i32) -> Self {
        Self(x as i128)
    }

    pub fn is_positive(&self) -> bool {
        return self.0 >= 0;
    }

    /// BigNum can only contain unsigned u64 values
    ///
    /// This function will return the BigNum representation
    /// only in case the underlying i128 value is positive.
    ///
    /// Otherwise nothing will be returned (undefined).
    pub fn as_positive(&self) -> Option<BigNum> {
        if self.is_positive() {
            Some((self.0 as u64).into())
        } else {
            None
        }
    }

    /// BigNum can only contain unsigned u64 values
    ///
    /// This function will return the *absolute* BigNum representation
    /// only in case the underlying i128 value is negative.
    ///
    /// Otherwise nothing will be returned (undefined).
    pub fn as_negative(&self) -> Option<BigNum> {
        if !self.is_positive() {
            Some(((-self.0) as u64).into())
        } else {
            None
        }
    }

    /// !!! DEPRECATED !!!
    /// Returns an i32 value in case the underlying original i128 value is within the limits.
    /// Otherwise will just return an empty value (undefined).
    #[deprecated(
    since = "10.0.0",
    note = "Unsafe ignoring of possible boundary error and it's not clear from the function name. Use `as_i32_or_nothing`, `as_i32_or_fail`, or `to_str`"
    )]
    pub fn as_i32(&self) -> Option<i32> {
        self.as_i32_or_nothing()
    }

    /// Returns the underlying value converted to i32 if possible (within limits)
    /// Otherwise will just return an empty value (undefined).
    pub fn as_i32_or_nothing(&self) -> Option<i32> {
        use std::convert::TryFrom;
        i32::try_from(self.0).ok()
    }

    /// Returns the underlying value converted to i32 if possible (within limits)
    /// JsError in case of out of boundary overflow
    pub fn as_i32_or_fail(&self) -> Result<i32, JsError> {
        use std::convert::TryFrom;
        i32::try_from(self.0).map_err(|e| JsError::from_str(&format!("{}", e)))
    }

    /// Returns string representation of the underlying i128 value directly.
    /// Might contain the minus sign (-) in case of negative value.
    pub fn to_str(&self) -> String {
        format!("{}", self.0)
    }

    // Create an Int from a standard rust string representation
    pub fn from_str(string: &str) -> Result<Int, JsError> {
        let x = string
            .parse::<i128>()
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))?;
        if x.abs() > u64::MAX as i128 {
            return Err(JsError::from_str(&format!(
                "{} out of bounds. Value (without sign) must fit within 4 bytes limit of {}",
                x,
                u64::MAX
            )));
        }
        Ok(Self(x))
    }
}

impl serde::Serialize for Int {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> serde::de::Deserialize<'de> for Int {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"string rep of a number",
            )
        })
    }
}

impl JsonSchema for Int {
    fn schema_name() -> String {
        String::from("Int")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}