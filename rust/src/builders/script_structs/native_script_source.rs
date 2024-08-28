use crate::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum NativeScriptSourceEnum {
    NativeScript(NativeScript, Option<RequiredSigners>),
    RefInput(TransactionInput, ScriptHash, Option<RequiredSigners>, usize),
}

impl NativeScriptSourceEnum {
    pub fn script_hash(&self) -> ScriptHash {
        match self {
            NativeScriptSourceEnum::NativeScript(script, _) => script.hash(),
            NativeScriptSourceEnum::RefInput(_, script_hash, _, _) => script_hash.clone(),
        }
    }

    pub fn required_signers(&self) -> Option<Ed25519KeyHashes> {
        match self {
            NativeScriptSourceEnum::NativeScript(script, required_signers) => {
                match required_signers {
                    Some(signers) => Some(signers.clone()),
                    None => Some(script.into())
                }
            }
            NativeScriptSourceEnum::RefInput(_, _, required_signers, _) => required_signers.clone(),
        }
    }

    pub fn set_required_signers(&mut self, key_hashes: &Ed25519KeyHashes) {
        match self {
            NativeScriptSourceEnum::NativeScript(_, required_signers) => {
                *required_signers = Some(key_hashes.clone());
            }
            NativeScriptSourceEnum::RefInput(_, _, required_signers, _) => {
                *required_signers = Some(key_hashes.clone());
            }
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NativeScriptSource(pub(crate) NativeScriptSourceEnum);

#[wasm_bindgen]
impl NativeScriptSource {
    pub fn new(script: &NativeScript) -> Self {
        Self(NativeScriptSourceEnum::NativeScript(script.clone(), None))
    }

    pub fn new_ref_input(
        script_hash: &ScriptHash,
        input: &TransactionInput,
        script_size: usize,
    ) -> Self {
        Self(NativeScriptSourceEnum::RefInput(
            input.clone(),
            script_hash.clone(),
            None,
            script_size
        ))
    }

    pub fn set_required_signers(&mut self, key_hashes: &Ed25519KeyHashes) {
        self.0.set_required_signers(key_hashes)
    }

    pub(crate) fn script_hash(&self) -> ScriptHash {
        self.0.script_hash()
    }

    pub fn get_ref_script_size(&self) -> Option<usize> {
        match &self.0 {
            NativeScriptSourceEnum::NativeScript(..) => None,
            NativeScriptSourceEnum::RefInput(.., size) => Some(*size)
        }
    }
}