use num_traits::ToPrimitive;
use crate::*;
use crate::serialization::map_names::CertificateIndexNames;
use crate::serialization::struct_checks::{check_len, deserialize_and_check_index, serialize_and_check_index};

impl cbor_event::se::Serialize for StakeVoteRegistrationAndDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(5))?;

        let proposal_index = CertificateIndexNames::StakeVoteRegistrationAndDelegation.to_u64();
        serialize_and_check_index(serializer, proposal_index, "StakeVoteRegistrationAndDelegation")?;

        self.stake_credential.serialize(serializer)?;
        self.pool_keyhash.serialize(serializer)?;
        self.drep.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_tuple!(StakeVoteRegistrationAndDelegation);

impl DeserializeEmbeddedGroup for StakeVoteRegistrationAndDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 5, "(cert_index, stake_credential, pool_keyhash, drep, coin)")?;
        let cert_index = CertificateIndexNames::StakeVoteRegistrationAndDelegation.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let stake_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        let pool_keyhash =
            Ed25519KeyHash::deserialize(raw).map_err(|e| e.annotate("pool_keyhash"))?;

        let drep = DRep::deserialize(raw).map_err(|e| e.annotate("drep"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        Ok(StakeVoteRegistrationAndDelegation {
            stake_credential,
            pool_keyhash,
            drep,
            coin,
        })
    }
}
