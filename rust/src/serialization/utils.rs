use std::io::SeekFrom;
use crate::*;

/// TODO: this function can be removed in case `cbor_event` library ever gets a fix on their side
/// See https://github.com/Emurgo/cardano-serialization-lib/pull/392
pub(crate) fn read_nint<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<i128, DeserializeError> {
    let found = raw.cbor_type()?;
    if found != cbor_event::Type::NegativeInteger {
        return Err(cbor_event::Error::Expected(cbor_event::Type::NegativeInteger, found).into());
    }
    let (len, len_sz) = raw.cbor_len()?;
    match len {
        cbor_event::Len::Indefinite => Err(cbor_event::Error::IndefiniteLenNotSupported(
            cbor_event::Type::NegativeInteger,
        )
            .into()),
        cbor_event::Len::Len(v) => {
            raw.advance(1 + len_sz)?;
            Ok(-(v as i128) - 1)
        }
    }
}

pub(super) fn deserialize_and_check_index<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    desired_index: Option<u64>,
    name: &'static str,
) -> Result<u64, DeserializeError> {
    let actual_index = raw.unsigned_integer()?;
    check_index(actual_index, desired_index, name)?;
    Ok(actual_index)
}

pub(super) fn check_index(
    actual_index: u64,
    desired_index: Option<u64>,
    name: &'static str,
) -> Result<(), DeserializeError> {
    let desired_index = desired_index
        .ok_or(DeserializeFailure::CustomError(
            "unknown desired index".to_string(),
        ))
        .map_err(|e| DeserializeError::from(e))?;
    if actual_index != desired_index {
        return Err(DeserializeFailure::FixedValueMismatch {
            found: Key::Uint(actual_index),
            expected: Key::Uint(desired_index),
        })
        .map_err(|e| DeserializeError::from(e).annotate(name));
    }

    Ok(())
}

pub(super) fn serialize_and_check_index<'se, W: Write>(
    serializer: &'se mut Serializer<W>,
    index: Option<u64>,
    name: &'static str,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    match index {
        Some(index) => serializer.write_unsigned_integer(index),
        None => Err(cbor_event::Error::CustomError(format!(
            "unknown index of {}",
            name
        ))),
    }
}

pub(super) fn check_len(
    len: cbor_event::Len,
    expected: u64,
    struct_description: &'static str,
) -> Result<(), DeserializeError> {
    if let cbor_event::Len::Len(n) = len {
        if n != expected {
            return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                expected as u64,
                len,
                struct_description,
            ))
            .into());
        }
    }
    Ok(())
}

pub(super) fn check_len_indefinite<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    len: cbor_event::Len,
) -> Result<(), DeserializeError> {
    if let cbor_event::Len::Indefinite = len {
        if raw.special()? != CBORSpecial::Break {
            return Err(DeserializeFailure::EndingBreakMissing.into());
        }
    }
    Ok(())
}

pub(crate) fn merge_option_plutus_list(
    left: Option<PlutusScripts>,
    right: Option<PlutusScripts>,
) -> Option<PlutusScripts> {
    if let Some(left) = left {
        if let Some(right) = right {
            return Some(left.merge(&right));
        } else {
            return Some(left);
        }
    } else {
        return right;
    }
}

pub(super) fn skip_tag<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    tag: u64,
) -> Result<bool, DeserializeError> {
    if let Ok(extracted_tag) = raw.tag() {
        if extracted_tag != tag {
            return Err(DeserializeError::new(
                "skip_tag",
                DeserializeFailure::TagMismatch {
                    found: extracted_tag,
                    expected: tag,
                },
            ));
        }
        return Ok(true);
    }
    Ok(false)
}

pub(super) fn skip_set_tag<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<bool, DeserializeError> {
    skip_tag(raw, 258)
}

pub(crate) fn is_break_tag<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    location: &str,
) -> Result<bool, DeserializeError> {
    if raw.cbor_type()? == CBORType::Special {
        if raw.special()? == CBORSpecial::Break {
            return Ok(true);
        }
        return Err(
            DeserializeError::from(DeserializeFailure::EndingBreakMissing).annotate(location),
        );
    }
    Ok(false)
}

pub(crate) fn deserilized_with_orig_bytes<R: BufRead + Seek, T>(
    raw: &mut Deserializer<R>,
    deserializer: fn(&mut Deserializer<R>) -> Result<T, DeserializeError>,
) -> Result<(T, Vec<u8>), DeserializeError> {
    let before = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
    let value = deserializer(raw)?;
    let after = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
    let bytes_read = (after - before) as usize;
    raw.as_mut_ref().seek(SeekFrom::Start(before)).unwrap();
    let original_bytes = raw.as_mut_ref().fill_buf().unwrap()[..bytes_read].to_vec();
    raw.as_mut_ref().seek(SeekFrom::Start(after)).unwrap();
    Ok((value, original_bytes))
}