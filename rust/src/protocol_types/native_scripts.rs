use crate::*;

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct NativeScripts(pub(crate) Vec<NativeScript>);

#[wasm_bindgen]
impl NativeScripts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> NativeScript {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &NativeScript) {
        self.0.push(elem.clone());
    }
}

impl From<Vec<NativeScript>> for NativeScripts {
    fn from(scripts: Vec<NativeScript>) -> Self {
        scripts.iter().fold(
            NativeScripts::new(),
            |mut scripts, s| {
                scripts.add(s);
                scripts
            },
        )
    }
}

impl NoneOrEmpty for NativeScripts {
    fn is_none_or_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&NativeScripts> for Ed25519KeyHashes {
    fn from(scripts: &NativeScripts) -> Self {
        scripts
            .0
            .iter()
            .fold(Ed25519KeyHashes::new(), |mut set, s| {
                set.extend_move(Ed25519KeyHashes::from(s));
                set
            })
    }
}
