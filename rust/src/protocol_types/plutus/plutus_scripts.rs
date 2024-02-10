use crate::*;

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct PlutusScripts(pub(crate) Vec<PlutusScript>);

impl_to_from!(PlutusScripts);

#[wasm_bindgen]
impl PlutusScripts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusScript {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &PlutusScript) {
        self.0.push(elem.clone());
    }

    pub(crate) fn by_version(&self, language: &Language) -> PlutusScripts {
        PlutusScripts(
            self.0
                .iter()
                .filter(|s| s.language_version().eq(language))
                .map(|s| s.clone())
                .collect(),
        )
    }

    pub(crate) fn has_version(&self, language: &Language) -> bool {
        self.0.iter().any(|s| s.language_version().eq(language))
    }

    pub(crate) fn merge(&self, other: &PlutusScripts) -> PlutusScripts {
        let mut res = self.clone();
        for s in &other.0 {
            res.add(s);
        }
        res
    }

    pub(crate) fn map_as_version(&self, language: &Language) -> PlutusScripts {
        let mut res = PlutusScripts::new();
        for s in &self.0 {
            res.add(&s.clone_as_version(language));
        }
        res
    }
}
