use super::*;
use crate::Epoch;
use crate::plutus::{Costmdls, CostModel, Language};

// A list of entries where the first element is the epoch after which the cost-model is the correct one to be used.
// The second element is the cost model, which is an array of 166 operations costs, ordered by asc operaion names.
// The third value is the pre-calculated `language_views_encoding` value required for the script hash creation.
// The cost-model values are taken from the genesis block - https://github.com/input-output-hk/cardano-node/blob/master/configuration/cardano/mainnet-alonzo-genesis.json#L26-L195
// The keys on the genesis block object (operation names) are sorted plain alphabetically.
const PLUTUS_DEFAULT_COST_MODELS: [(Epoch, [i32; 166], &str); 1] = [
    (
        0,
        [197209, 0, 1, 1, 396231, 621, 0, 1, 150000, 1000, 0, 1, 150000, 32, 2477736, 29175, 4, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 100, 100, 29773, 100, 150000, 32, 150000, 32, 150000, 32, 150000, 1000, 0, 1, 150000, 32, 150000, 1000, 0, 8, 148000, 425507, 118, 0, 1, 1, 150000, 1000, 0, 8, 150000, 112536, 247, 1, 150000, 10000, 1, 136542, 1326, 1, 1000, 150000, 1000, 1, 150000, 32, 150000, 32, 150000, 32, 1, 1, 150000, 1, 150000, 4, 103599, 248, 1, 103599, 248, 1, 145276, 1366, 1, 179690, 497, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 148000, 425507, 118, 0, 1, 1, 61516, 11218, 0, 1, 150000, 32, 148000, 425507, 118, 0, 1, 1, 148000, 425507, 118, 0, 1, 1, 2477736, 29175, 4, 0, 82363, 4, 150000, 5000, 0, 1, 150000, 32, 197209, 0, 1, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 3345831, 1, 1],
        "a141005901d59f1a000302590001011a00060bc719026d00011a000249f01903e800011a000249f018201a0025cea81971f70419744d186419744d186419744d186419744d186419744d186419744d18641864186419744d18641a000249f018201a000249f018201a000249f018201a000249f01903e800011a000249f018201a000249f01903e800081a000242201a00067e2318760001011a000249f01903e800081a000249f01a0001b79818f7011a000249f0192710011a0002155e19052e011903e81a000249f01903e8011a000249f018201a000249f018201a000249f0182001011a000249f0011a000249f0041a000194af18f8011a000194af18f8011a0002377c190556011a0002bdea1901f1011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000242201a00067e23187600010119f04c192bd200011a000249f018201a000242201a00067e2318760001011a000242201a00067e2318760001011a0025cea81971f704001a000141bb041a000249f019138800011a000249f018201a000302590001011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a00330da70101ff",
    ),
];

#[wasm_bindgen]
pub struct Constants();

#[wasm_bindgen]
impl Constants {

    /// The function accepts the number of the "current" epoch and returns the
    /// default Plutus cost-models object to be used in that epoch
    pub fn plutus_default_cost_models(_epoch: Epoch) -> Costmdls {
        let mut res = Costmdls::new();
        res.insert(
            &Language::new_plutus_v1(),
            &CostModel::from(PLUTUS_DEFAULT_COST_MODELS[0].1),
        );
        res
    }

    /// The function accepts the number of the "current" epoch and returns the
    /// default Plutus language-views-encoding object to be used in that epoch
    #[wasm_bindgen]
    pub fn plutus_default_language_views_encoding(_epoch: Epoch) -> LanguageViewEncoding {
        LanguageViewEncoding(hex::decode(PLUTUS_DEFAULT_COST_MODELS[0].2).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::*;

    #[test]
    pub fn cost_model_test() {
        assert_eq!(
            Constants::plutus_default_cost_models(0).language_views_encoding(),
            Constants::plutus_default_language_views_encoding(0),
        );
        assert_eq!(
            hex::encode(Constants::plutus_default_cost_models(0).language_views_encoding().0),
            PLUTUS_DEFAULT_COST_MODELS[0].2,
        );
    }
}
