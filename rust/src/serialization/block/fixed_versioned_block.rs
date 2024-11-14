use crate::serialization::utils::{check_len, check_len_indefinite};
use crate::*;

impl Deserialize for FixedVersionedBlock {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        check_len(len, 2, "VersionedBlock")?;
        let era_code = u32::deserialize(raw)?;
        let block = FixedBlock::deserialize(raw)?;
        check_len_indefinite(raw, len)?;
        Ok(FixedVersionedBlock {
            block,
            era_code,
        })
    }
}
