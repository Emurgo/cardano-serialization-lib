
use super::*;

// Specifies an amount of ADA in terms of lovelace
// String functions are for environments that don't support u64 or BigInt/etc
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coin(u64);

to_from_bytes!(Coin);

#[wasm_bindgen]
impl Coin {
    // May not be supported in all environments as it maps to BigInt with wasm_bindgen
    pub fn new(value: u64) -> Coin {
        Self(value)
    }
    pub fn unwrap(&self) -> u64 {
        self.0
    }

    // Create a Coin from a standard rust string representation
    pub fn from_str(string: &str) -> Result<Coin, JsValue> {
        string.parse::<u64>()
            .map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
            .map(Coin)
    }

    // String representation of the Coin value for use from environments that don't support BigInt
    pub fn to_str(&self) -> String {
        format!("{}", self.0)
    }

    pub fn checked_mul(&self, other: &Coin) -> Result<Coin, JsValue> {
        match self.0.checked_mul(other.0) {
            Some(value) => Ok(Coin(value)),
            None => Err(JsValue::from_str("overflow")),
        }
    }

    pub fn checked_add(&self, other: &Coin) -> Result<Coin, JsValue> {
        match self.0.checked_add(other.0) {
            Some(value) => Ok(Coin(value)),
            None => Err(JsValue::from_str("overflow")),
        }
    }

    pub fn checked_sub(&self, other: &Coin) -> Result<Coin, JsValue> {
        match self.0.checked_sub(other.0) {
            Some(value) => Ok(Coin(value)),
            None => Err(JsValue::from_str("underflow")),
        }
    }
}

impl cbor_event::se::Serialize for Coin {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
      serializer.write_unsigned_integer(self.0)
  }
}

impl Deserialize for Coin {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      match raw.unsigned_integer() {
          Ok(value) => Ok(Self(value)),
          Err(e) => Err(DeserializeError::new("Coin", DeserializeFailure::CBOR(e))),
      }
  }
}
