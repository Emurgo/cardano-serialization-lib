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
pub struct CostModel(pub(crate) Vec<Int>);

impl_to_from!(CostModel);

#[wasm_bindgen]
impl CostModel {
    /// Creates a new CostModels instance of an unrestricted length
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Sets the cost at the specified index to the specified value.
    /// In case the operation index is larger than the previous largest used index,
    /// it will fill any inbetween indexes with zeroes
    pub fn set(&mut self, operation: usize, cost: &Int) -> Result<Int, JsError> {
        let len = self.0.len();
        let idx = operation.clone();
        if idx >= len {
            for _ in 0..(idx - len + 1) {
                self.0.push(Int::new_i32(0));
            }
        }
        let old = self.0[idx].clone();
        self.0[idx] = cost.clone();
        Ok(old)
    }

    pub fn get(&self, operation: usize) -> Result<Int, JsError> {
        let max = self.0.len();
        if operation >= max {
            return Err(JsError::from_str(&format!(
                "CostModel operation {} out of bounds. Max is {}",
                operation, max
            )));
        }
        Ok(self.0[operation].clone())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<i128>> for CostModel {
    fn from(values: Vec<i128>) -> Self {
        CostModel(values.iter().map(|x| Int(*x)).collect())
    }
}
