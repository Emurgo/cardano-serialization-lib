use crate::*;

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
#[wasm_bindgen]
pub struct ParameterChangeAction {
    pub(crate) gov_action_id: Option<GovernanceActionId>,
    pub(crate) protocol_param_updates: ProtocolParamUpdate,
    pub(crate) policy_hash: Option<ScriptHash>,
}

impl_to_from!(ParameterChangeAction);

#[wasm_bindgen]
impl ParameterChangeAction {
    pub fn gov_action_id(&self) -> Option<GovernanceActionId> {
        self.gov_action_id.clone()
    }

    pub fn protocol_param_updates(&self) -> ProtocolParamUpdate {
        self.protocol_param_updates.clone()
    }

    pub fn policy_hash(&self) -> Option<ScriptHash> {
        self.policy_hash.clone()
    }

    pub fn new(protocol_param_updates: &ProtocolParamUpdate) -> Self {
        Self {
            gov_action_id: None,
            protocol_param_updates: protocol_param_updates.clone(),
            policy_hash: None,
        }
    }

    pub fn new_with_action_id(
        gov_action_id: &GovernanceActionId,
        protocol_param_updates: &ProtocolParamUpdate,
    ) -> Self {
        Self {
            gov_action_id: Some(gov_action_id.clone()),
            protocol_param_updates: protocol_param_updates.clone(),
            policy_hash: None,
        }
    }

    pub fn new_with_policy_hash(
        protocol_param_updates: &ProtocolParamUpdate,
        policy_hash: &ScriptHash,
    ) -> Self {
        Self {
            gov_action_id: None,
            protocol_param_updates: protocol_param_updates.clone(),
            policy_hash: Some(policy_hash.clone()),
        }
    }

    pub fn new_with_policy_hash_and_action_id(
        gov_action_id: &GovernanceActionId,
        protocol_param_updates: &ProtocolParamUpdate,
        policy_hash: &ScriptHash,
    ) -> Self {
        Self {
            gov_action_id: Some(gov_action_id.clone()),
            protocol_param_updates: protocol_param_updates.clone(),
            policy_hash: Some(policy_hash.clone()),
        }
    }
}
