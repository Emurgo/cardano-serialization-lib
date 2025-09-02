use crate::wasm_bindgen;
use crate::BigNum;
use crate::DeserializeError;
use crate::DeserializeFailure;
use crate::JsError;
use crate::{CertificateIndex, Slot32, SlotBigNum, TransactionIndex};

use std::convert::TryInto;
use std::io::{Cursor, Read};

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pointer {
    pub(crate) slot: BigNum,
    pub(crate) tx_index: BigNum,
    pub(crate) cert_index: BigNum,
}

#[wasm_bindgen]
impl Pointer {
    /// !!! DEPRECATED !!!
    /// This constructor uses outdated slot number format for the ttl value, tx_index and cert_index.
    /// Use `.new_pointer` instead
    #[deprecated(
        since = "10.1.0",
        note = "Underlying value capacity of ttl (BigNum u64) bigger then Slot32. Use new_pointer instead."
    )]
    pub fn new(slot: Slot32, tx_index: TransactionIndex, cert_index: CertificateIndex) -> Self {
        Self {
            slot: slot.into(),
            tx_index: tx_index.into(),
            cert_index: cert_index.into(),
        }
    }

    pub fn new_pointer(slot: &SlotBigNum, tx_index: &BigNum, cert_index: &BigNum) -> Self {
        Self {
            slot: slot.clone(),
            tx_index: tx_index.clone(),
            cert_index: cert_index.clone(),
        }
    }

    pub fn slot(&self) -> Result<u32, JsError> {
        self.slot.clone().try_into()
    }

    pub fn tx_index(&self) -> Result<u32, JsError> {
        self.tx_index.clone().try_into()
    }

    pub fn cert_index(&self) -> Result<u32, JsError> {
        self.cert_index.clone().try_into()
    }

    pub fn slot_bignum(&self) -> BigNum {
        self.slot.clone()
    }

    pub fn tx_index_bignum(&self) -> BigNum {
        self.tx_index.clone()
    }

    pub fn cert_index_bignum(&self) -> BigNum {
        self.cert_index.clone()
    }

    pub(crate) fn from_bytes(data: &[u8]) -> Result<(Pointer, usize), DeserializeError> {
        let (pointer, offset) = decode_pointer(data)?;
        Ok((pointer, offset))
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&encode_variable_length(self.slot.into()));
        buf.extend_from_slice(&encode_variable_length(self.tx_index.into()));
        buf.extend_from_slice(&encode_variable_length(self.cert_index.into()));
        buf
    }
}


fn decode_pointer(data: &[u8]) -> Result<(Pointer, usize), DeserializeError> {
    let mut cursor = Cursor::new(data);
    let mut offset = 0;
    let slot_no_64 = decode_variable_length_u64(&mut cursor, "SlotNo")?;
    let tx_ix_64 = decode_variable_length_u64(&mut cursor, "TxIx")?;
    let cert_ix_64 = decode_variable_length_u64(&mut cursor, "CertIx")?;

    offset += cursor.position() as usize;

    Ok((
        analyze_and_convert(slot_no_64, tx_ix_64, cert_ix_64),
        offset,
    ))
}

fn analyze_and_convert(slot_no: u64, tx_ix: u64, cert_ix: u64) -> Pointer {
    let needs_normalization =
        slot_no > u32::MAX as u64 || tx_ix > u16::MAX as u64 || cert_ix > u16::MAX as u64;

    // Normalization is (0,0,0). Ha-ha-ha. 
    if needs_normalization {
            Pointer {
                slot: 0u64.into(),
                tx_index: 0u64.into(),
                cert_index: 0u64.into(),
            }
    } else {
        Pointer {
            slot: slot_no.into(),
            tx_index: tx_ix.into(),
            cert_index: cert_ix.into(),
        }
    }
}

/// Decodes variable-length u64 exactly like Haskell decodeVariableLengthWord64
/// Non-recursive implementation that mimics: fix (decode7BitVarLength name buf) 0
///
/// Haskell code equivalent:
/// decode7BitVarLength name buf cont !acc = do
///   guardLength name 1 buf
///   offset <- state (\off -> (off, off + 1))
///   let b8 = bufUnsafeIndex buf offset
///   if b8 `testBit` 7
///     then cont (acc `shiftL` 7 .|. fromIntegral (b8 `clearBit` 7))
///     else pure (acc `shiftL` 7 .|. fromIntegral b8)
pub(crate) fn decode_variable_length_u64(
    cursor: &mut Cursor<&[u8]>,
    name: &'static str,
) -> Result<u64, DeserializeError> {
    let mut acc = 0u64;
    
    loop {
        // Equivalent of guardLength name 1 buf
        if cursor.position() >= cursor.get_ref().len() as u64 {
            return Err(DeserializeError::new(name, DeserializeFailure::VariableLenNatDecodeFailed));
        }

        // Equivalent of: offset <- state (\off -> (off, off + 1))
        let mut byte = [0u8; 1];
        cursor
            .read_exact(&mut byte)
            .map_err(|e| DeserializeError::new(name, DeserializeFailure::IoError(e.to_string())))?;
        let b8 = byte[0];

        // Equivalent of: acc `shiftL` 7 .|. fromIntegral (b8 `clearBit` 7)
        acc = (acc << 7)  | ((b8 & 0x7F) as u64);

        // Equivalent of: if b8 `testBit` 7
        if (b8 & 0x80) == 0 {
            // else pure (acc `shiftL` 7 .|. fromIntegral b8)
            return Ok(acc);
        }
    }
}

pub(crate) fn encode_variable_length(value: u64) -> Vec<u8> {
    let mut tmp = value;
    if tmp == 0 {
        return vec![0];
    }

    let mut bytes = Vec::new();
    while tmp > 0 {
        bytes.push((tmp & 0x7F) as u8);
        tmp >>= 7;
    }

    bytes.reverse();

    for i in 0..bytes.len() - 1 {
        bytes[i] |= 0x80;
    }

    bytes
}