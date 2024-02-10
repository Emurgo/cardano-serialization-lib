use crate::*;

impl cbor_event::se::Serialize for Redeemer {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.tag.serialize(serializer)?;
        self.index.serialize(serializer)?;
        self.data.serialize(serializer)?;
        self.ex_units.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for Redeemer {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(4)?;
            let tag = (|| -> Result<_, DeserializeError> { Ok(RedeemerTag::deserialize(raw)?) })()
                .map_err(|e| e.annotate("tag"))?;
            let index = (|| -> Result<_, DeserializeError> { Ok(BigNum::deserialize(raw)?) })()
                .map_err(|e| e.annotate("index"))?;
            let data = (|| -> Result<_, DeserializeError> { Ok(PlutusData::deserialize(raw)?) })()
                .map_err(|e| e.annotate("data"))?;
            let ex_units = (|| -> Result<_, DeserializeError> { Ok(ExUnits::deserialize(raw)?) })()
                .map_err(|e| e.annotate("ex_units"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Redeemer {
                tag,
                index,
                data,
                ex_units,
            })
        })()
            .map_err(|e| e.annotate("Redeemer"))
    }
}