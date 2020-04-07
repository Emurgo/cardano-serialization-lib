use super::*;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct TransactionInput {
    transaction_id: super::Hash,
    index: u32,
}

impl TransactionInput {
    pub (super) fn new(transaction_id: super::Hash, index: u32) -> Self {
        Self {
            transaction_id: transaction_id,
            index: index,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Rust("Hash")
        self.transaction_id.serialize(serializer)?;
        // DEBUG - generated from: Primitive("u32")
        self.index.clone().serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Address0 {
    index_1: super::Keyhash,
    index_2: super::Keyhash,
}

impl Address0 {
    pub (super) fn new(index_1: super::Keyhash, index_2: super::Keyhash) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_2.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Address1 {
    index_1: super::Keyhash,
    index_2: super::Scripthash,
}

impl Address1 {
    pub (super) fn new(index_1: super::Keyhash, index_2: super::Scripthash) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("Scripthash")
        self.index_2.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Address2 {
    index_1: super::Scripthash,
    index_2: super::Keyhash,
}

impl Address2 {
    pub (super) fn new(index_1: super::Scripthash, index_2: super::Keyhash) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2)?;
        // DEBUG - generated from: Rust("Scripthash")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_2.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Address3 {
    index_1: super::Scripthash,
    index_2: super::Scripthash,
}

impl Address3 {
    pub (super) fn new(index_1: super::Scripthash, index_2: super::Scripthash) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(3)?;
        // DEBUG - generated from: Rust("Scripthash")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("Scripthash")
        self.index_2.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Pointer {
    index_0: u32,
    index_1: u32,
    index_2: u32,
}

impl Pointer {
    pub (super) fn new(index_0: u32, index_1: u32, index_2: u32) -> Self {
        Self {
            index_0: index_0,
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Primitive("u32")
        self.index_0.clone().serialize(serializer)?;
        // DEBUG - generated from: Primitive("u32")
        self.index_1.clone().serialize(serializer)?;
        // DEBUG - generated from: Primitive("u32")
        self.index_2.clone().serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Address4 {
    index_1: super::Keyhash,
    index_2: super::Pointer,
}

impl Address4 {
    pub (super) fn new(index_1: super::Keyhash, index_2: super::Pointer) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(4)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("Pointer")
        self.index_2.group.serialize_as_embedded_array_group(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Address5 {
    index_1: super::Scripthash,
    index_2: super::Pointer,
}

impl Address5 {
    pub (super) fn new(index_1: super::Scripthash, index_2: super::Pointer) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(5)?;
        // DEBUG - generated from: Rust("Scripthash")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("Pointer")
        self.index_2.group.serialize_as_embedded_array_group(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Address6 {
    index_1: super::Keyhash,
}

impl Address6 {
    pub (super) fn new(index_1: super::Keyhash) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(6)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Address7 {
    index_1: super::Scripthash,
}

impl Address7 {
    pub (super) fn new(index_1: super::Scripthash) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(7)?;
        // DEBUG - generated from: Rust("Scripthash")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Address8 {
    index_1: super::Keyhash,
}

impl Address8 {
    pub (super) fn new(index_1: super::Keyhash) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(8)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) enum Address {
    Address0(Address0),
    Address1(Address1),
    Address2(Address2),
    Address3(Address3),
    Address4(Address4),
    Address5(Address5),
    Address6(Address6),
    Address7(Address7),
    Address8(Address8),
}

impl Address {
    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Address::Address0(x) => x.serialize_as_array(serializer),
            Address::Address1(x) => x.serialize_as_array(serializer),
            Address::Address2(x) => x.serialize_as_array(serializer),
            Address::Address3(x) => x.serialize_as_array(serializer),
            Address::Address4(x) => x.serialize_as_array(serializer),
            Address::Address5(x) => x.serialize_as_array(serializer),
            Address::Address6(x) => x.serialize_as_array(serializer),
            Address::Address7(x) => x.serialize_as_array(serializer),
            Address::Address8(x) => x.serialize_as_array(serializer),
        }
    }

    pub (super) fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Address::Address0(x) => x.serialize_as_embedded_array_group(serializer),
            Address::Address1(x) => x.serialize_as_embedded_array_group(serializer),
            Address::Address2(x) => x.serialize_as_embedded_array_group(serializer),
            Address::Address3(x) => x.serialize_as_embedded_array_group(serializer),
            Address::Address4(x) => x.serialize_as_embedded_array_group(serializer),
            Address::Address5(x) => x.serialize_as_embedded_array_group(serializer),
            Address::Address6(x) => x.serialize_as_embedded_array_group(serializer),
            Address::Address7(x) => x.serialize_as_embedded_array_group(serializer),
            Address::Address8(x) => x.serialize_as_embedded_array_group(serializer),
        }
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct TransactionOutput {
    index_0: super::Address,
    amount: u32,
}

impl TransactionOutput {
    pub (super) fn new(index_0: super::Address, amount: u32) -> Self {
        Self {
            index_0: index_0,
            amount: amount,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Rust("Address")
        self.index_0.group.serialize_as_embedded_array_group(serializer)?;
        // DEBUG - generated from: Primitive("u32")
        self.amount.clone().serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct TransactionBody {
    key_0: TaggedData<TransactionInputs>,
    key_1: TransactionOutputs,
    key_2: Option<DelegationCertificates>,
    key_3: Option<super::Withdrawals>,
    key_4: super::Coin,
    key_5: u32,
}

impl TransactionBody {
    pub (super) fn new(key_0: TaggedData<TransactionInputs>, key_1: TransactionOutputs, key_4: super::Coin, key_5: u32) -> Self {
        Self {
            key_0: key_0,
            key_1: key_1,
            key_2: None,
            key_3: None,
            key_4: key_4,
            key_5: key_5,
        }
    }

    pub (super) fn serialize_as_map<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_map_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_map_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0)?;
        // DEBUG - generated from: Tagged(258, Array(Rust("TransactionInput")))
        serializer.write_tag(258u64)?;
        // DEBUG - generated from: Array(Rust("TransactionInput"))
        serializer.write_array(cbor_event::Len::Len(self.key_0.data.data.len() as u64))?;
        for element in &self.key_0.data.data {
            element.serialize(serializer)?;
        }
        serializer.write_unsigned_integer(1)?;
        // DEBUG - generated from: Array(Rust("TransactionOutput"))
        serializer.write_array(cbor_event::Len::Len(self.key_1.data.len() as u64))?;
        for element in &self.key_1.data {
            element.serialize(serializer)?;
        }
        if let Some(field) = &self.key_2 {
            serializer.write_unsigned_integer(2)?;
            // DEBUG - generated from: Array(Rust("DelegationCertificate"))
            serializer.write_array(cbor_event::Len::Len(field.data.len() as u64))?;
            for element in &field.data {
                element.serialize(serializer)?;
            }
        }
        if let Some(field) = &self.key_3 {
            serializer.write_unsigned_integer(3)?;
            // DEBUG - generated from: Rust("Withdrawals")
            field.serialize(serializer)?;
        }
        serializer.write_unsigned_integer(4)?;
        // DEBUG - generated from: Rust("Coin")
        self.key_4.serialize(serializer)?;
        serializer.write_unsigned_integer(5)?;
        // DEBUG - generated from: Primitive("u32")
        self.key_5.clone().serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Vkeywitness {
    index_0: super::Vkey,
    index_1: super::Signature,
}

impl Vkeywitness {
    pub (super) fn new(index_0: super::Vkey, index_1: super::Signature) -> Self {
        Self {
            index_0: index_0,
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Rust("Vkey")
        self.index_0.serialize(serializer)?;
        // DEBUG - generated from: Rust("Signature")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct TransactionWitnessSet {
    key_0: Option<Vkeywitnesss>,
    key_1: Option<Scripts>,
}

impl TransactionWitnessSet {
    pub (super) fn new() -> Self {
        Self {
            key_0: None,
            key_1: None,
        }
    }

    pub (super) fn serialize_as_map<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_map_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_map_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        if let Some(field) = &self.key_0 {
            serializer.write_unsigned_integer(0)?;
            // DEBUG - generated from: Array(Rust("Vkeywitness"))
            serializer.write_array(cbor_event::Len::Len(field.data.len() as u64))?;
            for element in &field.data {
                element.serialize(serializer)?;
            }
        }
        if let Some(field) = &self.key_1 {
            serializer.write_unsigned_integer(1)?;
            // DEBUG - generated from: Array(Rust("Script"))
            serializer.write_array(cbor_event::Len::Len(field.data.len() as u64))?;
            for element in &field.data {
                element.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Script0 {
    index_1: super::Keyhash,
}

impl Script0 {
    pub (super) fn new(index_1: super::Keyhash) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Script1 {
    index_1: Scripts,
}

impl Script1 {
    pub (super) fn new(index_1: Scripts) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1)?;
        // DEBUG - generated from: Array(Rust("Script"))
        serializer.write_array(cbor_event::Len::Len(self.index_1.data.len() as u64))?;
        for element in &self.index_1.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Script2 {
    index_1: Scripts,
}

impl Script2 {
    pub (super) fn new(index_1: Scripts) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2)?;
        // DEBUG - generated from: Array(Rust("Script"))
        serializer.write_array(cbor_event::Len::Len(self.index_1.data.len() as u64))?;
        for element in &self.index_1.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Script3 {
    index_1: u32,
    index_2: Scripts,
}

impl Script3 {
    pub (super) fn new(index_1: u32, index_2: Scripts) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(3)?;
        // DEBUG - generated from: Primitive("u32")
        self.index_1.clone().serialize(serializer)?;
        // DEBUG - generated from: Array(Rust("Script"))
        serializer.write_array(cbor_event::Len::Len(self.index_2.data.len() as u64))?;
        for element in &self.index_2.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) enum Script {
    Script0(Script0),
    Script1(Script1),
    Script2(Script2),
    Script3(Script3),
}

impl Script {
    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Script::Script0(x) => x.serialize_as_array(serializer),
            Script::Script1(x) => x.serialize_as_array(serializer),
            Script::Script2(x) => x.serialize_as_array(serializer),
            Script::Script3(x) => x.serialize_as_array(serializer),
        }
    }

    pub (super) fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Script::Script0(x) => x.serialize_as_embedded_array_group(serializer),
            Script::Script1(x) => x.serialize_as_embedded_array_group(serializer),
            Script::Script2(x) => x.serialize_as_embedded_array_group(serializer),
            Script::Script3(x) => x.serialize_as_embedded_array_group(serializer),
        }
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Credential0 {
    index_1: super::Keyhash,
}

impl Credential0 {
    pub (super) fn new(index_1: super::Keyhash) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Credential1 {
    index_1: super::Scripthash,
}

impl Credential1 {
    pub (super) fn new(index_1: super::Scripthash) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1)?;
        // DEBUG - generated from: Rust("Scripthash")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Credential2 {
    index_1: super::Genesishash,
}

impl Credential2 {
    pub (super) fn new(index_1: super::Genesishash) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2)?;
        // DEBUG - generated from: Rust("Genesishash")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) enum Credential {
    Credential0(Credential0),
    Credential1(Credential1),
    Credential2(Credential2),
}

impl Credential {
    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Credential::Credential0(x) => x.serialize_as_array(serializer),
            Credential::Credential1(x) => x.serialize_as_array(serializer),
            Credential::Credential2(x) => x.serialize_as_array(serializer),
        }
    }

    pub (super) fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Credential::Credential0(x) => x.serialize_as_embedded_array_group(serializer),
            Credential::Credential1(x) => x.serialize_as_embedded_array_group(serializer),
            Credential::Credential2(x) => x.serialize_as_embedded_array_group(serializer),
        }
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct Withdrawals {
    pub (super) table: std::collections::BTreeMap<Credentials, super::Coin>,
}

impl Withdrawals {
    pub (super) fn new() -> Self {
        Self {
            table: std::collections::BTreeMap::new(),
        }
    }

    pub (super) fn serialize_as_map<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        for (key, value) in &self.table {
            // DEBUG - generated from: Array(Rust("Credential"))
            serializer.write_array(cbor_event::Len::Len(key.data.len() as u64))?;
            for element in &key.data {
                element.serialize(serializer)?;
            }
            // DEBUG - generated from: Rust("Coin")
            value.serialize(serializer)?;
        }
        serializer.write_special(cbor_event::Special::Break)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct UntaggedRational {
    numerator: u32,
    denominator: u32,
}

impl UntaggedRational {
    pub (super) fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator: numerator,
            denominator: denominator,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Primitive("u32")
        self.numerator.clone().serialize(serializer)?;
        // DEBUG - generated from: Primitive("u32")
        self.denominator.clone().serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct DelegationCertificate0 {
    index_1: super::Keyhash,
}

impl DelegationCertificate0 {
    pub (super) fn new(index_1: super::Keyhash) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct DelegationCertificate1 {
    index_1: super::Scripthash,
}

impl DelegationCertificate1 {
    pub (super) fn new(index_1: super::Scripthash) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1)?;
        // DEBUG - generated from: Rust("Scripthash")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct DelegationCertificate2 {
    index_1: super::Keyhash,
}

impl DelegationCertificate2 {
    pub (super) fn new(index_1: super::Keyhash) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct DelegationCertificate3 {
    index_1: super::Scripthash,
}

impl DelegationCertificate3 {
    pub (super) fn new(index_1: super::Scripthash) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(3)?;
        // DEBUG - generated from: Rust("Scripthash")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct DelegationCertificate4 {
    index_1: super::Keyhash,
    index_2: super::Keyhash,
}

impl DelegationCertificate4 {
    pub (super) fn new(index_1: super::Keyhash, index_2: super::Keyhash) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(4)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_2.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct DelegationCertificate5 {
    index_1: super::Scripthash,
    index_2: super::Keyhash,
}

impl DelegationCertificate5 {
    pub (super) fn new(index_1: super::Scripthash, index_2: super::Keyhash) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(5)?;
        // DEBUG - generated from: Rust("Scripthash")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_2.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct PoolParams {
    index_0: TaggedData<Keyhashs>,
    index_1: super::Coin,
    index_2: super::UnitInterval,
    index_3: super::Coin,
    index_4: super::Keyhash,
    index_5: super::VrfKeyhash,
    index_6: Credentials,
}

impl PoolParams {
    pub (super) fn new(index_0: TaggedData<Keyhashs>, index_1: super::Coin, index_2: super::UnitInterval, index_3: super::Coin, index_4: super::Keyhash, index_5: super::VrfKeyhash, index_6: Credentials) -> Self {
        Self {
            index_0: index_0,
            index_1: index_1,
            index_2: index_2,
            index_3: index_3,
            index_4: index_4,
            index_5: index_5,
            index_6: index_6,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Tagged(258, Array(Rust("Keyhash")))
        serializer.write_tag(258u64)?;
        // DEBUG - generated from: Array(Rust("Keyhash"))
        serializer.write_array(cbor_event::Len::Len(self.index_0.data.data.len() as u64))?;
        for element in &self.index_0.data.data {
            element.serialize(serializer)?;
        }
        // DEBUG - generated from: Rust("Coin")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("UnitInterval")
        self.index_2.serialize(serializer)?;
        // DEBUG - generated from: Rust("Coin")
        self.index_3.serialize(serializer)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_4.serialize(serializer)?;
        // DEBUG - generated from: Rust("VrfKeyhash")
        self.index_5.serialize(serializer)?;
        // DEBUG - generated from: Array(Rust("Credential"))
        serializer.write_array(cbor_event::Len::Len(self.index_6.data.len() as u64))?;
        for element in &self.index_6.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct DelegationCertificate6 {
    index_1: super::Keyhash,
    index_2: super::PoolParams,
}

impl DelegationCertificate6 {
    pub (super) fn new(index_1: super::Keyhash, index_2: super::PoolParams) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(6)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("PoolParams")
        self.index_2.group.serialize_as_embedded_array_group(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct DelegationCertificate7 {
    index_1: super::Keyhash,
    index_2: super::Epoch,
}

impl DelegationCertificate7 {
    pub (super) fn new(index_1: super::Keyhash, index_2: super::Epoch) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(7)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("Epoch")
        self.index_2.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct DelegationCertificate8 {
    index_1: super::Genesishash,
    index_2: super::Keyhash,
}

impl DelegationCertificate8 {
    pub (super) fn new(index_1: super::Genesishash, index_2: super::Keyhash) -> Self {
        Self {
            index_1: index_1,
            index_2: index_2,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(8)?;
        // DEBUG - generated from: Rust("Genesishash")
        self.index_1.serialize(serializer)?;
        // DEBUG - generated from: Rust("Keyhash")
        self.index_2.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct DelegationCertificate9 {
    index_1: super::MoveInstantaneousReward,
}

impl DelegationCertificate9 {
    pub (super) fn new(index_1: super::MoveInstantaneousReward) -> Self {
        Self {
            index_1: index_1,
        }
    }

    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_array_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }

    fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(9)?;
        // DEBUG - generated from: Rust("MoveInstantaneousReward")
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) enum DelegationCertificate {
    DelegationCertificate0(DelegationCertificate0),
    DelegationCertificate1(DelegationCertificate1),
    DelegationCertificate2(DelegationCertificate2),
    DelegationCertificate3(DelegationCertificate3),
    DelegationCertificate4(DelegationCertificate4),
    DelegationCertificate5(DelegationCertificate5),
    DelegationCertificate6(DelegationCertificate6),
    DelegationCertificate7(DelegationCertificate7),
    DelegationCertificate8(DelegationCertificate8),
    DelegationCertificate9(DelegationCertificate9),
}

impl DelegationCertificate {
    pub (super) fn serialize_as_array<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            DelegationCertificate::DelegationCertificate0(x) => x.serialize_as_array(serializer),
            DelegationCertificate::DelegationCertificate1(x) => x.serialize_as_array(serializer),
            DelegationCertificate::DelegationCertificate2(x) => x.serialize_as_array(serializer),
            DelegationCertificate::DelegationCertificate3(x) => x.serialize_as_array(serializer),
            DelegationCertificate::DelegationCertificate4(x) => x.serialize_as_array(serializer),
            DelegationCertificate::DelegationCertificate5(x) => x.serialize_as_array(serializer),
            DelegationCertificate::DelegationCertificate6(x) => x.serialize_as_array(serializer),
            DelegationCertificate::DelegationCertificate7(x) => x.serialize_as_array(serializer),
            DelegationCertificate::DelegationCertificate8(x) => x.serialize_as_array(serializer),
            DelegationCertificate::DelegationCertificate9(x) => x.serialize_as_array(serializer),
        }
    }

    pub (super) fn serialize_as_embedded_array_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            DelegationCertificate::DelegationCertificate0(x) => x.serialize_as_embedded_array_group(serializer),
            DelegationCertificate::DelegationCertificate1(x) => x.serialize_as_embedded_array_group(serializer),
            DelegationCertificate::DelegationCertificate2(x) => x.serialize_as_embedded_array_group(serializer),
            DelegationCertificate::DelegationCertificate3(x) => x.serialize_as_embedded_array_group(serializer),
            DelegationCertificate::DelegationCertificate4(x) => x.serialize_as_embedded_array_group(serializer),
            DelegationCertificate::DelegationCertificate5(x) => x.serialize_as_embedded_array_group(serializer),
            DelegationCertificate::DelegationCertificate6(x) => x.serialize_as_embedded_array_group(serializer),
            DelegationCertificate::DelegationCertificate7(x) => x.serialize_as_embedded_array_group(serializer),
            DelegationCertificate::DelegationCertificate8(x) => x.serialize_as_embedded_array_group(serializer),
            DelegationCertificate::DelegationCertificate9(x) => x.serialize_as_embedded_array_group(serializer),
        }
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub (super) struct MoveInstantaneousReward {
    pub (super) table: std::collections::BTreeMap<super::Keyhash, super::Coin>,
}

impl MoveInstantaneousReward {
    pub (super) fn new() -> Self {
        Self {
            table: std::collections::BTreeMap::new(),
        }
    }

    pub (super) fn serialize_as_map<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        for (key, value) in &self.table {
            // DEBUG - generated from: Rust("Keyhash")
            key.serialize(serializer)?;
            // DEBUG - generated from: Rust("Coin")
            value.serialize(serializer)?;
        }
        serializer.write_special(cbor_event::Special::Break)
    }
}