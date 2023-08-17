use num_traits::ToPrimitive;
use crate::*;
use crate::serialization::map_names::CertificateIndexNames;
use crate::serialization::struct_checks::{check_len, deserialize_and_check_index, serialize_and_check_index};

impl cbor_event::se::Serialize for GenesisKeyDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for GenesisKeyDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {

        let proposal_index = CertificateIndexNames::GenesisKeyDelegation.to_u64();
        serialize_and_check_index(serializer, proposal_index, "GenesisKeyDelegation")?;

        self.genesishash.serialize(serializer)?;
        self.genesis_delegate_hash.serialize(serializer)?;
        self.vrf_keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_tuple!(GenesisKeyDelegation);

impl DeserializeEmbeddedGroup for GenesisKeyDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {

        check_len(len, 4, "(cert_index, genesishash, genesis_delegate_hash, vrf_keyhash)")?;

        let cert_index = CertificateIndexNames::GenesisKeyDelegation.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let genesishash =
            (|| -> Result<_, DeserializeError> { Ok(GenesisHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("genesishash"))?;
        let genesis_delegate_hash =
            (|| -> Result<_, DeserializeError> { Ok(GenesisDelegateHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("genesis_delegate_hash"))?;
        let vrf_keyhash =
            (|| -> Result<_, DeserializeError> { Ok(VRFKeyHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("vrf_keyhash"))?;
        Ok(GenesisKeyDelegation {
            genesishash,
            genesis_delegate_hash,
            vrf_keyhash,
        })
    }
}
