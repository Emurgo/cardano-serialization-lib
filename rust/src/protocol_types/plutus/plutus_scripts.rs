use std::collections::HashMap;
use itertools::Itertools;
use std::slice;
use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PlutusScripts {
    scripts: Vec<PlutusScript>,
    cbor_set_type: Option<HashMap<Language, CborSetType>>,
}

impl_to_from!(PlutusScripts);

impl NoneOrEmpty for PlutusScripts {
    fn is_none_or_empty(&self) -> bool {
        self.scripts.is_empty()
    }
}

#[wasm_bindgen]
impl PlutusScripts {
    pub fn new() -> Self {
        Self {
            scripts: Vec::new(),
            cbor_set_type: None,
        }
    }

    pub(crate) fn from_vec(
        scripts: Vec<PlutusScript>,
        cbor_set_type: Option<CborSetType>,
    ) -> Self {
        Self {
            scripts,
            cbor_set_type: cbor_set_type.map(|t| {
                let mut m = HashMap::new();
                m.insert(Language::new_plutus_v1(), t.clone());
                m.insert(Language::new_plutus_v2(), t.clone());
                m.insert(Language::new_plutus_v3(), t.clone());
                m
            }),
        }
    }

    pub fn len(&self) -> usize {
        self.scripts.len()
    }

    pub fn get(&self, index: usize) -> PlutusScript {
        self.scripts[index].clone()
    }

    pub fn add(&mut self, elem: &PlutusScript) {
        self.scripts.push(elem.clone());
    }

    #[allow(dead_code)]
    pub(crate) fn by_version(&self, language: &Language) -> PlutusScripts {
        Self::from_vec(
            self.scripts
                .iter()
                .filter(|s| s.language_version().eq(language))
                .map(|s| s.clone())
                .collect(),
            self.cbor_set_type.as_ref().map(|x| x.get(language).cloned()).flatten(),
        )
    }

    pub(crate) fn has_version(&self, language: &Language) -> bool {
        self.scripts
            .iter()
            .any(|s| s.language_version().eq(language))
    }

    pub(crate) fn merge(&self, other: &PlutusScripts, version: &Language) -> PlutusScripts {
        let mut res = self.clone();
        for s in &other.scripts {
            res.add(s);
        }
        res.set_set_type(
            other.get_set_type(version).unwrap_or(CborSetType::Tagged),
            version,
        );
        res
    }

    pub(crate) fn view(&self, version: &Language) -> Vec<&PlutusScript> {
        let mut res = Vec::new();
        for script in &self.scripts {
            if !script.language_version().eq(version) {
                continue;
            }
            res.push(script);
        }
        res
    }

    pub(crate) fn deduplicated_view(&self, version: Option<&Language>) -> Vec<&PlutusScript> {
        let mut dedup = BTreeSet::new();
        let mut res = Vec::new();
        for script in &self.scripts {
            if let Some(version) = version {
                if !script.language_version().eq(version) {
                    continue;
                }
            }
            if dedup.insert(script) {
                res.push(script);
            }
        }
        res
    }

    pub(crate) fn deduplicated_clone(&self) -> PlutusScripts {
        let mut dedup = BTreeSet::new();
        let mut scripts = Vec::new();
        for script in &self.scripts {
            if dedup.insert(script.clone()) {
                scripts.push(script.clone());
            }
        }
        Self {
            scripts,
            cbor_set_type: self.cbor_set_type.clone(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn contains(&self, script: &PlutusScript) -> bool {
        self.scripts.contains(&script)
    }

    pub(crate) fn get_set_type(&self, language: &Language) -> Option<CborSetType> {
        self.cbor_set_type.as_ref().map(|m| m.get(language).cloned()).flatten()
    }

    pub(crate) fn set_set_type(&mut self, cbor_set_type: CborSetType, language: &Language) {
        if self.cbor_set_type.is_none() {
            self.cbor_set_type = Some(HashMap::new());
        }
        if let Some(m) = &mut self.cbor_set_type {
            m.insert(language.clone(), cbor_set_type);
        }
    }
}

impl<'a> IntoIterator for &'a PlutusScripts {
    type Item = &'a PlutusScript;
    type IntoIter = slice::Iter<'a, PlutusScript>;

    fn into_iter(self) -> Self::IntoIter {
        self.scripts.iter()
    }
}

impl PartialEq for PlutusScripts {
    fn eq(&self, other: &Self) -> bool {
        self.scripts == other.scripts
    }
}

impl Eq for PlutusScripts {}

impl PartialOrd for PlutusScripts {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.scripts.partial_cmp(&other.scripts)
    }
}

impl Ord for PlutusScripts {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.scripts.cmp(&other.scripts)
    }
}

impl serde::Serialize for PlutusScripts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.scripts
            .iter()
            .collect_vec()
            .serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for PlutusScripts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let scripts_vec = <Vec<PlutusScript> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::from_vec(scripts_vec, None))
    }
}

impl JsonSchema for PlutusScripts {
    fn schema_name() -> String {
        String::from("PlutusScripts")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<PlutusScript>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        Vec::<PlutusScript>::is_referenceable()
    }
}
