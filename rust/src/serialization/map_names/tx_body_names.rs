#[derive(Eq, Hash, PartialEq, Clone, Debug, FromPrimitive, ToPrimitive)]
pub(crate) enum TxBodyNames {
    Inputs = 0,
    Outputs = 1,
    Fee = 2,
    Ttl = 3,
    Certs = 4,
    Withdrawals = 5,
    Update = 6,
    AuxiliaryDataHash = 7,
    ValidityStartInterval = 8,
    Mint = 9,
    ScriptDataHash = 11,
    Collateral = 13,
    RequiredSigners = 14,
    NetworkId = 15,
    CollateralReturn = 16,
    TotalCollateral = 17,
    ReferenceInputs = 18,
}