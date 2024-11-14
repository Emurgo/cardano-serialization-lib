use crate::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct PlutusScriptRef {
    pub(crate) input_ref: TransactionInput,
    pub(crate) script_hash: ScriptHash,
    pub(crate) language: Language,
    pub(crate) script_size: usize,
}

impl PlutusScriptRef {
    pub(crate) fn new(
        input_ref: TransactionInput,
        script_hash: ScriptHash,
        language: Language,
        script_size: usize,
    ) -> Self {
        Self {
            input_ref,
            script_hash,
            language,
            script_size,
        }
    }
}
