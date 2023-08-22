use crate::*;
use crate::serialization::struct_checks::check_len;

impl cbor_event::se::Serialize for Anchor {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.anchor_url.serialize(serializer)?;
        self.anchor_data_hash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for Anchor {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;

            check_len(len, 2, "(anchor_url, anchor_data_hash)")?;

            let anchor_url = URL::deserialize(raw).map_err(|e| e.annotate("anchor_url"))?;

            let anchor_data_hash =
                AnchorDataHash::deserialize(raw).map_err(|e| e.annotate("anchor_data_hash"))?;

            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }

            return Ok(Anchor {
                anchor_url,
                anchor_data_hash,
            });
        })()
        .map_err(|e| e.annotate("Anchor"))
    }
}
