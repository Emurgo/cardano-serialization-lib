use crate::*;

impl_hash_type!(Ed25519KeyHash, 28);
impl_hash_type!(ScriptHash, 28);
impl_hash_type!(AnchorDataHash, 32);
impl_hash_type!(TransactionHash, 32);
impl_hash_type!(GenesisDelegateHash, 28);
impl_hash_type!(GenesisHash, 28);
impl_hash_type!(AuxiliaryDataHash, 32);
impl_hash_type!(PoolMetadataHash, 32);
impl_hash_type!(VRFKeyHash, 32);
impl_hash_type!(BlockHash, 32);
impl_hash_type!(DataHash, 32);
impl_hash_type!(ScriptDataHash, 32);
// We might want to make these two vkeys normal classes later but for now it's just arbitrary bytes for us (used in block parsing)
impl_hash_type!(VRFVKey, 32);
impl_hash_type!(KESVKey, 32);
// same for this signature
//impl_hash_type!(KESSignature, 448);
// TODO: when >32 size trait implementations are out of nightly and into stable
// remove the following manual struct definition and use the above macro again if we
// don't have proper crypto implementations for it.