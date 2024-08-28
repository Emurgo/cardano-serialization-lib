use crate::*;

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum HeaderLeaderCertEnum {
    NonceAndLeader(VRFCert, VRFCert),
    VrfResult(VRFCert),
}

#[wasm_bindgen]
#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct HeaderBody {
    pub(crate) block_number: u32,
    pub(crate) slot: SlotBigNum,
    pub(crate) prev_hash: Option<BlockHash>,
    pub(crate) issuer_vkey: Vkey,
    pub(crate) vrf_vkey: VRFVKey,
    pub(crate) leader_cert: HeaderLeaderCertEnum,
    pub(crate) block_body_size: u32,
    pub(crate) block_body_hash: BlockHash,
    pub(crate) operational_cert: OperationalCert,
    pub(crate) protocol_version: ProtocolVersion,
}

impl_to_from!(HeaderBody);

#[wasm_bindgen]
impl HeaderBody {
    pub fn block_number(&self) -> u32 {
        self.block_number.clone()
    }

    /// !!! DEPRECATED !!!
    /// Returns a Slot32 (u32) value in case the underlying original BigNum (u64) value is within the limits.
    /// Otherwise will just raise an error.
    #[deprecated(
        since = "10.1.0",
        note = "Possible boundary error. Use slot_bignum instead"
    )]
    pub fn slot(&self) -> Result<Slot32, JsError> {
        self.slot.clone().try_into()
    }

    pub fn slot_bignum(&self) -> SlotBigNum {
        self.slot.clone()
    }

    pub fn prev_hash(&self) -> Option<BlockHash> {
        self.prev_hash.clone()
    }

    pub fn issuer_vkey(&self) -> Vkey {
        self.issuer_vkey.clone()
    }

    pub fn vrf_vkey(&self) -> VRFVKey {
        self.vrf_vkey.clone()
    }

    /// If this function returns true, the `.nonce_vrf_or_nothing`
    /// and the `.leader_vrf_or_nothing` functions will return
    /// non-empty results
    pub fn has_nonce_and_leader_vrf(&self) -> bool {
        match &self.leader_cert {
            HeaderLeaderCertEnum::NonceAndLeader(_, _) => true,
            _ => false,
        }
    }

    /// Might return nothing in case `.has_nonce_and_leader_vrf` returns false
    pub fn nonce_vrf_or_nothing(&self) -> Option<VRFCert> {
        match &self.leader_cert {
            HeaderLeaderCertEnum::NonceAndLeader(nonce, _) => Some(nonce.clone()),
            _ => None,
        }
    }

    /// Might return nothing in case `.has_nonce_and_leader_vrf` returns false
    pub fn leader_vrf_or_nothing(&self) -> Option<VRFCert> {
        match &self.leader_cert {
            HeaderLeaderCertEnum::NonceAndLeader(_, leader) => Some(leader.clone()),
            _ => None,
        }
    }

    /// If this function returns true, the `.vrf_result_or_nothing`
    /// function will return a non-empty result
    pub fn has_vrf_result(&self) -> bool {
        match &self.leader_cert {
            HeaderLeaderCertEnum::VrfResult(_) => true,
            _ => false,
        }
    }

    /// Might return nothing in case `.has_vrf_result` returns false
    pub fn vrf_result_or_nothing(&self) -> Option<VRFCert> {
        match &self.leader_cert {
            HeaderLeaderCertEnum::VrfResult(cert) => Some(cert.clone()),
            _ => None,
        }
    }

    pub fn block_body_size(&self) -> u32 {
        self.block_body_size.clone()
    }

    pub fn block_body_hash(&self) -> BlockHash {
        self.block_body_hash.clone()
    }

    pub fn operational_cert(&self) -> OperationalCert {
        self.operational_cert.clone()
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version.clone()
    }

    /// !!! DEPRECATED !!!
    /// This constructor uses outdated slot number format.
    /// Use `.new_headerbody` instead
    #[deprecated(
        since = "10.1.0",
        note = "Underlying value capacity of slot (BigNum u64) bigger then Slot32. Use new_bignum instead."
    )]
    pub fn new(
        block_number: u32,
        slot: Slot32,
        prev_hash: Option<BlockHash>,
        issuer_vkey: &Vkey,
        vrf_vkey: &VRFVKey,
        vrf_result: &VRFCert,
        block_body_size: u32,
        block_body_hash: &BlockHash,
        operational_cert: &OperationalCert,
        protocol_version: &ProtocolVersion,
    ) -> Self {
        Self {
            block_number: block_number,
            slot: slot.clone().into(),
            prev_hash: prev_hash.clone(),
            issuer_vkey: issuer_vkey.clone(),
            vrf_vkey: vrf_vkey.clone(),
            leader_cert: HeaderLeaderCertEnum::VrfResult(vrf_result.clone()),
            block_body_size: block_body_size,
            block_body_hash: block_body_hash.clone(),
            operational_cert: operational_cert.clone(),
            protocol_version: protocol_version.clone(),
        }
    }

    pub fn new_headerbody(
        block_number: u32,
        slot: &SlotBigNum,
        prev_hash: Option<BlockHash>,
        issuer_vkey: &Vkey,
        vrf_vkey: &VRFVKey,
        vrf_result: &VRFCert,
        block_body_size: u32,
        block_body_hash: &BlockHash,
        operational_cert: &OperationalCert,
        protocol_version: &ProtocolVersion,
    ) -> Self {
        Self {
            block_number: block_number,
            slot: slot.clone(),
            prev_hash: prev_hash.clone(),
            issuer_vkey: issuer_vkey.clone(),
            vrf_vkey: vrf_vkey.clone(),
            leader_cert: HeaderLeaderCertEnum::VrfResult(vrf_result.clone()),
            block_body_size: block_body_size,
            block_body_hash: block_body_hash.clone(),
            operational_cert: operational_cert.clone(),
            protocol_version: protocol_version.clone(),
        }
    }
}