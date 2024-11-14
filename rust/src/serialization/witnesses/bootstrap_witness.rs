use std::io::{BufRead, Seek, Write};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use crate::{BootstrapWitness, DeserializeError, DeserializeFailure, Ed25519Signature, Vkey};
use crate::protocol_types::{CBORSpecial, Deserialize, DeserializeEmbeddedGroup};

impl cbor_event::se::Serialize for BootstrapWitness {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.vkey.serialize(serializer)?;
        self.signature.serialize(serializer)?;
        serializer.write_bytes(&self.chain_code)?;
        serializer.write_bytes(&self.attributes)?;
        Ok(serializer)
    }
}

impl Deserialize for BootstrapWitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) =>
                /* TODO: check finite len somewhere */
                    {
                        ()
                    }
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break =>
                    /* it's ok */
                        {
                            ()
                        }
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
            .map_err(|e| e.annotate("BootstrapWitness"))
    }
}

impl DeserializeEmbeddedGroup for BootstrapWitness {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let vkey = (|| -> Result<_, DeserializeError> { Ok(Vkey::deserialize(raw)?) })()
            .map_err(|e| e.annotate("vkey"))?;
        let signature =
            (|| -> Result<_, DeserializeError> { Ok(Ed25519Signature::deserialize(raw)?) })()
                .map_err(|e| e.annotate("signature"))?;
        let chain_code = (|| -> Result<_, DeserializeError> { Ok(raw.bytes()?) })()
            .map_err(|e| e.annotate("chain_code"))?;
        let attributes = (|| -> Result<_, DeserializeError> { Ok(raw.bytes()?) })()
            .map_err(|e| e.annotate("attributes"))?;
        Ok(BootstrapWitness {
            vkey,
            signature,
            chain_code,
            attributes,
        })
    }
}