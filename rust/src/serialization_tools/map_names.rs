
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum TxBodyNames {
    Inputs = 0,
    Outputs = 1,
    Fee = 2,
    // Ttl = 3,
    // Certs = 4,
    // Withdrawals = 5,
    // Update = 6,
    // AuxiliaryDataHash = 7,
    // ValidityStartInterval = 8,
    // Mint = 9,
    // ScriptDataHash = 11,
    // Collateral = 13,
    // RequiredSigners = 14,
    // NetworkId = 15,
    // CollateralReturn = 16,
    // TotalCollateral = 17,
    // ReferenceInputs = 18,
}

impl TxBodyNames {
    pub fn to_number(&self) -> u64 {
        self.clone() as u64
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum WitnessSetNames {
    Vkeys = 0,
    // NativeScripts = 1,
    Bootstraps = 2,
    // PlutusScriptsV1 = 3,
    // PlutusData = 4,
    // Redeemers = 5,
    // PlutusScriptsV2 = 6,
}

impl WitnessSetNames {
    pub fn to_number(&self) -> u64 {
        self.clone() as u64
    }
}