use crate::*;

#[derive(Clone, Debug)]
pub(crate) enum ScriptWitnessType {
    NativeScriptWitness(NativeScriptSourceEnum),
    PlutusScriptWitness(PlutusWitness),
}

impl ScriptWitnessType {
    #[allow(dead_code)]
    pub(crate) fn script_hash(&self) -> ScriptHash {
        match self {
            ScriptWitnessType::NativeScriptWitness(script) => script.script_hash(),
            ScriptWitnessType::PlutusScriptWitness(script) => script.script.script_hash(),
        }
    }

    pub(crate) fn get_required_signers(&self) -> Option<Ed25519KeyHashes> {
        match self {
            ScriptWitnessType::NativeScriptWitness(script) => script.required_signers(),
            ScriptWitnessType::PlutusScriptWitness(script) => script.get_required_signers(),
        }
    }

    pub(crate) fn get_script_ref_input(&self) -> Option<TransactionInput> {
        match self {
            ScriptWitnessType::NativeScriptWitness(NativeScriptSourceEnum::RefInput(input, ..)) => {
                Some(input.clone())
            }
            ScriptWitnessType::PlutusScriptWitness(plutus_witness) => {
                match &plutus_witness.script {
                    PlutusScriptSourceEnum::RefInput(script_ref, ..) => {
                        Some(script_ref.input_ref.clone())
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub(crate) fn get_datum_ref_input(&self) -> Option<TransactionInput> {
        match self {
            ScriptWitnessType::PlutusScriptWitness(plutus_witness) => match &plutus_witness.datum {
                Some(DatumSourceEnum::RefInput(input)) => Some(input.clone()),
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn get_script_ref_input_with_size(
        &self,
    ) -> Option<(&TransactionInput, usize)> {
        match self {
            ScriptWitnessType::PlutusScriptWitness(plutus_witness) => {
                match &plutus_witness.script {
                    PlutusScriptSourceEnum::RefInput(script_ref, ..) => {
                        Some((&script_ref.input_ref, script_ref.script_size))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
