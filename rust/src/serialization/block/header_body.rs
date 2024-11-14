use crate::*;

impl cbor_event::se::Serialize for HeaderBody {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(15))?;
        self.block_number.serialize(serializer)?;
        self.slot.serialize(serializer)?;
        match &self.prev_hash {
            Some(x) => x.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        self.issuer_vkey.serialize(serializer)?;
        self.vrf_vkey.serialize(serializer)?;
        match &self.leader_cert {
            HeaderLeaderCertEnum::NonceAndLeader(nonce_vrf, leader_vrf) => {
                nonce_vrf.serialize(serializer)?;
                leader_vrf.serialize(serializer)?;
            }
            HeaderLeaderCertEnum::VrfResult(vrf_cert) => {
                vrf_cert.serialize(serializer)?;
            }
        }
        self.block_body_size.serialize(serializer)?;
        self.block_body_hash.serialize(serializer)?;
        self.operational_cert
            .serialize_as_embedded_group(serializer)?;
        self.protocol_version
            .serialize_as_embedded_group(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for HeaderBody {
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
            .map_err(|e| e.annotate("HeaderBody"))
    }
}

impl DeserializeEmbeddedGroup for HeaderBody {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let block_number = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("block_number"))?;
        let slot = (|| -> Result<_, DeserializeError> { Ok(SlotBigNum::deserialize(raw)?) })()
            .map_err(|e| e.annotate("slot"))?;
        let prev_hash = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => Some(BlockHash::deserialize(raw)?),
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })()
            .map_err(|e| e.annotate("prev_hash"))?;
        let issuer_vkey = (|| -> Result<_, DeserializeError> { Ok(Vkey::deserialize(raw)?) })()
            .map_err(|e| e.annotate("issuer_vkey"))?;
        let vrf_vkey = (|| -> Result<_, DeserializeError> { Ok(VRFVKey::deserialize(raw)?) })()
            .map_err(|e| e.annotate("vrf_vkey"))?;
        let leader_cert = {
            // NONCE VFR CERT, first of two certs
            // or a single VRF RESULT CERT
            // depending on the protocol version
            let first_vrf_cert =
                (|| -> Result<_, DeserializeError> { Ok(VRFCert::deserialize(raw)?) })()
                    .map_err(|e| e.annotate("nonce_vrf"))?;
            let cbor_type: cbor_event::Type = raw.cbor_type()?;
            match cbor_type {
                cbor_event::Type::Array => {
                    // Legacy format, reading the second VRF cert
                    let leader_vrf =
                        (|| -> Result<_, DeserializeError> { Ok(VRFCert::deserialize(raw)?) })()
                            .map_err(|e| e.annotate("leader_vrf"))?;
                    HeaderLeaderCertEnum::NonceAndLeader(first_vrf_cert, leader_vrf)
                }
                cbor_event::Type::UnsignedInteger => {
                    // New format, no second VRF cert is present
                    HeaderLeaderCertEnum::VrfResult(first_vrf_cert)
                }
                t => {
                    return Err(DeserializeError::new(
                        "HeaderBody.leader_cert",
                        DeserializeFailure::UnexpectedKeyType(t),
                    ))
                }
            }
        };
        let block_body_size = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("block_body_size"))?;
        let block_body_hash =
            (|| -> Result<_, DeserializeError> { Ok(BlockHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("block_body_hash"))?;

        let operational_cert = (|| -> Result<_, DeserializeError> {
            if raw.cbor_type()? == CBORType::Array {
                Ok(OperationalCert::deserialize(raw)?)
            } else {
                Ok(OperationalCert::deserialize_as_embedded_group(raw, len)?)
            }
        })()
            .map_err(|e| e.annotate("operational_cert"))?;
        let protocol_version = (|| -> Result<_, DeserializeError> {
            if raw.cbor_type()? == CBORType::Array {
                Ok(ProtocolVersion::deserialize(raw)?)
            } else {
                Ok(ProtocolVersion::deserialize_as_embedded_group(raw, len)?)
            }
        })()
            .map_err(|e| e.annotate("protocol_version"))?;
        Ok(HeaderBody {
            block_number,
            slot,
            prev_hash,
            issuer_vkey,
            vrf_vkey,
            leader_cert,
            block_body_size,
            block_body_hash,
            operational_cert,
            protocol_version,
        })
    }
}