use std::io::{BufRead, Seek, Write};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use crate::{BootstrapWitnesses, CBORReadLen, DeserializeError, DeserializeFailure, Key, Language, NativeScripts, PlutusList, PlutusScripts, Redeemers, TransactionWitnessSet, Vkeywitnesses};
use crate::protocol_types::{CBORSpecial, CBORType, Deserialize, opt64};
use crate::serialization::utils::merge_option_plutus_list;
use crate::traits::NoneOrEmpty;
use crate::utils::opt64_non_empty;

impl cbor_event::se::Serialize for TransactionWitnessSet {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        let mut has_plutus_v1 = false;
        let mut has_plutus_v2 = false;
        let mut has_plutus_v3 = false;
        let plutus_added_length = match &self.plutus_scripts {
            Some(scripts) => {
                has_plutus_v1 = scripts.has_version(&Language::new_plutus_v1());
                has_plutus_v2 = scripts.has_version(&Language::new_plutus_v2());
                has_plutus_v3 = scripts.has_version(&Language::new_plutus_v3());
                (has_plutus_v1 as u64) + (has_plutus_v2 as u64) + (has_plutus_v3 as u64)
            },
            _ => 0,
        };
        serializer.write_map(cbor_event::Len::Len(
            opt64(&self.vkeys)
                + opt64_non_empty(&self.native_scripts)
                + opt64_non_empty(&self.bootstraps)
                + opt64_non_empty(&self.plutus_data)
                + opt64_non_empty(&self.redeemers)
                + plutus_added_length,
        ))?;
        if let Some(field) = &self.vkeys {
            if !field.is_none_or_empty() {
                serializer.write_unsigned_integer(0)?;
                field.serialize(serializer)?;
            }
        }
        if let Some(field) = &self.native_scripts {
            if !field.is_none_or_empty() {
                serializer.write_unsigned_integer(1)?;
                field.serialize(serializer)?;
            }
        }
        if let Some(field) = &self.bootstraps {
            if !field.is_none_or_empty() {
                serializer.write_unsigned_integer(2)?;
                field.serialize(serializer)?;
            }
        }

        //no need deduplication here because transaction witness set already has deduplicated plutus scripts
        if let Some(plutus_scripts) = &self.plutus_scripts {
            if has_plutus_v1 {
                serializer.write_unsigned_integer(3)?;
                plutus_scripts.serialize_as_set_by_version(false, &Language::new_plutus_v1(), serializer)?;
            }
            if has_plutus_v2 {
                serializer.write_unsigned_integer(6)?;
                plutus_scripts.serialize_as_set_by_version(false, &Language::new_plutus_v2(), serializer)?;
            }
            if has_plutus_v3 {
                serializer.write_unsigned_integer(7)?;
                plutus_scripts.serialize_as_set_by_version(false, &Language::new_plutus_v3(), serializer)?;
            }
        }
        if let Some(field) = &self.plutus_data {
            if !field.is_none_or_empty() {
                serializer.write_unsigned_integer(4)?;
                field.serialize(serializer)?;
            }
        }
        if let Some(field) = &self.redeemers {
            if !field.is_none_or_empty() {
                serializer.write_unsigned_integer(5)?;
                field.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionWitnessSet {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(len);
            let mut vkeys = None;
            let mut native_scripts = None;
            let mut bootstraps = None;
            let mut plutus_scripts_v1 = None;
            let mut plutus_scripts_v2 = None;
            let mut plutus_scripts_v3 = None;
            let mut plutus_data = None;
            let mut redeemers = None;
            let mut read = 0;
            while match len {
                cbor_event::Len::Len(n) => read < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                        0 => {
                            if vkeys.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            vkeys = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Vkeywitnesses::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("vkeys"))?,
                            );
                        }
                        1 => {
                            if native_scripts.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            native_scripts = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(NativeScripts::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("native_scripts"))?,
                            );
                        }
                        2 => {
                            if bootstraps.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            bootstraps = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(BootstrapWitnesses::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("bootstraps"))?,
                            );
                        }
                        3 => {
                            if plutus_scripts_v1.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            plutus_scripts_v1 = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(PlutusScripts::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("plutus_scripts_v1"))?,
                            );
                        }
                        4 => {
                            if plutus_data.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            plutus_data = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(PlutusList::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("plutus_data"))?,
                            );
                        }
                        5 => {
                            if redeemers.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            redeemers = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Redeemers::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("redeemers"))?,
                            );
                        }
                        6 => {
                            if plutus_scripts_v2.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            plutus_scripts_v2 = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(PlutusScripts::deserialize_with_version(raw, &Language::new_plutus_v2())?)
                                })()
                                    .map_err(|e| e.annotate("plutus_scripts_v2"))?,
                            );
                        }
                        7 => {
                            if plutus_scripts_v3.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            plutus_scripts_v3 = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(PlutusScripts::deserialize_with_version(raw, &Language::new_plutus_v3())?)
                                })()
                                    .map_err(|e| e.annotate("plutus_scripts_v3"))?,
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
            read_len.finish()?;
            let mut plutus_scripts = None;
            plutus_scripts = merge_option_plutus_list(plutus_scripts, plutus_scripts_v1);
            plutus_scripts = merge_option_plutus_list(plutus_scripts, plutus_scripts_v2);
            plutus_scripts = merge_option_plutus_list(plutus_scripts, plutus_scripts_v3);

            Ok(Self {
                vkeys,
                native_scripts,
                bootstraps,
                plutus_scripts,
                plutus_data,
                redeemers,
            })
        })()
            .map_err(|e| e.annotate("TransactionWitnessSet"))
    }
}