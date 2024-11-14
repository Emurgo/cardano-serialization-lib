use crate::serialization::map_names::CertificateIndexNames;
use crate::serialization::utils::{
    check_len, deserialize_and_check_index, serialize_and_check_index,
};
use crate::*;
use num_traits::ToPrimitive;

impl cbor_event::se::Serialize for CommitteeHotAuth {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;

        let proposal_index = CertificateIndexNames::CommitteeHotAuth.to_u64();
        serialize_and_check_index(serializer, proposal_index, "CommitteeHotAuth")?;

        self.committee_cold_credential.serialize(serializer)?;
        self.committee_hot_credential.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(CommitteeHotAuth);

impl DeserializeEmbeddedGroup for CommitteeHotAuth {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            3,
            "(cert_index, committee_cold_key, committee_hot_key)",
        )?;

        let cert_index = CertificateIndexNames::CommitteeHotAuth.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let committee_cold_key =
            Credential::deserialize(raw).map_err(|e| e.annotate("committee_cold_key"))?;

        let committee_hot_key =
            Credential::deserialize(raw).map_err(|e| e.annotate("committee_hot_key"))?;

        Ok(CommitteeHotAuth {
            committee_cold_credential: committee_cold_key,
            committee_hot_credential: committee_hot_key,
        })
    }
}
