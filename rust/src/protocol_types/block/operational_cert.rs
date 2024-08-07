use crate::*;

#[wasm_bindgen]
#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct OperationalCert {
    pub(crate) hot_vkey: KESVKey,
    pub(crate) sequence_number: u32,
    pub(crate) kes_period: u32,
    pub(crate) sigma: Ed25519Signature,
}

impl_to_from!(OperationalCert);

#[wasm_bindgen]
impl OperationalCert {
    pub fn hot_vkey(&self) -> KESVKey {
        self.hot_vkey.clone()
    }

    pub fn sequence_number(&self) -> u32 {
        self.sequence_number.clone()
    }

    pub fn kes_period(&self) -> u32 {
        self.kes_period.clone()
    }

    pub fn sigma(&self) -> Ed25519Signature {
        self.sigma.clone()
    }

    pub fn new(
        hot_vkey: &KESVKey,
        sequence_number: u32,
        kes_period: u32,
        sigma: &Ed25519Signature,
    ) -> Self {
        Self {
            hot_vkey: hot_vkey.clone(),
            sequence_number: sequence_number,
            kes_period: kes_period,
            sigma: sigma.clone(),
        }
    }
}