use crate::*;

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct Redeemers(pub(crate) Vec<Redeemer>);

impl_to_from!(Redeemers);

#[wasm_bindgen]
impl Redeemers {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Redeemer {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &Redeemer) {
        self.0.push(elem.clone());
    }

    pub fn total_ex_units(&self) -> Result<ExUnits, JsError> {
        let mut tot_mem = BigNum::zero();
        let mut tot_steps = BigNum::zero();
        for i in 0..self.0.len() {
            let r: &Redeemer = &self.0[i];
            tot_mem = tot_mem.checked_add(&r.ex_units().mem())?;
            tot_steps = tot_steps.checked_add(&r.ex_units().steps())?;
        }
        Ok(ExUnits::new(&tot_mem, &tot_steps))
    }
}

impl From<Vec<Redeemer>> for Redeemers {
    fn from(values: Vec<Redeemer>) -> Self {
        Self(values)
    }
}
