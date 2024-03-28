use std::io::{BufRead, Seek, Write};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use crate::protocol_types::Deserialize;
use crate::{DeserializeError, DeserializeFailure, Ed25519Signature, Vkey, Vkeywitness};

impl cbor_event::se::Serialize for Vkeywitness {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.vkey.serialize(serializer)?;
        self.signature.serialize(serializer)
    }
}

impl Deserialize for Vkeywitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let vkey = (|| -> Result<_, DeserializeError> { Ok(Vkey::deserialize(raw)?) })()
                .map_err(|e| e.annotate("vkey"))?;
            let signature =
                (|| -> Result<_, DeserializeError> { Ok(Ed25519Signature::deserialize(raw)?) })()
                    .map_err(|e| e.annotate("signature"))?;
            let ret = Ok(Vkeywitness::new(&vkey, &signature));
            match len {
                cbor_event::Len::Len(n) => match n {
                    2 => (),
                    _ => {
                        return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                            2, len, "",
                        ))
                            .into())
                    }
                },
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break =>
                    /* it's ok */
                        {
                            ()
                        }
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
            .map_err(|e| e.annotate("Vkeywitness"))
    }
}