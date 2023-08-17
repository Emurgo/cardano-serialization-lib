use crate::serialization::map_names::CertificateIndexNames;
use crate::serialization::struct_checks::{
    check_len, deserialize_and_check_index, serialize_and_check_index,
};
use crate::*;
use num_traits::ToPrimitive;

impl cbor_event::se::Serialize for DrepDeregistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;

        let proposal_index = CertificateIndexNames::DrepDeregistration.to_u64();
        serialize_and_check_index(serializer, proposal_index, "DrepDeregistration")?;

        self.voting_credential.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_tuple!(DrepDeregistration);

impl DeserializeEmbeddedGroup for DrepDeregistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 3, "(cert_index, voting_credential, coin)")?;

        let cert_index = CertificateIndexNames::DrepDeregistration.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let voting_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("voting_credential"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        Ok(DrepDeregistration {
            voting_credential,
            coin,
        })
    }
}
