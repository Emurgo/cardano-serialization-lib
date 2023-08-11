use crate::*;

impl cbor_event::se::Serialize for VotingProcedure {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        match self.vote {
            VoteKind::No => {
                serializer.write_unsigned_integer(0u64)?;
            }
            VoteKind::Yes => {
                serializer.write_unsigned_integer(1u64)?;
            }
            VoteKind::Abstain => {
                serializer.write_unsigned_integer(2u64)?;
            }
        };
        self.anchor.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for VotingProcedure {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            if let cbor_event::Len::Len(n) = len {
                if n != 2 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        2,
                        len,
                        "[vote, anchor]",
                    ))
                    .into());
                }
            }

            let vote = match raw.unsigned_integer()? {
                0 => VoteKind::No,
                1 => VoteKind::Yes,
                2 => VoteKind::Abstain,
                n => {
                    return Err(
                        DeserializeError::from(DeserializeFailure::FixedValuesMismatch {
                            found: Key::Uint(n),
                            expected: vec![Key::Uint(0), Key::Uint(1), Key::Uint(2)],
                        }).annotate("vote")
                    )
                }
            };

            let anchor = Anchor::deserialize(raw).map_err(
                |e| e.annotate("anchor")
            )?;

            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }

            Ok(Self { vote, anchor })
        })()
        .map_err(|e| e.annotate("VotingProcedure"))
    }
}
