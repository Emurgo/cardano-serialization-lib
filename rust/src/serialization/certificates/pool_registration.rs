use crate::serialization::map_names::CertificateIndexNames;
use crate::serialization::utils::{
    check_len, deserialize_and_check_index, serialize_and_check_index,
};
use crate::*;
use num_traits::ToPrimitive;

impl cbor_event::se::Serialize for Relays {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Relays {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(Relay::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("Relays"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for PoolParams {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(9))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for PoolParams {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.operator.serialize(serializer)?;
        self.vrf_keyhash.serialize(serializer)?;
        self.pledge.serialize(serializer)?;
        self.cost.serialize(serializer)?;
        self.margin.serialize(serializer)?;
        self.reward_account.serialize(serializer)?;
        self.pool_owners.serialize(serializer)?;
        self.relays.serialize(serializer)?;
        match &self.pool_metadata {
            Some(x) => x.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(PoolParams);

impl DeserializeEmbeddedGroup for PoolParams {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let operator =
            (|| -> Result<_, DeserializeError> { Ok(Ed25519KeyHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("operator"))?;
        let vrf_keyhash =
            (|| -> Result<_, DeserializeError> { Ok(VRFKeyHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("vrf_keyhash"))?;
        let pledge = (|| -> Result<_, DeserializeError> { Ok(Coin::deserialize(raw)?) })()
            .map_err(|e| e.annotate("pledge"))?;
        let cost = (|| -> Result<_, DeserializeError> { Ok(Coin::deserialize(raw)?) })()
            .map_err(|e| e.annotate("cost"))?;
        let margin = (|| -> Result<_, DeserializeError> { Ok(UnitInterval::deserialize(raw)?) })()
            .map_err(|e| e.annotate("margin"))?;
        let reward_account =
            (|| -> Result<_, DeserializeError> { Ok(RewardAddress::deserialize(raw)?) })()
                .map_err(|e| e.annotate("reward_account"))?;
        let pool_owners =
            (|| -> Result<_, DeserializeError> { Ok(Ed25519KeyHashesSet::deserialize(raw)?) })()
                .map_err(|e| e.annotate("pool_owners"))?;
        let relays = (|| -> Result<_, DeserializeError> { Ok(Relays::deserialize(raw)?) })()
            .map_err(|e| e.annotate("relays"))?;
        let pool_metadata = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => Some(PoolMetadata::deserialize(raw)?),
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })()
        .map_err(|e| e.annotate("pool_metadata"))?;
        Ok(PoolParams {
            operator,
            vrf_keyhash,
            pledge,
            cost,
            margin,
            reward_account,
            pool_owners,
            relays,
            pool_metadata,
        })
    }
}

impl cbor_event::se::Serialize for PoolRegistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(10))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for PoolRegistration {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        let proposal_index = CertificateIndexNames::PoolRegistration.to_u64();
        serialize_and_check_index(serializer, proposal_index, "PoolRegistration")?;

        self.pool_params.serialize_as_embedded_group(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(PoolRegistration);

impl DeserializeEmbeddedGroup for PoolRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 10, "(cert_index, pool_params (without array) )")?;

        let cert_index = CertificateIndexNames::PoolRegistration.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let pool_params = (|| -> Result<_, DeserializeError> {
            Ok(PoolParams::deserialize_as_embedded_group(raw, len)?)
        })()
        .map_err(|e| e.annotate("pool_params"))?;
        Ok(PoolRegistration { pool_params })
    }
}
