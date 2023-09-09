use crate::*;

impl cbor_event::se::Serialize for TransactionBody {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(
            3 + opt64(&self.ttl)
                + opt64(&self.certs)
                + opt64(&self.withdrawals)
                + opt64(&self.update)
                + opt64(&self.auxiliary_data_hash)
                + opt64(&self.validity_start_interval)
                + opt64(&self.mint)
                + opt64(&self.script_data_hash)
                + opt64(&self.collateral)
                + opt64(&self.required_signers)
                + opt64(&self.network_id)
                + opt64(&self.collateral_return)
                + opt64(&self.total_collateral)
                + opt64(&self.reference_inputs)
                + opt64(&self.voting_procedures)
                + opt64(&self.voting_proposals)
                + opt64(&self.donation)
                + opt64(&self.current_treasury_value),
        ))?;
        serializer.write_unsigned_integer(0)?;
        self.inputs.serialize(serializer)?;
        serializer.write_unsigned_integer(1)?;
        self.outputs.serialize(serializer)?;
        serializer.write_unsigned_integer(2)?;
        self.fee.serialize(serializer)?;
        if let Some(field) = &self.ttl {
            serializer.write_unsigned_integer(3)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.certs {
            serializer.write_unsigned_integer(4)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.withdrawals {
            serializer.write_unsigned_integer(5)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.update {
            serializer.write_unsigned_integer(6)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.auxiliary_data_hash {
            serializer.write_unsigned_integer(7)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.validity_start_interval {
            serializer.write_unsigned_integer(8)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.mint {
            serializer.write_unsigned_integer(9)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.script_data_hash {
            serializer.write_unsigned_integer(11)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.collateral {
            serializer.write_unsigned_integer(13)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.required_signers {
            serializer.write_unsigned_integer(14)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.network_id {
            serializer.write_unsigned_integer(15)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.collateral_return {
            serializer.write_unsigned_integer(16)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.total_collateral {
            serializer.write_unsigned_integer(17)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.reference_inputs {
            serializer.write_unsigned_integer(18)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.voting_procedures {
            serializer.write_unsigned_integer(19)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.voting_proposals {
            serializer.write_unsigned_integer(20)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.current_treasury_value {
            serializer.write_unsigned_integer(21)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.donation {
            serializer.write_unsigned_integer(22)?;
            field.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(3)?;
            let mut inputs = None;
            let mut outputs = None;
            let mut fee = None;
            let mut ttl = None;
            let mut certs = None;
            let mut withdrawals = None;
            let mut update = None;
            let mut auxiliary_data_hash = None;
            let mut validity_start_interval = None;
            let mut mint = None;
            let mut script_data_hash = None;
            let mut collateral = None;
            let mut required_signers = None;
            let mut network_id = None;
            let mut collateral_return = None;
            let mut total_collateral = None;
            let mut reference_inputs = None;
            let mut voting_procedures = None;
            let mut voting_proposals = None;
            let mut current_treasury_value = None;
            let mut donation = None;
            let mut read = 0;
            while match len {
                cbor_event::Len::Len(n) => read < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                        0 => {
                            if inputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            inputs = Some(
                                (|| -> Result<_, DeserializeError> {
                                    Ok(TransactionInputs::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("inputs"))?,
                            );
                        }
                        1 => {
                            if outputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            outputs = Some(
                                (|| -> Result<_, DeserializeError> {
                                    Ok(TransactionOutputs::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("outputs"))?,
                            );
                        }
                        2 => {
                            if fee.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            fee =
                                Some(
                                    (|| -> Result<_, DeserializeError> {
                                        Ok(Coin::deserialize(raw)?)
                                    })()
                                        .map_err(|e| e.annotate("fee"))?,
                                );
                        }
                        3 => {
                            if ttl.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            ttl = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(SlotBigNum::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("ttl"))?,
                            );
                        }
                        4 => {
                            if certs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            certs = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Certificates::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("certs"))?,
                            );
                        }
                        5 => {
                            if withdrawals.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            withdrawals = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Withdrawals::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("withdrawals"))?,
                            );
                        }
                        6 => {
                            if update.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            update = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Update::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("update"))?,
                            );
                        }
                        7 => {
                            if auxiliary_data_hash.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            auxiliary_data_hash = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(AuxiliaryDataHash::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("auxiliary_data_hash"))?,
                            );
                        }
                        8 => {
                            if validity_start_interval.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(8)).into());
                            }
                            validity_start_interval = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(SlotBigNum::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("validity_start_interval"))?,
                            );
                        }
                        9 => {
                            if mint.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(9)).into());
                            }
                            mint = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Mint::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("mint"))?,
                            );
                        }
                        11 => {
                            if script_data_hash.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(11)).into());
                            }
                            script_data_hash = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(ScriptDataHash::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("script_data_hash"))?,
                            );
                        }
                        13 => {
                            if collateral.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(13)).into());
                            }
                            collateral = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(TransactionInputs::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("collateral"))?,
                            );
                        }
                        14 => {
                            if required_signers.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(14)).into());
                            }
                            required_signers = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(RequiredSigners::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("required_signers"))?,
                            );
                        }
                        15 => {
                            if network_id.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(15)).into());
                            }
                            network_id = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(NetworkId::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("network_id"))?,
                            );
                        }
                        16 => {
                            if collateral_return.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(16)).into());
                            }
                            collateral_return = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(TransactionOutput::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("collateral_return"))?,
                            );
                        }
                        17 => {
                            if total_collateral.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(17)).into());
                            }
                            total_collateral = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("total_collateral"))?,
                            );
                        }
                        18 => {
                            if reference_inputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(18)).into());
                            }
                            reference_inputs = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(TransactionInputs::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("reference_inputs"))?,
                            );
                        }
                        19 => {
                            if voting_procedures.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(19)).into());
                            }
                            voting_procedures = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(VotingProcedures::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("voting_procedures"))?,
                            );
                        }
                        20 => {
                            if voting_proposals.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(20)).into());
                            }
                            voting_proposals = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(VotingProposals::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("voting_proposals"))?,
                            );
                        }
                        21 => {
                            if current_treasury_value.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(21)).into());
                            }
                            current_treasury_value = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("current_treasury_value"))?,
                            );
                        }
                        22 => {
                            if donation.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(22)).into());
                            }
                            donation = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("donation"))?,
                            );
                        }
                        unknown_key => {
                            return Err(
                                DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()
                            )
                        }
                    },
                    CBORType::Text => match raw.text()?.as_str() {
                        unknown_key => {
                            return Err(DeserializeFailure::UnknownKey(Key::Str(
                                unknown_key.to_owned(),
                            ))
                                .into())
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::Len::Len(_) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => {
                        return Err(DeserializeFailure::UnexpectedKeyType(other_type).into())
                    }
                }
                read += 1;
            }
            let inputs = match inputs {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(0)).into()),
            };
            let outputs = match outputs {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
            };
            let fee = match fee {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(2)).into()),
            };
            read_len.finish()?;
            Ok(Self {
                inputs,
                outputs,
                fee,
                ttl,
                certs,
                withdrawals,
                update,
                auxiliary_data_hash,
                validity_start_interval,
                mint,
                script_data_hash,
                collateral,
                required_signers,
                network_id,
                collateral_return,
                total_collateral,
                reference_inputs,
                voting_procedures,
                voting_proposals,
                donation,
                current_treasury_value,
            })
        })()
            .map_err(|e| e.annotate("TransactionBody"))
    }
}