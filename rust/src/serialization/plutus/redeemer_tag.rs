use crate::*;

impl cbor_event::se::Serialize for RedeemerTagKind {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            RedeemerTagKind::Spend => serializer.write_unsigned_integer(0u64),
            RedeemerTagKind::Mint => serializer.write_unsigned_integer(1u64),
            RedeemerTagKind::Cert => serializer.write_unsigned_integer(2u64),
            RedeemerTagKind::Reward => serializer.write_unsigned_integer(3u64),
            RedeemerTagKind::Vote => serializer.write_unsigned_integer(4u64),
            RedeemerTagKind::VotingProposal => serializer.write_unsigned_integer(5u64),
        }
    }
}

impl Deserialize for RedeemerTagKind {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.unsigned_integer() {
                Ok(0) => Ok(RedeemerTagKind::Spend),
                Ok(1) => Ok(RedeemerTagKind::Mint),
                Ok(2) => Ok(RedeemerTagKind::Cert),
                Ok(3) => Ok(RedeemerTagKind::Reward),
                Ok(4) => Ok(RedeemerTagKind::Vote),
                Ok(5) => Ok(RedeemerTagKind::VotingProposal),
                Ok(_) | Err(_) => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
            .map_err(|e| e.annotate("RedeemerTagEnum"))
    }
}

impl cbor_event::se::Serialize for RedeemerTag {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for RedeemerTag {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(RedeemerTagKind::deserialize(raw)?))
    }
}