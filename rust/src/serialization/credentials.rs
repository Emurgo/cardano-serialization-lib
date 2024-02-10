use crate::*;
use crate::serialization::utils::skip_set_tag;

impl cbor_event::se::Serialize for Credentials {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        //TODO: uncomment this line when we conway ero will come
        //serializer.write_tag(258)?;
        serializer.write_array(cbor_event::Len::Len(self.len() as u64))?;
        for element in self.to_vec() {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Credentials {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        skip_set_tag(raw)?;
        let mut creds = Credentials::new();
        let mut counter = 0u64;
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => counter < n,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                creds.add_move(Credential::deserialize(raw)?);
                counter += 1;
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("CredentialsSet"))?;
        Ok(creds)
    }
}