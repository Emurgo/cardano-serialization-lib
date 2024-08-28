use hashlink::LinkedHashSet;
use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vkeywitnesses {
    pub(crate) witnesses: Vec<Vkeywitness>,
    pub(crate) dedup: LinkedHashSet<Vkeywitness>,
}

impl_to_from!(Vkeywitnesses);

#[wasm_bindgen]
impl Vkeywitnesses {
    pub fn new() -> Self {
        Self {
            witnesses: Vec::new(),
            dedup: LinkedHashSet::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.witnesses.len()
    }

    pub fn get(&self, index: usize) -> Vkeywitness {
        self.witnesses[index].clone()
    }

    /// Add a new `Vkeywitness` to the set.
    /// Returns `true` if the element was not already present in the set.
    pub fn add(&mut self, elem: &Vkeywitness) -> bool {
        if self.dedup.insert(elem.clone()) {
            self.witnesses.push(elem.clone());
            true
        } else {
            false
        }
    }

    pub(crate) fn add_move(&mut self, elem: Vkeywitness) {
        if self.dedup.insert(elem.clone()) {
            self.witnesses.push(elem);
        }
    }

    pub(crate) fn from_vec(vec: Vec<Vkeywitness>) -> Self {
        let mut dedup = LinkedHashSet::new();
        let mut witnesses = Vec::new();
        for elem in vec {
            if dedup.insert(elem.clone()) {
                witnesses.push(elem);
            }
        }
        Self { witnesses, dedup }
    }
}

impl NoneOrEmpty for Vkeywitnesses {
    fn is_none_or_empty(&self) -> bool {
        self.witnesses.is_empty()
    }
}

impl serde::Serialize for Vkeywitnesses {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.witnesses.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Vkeywitnesses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let vec = <Vec<_> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::from_vec(vec))
    }
}

impl JsonSchema for Vkeywitnesses {
    fn schema_name() -> String {
        String::from("Vkeywitnesses")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<Vkeywitness>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        Vec::<Vkeywitness>::is_referenceable()
    }
}
