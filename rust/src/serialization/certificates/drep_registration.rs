use crate::serialization::map_names::CertificateIndexNames;
use crate::serialization::utils::{
    check_len, deserialize_and_check_index, serialize_and_check_index,
};
use crate::*;
use num_traits::ToPrimitive;

impl cbor_event::se::Serialize for DRepRegistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;

        let proposal_index = CertificateIndexNames::DRepRegistration.to_u64();
        serialize_and_check_index(serializer, proposal_index, "DRepRegistration")?;

        self.voting_credential.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        match &self.anchor {
            Some(anchor) => anchor.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(DRepRegistration);

impl DeserializeEmbeddedGroup for DRepRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            4,
            "(cert_index, voting_credential, coin, anchor / null)",
        )?;

        let cert_index = CertificateIndexNames::DRepRegistration.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let voting_credential =
            Credential::deserialize(raw).map_err(|e| e.annotate("voting_credential"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        let anchor = Anchor::deserialize_nullable(raw).map_err(|e| e.annotate("anchor"))?;

        Ok(DRepRegistration {
            voting_credential,
            coin,
            anchor,
        })
    }
}
