use crate::*;
use std::io::SeekFrom;
use hashlink::LinkedHashMap;
use crate::serialization::utils::skip_set_tag;

impl cbor_event::se::Serialize for PlutusScript {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.bytes)
    }
}

impl Deserialize for PlutusScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self::new(raw.bytes()?))
    }
}

impl cbor_event::se::Serialize for PlutusScripts {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        //TODO: uncomment this line when we conway ero will come
        //serializer.write_tag(258)?;
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for PlutusScripts {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            skip_set_tag(raw)?;
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(PlutusScript::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("PlutusScripts"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for ConstrPlutusData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if let Some(compact_tag) =
            Self::alternative_to_compact_cbor_tag(from_bignum(&self.alternative))
        {
            // compact form
            serializer.write_tag(compact_tag as u64)?;
            self.data.serialize(serializer)
        } else {
            // general form
            serializer.write_tag(Self::GENERAL_FORM_TAG)?;
            serializer.write_array(cbor_event::Len::Len(2))?;
            self.alternative.serialize(serializer)?;
            self.data.serialize(serializer)
        }
    }
}

impl Deserialize for ConstrPlutusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let (alternative, data) = match raw.tag()? {
                // general form
                Self::GENERAL_FORM_TAG => {
                    let len = raw.array()?;
                    let mut read_len = CBORReadLen::new(len);
                    read_len.read_elems(2)?;
                    let alternative = BigNum::deserialize(raw)?;
                    let data =
                        (|| -> Result<_, DeserializeError> { Ok(PlutusList::deserialize(raw)?) })()
                            .map_err(|e| e.annotate("datas"))?;
                    match len {
                        cbor_event::Len::Len(_) => (),
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => (),
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    (alternative, data)
                }
                // concise form
                tag => {
                    if let Some(alternative) = Self::compact_cbor_tag_to_alternative(tag) {
                        (to_bignum(alternative), PlutusList::deserialize(raw)?)
                    } else {
                        return Err(DeserializeFailure::TagMismatch {
                            found: tag,
                            expected: Self::GENERAL_FORM_TAG,
                        }
                            .into());
                    }
                }
            };
            Ok(ConstrPlutusData { alternative, data })
        })()
            .map_err(|e| e.annotate("ConstrPlutusData"))
    }
}

impl cbor_event::se::Serialize for CostModel {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for cost in &self.0 {
            cost.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for CostModel {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(Int::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("CostModel"))?;
        Ok(Self(arr.try_into().unwrap()))
    }
}

impl cbor_event::se::Serialize for Costmdls {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Costmdls {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = std::collections::BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => table.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = Language::deserialize(raw)?;
                let value = CostModel::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                        .into());
                }
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("Costmdls"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for ExUnitPrices {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.mem_price.serialize(serializer)?;
        self.step_price.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ExUnitPrices {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let mem_price =
                (|| -> Result<_, DeserializeError> { Ok(SubCoin::deserialize(raw)?) })()
                    .map_err(|e| e.annotate("mem_price"))?;
            let step_price =
                (|| -> Result<_, DeserializeError> { Ok(SubCoin::deserialize(raw)?) })()
                    .map_err(|e| e.annotate("step_price"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ExUnitPrices {
                mem_price,
                step_price,
            })
        })()
            .map_err(|e| e.annotate("ExUnitPrices"))
    }
}

impl cbor_event::se::Serialize for ExUnits {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.mem.serialize(serializer)?;
        self.steps.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ExUnits {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let mem = (|| -> Result<_, DeserializeError> { Ok(BigNum::deserialize(raw)?) })()
                .map_err(|e| e.annotate("mem"))?;
            let steps = (|| -> Result<_, DeserializeError> { Ok(BigNum::deserialize(raw)?) })()
                .map_err(|e| e.annotate("steps"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ExUnits { mem, steps })
        })()
            .map_err(|e| e.annotate("ExUnits"))
    }
}

impl cbor_event::se::Serialize for Language {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        // https://github.com/input-output-hk/cardano-ledger/blob/master/eras/babbage/test-suite/cddl-files/babbage.cddl#L324-L327
        serializer.write_unsigned_integer(self.kind() as u64)
    }
}

impl Deserialize for Language {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match LanguageKind::from_u64(raw.unsigned_integer()?) {
                Some(kind) => Ok(Language(kind)),
                _ => Err(DeserializeError::new(
                    "Language",
                    DeserializeFailure::NoVariantMatched.into(),
                )),
            }
        })()
            .map_err(|e| e.annotate("Language"))
    }
}

impl cbor_event::se::Serialize for Languages {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Languages {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(Language::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("Languages"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for PlutusMap {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for PlutusMap {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = LinkedHashMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => table.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = PlutusData::deserialize(raw)?;
                let value = PlutusData::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                        .into());
                }
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("PlutusMap"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for PlutusDataEnum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            PlutusDataEnum::ConstrPlutusData(x) => x.serialize(serializer),
            PlutusDataEnum::Map(x) => x.serialize(serializer),
            PlutusDataEnum::List(x) => x.serialize(serializer),
            PlutusDataEnum::Integer(x) => x.serialize(serializer),
            PlutusDataEnum::Bytes(x) => write_bounded_bytes(serializer, &x),
        }
    }
}

impl Deserialize for PlutusDataEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ConstrPlutusData::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::ConstrPlutusData(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PlutusMap::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::Map(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PlutusList::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::List(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(BigInt::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::Integer(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(read_bounded_bytes(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::Bytes(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            Err(DeserializeError::new(
                "PlutusDataEnum",
                DeserializeFailure::NoVariantMatched.into(),
            ))
        })()
            .map_err(|e| e.annotate("PlutusDataEnum"))
    }
}

impl cbor_event::se::Serialize for PlutusData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match &self.original_bytes {
            Some(bytes) => serializer.write_raw_bytes(bytes),
            None => self.datum.serialize(serializer),
        }
    }
}

impl Deserialize for PlutusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        // these unwraps are fine since we're seeking the current position
        let before = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        let datum = PlutusDataEnum::deserialize(raw)?;
        let after = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        let bytes_read = (after - before) as usize;
        raw.as_mut_ref().seek(SeekFrom::Start(before)).unwrap();
        // these unwraps are fine since we read the above already
        let original_bytes = raw.as_mut_ref().fill_buf().unwrap()[..bytes_read].to_vec();
        raw.as_mut_ref().consume(bytes_read);
        Ok(Self {
            datum,
            original_bytes: Some(original_bytes),
        })
    }
}

impl cbor_event::se::Serialize for PlutusList {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        let use_definite_encoding = match self.definite_encoding {
            Some(definite) => definite,
            None => self.elems.is_empty(),
        };
        if use_definite_encoding {
            serializer.write_array(cbor_event::Len::Len(self.elems.len() as u64))?;
        } else {
            serializer.write_array(cbor_event::Len::Indefinite)?;
        }
        for element in &self.elems {
            element.serialize(serializer)?;
        }
        if !use_definite_encoding {
            serializer.write_special(cbor_event::Special::Break)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for PlutusList {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        let len = (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(PlutusData::deserialize(raw)?);
            }
            Ok(len)
        })()
            .map_err(|e| e.annotate("PlutusList"))?;
        Ok(Self {
            elems: arr,
            definite_encoding: Some(len != cbor_event::Len::Indefinite),
        })
    }
}

impl cbor_event::se::Serialize for Redeemer {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.tag.serialize(serializer)?;
        self.index.serialize(serializer)?;
        self.data.serialize(serializer)?;
        self.ex_units.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for Redeemer {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(4)?;
            let tag = (|| -> Result<_, DeserializeError> { Ok(RedeemerTag::deserialize(raw)?) })()
                .map_err(|e| e.annotate("tag"))?;
            let index = (|| -> Result<_, DeserializeError> { Ok(BigNum::deserialize(raw)?) })()
                .map_err(|e| e.annotate("index"))?;
            let data = (|| -> Result<_, DeserializeError> { Ok(PlutusData::deserialize(raw)?) })()
                .map_err(|e| e.annotate("data"))?;
            let ex_units = (|| -> Result<_, DeserializeError> { Ok(ExUnits::deserialize(raw)?) })()
                .map_err(|e| e.annotate("ex_units"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Redeemer {
                tag,
                index,
                data,
                ex_units,
            })
        })()
            .map_err(|e| e.annotate("Redeemer"))
    }
}

impl cbor_event::se::Serialize for RedeemerTagKind {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            RedeemerTagKind::Spend => serializer.write_unsigned_integer(0u64),
            RedeemerTagKind::Mint => serializer.write_unsigned_integer(1u64),
            RedeemerTagKind::Cert => serializer.write_unsigned_integer(2u64),
            RedeemerTagKind::Reward => serializer.write_unsigned_integer(3u64),
            RedeemerTagKind::Vote => serializer.write_unsigned_integer(4u64),
            RedeemerTagKind::VotingProposal => serializer.write_unsigned_integer(5u64),
        }
    }
}

impl Deserialize for RedeemerTagKind {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.unsigned_integer() {
                Ok(0) => Ok(RedeemerTagKind::Spend),
                Ok(1) => Ok(RedeemerTagKind::Mint),
                Ok(2) => Ok(RedeemerTagKind::Cert),
                Ok(3) => Ok(RedeemerTagKind::Reward),
                Ok(4) => Ok(RedeemerTagKind::Vote),
                Ok(5) => Ok(RedeemerTagKind::VotingProposal),
                Ok(_) | Err(_) => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
            .map_err(|e| e.annotate("RedeemerTagEnum"))
    }
}

impl cbor_event::se::Serialize for RedeemerTag {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for RedeemerTag {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(RedeemerTagKind::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for Redeemers {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Redeemers {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(Redeemer::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("Redeemers"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for Strings {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            serializer.write_text(&element)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Strings {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(String::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("Strings"))?;
        Ok(Self(arr))
    }
}