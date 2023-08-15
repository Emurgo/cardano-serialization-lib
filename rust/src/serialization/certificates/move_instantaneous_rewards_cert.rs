use crate::*;

pub(super) const MIR_CERT_INDEX: u64 = 6;

impl cbor_event::se::Serialize for MIRToStakeCredentials {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.rewards.len() as u64))?;
        for (key, value) in &self.rewards {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for MIRToStakeCredentials {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let mut table = linked_hash_map::LinkedHashMap::new();
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => table.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = StakeCredential::deserialize(raw)?;
                let value = DeltaCoin::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(format!(
                        "StakeCred: {} (hex bytes)",
                        hex::encode(key.to_bytes())
                    )))
                    .into());
                }
            }
            Ok(Self { rewards: table })
        })()
        .map_err(|e| e.annotate("MIRToStakeCredentials"))
    }
}

impl cbor_event::se::Serialize for MoveInstantaneousReward {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        match self.pot {
            MIRPot::Reserves => serializer.write_unsigned_integer(0u64),
            MIRPot::Treasury => serializer.write_unsigned_integer(1u64),
        }?;
        match &self.variant {
            MIREnum::ToOtherPot(amount) => amount.serialize(serializer),
            MIREnum::ToStakeCredentials(amounts) => amounts.serialize(serializer),
        }
    }
}

impl Deserialize for MoveInstantaneousReward {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let outer_len = raw.array()?;
            let pot = match raw.unsigned_integer()? {
                0 => MIRPot::Reserves,
                1 => MIRPot::Treasury,
                n => return Err(DeserializeFailure::UnknownKey(Key::Uint(n)).into()),
            };
            let variant = match raw.cbor_type()? {
                CBORType::UnsignedInteger => MIREnum::ToOtherPot(Coin::deserialize(raw)?),
                CBORType::Map => {
                    MIREnum::ToStakeCredentials(MIRToStakeCredentials::deserialize(raw)?)
                }
                _ => return Err(DeserializeFailure::NoVariantMatched.into()),
            };
            match outer_len {
                cbor_event::Len::Len(n) => {
                    if n != 2 {
                        return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                            n,
                            outer_len,
                            "MoveInstantaneousReward",
                        ))
                        .into());
                    }
                }
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break =>
                    /* it's ok */
                    {
                        ()
                    }
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            };
            Ok(Self { pot, variant })
        })()
        .map_err(|e| e.annotate("MoveInstantaneousReward"))
    }
}

impl cbor_event::se::Serialize for MoveInstantaneousRewardsCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for MoveInstantaneousRewardsCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(MIR_CERT_INDEX)?;
        self.move_instantaneous_reward.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MoveInstantaneousRewardsCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) =>
                /* TODO: check finite len somewhere */
                {
                    ()
                }
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break =>
                    /* it's ok */
                    {
                        ()
                    }
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
        .map_err(|e| e.annotate("MoveInstantaneousRewardsCert"))
    }
}

impl DeserializeEmbeddedGroup for MoveInstantaneousRewardsCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let cert_index = raw.unsigned_integer()?;
            if cert_index != MIR_CERT_INDEX {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(cert_index),
                    expected: Key::Uint(MIR_CERT_INDEX),
                }
                .into());
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("cert_index"))?;
        let move_instantaneous_reward =
            (|| -> Result<_, DeserializeError> { Ok(MoveInstantaneousReward::deserialize(raw)?) })(
            )
            .map_err(|e| e.annotate("move_instantaneous_reward"))?;
        Ok(MoveInstantaneousRewardsCert {
            move_instantaneous_reward,
        })
    }
}
