use crate::*;

#[wasm_bindgen]
#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct TransactionBody {
    pub(crate) inputs: TransactionInputs,
    pub(crate) outputs: TransactionOutputs,
    pub(crate) fee: Coin,
    pub(crate) ttl: Option<SlotBigNum>,
    pub(crate) certs: Option<Certificates>,
    pub(crate) withdrawals: Option<Withdrawals>,
    pub(crate) update: Option<Update>,
    pub(crate) auxiliary_data_hash: Option<AuxiliaryDataHash>,
    pub(crate) validity_start_interval: Option<SlotBigNum>,
    pub(crate) mint: Option<Mint>,
    pub(crate) script_data_hash: Option<ScriptDataHash>,
    pub(crate) collateral: Option<TransactionInputs>,
    pub(crate) required_signers: Option<RequiredSigners>,
    pub(crate) network_id: Option<NetworkId>,
    pub(crate) collateral_return: Option<TransactionOutput>,
    pub(crate) total_collateral: Option<Coin>,
    pub(crate) reference_inputs: Option<TransactionInputs>,
    pub(crate) voting_procedures: Option<VotingProcedures>,
    pub(crate) voting_proposals: Option<VotingProposals>,
    pub(crate) donation: Option<Coin>,
    pub(crate) current_treasury_value: Option<Coin>,
}

impl_to_from!(TransactionBody);

#[wasm_bindgen]
impl TransactionBody {
    pub fn inputs(&self) -> TransactionInputs {
        self.inputs.clone()
    }

    pub fn outputs(&self) -> TransactionOutputs {
        self.outputs.clone()
    }

    pub fn fee(&self) -> Coin {
        self.fee.clone()
    }

    /// !!! DEPRECATED !!!
    /// Returns a Slot32 (u32) value in case the underlying original BigNum (u64) value is within the limits.
    /// Otherwise will just raise an error.
    #[deprecated(
    since = "10.1.0",
    note = "Possible boundary error. Use ttl_bignum instead"
    )]
    pub fn ttl(&self) -> Result<Option<Slot32>, JsError> {
        match self.ttl {
            Some(ttl) => match ttl.try_into() {
                Ok(ttl32) => Ok(Some(ttl32)),
                Err(err) => Err(err),
            },
            None => Ok(None),
        }
    }

    pub fn ttl_bignum(&self) -> Option<SlotBigNum> {
        self.ttl
    }

    pub fn set_ttl(&mut self, ttl: &SlotBigNum) {
        self.ttl = Some(ttl.clone())
    }

    pub fn remove_ttl(&mut self) {
        self.ttl = None
    }

    pub fn set_certs(&mut self, certs: &Certificates) {
        self.certs = Some(certs.clone())
    }

    pub fn certs(&self) -> Option<Certificates> {
        self.certs.clone()
    }

    pub fn set_withdrawals(&mut self, withdrawals: &Withdrawals) {
        self.withdrawals = Some(withdrawals.clone())
    }

    pub fn withdrawals(&self) -> Option<Withdrawals> {
        self.withdrawals.clone()
    }

    pub fn set_update(&mut self, update: &Update) {
        self.update = Some(update.clone())
    }

    pub fn update(&self) -> Option<Update> {
        self.update.clone()
    }

    pub fn set_auxiliary_data_hash(&mut self, auxiliary_data_hash: &AuxiliaryDataHash) {
        self.auxiliary_data_hash = Some(auxiliary_data_hash.clone())
    }

    pub fn auxiliary_data_hash(&self) -> Option<AuxiliaryDataHash> {
        self.auxiliary_data_hash.clone()
    }

    /// !!! DEPRECATED !!!
    /// Uses outdated slot number format.
    #[deprecated(
    since = "10.1.0",
    note = "Underlying value capacity of slot (BigNum u64) bigger then Slot32. Use set_validity_start_interval_bignum instead."
    )]
    pub fn set_validity_start_interval(&mut self, validity_start_interval: Slot32) {
        self.validity_start_interval = Some(validity_start_interval.into())
    }

    pub fn set_validity_start_interval_bignum(&mut self, validity_start_interval: &SlotBigNum) {
        self.validity_start_interval = Some(validity_start_interval.clone())
    }

    pub fn validity_start_interval_bignum(&self) -> Option<SlotBigNum> {
        self.validity_start_interval.clone()
    }

    /// !!! DEPRECATED !!!
    /// Returns a Option<Slot32> (u32) value in case the underlying original Option<BigNum> (u64) value is within the limits.
    /// Otherwise will just raise an error.
    /// Use `.validity_start_interval_bignum` instead.
    #[deprecated(
    since = "10.1.0",
    note = "Possible boundary error. Use validity_start_interval_bignum instead"
    )]
    pub fn validity_start_interval(&self) -> Result<Option<Slot32>, JsError> {
        match self.validity_start_interval.clone() {
            Some(interval) => match interval.try_into() {
                Ok(internal32) => Ok(Some(internal32)),
                Err(err) => Err(err),
            },
            None => Ok(None),
        }
    }

    pub fn set_mint(&mut self, mint: &Mint) {
        self.mint = Some(mint.clone())
    }

    pub fn mint(&self) -> Option<Mint> {
        self.mint.clone()
    }

    pub fn set_reference_inputs(&mut self, reference_inputs: &TransactionInputs) {
        self.reference_inputs = Some(reference_inputs.clone())
    }

    pub fn reference_inputs(&self) -> Option<TransactionInputs> {
        self.reference_inputs.clone()
    }

    pub fn set_script_data_hash(&mut self, script_data_hash: &ScriptDataHash) {
        self.script_data_hash = Some(script_data_hash.clone())
    }

    pub fn script_data_hash(&self) -> Option<ScriptDataHash> {
        self.script_data_hash.clone()
    }

    pub fn set_collateral(&mut self, collateral: &TransactionInputs) {
        self.collateral = Some(collateral.clone())
    }

    pub fn collateral(&self) -> Option<TransactionInputs> {
        self.collateral.clone()
    }

    pub fn set_required_signers(&mut self, required_signers: &RequiredSigners) {
        self.required_signers = Some(required_signers.clone())
    }

    pub fn required_signers(&self) -> Option<RequiredSigners> {
        self.required_signers.clone()
    }

    pub fn set_network_id(&mut self, network_id: &NetworkId) {
        self.network_id = Some(network_id.clone())
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        self.network_id.clone()
    }

    pub fn set_collateral_return(&mut self, collateral_return: &TransactionOutput) {
        self.collateral_return = Some(collateral_return.clone());
    }

    pub fn collateral_return(&self) -> Option<TransactionOutput> {
        self.collateral_return.clone()
    }

    pub fn set_total_collateral(&mut self, total_collateral: &Coin) {
        self.total_collateral = Some(total_collateral.clone());
    }

    pub fn total_collateral(&self) -> Option<Coin> {
        self.total_collateral.clone()
    }

    pub fn set_voting_procedures(&mut self, voting_procedures: &VotingProcedures) {
        self.voting_procedures = Some(voting_procedures.clone());
    }

    pub fn voting_procedures(&self) -> Option<VotingProcedures> {
        self.voting_procedures.clone()
    }

    pub fn set_voting_proposals(&mut self, voting_proposals: &VotingProposals) {
        self.voting_proposals = Some(voting_proposals.clone());
    }

    pub fn voting_proposals(&self) -> Option<VotingProposals> {
        self.voting_proposals.clone()
    }

    pub fn set_donation(&mut self, donation: &Coin) {
        self.donation = Some(donation.clone());
    }

    pub fn donation(&self) -> Option<Coin> {
        self.donation.clone()
    }

    pub fn set_current_treasury_value(&mut self, current_treasury_value: &Coin) {
        self.current_treasury_value = Some(current_treasury_value.clone());
    }

    pub fn current_treasury_value(&self) -> Option<Coin> {
        self.current_treasury_value.clone()
    }

    /// !!! DEPRECATED !!!
    /// This constructor uses outdated slot number format for the ttl value.
    /// Use `.new_tx_body` and then `.set_ttl` instead
    #[deprecated(
    since = "10.1.0",
    note = "Underlying value capacity of ttl (BigNum u64) bigger then Slot32. Use new_tx_body instead."
    )]
    pub fn new(
        inputs: &TransactionInputs,
        outputs: &TransactionOutputs,
        fee: &Coin,
        ttl: Option<Slot32>,
    ) -> Self {
        let mut tx = Self::new_tx_body(inputs, outputs, fee);
        if let Some(slot32) = ttl {
            tx.set_ttl(&to_bignum(slot32 as u64));
        }
        tx
    }

    /// Returns a new TransactionBody.
    /// In the new version of "new" we removed optional ttl for support it by wasm_bingen.
    /// Your can use "set_ttl" and "remove_ttl" to set a new value for ttl or set it as None.
    pub fn new_tx_body(
        inputs: &TransactionInputs,
        outputs: &TransactionOutputs,
        fee: &Coin,
    ) -> Self {
        Self {
            inputs: inputs.clone(),
            outputs: outputs.clone(),
            fee: fee.clone(),
            ttl: None,
            certs: None,
            withdrawals: None,
            update: None,
            auxiliary_data_hash: None,
            validity_start_interval: None,
            mint: None,
            script_data_hash: None,
            collateral: None,
            required_signers: None,
            network_id: None,
            collateral_return: None,
            total_collateral: None,
            reference_inputs: None,
            voting_procedures: None,
            voting_proposals: None,
            donation: None,
            current_treasury_value: None,
        }
    }
}