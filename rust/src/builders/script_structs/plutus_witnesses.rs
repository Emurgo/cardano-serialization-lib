use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusWitnesses(pub(crate) Vec<PlutusWitness>);

#[wasm_bindgen]
impl PlutusWitnesses {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusWitness {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &PlutusWitness) {
        self.0.push(elem.clone());
    }

    pub(crate) fn collect(&self) -> (PlutusScripts, Option<PlutusList>, Redeemers) {
        let mut used_scripts = BTreeSet::new();
        let mut used_datums = BTreeSet::new();
        let mut used_redeemers = BTreeSet::new();
        let mut s = PlutusScripts::new();
        let mut d: Option<PlutusList> = None;
        let mut r = Redeemers::new();
        self.0.iter().for_each(|w| {
            if let PlutusScriptSourceEnum::Script(script, ..) = &w.script {
                if used_scripts.insert(script.clone()) {
                    s.add(script);
                }
            }
            if let Some(DatumSourceEnum::Datum(datum)) = &w.datum {
                if used_datums.insert(datum) {
                    match d {
                        Some(ref mut d) => d.add(datum),
                        None => {
                            d = {
                                let mut initial_list = PlutusList::new();
                                initial_list.add(datum);
                                Some(initial_list)
                            }
                        }
                    }
                }
            }
            if used_redeemers.insert(w.redeemer.clone()) {
                r.add(&w.redeemer);
            }
        });
        (s, d, r)
    }
}

impl From<Vec<PlutusWitness>> for PlutusWitnesses {
    fn from(scripts: Vec<PlutusWitness>) -> Self {
        Self(scripts)
    }
}