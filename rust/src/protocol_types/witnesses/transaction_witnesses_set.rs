use crate::*;

#[wasm_bindgen]
#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct TransactionWitnessSet {
    pub(crate) vkeys: Option<Vkeywitnesses>,
    pub(crate) native_scripts: Option<NativeScripts>,
    pub(crate) bootstraps: Option<BootstrapWitnesses>,
    pub(crate) plutus_scripts: Option<PlutusScripts>,
    pub(crate) plutus_data: Option<PlutusList>,
    pub(crate) redeemers: Option<Redeemers>,
}

impl_to_from!(TransactionWitnessSet);

#[wasm_bindgen]
impl TransactionWitnessSet {
    pub fn set_vkeys(&mut self, vkeys: &Vkeywitnesses) {
        if vkeys.len() > 0 {
            self.vkeys = Some(vkeys.clone())
        }
    }

    pub fn vkeys(&self) -> Option<Vkeywitnesses> {
        self.vkeys.clone()
    }

    pub fn set_native_scripts(&mut self, native_scripts: &NativeScripts) {
        if native_scripts.len() > 0 {
            self.native_scripts = Some(native_scripts.deduplicated_clone())
        }
    }

    pub fn native_scripts(&self) -> Option<NativeScripts> {
        self.native_scripts.clone()
    }

    pub fn set_bootstraps(&mut self, bootstraps: &BootstrapWitnesses) {
        self.bootstraps = Some(bootstraps.clone())
    }

    pub fn bootstraps(&self) -> Option<BootstrapWitnesses> {
        self.bootstraps.clone()
    }

    pub fn set_plutus_scripts(&mut self, plutus_scripts: &PlutusScripts) {
        if plutus_scripts.len() > 0 {
            self.plutus_scripts = Some(plutus_scripts.deduplicated_clone())
        }
    }

    pub fn plutus_scripts(&self) -> Option<PlutusScripts> {
        self.plutus_scripts.clone()
    }

    pub fn set_plutus_data(&mut self, plutus_data: &PlutusList) {
        if plutus_data.len() > 0 {
            self.plutus_data = Some(plutus_data.deduplicated_clone())
        }
    }

    pub fn plutus_data(&self) -> Option<PlutusList> {
        self.plutus_data.clone()
    }

    pub fn set_redeemers(&mut self, redeemers: &Redeemers) {
        self.redeemers = Some(redeemers.clone())
    }

    pub fn redeemers(&self) -> Option<Redeemers> {
        self.redeemers.clone()
    }

    pub fn new() -> Self {
        Self {
            vkeys: None,
            native_scripts: None,
            bootstraps: None,
            plutus_scripts: None,
            plutus_data: None,
            redeemers: None,
        }
    }
}