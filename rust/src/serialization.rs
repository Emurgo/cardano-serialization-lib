use super::*;

pub trait SerializeEmbeddedGroup {
    fn serialize_as_embedded_group<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>>;
}

impl cbor_event::se::Serialize for Hash {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Keyhash {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Scripthash {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Genesishash {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Vkey {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Signature {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for VrfKeyhash {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for VrfVkey {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for VrfProof {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for KesVkey {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for KesSignature {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for TransactionInput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for TransactionInput {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.transaction_id.serialize(serializer)?;
        self.index.clone().serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for BaseAddress {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for BaseAddress {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0)?;
        self.spending.serialize(serializer)?;
        self.deleg.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for BaseAddressScriptDelegation {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for BaseAddressScriptDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1)?;
        self.spending.serialize(serializer)?;
        self.deleg.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for BaseScriptAddress {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for BaseScriptAddress {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2)?;
        self.spending.serialize(serializer)?;
        self.deleg.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for BaseScriptAddressScriptDeleg {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for BaseScriptAddressScriptDeleg {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(3)?;
        self.spending.serialize(serializer)?;
        self.deleg.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Pointer {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Pointer {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.slot.clone().serialize(serializer)?;
        self.tx_index.clone().serialize(serializer)?;
        self.cert_index.clone().serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for PointerAddress {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for PointerAddress {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(4)?;
        self.spending.serialize(serializer)?;
        self.deleg.serialize_as_embedded_group(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for PointerMultisigAddress {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for PointerMultisigAddress {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(5)?;
        self.spending.serialize(serializer)?;
        self.deleg.serialize_as_embedded_group(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for EnterpriseAddress {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for EnterpriseAddress {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(6)?;
        self.spending.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for EnterpriseMultisigAddress {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for EnterpriseMultisigAddress {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(7)?;
        self.spending.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for BootstrapAddress {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for BootstrapAddress {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(8)?;
        self.spending.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for AddressEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for AddressEnum {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            AddressEnum::BaseAddress(x) => x.serialize_as_embedded_group(serializer),
            AddressEnum::BaseAddressScriptDelegation(x) => x.serialize_as_embedded_group(serializer),
            AddressEnum::BaseScriptAddress(x) => x.serialize_as_embedded_group(serializer),
            AddressEnum::BaseScriptAddressScriptDeleg(x) => x.serialize_as_embedded_group(serializer),
            AddressEnum::PointerAddress(x) => x.serialize_as_embedded_group(serializer),
            AddressEnum::PointerMultisigAddress(x) => x.serialize_as_embedded_group(serializer),
            AddressEnum::EnterpriseAddress(x) => x.serialize_as_embedded_group(serializer),
            AddressEnum::EnterpriseMultisigAddress(x) => x.serialize_as_embedded_group(serializer),
            AddressEnum::BootstrapAddress(x) => x.serialize_as_embedded_group(serializer),
        }
    }
}

impl cbor_event::se::Serialize for Address {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Address {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize_as_embedded_group(serializer)
    }
}

impl cbor_event::se::Serialize for TransactionOutput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for TransactionOutput {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.address.serialize_as_embedded_group(serializer)?;
        self.amount.clone().serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for TransactionInputs {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for TransactionOutputs {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificates {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for TransactionBody {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for TransactionBody {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0)?;
        serializer.write_tag(258u64)?;
        serializer.write_array(cbor_event::Len::Len(self.inputs.0.len() as u64))?;
        for element in &self.inputs.0 {
            element.serialize(serializer)?;
        }
        serializer.write_unsigned_integer(1)?;
        serializer.write_array(cbor_event::Len::Len(self.outputs.0.len() as u64))?;
        for element in &self.outputs.0 {
            element.serialize(serializer)?;
        }
        if let Some(field) = &self.certs {
            serializer.write_unsigned_integer(2)?;
            serializer.write_array(cbor_event::Len::Len(field.0.len() as u64))?;
            for element in &field.0 {
                element.serialize(serializer)?;
            }
        }
        if let Some(field) = &self.withdrawals {
            serializer.write_unsigned_integer(3)?;
            field.serialize(serializer)?;
        }
        serializer.write_unsigned_integer(4)?;
        self.fee.serialize(serializer)?;
        serializer.write_unsigned_integer(5)?;
        self.ttl.clone().serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Vkeywitness {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Vkeywitness {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.vkey.serialize(serializer)?;
        self.signature.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Vkeywitnesss {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Scripts {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for TransactionWitnessSet {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for TransactionWitnessSet {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        if let Some(field) = &self.key_witnesses {
            serializer.write_unsigned_integer(0)?;
            serializer.write_array(cbor_event::Len::Len(field.0.len() as u64))?;
            for element in &field.0 {
                element.serialize(serializer)?;
            }
        }
        if let Some(field) = &self.script_witnesses {
            serializer.write_unsigned_integer(1)?;
            serializer.write_array(cbor_event::Len::Len(field.0.len() as u64))?;
            for element in &field.0 {
                element.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Script0 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Script0 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0)?;
        self.keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Script1 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Script1 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1)?;
        serializer.write_array(cbor_event::Len::Len(self.scripts.0.len() as u64))?;
        for element in &self.scripts.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Script2 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Script2 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2)?;
        serializer.write_array(cbor_event::Len::Len(self.scripts.0.len() as u64))?;
        for element in &self.scripts.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Script3 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Script3 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(3)?;
        self.m.clone().serialize(serializer)?;
        serializer.write_array(cbor_event::Len::Len(self.scripts.0.len() as u64))?;
        for element in &self.scripts.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for ScriptEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for ScriptEnum {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            ScriptEnum::Script0(x) => x.serialize_as_embedded_group(serializer),
            ScriptEnum::Script1(x) => x.serialize_as_embedded_group(serializer),
            ScriptEnum::Script2(x) => x.serialize_as_embedded_group(serializer),
            ScriptEnum::Script3(x) => x.serialize_as_embedded_group(serializer),
        }
    }
}

impl cbor_event::se::Serialize for Script {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Script {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize_as_embedded_group(serializer)
    }
}

impl cbor_event::se::Serialize for Credential0 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Credential0 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0)?;
        self.keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Credential1 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Credential1 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1)?;
        self.scripthash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Credential2 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Credential2 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2)?;
        self.genesishash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for CredentialEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for CredentialEnum {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            CredentialEnum::Credential0(x) => x.serialize_as_embedded_group(serializer),
            CredentialEnum::Credential1(x) => x.serialize_as_embedded_group(serializer),
            CredentialEnum::Credential2(x) => x.serialize_as_embedded_group(serializer),
        }
    }
}

impl cbor_event::se::Serialize for Credential {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Credential {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize_as_embedded_group(serializer)
    }
}

impl cbor_event::se::Serialize for Credentials {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Withdrawals {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Withdrawals {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        for (key, value) in &self.table {
            serializer.write_array(cbor_event::Len::Len(key.0.len() as u64))?;
            for element in &key.0 {
                element.serialize(serializer)?;
            }
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Rational {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag(30u64)?;
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for Rational {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.numerator.clone().serialize(serializer)?;
        self.denominator.clone().serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificate0 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificate0 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0)?;
        self.keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificate1 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificate1 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1)?;
        self.scripthash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificate2 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificate2 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2)?;
        self.keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificate3 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificate3 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(3)?;
        self.scripthash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificate4 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificate4 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(4)?;
        self.deleg_from.serialize(serializer)?;
        self.deleg_to.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificate5 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificate5 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(5)?;
        self.deleg_from.serialize(serializer)?;
        self.deleg_to.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Keyhashs {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for PoolParams {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for PoolParams {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag(258u64)?;
        serializer.write_array(cbor_event::Len::Len(self.owners.0.len() as u64))?;
        for element in &self.owners.0 {
            element.serialize(serializer)?;
        }
        self.cost.serialize(serializer)?;
        self.margin.serialize(serializer)?;
        self.pledge.serialize(serializer)?;
        self.operator.serialize(serializer)?;
        self.vrf_keyhash.serialize(serializer)?;
        serializer.write_array(cbor_event::Len::Len(self.reward_account.0.len() as u64))?;
        for element in &self.reward_account.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificate6 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificate6 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(6)?;
        self.keyhash.serialize(serializer)?;
        self.pool_params.serialize_as_embedded_group(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificate7 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificate7 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(7)?;
        self.keyhash.serialize(serializer)?;
        self.epoch.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificate8 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificate8 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(8)?;
        self.deleg_from.serialize(serializer)?;
        self.deleg_to.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificate9 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificate9 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(9)?;
        self.move_instantaneous_reward.serialize(serializer)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for DelegationCertificateEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificateEnum {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            DelegationCertificateEnum::DelegationCertificate0(x) => x.serialize_as_embedded_group(serializer),
            DelegationCertificateEnum::DelegationCertificate1(x) => x.serialize_as_embedded_group(serializer),
            DelegationCertificateEnum::DelegationCertificate2(x) => x.serialize_as_embedded_group(serializer),
            DelegationCertificateEnum::DelegationCertificate3(x) => x.serialize_as_embedded_group(serializer),
            DelegationCertificateEnum::DelegationCertificate4(x) => x.serialize_as_embedded_group(serializer),
            DelegationCertificateEnum::DelegationCertificate5(x) => x.serialize_as_embedded_group(serializer),
            DelegationCertificateEnum::DelegationCertificate6(x) => x.serialize_as_embedded_group(serializer),
            DelegationCertificateEnum::DelegationCertificate7(x) => x.serialize_as_embedded_group(serializer),
            DelegationCertificateEnum::DelegationCertificate8(x) => x.serialize_as_embedded_group(serializer),
            DelegationCertificateEnum::DelegationCertificate9(x) => x.serialize_as_embedded_group(serializer),
        }
    }
}

impl cbor_event::se::Serialize for DelegationCertificate {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for DelegationCertificate {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize_as_embedded_group(serializer)
    }
}

impl cbor_event::se::Serialize for MoveInstantaneousReward {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(cbor_event::Special::Break)
    }
}

impl SerializeEmbeddedGroup for MoveInstantaneousReward {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        for (key, value) in &self.table {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}