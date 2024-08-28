use std::io::{BufRead, Seek, Write};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use crate::{Credential, CredType, DeserializeError, DeserializeFailure, Ed25519KeyHash, Key, ScriptHash};
use crate::protocol_types::{CBORSpecial, Deserialize};

impl cbor_event::se::Serialize for Credential {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        match &self.0 {
            CredType::Key(keyhash) => {
                serializer.write_unsigned_integer(0u64)?;
                serializer.write_bytes(keyhash.to_bytes())
            }
            CredType::Script(scripthash) => {
                serializer.write_unsigned_integer(1u64)?;
                serializer.write_bytes(scripthash.to_bytes())
            }
        }
    }
}

impl Deserialize for Credential {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            if let cbor_event::Len::Len(n) = len {
                if n != 2 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        2,
                        len,
                        "[id, hash]",
                    ))
                        .into());
                }
            }
            let cred_type = match raw.unsigned_integer()? {
                0 => CredType::Key(Ed25519KeyHash::deserialize(raw)?),
                1 => CredType::Script(ScriptHash::deserialize(raw)?),
                n => {
                    return Err(DeserializeFailure::FixedValuesMismatch {
                        found: Key::Uint(n),
                        expected: vec![Key::Uint(0), Key::Uint(1)],
                    }
                        .into());
                }
            };
            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }
            Ok(Credential(cred_type))
        })()
            .map_err(|e| e.annotate("StakeCredential"))
    }
}