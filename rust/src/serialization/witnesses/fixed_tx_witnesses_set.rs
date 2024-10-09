use std::io::{BufRead, Seek, Write};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use crate::protocol_types::{Deserialize, FixedTxWitnessesSet};
use crate::{DeserializeError};

impl cbor_event::se::Serialize for FixedTxWitnessesSet {
    fn serialize<'a, W: Write + Sized>(&self, serializer: &'a mut Serializer<W>) -> cbor_event::Result<&'a mut Serializer<W>> {
        super::transaction_witnesses_set::serialize(self.tx_witnesses_set_ref(), Some(self.raw_parts_ref()), serializer)
    }
}

impl Deserialize for FixedTxWitnessesSet {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError>
    where
        Self: Sized
    {
        let (witness_set, raw_parts) = super::transaction_witnesses_set::deserialize(raw, true)?;
        Ok(Self::new(witness_set, raw_parts))
    }
}

