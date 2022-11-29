use super::super::*;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct UtxoIndex(pub(super) usize);

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct AssetIndex(pub(super) usize);

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PolicyIndex(pub(super) usize);

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct PlaneAssetId(pub(super) PolicyIndex, pub(super) AssetName);