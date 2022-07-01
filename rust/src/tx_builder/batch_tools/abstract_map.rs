struct AbstractMap(BigNum);

impl AbstractMap {
    fn get_pure_len(worst_len: BigNum) {
        to_bytes(Self(worst_len));
    }
}

impl cbor_event::se::Serialize for AbstractMap {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0 as u64))?;
        Ok(serializer)
    }
}