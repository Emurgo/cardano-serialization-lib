use crate::*;

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
            script: PlutusScriptSourceEnum::Script(script.clone(), None),
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
            script: PlutusScriptSourceEnum::Script(script.clone(), None),
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
            PlutusScriptSourceEnum::Script(script, _) => Some(script.clone()),
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

    pub(crate) fn get_required_signers(&self) -> Option<Ed25519KeyHashes> {
        self.script.get_required_signers()
    }
}