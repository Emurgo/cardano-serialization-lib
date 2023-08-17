#[derive(Eq, Hash, PartialEq, Clone, Debug, FromPrimitive, ToPrimitive)]
pub(crate) enum WitnessSetNames {
    Vkeys = 0,
    NativeScripts = 1,
    Bootstraps = 2,
    PlutusScriptsV1 = 3,
    PlutusData = 4,
    Redeemers = 5,
    PlutusScriptsV2 = 6,
    PlutusScriptsV3 = 7,
}