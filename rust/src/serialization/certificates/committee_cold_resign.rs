use crate::serialization::map_names::CertificateIndexNames;
use crate::serialization::utils::{
    check_len, deserialize_and_check_index, serialize_and_check_index,
};
use crate::*;
use num_traits::ToPrimitive;

impl cbor_event::se::Serialize for CommitteeColdResign {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        let proposal_index = CertificateIndexNames::CommitteeColdResign.to_u64();
        serialize_and_check_index(serializer, proposal_index, "CommitteeColdResign")?;

        self.committee_cold_credential.serialize(serializer)?;
        self.anchor.serialize_nullable(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(CommitteeColdResign);

impl DeserializeEmbeddedGroup for CommitteeColdResign {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 3, "(cert_index, committee_cold_key, anchor)")?;

        let cert_index = CertificateIndexNames::CommitteeColdResign.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let committee_cold_key =
            Credential::deserialize(raw).map_err(|e| e.annotate("committee_cold_key"))?;
        let anchor = Anchor::deserialize_nullable(raw).map_err(|e| e.annotate("anchor"))?;

        Ok(CommitteeColdResign { committee_cold_credential: committee_cold_key, anchor })
    }
}
