use std::io::{BufRead, Seek, Write};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use crate::{DeserializeError, Vkey, Vkeys};
use crate::protocol_types::{CBORSpecial, CBORType, Deserialize};

impl cbor_event::se::Serialize for Vkeys {
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

impl Deserialize for Vkeys {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(Vkey::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("Vkeys"))?;
        Ok(Self(arr))
    }
}