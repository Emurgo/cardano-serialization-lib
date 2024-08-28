use crate::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum PlutusScriptSourceEnum {
    Script(PlutusScript, Option<RequiredSigners>),
    RefInput(PlutusScriptRef, Option<RequiredSigners>),
}

impl PlutusScriptSourceEnum {
    pub fn script_hash(&self) -> ScriptHash {
        match self {
            PlutusScriptSourceEnum::Script(script, ..) => script.hash(),
            PlutusScriptSourceEnum::RefInput(script_ref, ..) => script_ref.script_hash.clone()
        }
    }

    pub fn language(&self) -> Language {
        match self {
            PlutusScriptSourceEnum::Script(script, ..) => script.language_version(),
            PlutusScriptSourceEnum::RefInput(script_ref, ..) => script_ref.language.clone(),
        }
    }

    pub(crate) fn get_required_signers(&self) -> Option<Ed25519KeyHashes> {
        match self {
            PlutusScriptSourceEnum::Script(_, signers) => signers.clone(),
            PlutusScriptSourceEnum::RefInput(_, signers) => signers.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusScriptSource(pub(crate) PlutusScriptSourceEnum);

#[wasm_bindgen]
impl PlutusScriptSource {
    pub fn new(script: &PlutusScript) -> Self {
        Self(PlutusScriptSourceEnum::Script(script.clone(), None))
    }
    pub fn new_ref_input(
        script_hash: &ScriptHash,
        input: &TransactionInput,
        lang_ver: &Language,
        script_size: usize,
    ) -> Self {
        Self(PlutusScriptSourceEnum::RefInput(
            PlutusScriptRef::new(
                input.clone(),
                script_hash.clone(),
                lang_ver.clone(),
                script_size,
            ),
            None,
        ))
    }

    pub fn set_required_signers(&mut self, key_hashes: &Ed25519KeyHashes) {
        match &mut self.0 {
            PlutusScriptSourceEnum::Script(_, signers) => {
                *signers = Some(key_hashes.clone());
            }
            PlutusScriptSourceEnum::RefInput(_ , signers) => {
                *signers = Some(key_hashes.clone());
            }
        }
    }

    pub fn get_ref_script_size(&self) -> Option<usize> {
        match &self.0 {
            PlutusScriptSourceEnum::Script(..) => None,
            PlutusScriptSourceEnum::RefInput(script_ref, ..) => Some(script_ref.script_size)
        }
    }
}