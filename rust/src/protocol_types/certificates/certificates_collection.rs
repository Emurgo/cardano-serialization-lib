use crate::*;

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct Certificates {
    pub(crate) certs: Vec<Certificate>,
    pub(crate) dedup: BTreeSet<Certificate>
}

impl_to_from!(Certificates);

impl NoneOrEmpty for Certificates {
    fn is_none_or_empty(&self) -> bool {
        self.certs.is_empty()
    }
}

#[wasm_bindgen]
impl Certificates {
    pub fn new() -> Self {
        Self {
            certs: Vec::new(),
            dedup: BTreeSet::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.certs.len()
    }

    pub fn get(&self, index: usize) -> Certificate {
        self.certs[index].clone()
    }

    /// Add a new `Certificate` to the set.
    /// Returns `true` if the element was not already present in the set.
    pub fn add(&mut self, elem: &Certificate) -> bool {
        if self.dedup.insert(elem.clone()) {
            self.certs.push(elem.clone());
            true
        } else {
            false
        }
    }

    pub(crate) fn add_move(&mut self, elem: Certificate) {
        if self.dedup.insert(elem.clone()) {
            self.certs.push(elem);
        }
    }


    pub(crate) fn from_vec(certs_vec: Vec<Certificate>) -> Self {
        let mut certs = Self::new();
        for cert in certs_vec {
            certs.add_move(cert);
        }
        certs
    }
}
