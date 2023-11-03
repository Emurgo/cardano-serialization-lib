use crate::*;

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

pub(super) fn skip_tag_wrapper<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    tag: u64,
) -> Result<(), DeserializeError> {
    if let Ok(extracted_tag) = raw.tag() {
        if extracted_tag != tag {
            return Err(DeserializeError::new(
                "skip_tag_wrapper",
                DeserializeFailure::TagMismatch {
                    found: extracted_tag,
                    expected: tag,
                },
            ));
        }
        return Ok(());
    }
    Ok(())
}

pub(super) fn skip_set_tag_wrapperr<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<(), DeserializeError> {
    skip_tag_wrapper(raw, 258)
}
