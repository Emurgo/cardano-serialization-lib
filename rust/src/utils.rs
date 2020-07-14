
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

fn make_byron_pad_prefix() -> Vec<u8> {
    [
        0x83, // CBOR list-len (3)
        0x00, // address type = 0
        0x82, // CBOR list-len (2)
        0x00,
        0x52, 0x54 // CBOR bytestring (64)
    ].to_vec()
}
fn make_byron_pad_suffix(
    addr: &ByronAddress,
) -> Vec<u8> {
    let mut attributes_bytes = Serializer::new_vec();
    addr.0.attributes.serialize(&mut attributes_bytes).unwrap();
    attributes_bytes.finalize()
}

#[wasm_bindgen]
pub fn make_daedalus_bootstrap_witness(
    tx_body_hash: &TransactionHash,
    addr: &ByronAddress,
    key: &LegacyDaedalusPrivateKey,
) -> BootstrapWitness {
    let chain_code = key.chaincode();

    let pubkey = Bip32PublicKey::from_bytes(&key.0.to_public().as_ref()).unwrap();
    let vkey = Vkey::new(&pubkey.to_raw_key());
    let signature = Ed25519Signature::from_bytes(key.0.sign(&tx_body_hash.to_bytes()).as_ref().to_vec()).unwrap();

    BootstrapWitness::new(
        &vkey,
        &signature,
        chain_code,
        make_byron_pad_prefix(),
        make_byron_pad_suffix(addr),
    )
}

#[wasm_bindgen]
pub fn make_icarus_bootstrap_witness(
    tx_body_hash: &TransactionHash,
    addr: &ByronAddress,
    key: &Bip32PrivateKey,
) -> BootstrapWitness {
    let chain_code = key.chaincode();

    let raw_key = key.to_raw_key();
    let vkey = Vkey::new(&raw_key.to_public());
    let signature = raw_key.sign(&tx_body_hash.to_bytes());

    BootstrapWitness::new(
        &vkey,
        &signature,
        chain_code,
        make_byron_pad_prefix(),
        make_byron_pad_suffix(addr),
    )
}
