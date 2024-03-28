use crate::*;

#[wasm_bindgen]
#[derive(
Clone,
Debug,
Hash,
Eq,
Ord,
PartialEq,
PartialOrd,
serde::Serialize,
serde::Deserialize,
JsonSchema,
)]
pub struct ExUnitPrices {
    pub(crate) mem_price: SubCoin,
    pub(crate) step_price: SubCoin,
}

impl_to_from!(ExUnitPrices);

#[wasm_bindgen]
impl ExUnitPrices {
    pub fn mem_price(&self) -> SubCoin {
        self.mem_price.clone()
    }

    pub fn step_price(&self) -> SubCoin {
        self.step_price.clone()
    }

    pub fn new(mem_price: &SubCoin, step_price: &SubCoin) -> Self {
        Self {
            mem_price: mem_price.clone(),
            step_price: step_price.clone(),
        }
    }
}