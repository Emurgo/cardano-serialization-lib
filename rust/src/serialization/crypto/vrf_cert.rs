use crate::*;

impl cbor_event::se::Serialize for VRFCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_bytes(&self.output)?;
        serializer.write_bytes(&self.proof)?;
        Ok(serializer)
    }
}

impl Deserialize for VRFCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let output = (|| -> Result<_, DeserializeError> { Ok(raw.bytes()?) })()
                .map_err(|e| e.annotate("output"))?;
            let proof = (|| -> Result<_, DeserializeError> { Ok(raw.bytes()?) })()
                .map_err(|e| e.annotate("proof"))?;
            if proof.len() != Self::PROOF_LEN {
                return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                    Self::PROOF_LEN as u64,
                    cbor_event::Len::Len(proof.len() as u64),
                    "proof length",
                ))
                .into());
            }
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
            Ok(VRFCert { output, proof })
        })()
        .map_err(|e| e.annotate("VRFCert"))
    }
}