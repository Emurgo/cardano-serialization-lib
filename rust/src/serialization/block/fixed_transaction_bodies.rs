use crate::*;
use crate::serialization::utils::is_break_tag;

impl Deserialize for FixedTransactionBodies {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "FixedTransactionBodies")? {
                    break;
                }
                arr.push(FixedTransactionBody::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("FixedTransactionBodies"))?;
        Ok(Self(arr))
    }
}
