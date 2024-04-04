use crate::*;
use crate::serialization::utils::is_break_tag;

impl cbor_event::se::Serialize for Redeemers {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self.serialization_format {
            Some(CborContainerType::Map) => {
                serializer.write_map(Len::Len(self.redeemers.len() as u64))?;
                for element in &self.redeemers {
                    element.serialize_as_map_item(serializer)?;
                }
            }
            _ => {
                serializer.write_array(Len::Len(self.redeemers.len() as u64))?;
                for element in &self.redeemers {
                    element.serialize_as_array_item(serializer)?;
                }
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for Redeemers {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<Self, DeserializeError> {
            let cbor_type = raw.cbor_type()?;
            match cbor_type {
                cbor_event::Type::Array => Self::deserialize_as_array(raw),
                cbor_event::Type::Map => Self::deserialize_as_map(raw),
                _ => return Err(DeserializeFailure::ExpectedType("Array or Map".to_string(), cbor_type).into()),
            }
        })().map_err(|e| e.annotate("Redeemers"))
    }
}

impl Redeemers {
    fn deserialize_as_map<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        let len = raw.map()?;
        while match len {
            Len::Len(n) => arr.len() < n as usize,
            Len::Indefinite => true,
        } {
            if is_break_tag(raw, "Redeemers")? {
                break;
            }
            arr.push(Redeemer::deserialize_as_map_item(raw)?);
        }
        Ok(Self {
            redeemers: arr,
            serialization_format: Some(CborContainerType::Map),
        })
    }

    fn deserialize_as_array<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        let len = raw.array()?;
        while match len {
            Len::Len(n) => arr.len() < n as usize,
            Len::Indefinite => true,
        } {
            if is_break_tag(raw, "Redeemers")? {
                break;
            }
            arr.push(Redeemer::deserialize_as_array_item(raw)?);
        }
        Ok(Self {
            redeemers: arr,
            serialization_format: Some(CborContainerType::Array),
        })
    }
}