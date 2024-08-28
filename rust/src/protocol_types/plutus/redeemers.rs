use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Ord, PartialOrd)]
pub struct Redeemers {
    pub(crate) redeemers: Vec<Redeemer>,
    pub(crate) serialization_format: Option<CborContainerType>,
}

impl_to_from!(Redeemers);

#[wasm_bindgen]
impl Redeemers {
    pub fn new() -> Self {
        Self {
            redeemers: Vec::new(),
            serialization_format: None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_with_serialization_format(
        redeemers: Vec<Redeemer>,
        serialization_format: CborContainerType,
    ) -> Self {
        Self {
            redeemers,
            serialization_format: Some(serialization_format),
        }
    }

    pub fn len(&self) -> usize {
        self.redeemers.len()
    }

    pub fn get(&self, index: usize) -> Redeemer {
        self.redeemers[index].clone()
    }

    pub fn add(&mut self, elem: &Redeemer) {
        self.redeemers.push(elem.clone());
    }

    pub fn total_ex_units(&self) -> Result<ExUnits, JsError> {
        let mut tot_mem = BigNum::zero();
        let mut tot_steps = BigNum::zero();
        for i in 0..self.redeemers.len() {
            let r: &Redeemer = &self.redeemers[i];
            tot_mem = tot_mem.checked_add(&r.ex_units().mem())?;
            tot_steps = tot_steps.checked_add(&r.ex_units().steps())?;
        }
        Ok(ExUnits::new(&tot_mem, &tot_steps))
    }
}

impl NoneOrEmpty for Redeemers {
    fn is_none_or_empty(&self) -> bool {
        self.redeemers.is_empty()
    }
}

impl PartialEq<Redeemers> for Redeemers {
    fn eq(&self, other: &Redeemers) -> bool {
        self.redeemers == other.redeemers
    }

}

impl Eq for Redeemers {}

impl From<Vec<Redeemer>> for Redeemers {
    fn from(values: Vec<Redeemer>) -> Self {
        Self {
            redeemers: values,
            serialization_format: None,
        }
    }
}

impl serde::Serialize for Redeemers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.redeemers.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Redeemers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let vec = <Vec<_> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self {
            redeemers: vec,
            serialization_format: None,
        })
    }
}

impl JsonSchema for Redeemers {
    fn is_referenceable() -> bool {
        Vec::<Redeemer>::is_referenceable()
    }
    fn schema_name() -> String {
        String::from("Redeemers")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<Redeemer>::json_schema(gen)
    }
}
