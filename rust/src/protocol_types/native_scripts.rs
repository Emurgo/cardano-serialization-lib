use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NativeScripts {
    scripts: Vec<NativeScript>,
    dedup: BTreeSet<NativeScript>,
}

#[wasm_bindgen]
impl NativeScripts {
    pub fn new() -> Self {
        Self {
            scripts: Vec::new(),
            dedup: BTreeSet::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.scripts.len()
    }

    pub fn get(&self, index: usize) -> NativeScript {
        self.scripts[index].clone()
    }

    pub fn add(&mut self, elem: &NativeScript) {
        if self.dedup.insert(elem.clone()) {
            self.scripts.push(elem.clone());
        }
    }

    pub(crate) fn add_move(&mut self, elem: NativeScript) {
        if self.dedup.insert(elem.clone()) {
            self.scripts.push(elem);
        }
    }

    pub(crate) fn to_vec(&self) -> &Vec<NativeScript> {
        &self.scripts
    }

    pub(crate) fn from_vec(other: Vec<NativeScript>) -> Self {
        let mut dedup = BTreeSet::new();
        let mut scripts = Vec::new();
        for script in other {
            if dedup.insert(script.clone()) {
                scripts.push(script);
            }
        }

        Self { scripts, dedup }
    }
}

impl From<Vec<NativeScript>> for NativeScripts {
    fn from(scripts: Vec<NativeScript>) -> Self {
        scripts.iter().fold(NativeScripts::new(), |mut scripts, s| {
            scripts.add(s);
            scripts
        })
    }
}

impl NoneOrEmpty for NativeScripts {
    fn is_none_or_empty(&self) -> bool {
        self.scripts.is_empty()
    }
}

impl From<&NativeScripts> for Ed25519KeyHashes {
    fn from(scripts: &NativeScripts) -> Self {
        scripts
            .scripts
            .iter()
            .fold(Ed25519KeyHashes::new(), |mut set, s| {
                set.extend_move(Ed25519KeyHashes::from(s));
                set
            })
    }
}

impl serde::Serialize for NativeScripts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.scripts.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NativeScripts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let vec = <Vec<_> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::from_vec(vec))
    }
}

impl JsonSchema for NativeScripts {
    fn schema_name() -> String {
        String::from("NativeScripts")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<NativeScript>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        Vec::<NativeScript>::is_referenceable()
    }
}
