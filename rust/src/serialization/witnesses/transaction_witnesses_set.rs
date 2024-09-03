use std::io::{BufRead, Seek, Write};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cbor_event::Serialize;
use crate::{BootstrapWitnesses, CBORReadLen, DeserializeError, DeserializeFailure, Key, Language, NativeScripts, PlutusList, PlutusScripts, Redeemers, TransactionWitnessSet, Vkeywitnesses};
use crate::protocol_types::{CBORSpecial, CBORType, Deserialize, opt64, TransactionWitnessSetRaw};
use crate::serialization::utils::{deserilized_with_orig_bytes, merge_option_plutus_list};
use crate::traits::NoneOrEmpty;
use crate::utils::opt64_non_empty;

impl cbor_event::se::Serialize for TransactionWitnessSet {
    fn serialize<'a, W: Write + Sized>(&self, serializer: &'a mut Serializer<W>) -> cbor_event::Result<&'a mut Serializer<W>> {
        serialize(self, None, serializer)
    }
}

impl Deserialize for TransactionWitnessSet {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError>
    where
        Self: Sized
    {
        let (witness_set, _) = deserialize(raw, false)?;
        Ok(witness_set)
    }
}

pub(super) fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>, with_raw_parts: bool) -> Result<(TransactionWitnessSet, TransactionWitnessSetRaw), DeserializeError> {
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
        let mut raw_part = TransactionWitnessSetRaw::new();

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
                        read_len.read_elems(1)?;
                        let (vkeys_deser, raw) = deserilized_with_orig_bytes(raw, |raw|
                            Vkeywitnesses::deserialize(raw)
                        ).map_err(|e| e.annotate("vkeys"))?;

                        vkeys = Some(vkeys_deser);
                        if with_raw_parts {
                            raw_part.vkeys = Some(raw);
                        }
                    }
                    1 => {
                        if native_scripts.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                        }
                        read_len.read_elems(1)?;
                        let (native_scripts_deser, raw) = deserilized_with_orig_bytes(raw, |raw|
                            NativeScripts::deserialize(raw)
                        ).map_err(|e| e.annotate("native_scripts"))?;
                        native_scripts = Some(native_scripts_deser);
                        if with_raw_parts {
                            raw_part.native_scripts = Some(raw);
                        }
                    }
                    2 => {
                        if bootstraps.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                        }
                        read_len.read_elems(1)?;
                        let (bootstraps_deser, raw) = deserilized_with_orig_bytes(raw, |raw|
                            BootstrapWitnesses::deserialize(raw)
                        ).map_err(|e| e.annotate("bootstraps"))?;
                        bootstraps = Some(bootstraps_deser);
                        if with_raw_parts {
                            raw_part.bootstraps = Some(raw);
                        }
                    }
                    3 => {
                        if plutus_scripts_v1.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                        }
                        read_len.read_elems(1)?;
                        let (plutus_scripts_v1_deser, raw) = deserilized_with_orig_bytes(raw, |raw|
                            PlutusScripts::deserialize_with_version(raw, &Language::new_plutus_v1())
                        ).map_err(|e| e.annotate("plutus_scripts_v1"))?;
                        plutus_scripts_v1 = Some(plutus_scripts_v1_deser);
                        if with_raw_parts {
                            raw_part.plutus_scripts_v1 = Some(raw);
                        }
                    }
                    4 => {
                        if plutus_data.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                        }
                        read_len.read_elems(1)?;
                        let (plutus_data_deser, raw) = deserilized_with_orig_bytes(raw, |raw|
                            PlutusList::deserialize(raw)
                        ).map_err(|e| e.annotate("plutus_data"))?;
                        plutus_data = Some(plutus_data_deser);
                        if with_raw_parts {
                            raw_part.plutus_data = Some(raw);
                        }
                    }
                    5 => {
                        if redeemers.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                        }
                        read_len.read_elems(1)?;
                        let (redeemers_deser, raw) = deserilized_with_orig_bytes(raw, |raw|
                            Redeemers::deserialize(raw)
                        ).map_err(|e| e.annotate("redeemers"))?;
                        redeemers = Some(redeemers_deser);
                        if with_raw_parts {
                            raw_part.redeemers = Some(raw);
                        }
                    }
                    6 => {
                        if plutus_scripts_v2.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                        }
                        read_len.read_elems(1)?;
                        let (plutus_scripts_v2_deser, raw) = deserilized_with_orig_bytes(raw, |raw|
                            PlutusScripts::deserialize_with_version(raw, &Language::new_plutus_v2())
                        ).map_err(|e| e.annotate("plutus_scripts_v2"))?;
                        plutus_scripts_v2 = Some(plutus_scripts_v2_deser);
                        if with_raw_parts {
                            raw_part.plutus_scripts_v2 = Some(raw);
                        }
                    }
                    7 => {
                        if plutus_scripts_v3.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                        }
                        read_len.read_elems(1)?;
                        let (plutus_scripts_v3_deser, raw) = deserilized_with_orig_bytes(raw, |raw|
                            PlutusScripts::deserialize_with_version(raw, &Language::new_plutus_v3())
                        ).map_err(|e| e.annotate("plutus_scripts_v3"))?;
                        plutus_scripts_v3 = Some(plutus_scripts_v3_deser);
                        if with_raw_parts {
                            raw_part.plutus_scripts_v3 = Some(raw);
                        }
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

        Ok((TransactionWitnessSet {
            vkeys,
            native_scripts,
            bootstraps,
            plutus_scripts,
            plutus_data,
            redeemers,
        }, raw_part))
    })()
        .map_err(|e| e.annotate("TransactionWitnessSet"))
}

pub(super) fn serialize<'se, W: Write>(
    wit_set: &TransactionWitnessSet,
    raw_parts: Option<&TransactionWitnessSetRaw>,
    serializer: &'se mut Serializer<W>,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    let mut has_plutus_v1 = false;
    let mut has_plutus_v2 = false;
    let mut has_plutus_v3 = false;
    let plutus_added_length = match &wit_set.plutus_scripts {
        Some(scripts) => {
            has_plutus_v1 = scripts.has_version(&Language::new_plutus_v1());
            has_plutus_v2 = scripts.has_version(&Language::new_plutus_v2());
            has_plutus_v3 = scripts.has_version(&Language::new_plutus_v3());
            (has_plutus_v1 as u64) + (has_plutus_v2 as u64) + (has_plutus_v3 as u64)
        },
        _ => 0,
    };
    serializer.write_map(cbor_event::Len::Len(
        opt64(&wit_set.vkeys)
            + opt64_non_empty(&wit_set.native_scripts)
            + opt64_non_empty(&wit_set.bootstraps)
            + opt64_non_empty(&wit_set.plutus_data)
            + opt64_non_empty(&wit_set.redeemers)
            + plutus_added_length,
    ))?;
    if let Some(field) = &wit_set.vkeys {
        if let Some(raw_vkeys) = raw_parts.map(|x| x.vkeys.as_ref()).flatten() {
            serializer.write_unsigned_integer(0)?;
            serializer.write_bytes(raw_vkeys)?;
        } else if !field.is_none_or_empty() {
            serializer.write_unsigned_integer(0)?;
            field.serialize(serializer)?;
        }
    }
    if let Some(field) = &wit_set.native_scripts {
        if let Some(raw) = raw_parts.as_ref().map(|x| x.native_scripts.as_ref()).flatten() {
            serializer.write_unsigned_integer(1)?;
            serializer.write_bytes(raw)?;
        } else if !field.is_none_or_empty() {
            serializer.write_unsigned_integer(1)?;
            //transaction witness set already has deduplicated native scripts
            field.serialize_as_set(false, serializer)?;
        }
    }
    if let Some(field) = &wit_set.bootstraps {
        if let Some(raw) = raw_parts.as_ref().map(|x| x.bootstraps.as_ref()).flatten() {
            serializer.write_unsigned_integer(2)?;
            serializer.write_bytes(raw)?;
        } else if !field.is_none_or_empty() {
            serializer.write_unsigned_integer(2)?;
            field.serialize(serializer)?;
        }
    }

    //no need deduplication here because transaction witness set already has deduplicated plutus scripts
    if let Some(plutus_scripts) = &wit_set.plutus_scripts {
        if has_plutus_v1 {
            if let Some(raw) = raw_parts.as_ref().map(|x| x.plutus_scripts_v1.as_ref()).flatten() {
                serializer.write_unsigned_integer(3)?;
                serializer.write_bytes(raw)?;
            } else {
                serializer.write_unsigned_integer(3)?;
                plutus_scripts.serialize_as_set_by_version(false, &Language::new_plutus_v1(), serializer)?;
            }
        }
        if has_plutus_v2 {
            if let Some(raw) = raw_parts.as_ref().map(|x| x.plutus_scripts_v2.as_ref()).flatten() {
                serializer.write_unsigned_integer(6)?;
                serializer.write_bytes(raw)?;
            } else {
                serializer.write_unsigned_integer(6)?;
                plutus_scripts.serialize_as_set_by_version(false, &Language::new_plutus_v2(), serializer)?;
            }
        }
        if has_plutus_v3 {
            if let Some(raw) = raw_parts.as_ref().map(|x| x.plutus_scripts_v3.as_ref()).flatten() {
                serializer.write_unsigned_integer(7)?;
                serializer.write_bytes(raw)?;
            } else {
                serializer.write_unsigned_integer(7)?;
                plutus_scripts.serialize_as_set_by_version(false, &Language::new_plutus_v3(), serializer)?;
            }
        }
    }
    if let Some(field) = &wit_set.plutus_data {
        if let Some(raw) = raw_parts.as_ref().map(|x| x.plutus_data.as_ref()).flatten() {
            serializer.write_unsigned_integer(4)?;
            serializer.write_bytes(&raw)?;
        } else if !field.is_none_or_empty() {
            serializer.write_unsigned_integer(4)?;
            //transaction witness set already has deduplicated plutus data
            field.serialize_as_set(false, serializer)?;
        }
    }
    if let Some(field) = &wit_set.redeemers {
        if let Some(raw) = raw_parts.as_ref().map(|x| x.redeemers.as_ref()).flatten() {
            serializer.write_unsigned_integer(5)?;
            serializer.write_bytes(raw)?;
        } else if !field.is_none_or_empty() {
            serializer.write_unsigned_integer(5)?;
            field.serialize(serializer)?;
        }
    }
    Ok(serializer)
}
