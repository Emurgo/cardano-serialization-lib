use crate::*;

#[wasm_bindgen]
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
pub struct PoolVotingThresholds {
    pub(crate) motion_no_confidence: UnitInterval,
    pub(crate) committee_normal: UnitInterval,
    pub(crate) committee_no_confidence: UnitInterval,
    pub(crate) hard_fork_initiation: UnitInterval,
}

impl_to_from!(PoolVotingThresholds);

#[wasm_bindgen]
impl PoolVotingThresholds {
    pub fn new(
        motion_no_confidence: &UnitInterval,
        committee_normal: &UnitInterval,
        committee_no_confidence: &UnitInterval,
        hard_fork_initiation: &UnitInterval,
    ) -> Self {
        Self {
            motion_no_confidence: motion_no_confidence.clone(),
            committee_normal: committee_normal.clone(),
            committee_no_confidence: committee_no_confidence.clone(),
            hard_fork_initiation: hard_fork_initiation.clone(),
        }
    }

    pub fn motion_no_confidence(&self) -> UnitInterval {
        self.motion_no_confidence.clone()
    }

    pub fn committee_normal(&self) -> UnitInterval {
        self.committee_normal.clone()
    }

    pub fn committee_no_confidence(&self) -> UnitInterval {
        self.committee_no_confidence.clone()
    }

    pub fn hard_fork_initiation(&self) -> UnitInterval {
        self.hard_fork_initiation.clone()
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Default,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct DrepVotingThresholds {
    pub(crate) motion_no_confidence: UnitInterval,
    pub(crate) committee_normal: UnitInterval,
    pub(crate) committee_no_confidence: UnitInterval,
    pub(crate) update_constitution: UnitInterval,
    pub(crate) hard_fork_initiation: UnitInterval,
    pub(crate) pp_network_group: UnitInterval,
    pub(crate) pp_economic_group: UnitInterval,
    pub(crate) pp_technical_group: UnitInterval,
    pub(crate) pp_governance_group: UnitInterval,
    pub(crate) treasury_withdrawal: UnitInterval,
}

impl_to_from!(DrepVotingThresholds);

#[wasm_bindgen]
impl DrepVotingThresholds {
    pub fn new(
        motion_no_confidence: &UnitInterval,
        committee_normal: &UnitInterval,
        committee_no_confidence: &UnitInterval,
        update_constitution: &UnitInterval,
        hard_fork_initiation: &UnitInterval,
        pp_network_group: &UnitInterval,
        pp_economic_group: &UnitInterval,
        pp_technical_group: &UnitInterval,
        pp_governance_group: &UnitInterval,
        treasury_withdrawal: &UnitInterval,
    ) -> Self {
        Self {
            motion_no_confidence: motion_no_confidence.clone(),
            committee_normal: committee_normal.clone(),
            committee_no_confidence: committee_no_confidence.clone(),
            update_constitution: update_constitution.clone(),
            hard_fork_initiation: hard_fork_initiation.clone(),
            pp_network_group: pp_network_group.clone(),
            pp_economic_group: pp_economic_group.clone(),
            pp_technical_group: pp_technical_group.clone(),
            pp_governance_group: pp_governance_group.clone(),
            treasury_withdrawal: treasury_withdrawal.clone(),
        }
    }

    pub fn new_default() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_motion_no_confidence(&mut self, motion_no_confidence: &UnitInterval) {
        self.motion_no_confidence = motion_no_confidence.clone()
    }

    pub fn set_committee_normal(&mut self, committee_normal: &UnitInterval) {
        self.committee_normal = committee_normal.clone()
    }

    pub fn set_committee_no_confidence(&mut self, committee_no_confidence: &UnitInterval) {
        self.committee_no_confidence = committee_no_confidence.clone()
    }

    pub fn set_update_constitution(&mut self, update_constitution: &UnitInterval) {
        self.update_constitution = update_constitution.clone()
    }

    pub fn set_hard_fork_initiation(&mut self, hard_fork_initiation: &UnitInterval) {
        self.hard_fork_initiation = hard_fork_initiation.clone()
    }

    pub fn set_pp_network_group(&mut self, pp_network_group: &UnitInterval) {
        self.pp_network_group = pp_network_group.clone()
    }

    pub fn set_pp_economic_group(&mut self, pp_economic_group: &UnitInterval) {
        self.pp_economic_group = pp_economic_group.clone()
    }

    pub fn set_pp_technical_group(&mut self, pp_technical_group: &UnitInterval) {
        self.pp_technical_group = pp_technical_group.clone()
    }

    pub fn set_pp_governance_group(&mut self, pp_governance_group: &UnitInterval) {
        self.pp_governance_group = pp_governance_group.clone()
    }

    pub fn set_treasury_withdrawal(&mut self, treasury_withdrawal: &UnitInterval) {
        self.treasury_withdrawal = treasury_withdrawal.clone()
    }

    pub fn motion_no_confidence(&self) -> UnitInterval {
        self.motion_no_confidence.clone()
    }

    pub fn committee_normal(&self) -> UnitInterval {
        self.committee_normal.clone()
    }

    pub fn committee_no_confidence(&self) -> UnitInterval {
        self.committee_no_confidence.clone()
    }

    pub fn update_constitution(&self) -> UnitInterval {
        self.update_constitution.clone()
    }

    pub fn hard_fork_initiation(&self) -> UnitInterval {
        self.hard_fork_initiation.clone()
    }

    pub fn pp_network_group(&self) -> UnitInterval {
        self.pp_network_group.clone()
    }

    pub fn pp_economic_group(&self) -> UnitInterval {
        self.pp_economic_group.clone()
    }

    pub fn pp_technical_group(&self) -> UnitInterval {
        self.pp_technical_group.clone()
    }

    pub fn pp_governance_group(&self) -> UnitInterval {
        self.pp_governance_group.clone()
    }

    pub fn treasury_withdrawal(&self) -> UnitInterval {
        self.treasury_withdrawal.clone()
    }
}

#[wasm_bindgen]
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
pub struct ProtocolParamUpdate {
    pub(crate) minfee_a: Option<Coin>,
    pub(crate) minfee_b: Option<Coin>,
    pub(crate) max_block_body_size: Option<u32>,
    pub(crate) max_tx_size: Option<u32>,
    pub(crate) max_block_header_size: Option<u32>,
    pub(crate) key_deposit: Option<Coin>,
    pub(crate) pool_deposit: Option<Coin>,
    pub(crate) max_epoch: Option<Epoch>,
    // desired number of stake pools
    pub(crate) n_opt: Option<u32>,
    pub(crate) pool_pledge_influence: Option<Rational>,
    pub(crate) expansion_rate: Option<UnitInterval>,
    pub(crate) treasury_growth_rate: Option<UnitInterval>,
    // decentralization constant
    pub(crate) d: Option<UnitInterval>,
    pub(crate) extra_entropy: Option<Nonce>,
    pub(crate) protocol_version: Option<ProtocolVersion>,
    pub(crate) min_pool_cost: Option<Coin>,
    pub(crate) ada_per_utxo_byte: Option<Coin>,
    pub(crate) cost_models: Option<Costmdls>,
    pub(crate) execution_costs: Option<ExUnitPrices>,
    pub(crate) max_tx_ex_units: Option<ExUnits>,
    pub(crate) max_block_ex_units: Option<ExUnits>,
    pub(crate) max_value_size: Option<u32>,
    pub(crate) collateral_percentage: Option<u32>,
    pub(crate) max_collateral_inputs: Option<u32>,
    pub(crate) pool_voting_thresholds: Option<PoolVotingThresholds>,
    pub(crate) drep_voting_thresholds: Option<DrepVotingThresholds>,
    pub(crate) min_committee_size: Option<u32>,
    pub(crate) committee_term_limit: Option<u32>,
    pub(crate) governance_action_validity_period: Option<Epoch>,
    pub(crate) governance_action_deposit: Option<Coin>,
    pub(crate) drep_deposit: Option<Coin>,
    pub(crate) drep_inactivity_period: Option<Epoch>,
}

impl_to_from!(ProtocolParamUpdate);

#[wasm_bindgen]
impl ProtocolParamUpdate {
    pub fn set_minfee_a(&mut self, minfee_a: &Coin) {
        self.minfee_a = Some(minfee_a.clone())
    }

    pub fn minfee_a(&self) -> Option<Coin> {
        self.minfee_a.clone()
    }

    pub fn set_minfee_b(&mut self, minfee_b: &Coin) {
        self.minfee_b = Some(minfee_b.clone())
    }

    pub fn minfee_b(&self) -> Option<Coin> {
        self.minfee_b.clone()
    }

    pub fn set_max_block_body_size(&mut self, max_block_body_size: u32) {
        self.max_block_body_size = Some(max_block_body_size)
    }

    pub fn max_block_body_size(&self) -> Option<u32> {
        self.max_block_body_size.clone()
    }

    pub fn set_max_tx_size(&mut self, max_tx_size: u32) {
        self.max_tx_size = Some(max_tx_size)
    }

    pub fn max_tx_size(&self) -> Option<u32> {
        self.max_tx_size.clone()
    }

    pub fn set_max_block_header_size(&mut self, max_block_header_size: u32) {
        self.max_block_header_size = Some(max_block_header_size)
    }

    pub fn max_block_header_size(&self) -> Option<u32> {
        self.max_block_header_size.clone()
    }

    pub fn set_key_deposit(&mut self, key_deposit: &Coin) {
        self.key_deposit = Some(key_deposit.clone())
    }

    pub fn key_deposit(&self) -> Option<Coin> {
        self.key_deposit.clone()
    }

    pub fn set_pool_deposit(&mut self, pool_deposit: &Coin) {
        self.pool_deposit = Some(pool_deposit.clone())
    }

    pub fn pool_deposit(&self) -> Option<Coin> {
        self.pool_deposit.clone()
    }

    pub fn set_max_epoch(&mut self, max_epoch: Epoch) {
        self.max_epoch = Some(max_epoch.clone())
    }

    pub fn max_epoch(&self) -> Option<Epoch> {
        self.max_epoch.clone()
    }

    pub fn set_n_opt(&mut self, n_opt: u32) {
        self.n_opt = Some(n_opt)
    }

    pub fn n_opt(&self) -> Option<u32> {
        self.n_opt.clone()
    }

    pub fn set_pool_pledge_influence(&mut self, pool_pledge_influence: &Rational) {
        self.pool_pledge_influence = Some(pool_pledge_influence.clone())
    }

    pub fn pool_pledge_influence(&self) -> Option<Rational> {
        self.pool_pledge_influence.clone()
    }

    pub fn set_expansion_rate(&mut self, expansion_rate: &UnitInterval) {
        self.expansion_rate = Some(expansion_rate.clone())
    }

    pub fn expansion_rate(&self) -> Option<UnitInterval> {
        self.expansion_rate.clone()
    }

    pub fn set_treasury_growth_rate(&mut self, treasury_growth_rate: &UnitInterval) {
        self.treasury_growth_rate = Some(treasury_growth_rate.clone())
    }

    pub fn treasury_growth_rate(&self) -> Option<UnitInterval> {
        self.treasury_growth_rate.clone()
    }

    /// !!! DEPRECATED !!!
    /// Since babbage era this param is outdated. But this param you can meet in a pre-babbage block.
    #[deprecated(
        since = "11.0.0",
        note = "Since babbage era this param is outdated. But this param you can meet in a pre-babbage block."
    )]
    pub fn d(&self) -> Option<UnitInterval> {
        self.d.clone()
    }

    /// !!! DEPRECATED !!!
    /// Since babbage era this param is outdated. But this param you can meet in a pre-babbage block.
    #[deprecated(
        since = "11.0.0",
        note = "Since babbage era this param is outdated. But this param you can meet in a pre-babbage block."
    )]
    pub fn extra_entropy(&self) -> Option<Nonce> {
        self.extra_entropy.clone()
    }

    /// !!! DEPRECATED !!!
    /// Since conway era this param is outdated. But this param you can meet in a pre-conway block.
    #[deprecated(
        since = "12.0.0",
        note = "Since conway era this param is outdated. But this param you can meet in a pre-conway block."
    )]
    pub fn set_protocol_version(&mut self, protocol_version: &ProtocolVersion) {
        self.protocol_version = Some(protocol_version.clone())
    }

    pub fn protocol_version(&self) -> Option<ProtocolVersion> {
        self.protocol_version.clone()
    }

    pub fn set_min_pool_cost(&mut self, min_pool_cost: &Coin) {
        self.min_pool_cost = Some(min_pool_cost.clone())
    }

    pub fn min_pool_cost(&self) -> Option<Coin> {
        self.min_pool_cost.clone()
    }

    pub fn set_ada_per_utxo_byte(&mut self, ada_per_utxo_byte: &Coin) {
        self.ada_per_utxo_byte = Some(ada_per_utxo_byte.clone())
    }

    pub fn ada_per_utxo_byte(&self) -> Option<Coin> {
        self.ada_per_utxo_byte.clone()
    }

    pub fn set_cost_models(&mut self, cost_models: &Costmdls) {
        self.cost_models = Some(cost_models.clone())
    }

    pub fn cost_models(&self) -> Option<Costmdls> {
        self.cost_models.clone()
    }

    pub fn set_execution_costs(&mut self, execution_costs: &ExUnitPrices) {
        self.execution_costs = Some(execution_costs.clone())
    }

    pub fn execution_costs(&self) -> Option<ExUnitPrices> {
        self.execution_costs.clone()
    }

    pub fn set_max_tx_ex_units(&mut self, max_tx_ex_units: &ExUnits) {
        self.max_tx_ex_units = Some(max_tx_ex_units.clone())
    }

    pub fn max_tx_ex_units(&self) -> Option<ExUnits> {
        self.max_tx_ex_units.clone()
    }

    pub fn set_max_block_ex_units(&mut self, max_block_ex_units: &ExUnits) {
        self.max_block_ex_units = Some(max_block_ex_units.clone())
    }

    pub fn max_block_ex_units(&self) -> Option<ExUnits> {
        self.max_block_ex_units.clone()
    }

    pub fn set_max_value_size(&mut self, max_value_size: u32) {
        self.max_value_size = Some(max_value_size.clone())
    }

    pub fn max_value_size(&self) -> Option<u32> {
        self.max_value_size.clone()
    }

    pub fn set_collateral_percentage(&mut self, collateral_percentage: u32) {
        self.collateral_percentage = Some(collateral_percentage)
    }

    pub fn collateral_percentage(&self) -> Option<u32> {
        self.collateral_percentage.clone()
    }

    pub fn set_max_collateral_inputs(&mut self, max_collateral_inputs: u32) {
        self.max_collateral_inputs = Some(max_collateral_inputs)
    }

    pub fn max_collateral_inputs(&self) -> Option<u32> {
        self.max_collateral_inputs.clone()
    }

    pub fn set_pool_voting_thresholds(&mut self, pool_voting_thresholds: &PoolVotingThresholds) {
        self.pool_voting_thresholds = Some(pool_voting_thresholds.clone())
    }

    pub fn pool_voting_thresholds(&self) -> Option<PoolVotingThresholds> {
        self.pool_voting_thresholds.clone()
    }

    pub fn set_drep_voting_thresholds(&mut self, drep_voting_thresholds: &DrepVotingThresholds) {
        self.drep_voting_thresholds = Some(drep_voting_thresholds.clone())
    }

    pub fn drep_voting_thresholds(&self) -> Option<DrepVotingThresholds> {
        self.drep_voting_thresholds.clone()
    }

    pub fn set_min_committee_size(&mut self, min_committee_size: u32) {
        self.min_committee_size = Some(min_committee_size)
    }

    pub fn min_committee_size(&self) -> Option<u32> {
        self.min_committee_size.clone()
    }

    pub fn set_committee_term_limit(&mut self, committee_term_limit: u32) {
        self.committee_term_limit = Some(committee_term_limit)
    }

    pub fn committee_term_limit(&self) -> Option<u32> {
        self.committee_term_limit.clone()
    }

    pub fn set_governance_action_validity_period(&mut self, governance_action_validity_period: Epoch) {
        self.governance_action_validity_period = Some(governance_action_validity_period)
    }

    pub fn governance_action_validity_period(&self) -> Option<Epoch> {
        self.governance_action_validity_period.clone()
    }

    pub fn set_governance_action_deposit(&mut self, governance_action_deposit: &Coin) {
        self.governance_action_deposit = Some(governance_action_deposit.clone());
    }

    pub fn governance_action_deposit(&self) -> Option<Coin> {
        self.governance_action_deposit.clone()
    }

    pub fn set_drep_deposit(&mut self, drep_deposit: &Coin) {
        self.drep_deposit = Some(drep_deposit.clone());
    }

    pub fn drep_deposit(&self) -> Option<Coin> {
        self.drep_deposit.clone()
    }

    pub fn set_drep_inactivity_period(&mut self, drep_inactivity_period: Epoch) {
        self.drep_inactivity_period = Some(drep_inactivity_period)
    }

    pub fn drep_inactivity_period(&self) -> Option<Epoch> {
        self.drep_inactivity_period.clone()
    }

    pub fn new() -> Self {
        Self {
            minfee_a: None,
            minfee_b: None,
            max_block_body_size: None,
            max_tx_size: None,
            max_block_header_size: None,
            key_deposit: None,
            pool_deposit: None,
            max_epoch: None,
            n_opt: None,
            pool_pledge_influence: None,
            expansion_rate: None,
            treasury_growth_rate: None,
            d: None,
            extra_entropy: None,
            protocol_version: None,
            min_pool_cost: None,
            ada_per_utxo_byte: None,
            cost_models: None,
            execution_costs: None,
            max_tx_ex_units: None,
            max_block_ex_units: None,
            max_value_size: None,
            collateral_percentage: None,
            max_collateral_inputs: None,
            pool_voting_thresholds: None,
            drep_voting_thresholds: None,
            min_committee_size: None,
            committee_term_limit: None,
            governance_action_validity_period: None,
            governance_action_deposit: None,
            drep_deposit: None,
            drep_inactivity_period: None,
        }
    }
}
