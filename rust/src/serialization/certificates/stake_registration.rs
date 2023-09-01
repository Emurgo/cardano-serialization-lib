use crate::serialization::map_names::CertificateIndexNames;
use crate::serialization::struct_checks::{check_index, check_len, serialize_and_check_index};
use crate::*;
use cbor_event::Len;
use num_traits::{FromPrimitive, ToPrimitive};

impl cbor_event::se::Serialize for StakeRegistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if self.coin.is_some() {
            serialize_as_conway(self, serializer)
        } else {
            serialize_as_legacy(self, serializer)
        }
    }
}

fn serialize_as_legacy<'se, W: Write>(
    cert: &StakeRegistration,
    serializer: &'se mut Serializer<W>,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    serializer.write_array(cbor_event::Len::Len(2))?;

    let proposal_index = CertificateIndexNames::StakeRegistrationLegacy.to_u64();
    serialize_and_check_index(serializer, proposal_index, "StakeRegistrationLegacy")?;

    cert.stake_credential.serialize(serializer)?;
    Ok(serializer)
}

fn serialize_as_conway<'se, W: Write>(
    cert: &StakeRegistration,
    serializer: &'se mut Serializer<W>,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    serializer.write_array(cbor_event::Len::Len(3))?;

    let proposal_index = CertificateIndexNames::StakeRegistrationConway.to_u64();
    serialize_and_check_index(serializer, proposal_index, "StakeRegistrationConway")?;

    cert.stake_credential.serialize(serializer)?;
    if let Some(coin) = cert.coin {
        coin.serialize(serializer)?;
    }
    Ok(serializer)
}

impl_deserialize_for_wrapped_tuple!(StakeRegistration);

impl DeserializeEmbeddedGroup for StakeRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: Len,
    ) -> Result<Self, DeserializeError> {
        let cert_index = raw.unsigned_integer()?;
        let index_enum = CertificateIndexNames::from_u64(cert_index);
        match index_enum {
            Some(CertificateIndexNames::StakeRegistrationLegacy) => {
                deserialize_legacy(raw, cert_index, len)
            }
            Some(CertificateIndexNames::StakeRegistrationConway) => {
                deserialize_conway(raw, cert_index, len)
            }
            _ => Err(DeserializeFailure::FixedValuesMismatch {
                found: Key::Uint(cert_index),
                expected: vec![
                    Key::OptUint(CertificateIndexNames::StakeRegistrationLegacy.to_u64()),
                    Key::OptUint(CertificateIndexNames::StakeRegistrationConway.to_u64()),
                ],
            })
            .map_err(|e| DeserializeError::from(e).annotate("cert_index")),
        }
    }
}

fn deserialize_legacy<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    cert_index: u64,
    len: Len,
) -> Result<StakeRegistration, DeserializeError> {
    (|| -> Result<_, DeserializeError> {
        check_len(len, 2, "(cert_index, stake_credential)")?;
        let desired_index = CertificateIndexNames::StakeRegistrationLegacy.to_u64();
        check_index(cert_index, desired_index, "cert_index")?;

        let stake_credential =
            Credential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        return Ok(StakeRegistration {
            stake_credential,
            coin: None,
        });
    })()
    .map_err(|e| e.annotate("StakeRegistration (legacy)"))
}

fn deserialize_conway<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    cert_index: u64,
    len: Len,
) -> Result<StakeRegistration, DeserializeError> {
    (|| -> Result<_, DeserializeError> {
        check_len(len, 3, "(cert_index, stake_credential, coin)")?;
        let desired_index = CertificateIndexNames::StakeRegistrationConway.to_u64();
        check_index(cert_index, desired_index, "cert_index")?;

        let stake_credential =
            Credential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        return Ok(StakeRegistration {
            stake_credential,
            coin: Some(coin),
        });
    })()
    .map_err(|e| e.annotate("StakeRegistration (conway)"))
}
