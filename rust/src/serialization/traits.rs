use crate::*;
// we use the cbor_event::Serialize trait directly

// This is only for use for plain cddl groups who need to be embedded within outer groups.
pub(crate) trait SerializeEmbeddedGroup {
    fn serialize_as_embedded_group<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>>;
}

// same as cbor_event::de::Deserialize but with our DeserializeError
pub trait Deserialize {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

// auto-implement for all cbor_event Deserialize implementors
impl<T: cbor_event::de::Deserialize> Deserialize for T {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<T, DeserializeError> {
        T::deserialize(raw).map_err(|e| DeserializeError::from(e))
    }
}

// This is only for use for plain cddl groups who need to be embedded within outer groups.
pub trait DeserializeEmbeddedGroup {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

pub trait DeserializeNullable {
    fn deserialize_nullable<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<Option<Self>, DeserializeError>
    where
        Self: Sized;
}

impl<T: Deserialize> DeserializeNullable for T {
    fn deserialize_nullable<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<Option<Self>, DeserializeError>
    where
        Self: Sized,
    {
        if raw.cbor_type()? == CBORType::Special {
            if raw.special()? != CBORSpecial::Null {
                return Err(DeserializeFailure::ExpectedNull.into());
            }
            Ok(None)
        } else {
            Ok(Some(T::deserialize(raw)?))
        }
    }
}

pub trait SerializeNullable {
    fn serialize_nullable<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>>;
}

impl<T: Serialize> SerializeNullable for Option<T> {
    fn serialize_nullable<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>> {
        match self {
            Some(x) => x.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }
    }
}
