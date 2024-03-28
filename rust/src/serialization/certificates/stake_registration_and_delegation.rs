use crate::serialization::map_names::CertificateIndexNames;
use crate::serialization::utils::{
    check_len, deserialize_and_check_index, serialize_and_check_index,
};
use crate::*;
use num_traits::ToPrimitive;

impl cbor_event::se::Serialize for StakeRegistrationAndDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;

        let proposal_index = CertificateIndexNames::StakeRegistrationAndDelegation.to_u64();
        serialize_and_check_index(serializer, proposal_index, "StakeRegistrationAndDelegation")?;

        self.stake_credential.serialize(serializer)?;
        self.pool_keyhash.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(StakeRegistrationAndDelegation);

impl DeserializeEmbeddedGroup for StakeRegistrationAndDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 4, "(cert_index, stake_credential, pool_keyhash, coin)")?;
        let cert_index = CertificateIndexNames::StakeRegistrationAndDelegation.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let stake_credential =
            Credential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        let pool_keyhash =
            Ed25519KeyHash::deserialize(raw).map_err(|e| e.annotate("pool_keyhash"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        Ok(StakeRegistrationAndDelegation {
            stake_credential,
            pool_keyhash,
            coin,
        })
    }
}
