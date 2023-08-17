use crate::serialization::map_names::CertificateIndexNames;
use crate::serialization::struct_checks::{
    check_len, deserialize_and_check_index, serialize_and_check_index,
};
use crate::*;
use num_traits::ToPrimitive;

impl cbor_event::se::Serialize for VoteDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;

        let proposal_index = CertificateIndexNames::VoteDelegation.to_u64();
        serialize_and_check_index(serializer, proposal_index, "VoteDelegation")?;

        self.stake_credential.serialize(serializer)?;
        self.drep.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_tuple!(VoteDelegation);

impl DeserializeEmbeddedGroup for VoteDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 3, "(cert_index, stake_credential, drep)")?;
        let cert_index = CertificateIndexNames::VoteDelegation.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let stake_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        let drep = DRep::deserialize(raw).map_err(|e| e.annotate("drep"))?;

        Ok(VoteDelegation {
            stake_credential,
            drep,
        })
    }
}
