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
pub(crate) enum GovernanceActionEnum {
    ParameterChangeAction(ParameterChangeAction),
    HardForkInitiationAction(HardForkInitiationAction),
    TreasuryWithdrawalsAction(TreasuryWithdrawalsAction),
    NoConfidenceAction(NoConfidenceAction),
    UpdateCommitteeAction(UpdateCommitteeAction),
    NewConstitutionAction(NewConstitutionAction),
    InfoAction(InfoAction),
}

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
pub enum GovernanceActionKind {
    ParameterChangeAction = 0,
    HardForkInitiationAction = 1,
    TreasuryWithdrawalsAction = 2,
    NoConfidenceAction = 3,
    UpdateCommitteeAction = 4,
    NewConstitutionAction = 5,
    InfoAction = 6,
}

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
pub struct GovernanceAction(pub(crate) GovernanceActionEnum);

impl_to_from!(GovernanceAction);

#[wasm_bindgen]
impl GovernanceAction {
    pub fn new_parameter_change_action(
        parameter_change_action: &ParameterChangeAction,
    ) -> Self {
        Self(GovernanceActionEnum::ParameterChangeAction(
            parameter_change_action.clone(),
        ))
    }

    pub fn new_hard_fork_initiation_action(
        hard_fork_initiation_action: &HardForkInitiationAction,
    ) -> Self {
        Self(GovernanceActionEnum::HardForkInitiationAction(
            hard_fork_initiation_action.clone(),
        ))
    }

    pub fn new_treasury_withdrawals_action(
        treasury_withdrawals_action: &TreasuryWithdrawalsAction,
    ) -> Self {
        Self(GovernanceActionEnum::TreasuryWithdrawalsAction(
            treasury_withdrawals_action.clone(),
        ))
    }

    pub fn new_no_confidence_action(no_confidence_action: &NoConfidenceAction) -> Self {
        Self(GovernanceActionEnum::NoConfidenceAction(
            no_confidence_action.clone(),
        ))
    }

    pub fn new_new_committee_action(new_committee_action: &UpdateCommitteeAction) -> Self {
        Self(GovernanceActionEnum::UpdateCommitteeAction(
            new_committee_action.clone(),
        ))
    }

    pub fn new_new_constitution_action(
        new_constitution_action: &NewConstitutionAction,
    ) -> Self {
        Self(GovernanceActionEnum::NewConstitutionAction(
            new_constitution_action.clone(),
        ))
    }

    pub fn new_info_action(info_action: &InfoAction) -> Self {
        Self(GovernanceActionEnum::InfoAction(info_action.clone()))
    }

    pub fn kind(&self) -> GovernanceActionKind {
        match &self.0 {
            GovernanceActionEnum::ParameterChangeAction(_) => {
                GovernanceActionKind::ParameterChangeAction
            }
            GovernanceActionEnum::HardForkInitiationAction(_) => {
                GovernanceActionKind::HardForkInitiationAction
            }
            GovernanceActionEnum::TreasuryWithdrawalsAction(_) => {
                GovernanceActionKind::TreasuryWithdrawalsAction
            }
            GovernanceActionEnum::NoConfidenceAction(_) => GovernanceActionKind::NoConfidenceAction,
            GovernanceActionEnum::UpdateCommitteeAction(_) => GovernanceActionKind::UpdateCommitteeAction,
            GovernanceActionEnum::NewConstitutionAction(_) => {
                GovernanceActionKind::NewConstitutionAction
            }
            GovernanceActionEnum::InfoAction(_) => GovernanceActionKind::InfoAction,
        }
    }

    pub fn as_parameter_change_action(&self) -> Option<ParameterChangeAction> {
        match &self.0 {
            GovernanceActionEnum::ParameterChangeAction(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_hard_fork_initiation_action(&self) -> Option<HardForkInitiationAction> {
        match &self.0 {
            GovernanceActionEnum::HardForkInitiationAction(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_treasury_withdrawals_action(&self) -> Option<TreasuryWithdrawalsAction> {
        match &self.0 {
            GovernanceActionEnum::TreasuryWithdrawalsAction(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_no_confidence_action(&self) -> Option<NoConfidenceAction> {
        match &self.0 {
            GovernanceActionEnum::NoConfidenceAction(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_new_committee_action(&self) -> Option<UpdateCommitteeAction> {
        match &self.0 {
            GovernanceActionEnum::UpdateCommitteeAction(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_new_constitution_action(&self) -> Option<NewConstitutionAction> {
        match &self.0 {
            GovernanceActionEnum::NewConstitutionAction(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_info_action (&self) -> Option<InfoAction> {
        match &self.0 {
            GovernanceActionEnum::InfoAction(p) => Some(p.clone()),
            _ => None,
        }
    }
}
