use crate::*;
use std::vec::IntoIter;
use std::slice::{Iter, IterMut};

#[wasm_bindgen]
#[derive(
    Clone, Debug
)]
pub struct NativeScripts {
    pub(crate) scripts: Vec<NativeScript>,
    pub(crate) cbor_tag_type: Option<CborSetType>,
}

#[wasm_bindgen]
impl NativeScripts {
    pub fn new() -> Self {
        Self {
            scripts: Vec::new(),
            cbor_tag_type: None,
        }
    }

    pub fn len(&self) -> usize {
        self.scripts.len()
    }

    pub fn get(&self, index: usize) -> NativeScript {
        self.scripts[index].clone()
    }

    pub fn add(&mut self, elem: &NativeScript) {
        self.scripts.push(elem.clone());
    }

    pub(crate) fn deduplicated_view(&self) -> Vec<&NativeScript> {
        let mut dedup = BTreeSet::new();
        let mut scripts = Vec::new();
        for elem in &self.scripts {
            if dedup.insert(elem) {
                scripts.push(elem);
            }
        }
        scripts
    }

    pub(crate) fn deduplicated_clone(&self) -> NativeScripts {
        let mut dedup = BTreeSet::new();
        let mut scripts = Vec::new();
        for script in &self.scripts {
            if dedup.insert(script.clone()) {
                scripts.push(script.clone());
            }
        }
        NativeScripts {
            scripts,
            cbor_tag_type: self.cbor_tag_type.clone(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn contains(&self, script: &NativeScript) -> bool {
        self.scripts.contains(script)
    }

    pub(crate) fn get_set_type(&self) -> CborSetType {
        match &self.cbor_tag_type {
            Some(set_type) => set_type.clone(),
            None => CborSetType::Tagged,
        }
    }

    pub(crate) fn iter(&self) -> Iter<'_, NativeScript> {
        self.scripts.iter()
    }
}

impl_to_from!(NativeScripts);

impl PartialEq for NativeScripts {
    fn eq(&self, other: &Self) -> bool {
        self.scripts == other.scripts
    }
}

impl Eq for NativeScripts {}

impl PartialOrd for NativeScripts {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.scripts.partial_cmp(&other.scripts)
    }
}

impl Ord for NativeScripts {
    fn cmp(&self, other: &Self) -> Ordering {
        self.scripts.cmp(&other.scripts)
    }
}

impl From<Vec<NativeScript>> for NativeScripts {
    fn from(scripts: Vec<NativeScript>) -> Self {
        Self {
            scripts,
            cbor_tag_type: None,
        }
    }
}

impl From<Vec<&NativeScript>> for NativeScripts {
    fn from(scripts: Vec<&NativeScript>) -> Self {
        Self {
            scripts: scripts.into_iter().cloned().collect(),
            cbor_tag_type: None,
        }
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
            .iter()
            .fold(Ed25519KeyHashes::new(), |mut set, s| {
                set.extend_move(Ed25519KeyHashes::from(s));
                set
            })
    }
}

impl IntoIterator for NativeScripts {
    type Item = NativeScript;
    type IntoIter = IntoIter<NativeScript>;

    fn into_iter(self) -> Self::IntoIter {
        self.scripts.into_iter()
    }
}

impl<'a> IntoIterator for &'a NativeScripts {
    type Item = &'a NativeScript;
    type IntoIter = Iter<'a, NativeScript>;

    fn into_iter(self) -> Self::IntoIter {
        self.scripts.iter()
    }
}

impl<'a> IntoIterator for &'a mut NativeScripts {
    type Item = &'a mut NativeScript;
    type IntoIter = IterMut<'a, NativeScript>;

    fn into_iter(self) -> Self::IntoIter {
        self.scripts.iter_mut()
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
        Ok(Self {
            scripts: vec,
            cbor_tag_type: None,
        })
    }
}

impl JsonSchema for NativeScripts {
    fn is_referenceable() -> bool {
        Vec::<NativeScripts>::is_referenceable()
    }
    fn schema_name() -> String {
        String::from("NativeScripts")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<NativeScripts>::json_schema(gen)
    }
}

