use crate::*;
use crate::serialization::utils::read_nint;

impl Serialize for BigInt {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        match u64_digits.len() {
            0 => serializer.write_unsigned_integer(0),
            // we use the uint/nint encodings to use a minimum of space
            1 => match sign {
                // uint
                num_bigint::Sign::Plus | num_bigint::Sign::NoSign => {
                    serializer.write_unsigned_integer(*u64_digits.first().unwrap())
                }
                // nint
                num_bigint::Sign::Minus => serializer
                    .write_negative_integer(-(*u64_digits.first().unwrap() as i128) as i64),
            },
            _ => {
                // Small edge case: nint's minimum is -18446744073709551616 but in this bigint lib
                // that takes 2 u64 bytes so we put that as a special case here:
                if sign == num_bigint::Sign::Minus && u64_digits == vec![0, 1] {
                    serializer.write_negative_integer(-18446744073709551616i128 as i64)
                } else {
                    let (sign, bytes) = self.0.to_bytes_be();
                    match sign {
                        // positive bigint
                        num_bigint::Sign::Plus | num_bigint::Sign::NoSign => {
                            serializer.write_tag(2u64)?;
                            write_bounded_bytes(serializer, &bytes)
                        }
                        // negative bigint
                        num_bigint::Sign::Minus => {
                            serializer.write_tag(3u64)?;
                            use std::ops::Neg;
                            // CBOR RFC defines this as the bytes of -n -1
                            let adjusted = self
                                .0
                                .clone()
                                .neg()
                                .checked_sub(&num_bigint::BigInt::from(1u32))
                                .unwrap()
                                .to_biguint()
                                .unwrap();
                            write_bounded_bytes(serializer, &adjusted.to_bytes_be())
                        }
                    }
                }
            }
        }
    }
}

impl Deserialize for BigInt {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                // bigint
                CBORType::Tag => {
                    let tag = raw.tag()?;
                    let bytes = read_bounded_bytes(raw)?;
                    match tag {
                        // positive bigint
                        2 => Ok(Self(num_bigint::BigInt::from_bytes_be(
                            num_bigint::Sign::Plus,
                            &bytes,
                        ))),
                        // negative bigint
                        3 => {
                            // CBOR RFC defines this as the bytes of -n -1
                            let initial =
                                num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes);
                            use std::ops::Neg;
                            let adjusted = initial
                                .checked_add(&num_bigint::BigInt::from(1u32))
                                .unwrap()
                                .neg();
                            Ok(Self(adjusted))
                        }
                        _ => {
                            return Err(DeserializeFailure::TagMismatch {
                                found: tag,
                                expected: 2,
                            }
                                .into());
                        }
                    }
                }
                // uint
                CBORType::UnsignedInteger => {
                    Ok(Self(num_bigint::BigInt::from(raw.unsigned_integer()?)))
                }
                // nint
                CBORType::NegativeInteger => Ok(Self(num_bigint::BigInt::from(read_nint(raw)?))),
                _ => return Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
            .map_err(|e| e.annotate("BigInt"))
    }
}