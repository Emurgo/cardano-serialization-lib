use super::super::*;

pub(super) struct WitnessesCalculator {
    adresses: BTreeSet<Address>,
    vkeys_count: usize,
    boostrap_count: usize,
    total_size: usize,
}

impl WitnessesCalculator {

    pub(super) fn new() -> Self {
        Self {
            adresses: BTreeSet::new(),
            vkeys_count: 0,
            boostrap_count: 0,
            total_size: 0
        }
    }

    pub(super) fn add_address(&mut self, address: &Address) {
        if !self.adresses.contains(address) {
            self.adresses.insert(address.clone());
        }
    }

    pub(super) fn get_full_size(&self) -> usize {
        self.total_size;
    }

    //TODO: replace it by size calculation without serialization and fake entity
    fn make_fake_vkey_witness(&self, address: &Address) -> Vkeywitness {
        unimplemented!()
    }
}