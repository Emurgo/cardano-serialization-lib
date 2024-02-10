use std::convert::TryInto;
use crate::protocol_types::{CBORSpecial, Deserialize};
use crate::{DeserializeError, DeserializeFailure, Nonce};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;

impl cbor_event::se::Serialize for Nonce {
    fn serialize<'se, W: std::io::Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match &self.hash {
            Some(hash) => {
                serializer.write_array(cbor_event::Len::Len(2))?;
                serializer.write_unsigned_integer(1)?;
                serializer.write_bytes(hash)
            }
            None => {
                serializer.write_array(cbor_event::Len::Len(1))?;
                serializer.write_unsigned_integer(0)
            }
        }
    }
}

impl Deserialize for Nonce {
    fn deserialize<R: std::io::BufRead>(
        raw: &mut Deserializer<R>,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<Self, DeserializeError> {
            let len = raw.array()?;
            let hash = match raw.unsigned_integer()? {
                0 => None,
                1 => {
                    let bytes = raw.bytes()?;
                    if bytes.len() != Self::HASH_LEN {
                        return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                            Self::HASH_LEN as u64,
                            cbor_event::Len::Len(bytes.len() as u64),
                            "hash length",
                        ))
                        .into());
                    }
                    Some(bytes[..Self::HASH_LEN].try_into().unwrap())
                }
                _ => return Err(DeserializeFailure::NoVariantMatched.into()),
            };
            match len {
                cbor_event::Len::Len(n) => {
                    let correct_len = match n {
                        1 => hash.is_none(),
                        2 => hash.is_some(),
                        _ => false,
                    };
                    if !correct_len {
                        return Err(DeserializeFailure::NoVariantMatched.into());
                    }
                }
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break =>
                    /* it's ok */
                    {
                        ()
                    }
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            };
            Ok(Self { hash })
        })()
        .map_err(|e| e.annotate(stringify!($name)))
    }
}
