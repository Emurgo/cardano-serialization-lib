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
pub struct Costmdls(pub(crate) std::collections::BTreeMap<Language, CostModel>);

impl_to_from!(Costmdls);

#[wasm_bindgen]
impl Costmdls {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &Language, value: &CostModel) -> Option<CostModel> {
        self.0.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &Language) -> Option<CostModel> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> Languages {
        Languages(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }

    pub(crate) fn language_views_encoding(&self) -> Vec<u8> {
        let mut serializer = Serializer::new_vec();
        fn key_len(l: &Language) -> usize {
            if l.kind() == LanguageKind::PlutusV1 {
                let mut serializer = Serializer::new_vec();
                serializer.write_bytes(l.to_bytes()).unwrap();
                return serializer.finalize().len();
            }
            l.to_bytes().len()
        }
        let mut keys: Vec<Language> = self.0.iter().map(|(k, _v)| k.clone()).collect();
        // keys must be in canonical ordering first
        keys.sort_by(|lhs, rhs| match key_len(lhs).cmp(&key_len(rhs)) {
            std::cmp::Ordering::Equal => lhs.cmp(&rhs),
            len_order => len_order,
        });
        serializer
            .write_map(cbor_event::Len::Len(self.0.len() as u64))
            .unwrap();
        for key in keys.iter() {
            if key.kind() == LanguageKind::PlutusV1 {
                serializer.write_bytes(key.to_bytes()).unwrap();
                let cost_model = self.0.get(&key).unwrap();
                // Due to a bug in the cardano-node input-output-hk/cardano-ledger-specs/issues/2512
                // we must use indefinite length serialization in this inner bytestring to match it
                let mut cost_model_serializer = Serializer::new_vec();
                cost_model_serializer
                    .write_array(cbor_event::Len::Indefinite)
                    .unwrap();
                for cost in &cost_model.0 {
                    cost.serialize(&mut cost_model_serializer).unwrap();
                }
                cost_model_serializer
                    .write_special(cbor_event::Special::Break)
                    .unwrap();
                serializer
                    .write_bytes(cost_model_serializer.finalize())
                    .unwrap();
            } else {
                serializer.serialize(key).unwrap();
                serializer.serialize(self.0.get(&key).unwrap()).unwrap();
            }
        }
        serializer.finalize()
    }

    pub fn retain_language_versions(&self, languages: &Languages) -> Costmdls {
        let mut result = Costmdls::new();
        for lang in &languages.0 {
            match self.get(&lang) {
                Some(costmodel) => {
                    result.insert(&lang, &costmodel);
                }
                _ => {}
            }
        }
        result
    }
}
