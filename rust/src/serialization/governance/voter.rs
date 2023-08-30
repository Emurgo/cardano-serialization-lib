use crate::*;

impl cbor_event::se::Serialize for Voter {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for Voter {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let voter_enum = VoterEnum::deserialize(raw)?;
            Ok(Self(voter_enum))
        })()
        .map_err(|e| e.annotate("Voter"))
    }
}

impl cbor_event::se::Serialize for VoterEnum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        match &self {
            VoterEnum::ConstitutionalCommitteeHotKey(cred) => match &cred.0 {
                StakeCredType::Key(key_hash) => {
                    serializer.write_unsigned_integer(0u64)?;
                    key_hash.serialize(serializer)?;
                }
                StakeCredType::Script(script_hash) => {
                    serializer.write_unsigned_integer(1u64)?;
                    script_hash.serialize(serializer)?;
                }
            },
            VoterEnum::DRep(cred) => match &cred.0 {
                StakeCredType::Key(key_hash) => {
                    serializer.write_unsigned_integer(2u64)?;
                    key_hash.serialize(serializer)?;
                }
                StakeCredType::Script(script_hash) => {
                    serializer.write_unsigned_integer(3u64)?;
                    script_hash.serialize(serializer)?;
                }
            },
            VoterEnum::StakingPool(scripthash) => {
                serializer.write_unsigned_integer(4u64)?;
                scripthash.serialize(serializer)?;
            }
        };
        Ok(serializer)
    }
}

impl Deserialize for VoterEnum {
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
            let voter = match raw.unsigned_integer()? {
                0 => VoterEnum::ConstitutionalCommitteeHotKey(Credential(StakeCredType::Key(
                    Ed25519KeyHash::deserialize(raw)?,
                ))),
                1 => VoterEnum::ConstitutionalCommitteeHotKey(Credential(
                    StakeCredType::Script(ScriptHash::deserialize(raw)?),
                )),
                2 => VoterEnum::DRep(Credential(StakeCredType::Key(
                    Ed25519KeyHash::deserialize(raw)?,
                ))),
                3 => VoterEnum::DRep(Credential(StakeCredType::Script(
                    ScriptHash::deserialize(raw)?,
                ))),
                4 => VoterEnum::StakingPool(Ed25519KeyHash::deserialize(raw)?),
                n => {
                    return Err(DeserializeFailure::FixedValuesMismatch {
                        found: Key::Uint(n),
                        expected: vec![
                            Key::Uint(0),
                            Key::Uint(1),
                            Key::Uint(2),
                            Key::Uint(3),
                            Key::Uint(4),
                        ],
                    }
                    .into())
                }
            };
            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }
            Ok(voter)
        })()
        .map_err(|e| e.annotate("VoterEnum"))
    }
}
