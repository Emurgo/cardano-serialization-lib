use crate::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlutusScriptSourceEnum {
    Script(PlutusScript),
    RefInput(TransactionInput, ScriptHash, Option<Language>),
}

impl PlutusScriptSourceEnum {
    pub fn script_hash(&self) -> ScriptHash {
        match self {
            PlutusScriptSourceEnum::Script(script) => script.hash(),
            PlutusScriptSourceEnum::RefInput(_, script_hash, _) => script_hash.clone(),
        }
    }

    pub fn language(&self) -> Option<Language> {
        match self {
            PlutusScriptSourceEnum::Script(script) => Some(script.language_version()),
            PlutusScriptSourceEnum::RefInput(_, _, language) => language.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusScriptSource(pub(crate) PlutusScriptSourceEnum);

#[wasm_bindgen]
impl PlutusScriptSource {
    pub fn new(script: &PlutusScript) -> Self {
        Self(PlutusScriptSourceEnum::Script(script.clone()))
    }

    /// !!! DEPRECATED !!!
    /// This constructor has missed information about plutus script language vesrion. That can affect
    /// the script data hash calculation.
    /// Use `.new_ref_input_with_lang_ver` instead
    #[deprecated(
        since = "11.3.0",
        note = "This constructor has missed information about plutus script language vesrion. That can affect the script data hash calculation. Use `.new_ref_input_with_lang_ver` instead."
    )]
    pub fn new_ref_input(script_hash: &ScriptHash, input: &TransactionInput) -> Self {
        Self(PlutusScriptSourceEnum::RefInput(
            input.clone(),
            script_hash.clone(),
            None,
        ))
    }

    pub fn new_ref_input_with_lang_ver(
        script_hash: &ScriptHash,
        input: &TransactionInput,
        lang_ver: &Language,
    ) -> Self {
        Self(PlutusScriptSourceEnum::RefInput(
            input.clone(),
            script_hash.clone(),
            Some(lang_ver.clone()),
        ))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DatumSourceEnum {
    Datum(PlutusData),
    RefInput(TransactionInput),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DatumSource(DatumSourceEnum);

#[wasm_bindgen]
impl DatumSource {
    pub fn new(datum: &PlutusData) -> Self {
        Self(DatumSourceEnum::Datum(datum.clone()))
    }

    pub fn new_ref_input(input: &TransactionInput) -> Self {
        Self(DatumSourceEnum::RefInput(input.clone()))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum NativeScriptSourceEnum {
    NativeScript(NativeScript),
    RefInput(TransactionInput, ScriptHash, RequiredSigners),
}

impl NativeScriptSourceEnum {
    pub fn script_hash(&self) -> ScriptHash {
        match self {
            NativeScriptSourceEnum::NativeScript(script) => script.hash(),
            NativeScriptSourceEnum::RefInput(_, script_hash, _) => script_hash.clone(),
        }
    }

    pub fn required_signers(&self) -> RequiredSignersSet {
        match self {
            NativeScriptSourceEnum::NativeScript(script) => RequiredSignersSet::from(script),
            NativeScriptSourceEnum::RefInput(_, _, required_signers) => required_signers.into(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NativeScriptSource(pub(crate) NativeScriptSourceEnum);

#[wasm_bindgen]
impl NativeScriptSource {
    pub fn new(script: &NativeScript) -> Self {
        Self(NativeScriptSourceEnum::NativeScript(script.clone()))
    }

    pub fn new_ref_input(
        script_hash: &ScriptHash,
        input: &TransactionInput,
        required_signers: &RequiredSigners,
    ) -> Self {
        Self(NativeScriptSourceEnum::RefInput(
            input.clone(),
            script_hash.clone(),
            required_signers.clone(),
        ))
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusWitness {
    pub(crate) script: PlutusScriptSourceEnum,
    pub(crate) datum: Option<DatumSourceEnum>,
    pub(crate) redeemer: Redeemer,
}

#[wasm_bindgen]
impl PlutusWitness {
    pub fn new(script: &PlutusScript, datum: &PlutusData, redeemer: &Redeemer) -> Self {
        Self {
            script: PlutusScriptSourceEnum::Script(script.clone()),
            datum: Some(DatumSourceEnum::Datum(datum.clone())),
            redeemer: redeemer.clone(),
        }
    }

    pub fn new_with_ref(
        script: &PlutusScriptSource,
        datum: &DatumSource,
        redeemer: &Redeemer,
    ) -> Self {
        Self {
            script: script.0.clone(),
            datum: Some(datum.0.clone()),
            redeemer: redeemer.clone(),
        }
    }

    pub fn new_without_datum(script: &PlutusScript, redeemer: &Redeemer) -> Self {
        Self {
            script: PlutusScriptSourceEnum::Script(script.clone()),
            datum: None,
            redeemer: redeemer.clone(),
        }
    }

    pub fn new_with_ref_without_datum(script: &PlutusScriptSource, redeemer: &Redeemer) -> Self {
        Self {
            script: script.0.clone(),
            datum: None,
            redeemer: redeemer.clone(),
        }
    }

    pub fn script(&self) -> Option<PlutusScript> {
        match &self.script {
            PlutusScriptSourceEnum::Script(script) => Some(script.clone()),
            _ => None,
        }
    }

    pub fn datum(&self) -> Option<PlutusData> {
        match &self.datum {
            Some(DatumSourceEnum::Datum(datum)) => Some(datum.clone()),
            _ => None,
        }
    }

    pub fn redeemer(&self) -> Redeemer {
        self.redeemer.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn clone_with_redeemer_index(&self, index: &BigNum) -> Self {
        Self {
            script: self.script.clone(),
            datum: self.datum.clone(),
            redeemer: self.redeemer.clone_with_index(index),
        }
    }

    pub(crate) fn clone_with_redeemer_index_and_tag(
        &self,
        index: &BigNum,
        tag: &RedeemerTag,
    ) -> Self {
        Self {
            script: self.script.clone(),
            datum: self.datum.clone(),
            redeemer: self.redeemer.clone_with_index_and_tag(index, tag),
        }
    }
}

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
            if let PlutusScriptSourceEnum::Script(script) = &w.script {
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

#[derive(Clone, Debug)]
pub(crate) enum ScriptWitnessType {
    NativeScriptWitness(NativeScriptSourceEnum),
    PlutusScriptWitness(PlutusWitness),
}

impl ScriptWitnessType {
    pub fn script_hash(&self) -> ScriptHash {
        match self {
            ScriptWitnessType::NativeScriptWitness(script) => script.script_hash(),
            ScriptWitnessType::PlutusScriptWitness(script) => script.script.script_hash(),
        }
    }
}
