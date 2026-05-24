use crate::*;
use crate::serialization::utils::read_nint;

impl cbor_event::se::Serialize for Int {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        // Invariant: Int::CBOR_MIN <= self.0 <= Int::CBOR_MAX, i.e. fits in CBOR int.
        // For negatives we must use the i128-aware writer because nint payload
        // (-self.0 - 1) can be up to u64::MAX, which does not fit in i64.
        if self.0 < 0 {
            let payload = (-self.0 - 1) as u64;
            serializer.write_negative_integer_sz(self.0, cbor_event::Sz::canonical(payload))
        } else {
            serializer.write_unsigned_integer(self.0 as u64)
        }
    }
}

impl Deserialize for Int {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::UnsignedInteger => {
                    // raw u64 fits Int::CBOR_MAX by construction.
                    Ok(Self(raw.unsigned_integer()? as i128))
                }
                cbor_event::Type::NegativeInteger => {
                    let n = read_nint(raw)?;
                    // read_nint returns i128 in [-2^64, -1] which exactly matches
                    // [Int::CBOR_MIN, -1]. Validate as defense-in-depth.
                    Int::new_checked(n).map_err(|e| {
                        DeserializeFailure::CustomError(format!("{:?}", e)).into()
                    })
                }
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
            .map_err(|e| e.annotate("Int"))
    }
}
