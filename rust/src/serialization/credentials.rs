use crate::*;
use crate::serialization::utils::skip_set_tag;

impl cbor_event::se::Serialize for CredentialsSet {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        //TODO: uncomment this line when we conway ero will come
        //serializer.write_tag(258)?;
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for CredentialsSet {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        skip_set_tag(raw)?;
        let mut creds = CredentialsSet::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => creds.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                creds.add_move(Credential::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("CredentialsSet"))?;
        Ok(creds)
    }
}

impl cbor_event::se::Serialize for Credentials {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Credentials {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut creds = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => creds.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                creds.push(Credential::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("Credentials"))?;
        Ok(Credentials(creds))
    }
}

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