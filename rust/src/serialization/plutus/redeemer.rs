use crate::*;
use crate::serialization::utils::check_len_indefinite;

impl cbor_event::se::Serialize for Redeemer {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.serialize_as_array_item(serializer)?;
        Ok(serializer)
    }
}

impl Redeemer {
    pub(crate) fn serialize_as_array_item<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(Len::Len(4))?;
        self.tag.serialize(serializer)?;
        self.index.serialize(serializer)?;
        self.data.serialize(serializer)?;
        self.ex_units.serialize(serializer)?;
        Ok(serializer)
    }

    pub(crate) fn serialize_as_map_item<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.serialize_as_map_key(serializer)?;
        self.serialize_as_map_value(serializer)
    }
    fn serialize_as_map_key<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(Len::Len(2))?;
        self.tag.serialize(serializer)?;
        self.index.serialize(serializer)?;
        Ok(serializer)
    }

    fn serialize_as_map_value<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(Len::Len(2))?;
        self.data.serialize(serializer)?;
        self.ex_units.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for Redeemer {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Self::deserialize_as_array_item(raw).map_err(|e| e.annotate("Redeemer"))
    }
}

impl Redeemer {
    pub(crate) fn deserialize_as_array_item<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<Self, DeserializeError> {
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
        check_len_indefinite(raw, len)?;
        Ok(Redeemer {
            tag,
            index,
            data,
            ex_units,
        })
    }

    pub(crate) fn deserialize_as_map_item<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<Self, DeserializeError> {
        let (tag, index) = Self::deserialize_map_key(raw)?;
        let (data, ex_units) = Self::deserialize_map_value(raw)?;

        Ok(Self {
            tag,
            index,
            data,
            ex_units,
        })
    }

    fn deserialize_map_key<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<(RedeemerTag, BigNum), DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;

        let tag = RedeemerTag::deserialize(raw)?;
        let index = BigNum::deserialize(raw)?;

        check_len_indefinite(raw, len)?;

        Ok((tag, index))
    }

    fn deserialize_map_value<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<(PlutusData, ExUnits), DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;

        let data = PlutusData::deserialize(raw)?;
        let ex_units = ExUnits::deserialize(raw)?;

        check_len_indefinite(raw, len)?;

        Ok((data, ex_units))
    }
}