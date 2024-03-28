use crate::*;

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct VRFCert {
    pub(crate) output: Vec<u8>,
    pub(crate) proof: Vec<u8>,
}

impl VRFCert {
    pub const PROOF_LEN: usize = 80;
}

impl_to_from!(VRFCert);

#[wasm_bindgen]
impl VRFCert {
    pub fn output(&self) -> Vec<u8> {
        self.output.clone()
    }

    pub fn proof(&self) -> Vec<u8> {
        self.proof.clone()
    }

    pub fn new(output: Vec<u8>, proof: Vec<u8>) -> Result<VRFCert, JsError> {
        if proof.len() != Self::PROOF_LEN {
            return Err(JsError::from_str(&format!(
                "proof len must be {} - found {}",
                Self::PROOF_LEN,
                proof.len()
            )));
        }
        Ok(Self {
            output: output,
            proof: proof,
        })
    }
}