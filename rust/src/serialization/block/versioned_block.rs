use crate::serialization::utils::{check_len, check_len_indefinite};
use crate::*;

impl Serialize for VersionedBlock {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(Len::Len(2))?;
        self.era_code.serialize(serializer)?;
        self.block.serialize(serializer)
    }
}

impl Deserialize for VersionedBlock {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        check_len(len, 2, "VersionedBlock")?;
        let era_code = u32::deserialize(raw)?;
        let block = Block::deserialize(raw)?;
        check_len_indefinite(raw, len)?;
        Ok(VersionedBlock { era_code, block })
    }
}
