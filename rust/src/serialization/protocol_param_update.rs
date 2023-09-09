use crate::*;
use crate::serialization::utils::check_len;

impl Serialize for PoolVotingThresholds {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.motion_no_confidence.serialize(serializer)?;
        self.committee_normal.serialize(serializer)?;
        self.committee_no_confidence.serialize(serializer)?;
        self.hard_fork_initiation.serialize(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(PoolVotingThresholds);

impl DeserializeEmbeddedGroup for PoolVotingThresholds {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            4,
            "[\
            motion_no_confidence, \
            committee_normal, \
            committee_no_confidence, \
            hard_fork_initiation\
            ]",
        )?;

        let motion_no_confidence = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("motion_no_confidence"))?;
        let committee_normal = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("committee_normal"))?;
        let committee_no_confidence = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("committee_no_confidence"))?;
        let hard_fork_initiation = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("hard_fork_initiation"))?;

        return Ok(PoolVotingThresholds {
            motion_no_confidence,
            committee_normal,
            committee_no_confidence,
            hard_fork_initiation,
        });
    }
}

impl Serialize for DrepVotingThresholds {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(10))?;
        self.motion_no_confidence.serialize(serializer)?;
        self.committee_normal.serialize(serializer)?;
        self.committee_no_confidence.serialize(serializer)?;
        self.update_constitution.serialize(serializer)?;
        self.hard_fork_initiation.serialize(serializer)?;
        self.pp_network_group.serialize(serializer)?;
        self.pp_economic_group.serialize(serializer)?;
        self.pp_technical_group.serialize(serializer)?;
        self.pp_governance_group.serialize(serializer)?;
        self.treasury_withdrawal.serialize(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(DrepVotingThresholds);

impl DeserializeEmbeddedGroup for DrepVotingThresholds {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            10,
            "[\
            motion_no_confidence, \
            committee_normal, \
            committee_no_confidence, \
            update_constitution, \
            hard_fork_initiation, \
            pp_network_group, \
            pp_economic_group, \
            pp_technical_group, \
            pp_governance_group, \
            treasury_withdrawal\
            ]",
        )?;

        let motion_no_confidence = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("motion_no_confidence"))?;
        let committee_normal = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("committee_normal"))?;
        let committee_no_confidence = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("committee_no_confidence"))?;
        let update_constitution = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("update_constitution"))?;
        let hard_fork_initiation = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("hard_fork_initiation"))?;
        let pp_network_group = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("pp_network_group"))?;
        let pp_economic_group = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("pp_economic_group"))?;
        let pp_technical_group = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("pp_technical_group"))?;
        let pp_governance_group = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("pp_governance_group"))?;
        let treasury_withdrawal = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("treasury_withdrawal"))?;

        return Ok(DrepVotingThresholds {
            motion_no_confidence,
            committee_normal,
            committee_no_confidence,
            update_constitution,
            hard_fork_initiation,
            pp_network_group,
            pp_economic_group,
            pp_technical_group,
            pp_governance_group,
            treasury_withdrawal,
        });
    }
}

impl cbor_event::se::Serialize for ProtocolParamUpdate {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(
            match &self.minfee_a {
                Some(_) => 1,
                None => 0,
            } + match &self.minfee_b {
                Some(_) => 1,
                None => 0,
            } + match &self.max_block_body_size {
                Some(_) => 1,
                None => 0,
            } + match &self.max_tx_size {
                Some(_) => 1,
                None => 0,
            } + match &self.max_block_header_size {
                Some(_) => 1,
                None => 0,
            } + match &self.key_deposit {
                Some(_) => 1,
                None => 0,
            } + match &self.pool_deposit {
                Some(_) => 1,
                None => 0,
            } + match &self.max_epoch {
                Some(_) => 1,
                None => 0,
            } + match &self.n_opt {
                Some(_) => 1,
                None => 0,
            } + match &self.pool_pledge_influence {
                Some(_) => 1,
                None => 0,
            } + match &self.expansion_rate {
                Some(_) => 1,
                None => 0,
            } + match &self.treasury_growth_rate {
                Some(_) => 1,
                None => 0,
            } + match &self.d {
                Some(_) => 1,
                None => 0,
            } + match &self.extra_entropy {
                Some(_) => 1,
                None => 0,
            } + match &self.protocol_version {
                Some(_) => 1,
                None => 0,
            } + match &self.min_pool_cost {
                Some(_) => 1,
                None => 0,
            } + match &self.ada_per_utxo_byte {
                Some(_) => 1,
                None => 0,
            } + match &self.cost_models {
                Some(_) => 1,
                None => 0,
            } + match &self.execution_costs {
                Some(_) => 1,
                None => 0,
            } + match &self.max_tx_ex_units {
                Some(_) => 1,
                None => 0,
            } + match &self.max_block_ex_units {
                Some(_) => 1,
                None => 0,
            } + match &self.max_value_size {
                Some(_) => 1,
                None => 0,
            } + match &self.collateral_percentage {
                Some(_) => 1,
                None => 0,
            } + match &self.max_collateral_inputs {
                Some(_) => 1,
                None => 0,
            } + match &self.pool_voting_thresholds {
                Some(_) => 1,
                None => 0,
            } + match &self.drep_voting_thresholds {
                Some(_) => 1,
                None => 0,
            } + match &self.min_committee_size {
                Some(_) => 1,
                None => 0,
            } + match &self.committee_term_limit {
                Some(_) => 1,
                None => 0,
            } + match &self.governance_action_validity_period {
                Some(_) => 1,
                None => 0,
            } + match &self.governance_action_deposit {
                Some(_) => 1,
                None => 0,
            } + match &self.drep_deposit {
                Some(_) => 1,
                None => 0,
            } + match &self.drep_inactivity_period {
                Some(_) => 1,
                None => 0,
            },
        ))?;
        if let Some(field) = &self.minfee_a {
            serializer.write_unsigned_integer(0)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.minfee_b {
            serializer.write_unsigned_integer(1)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_block_body_size {
            serializer.write_unsigned_integer(2)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_tx_size {
            serializer.write_unsigned_integer(3)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_block_header_size {
            serializer.write_unsigned_integer(4)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.key_deposit {
            serializer.write_unsigned_integer(5)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.pool_deposit {
            serializer.write_unsigned_integer(6)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_epoch {
            serializer.write_unsigned_integer(7)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.n_opt {
            serializer.write_unsigned_integer(8)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.pool_pledge_influence {
            serializer.write_unsigned_integer(9)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.expansion_rate {
            serializer.write_unsigned_integer(10)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.treasury_growth_rate {
            serializer.write_unsigned_integer(11)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.d {
            serializer.write_unsigned_integer(12)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.extra_entropy {
            serializer.write_unsigned_integer(13)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.protocol_version {
            serializer.write_unsigned_integer(14)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.min_pool_cost {
            serializer.write_unsigned_integer(16)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.ada_per_utxo_byte {
            serializer.write_unsigned_integer(17)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.cost_models {
            serializer.write_unsigned_integer(18)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.execution_costs {
            serializer.write_unsigned_integer(19)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_tx_ex_units {
            serializer.write_unsigned_integer(20)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_block_ex_units {
            serializer.write_unsigned_integer(21)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_value_size {
            serializer.write_unsigned_integer(22)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.collateral_percentage {
            serializer.write_unsigned_integer(23)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_collateral_inputs {
            serializer.write_unsigned_integer(24)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.pool_voting_thresholds {
            serializer.write_unsigned_integer(25)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.drep_voting_thresholds {
            serializer.write_unsigned_integer(26)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.min_committee_size {
            serializer.write_unsigned_integer(27)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.committee_term_limit {
            serializer.write_unsigned_integer(28)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.governance_action_validity_period {
            serializer.write_unsigned_integer(29)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.governance_action_deposit {
            serializer.write_unsigned_integer(30)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.drep_deposit {
            serializer.write_unsigned_integer(31)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.drep_inactivity_period {
            serializer.write_unsigned_integer(32)?;
            field.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for ProtocolParamUpdate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(len);
            let mut minfee_a = None;
            let mut minfee_b = None;
            let mut max_block_body_size = None;
            let mut max_tx_size = None;
            let mut max_block_header_size = None;
            let mut key_deposit = None;
            let mut pool_deposit = None;
            let mut max_epoch = None;
            let mut n_opt = None;
            let mut pool_pledge_influence = None;
            let mut expansion_rate = None;
            let mut treasury_growth_rate = None;
            let mut d = None;
            let mut extra_entropy = None;
            let mut protocol_version = None;
            let mut min_pool_cost = None;
            let mut ada_per_utxo_byte = None;
            let mut cost_models = None;
            let mut execution_costs = None;
            let mut max_tx_ex_units = None;
            let mut max_block_ex_units = None;
            let mut max_value_size = None;
            let mut collateral_percentage = None;
            let mut max_collateral_inputs = None;
            let mut pool_voting_thresholds = None;
            let mut drep_voting_thresholds = None;
            let mut min_committee_size = None;
            let mut committee_term_limit = None;
            let mut governance_action_validity_period = None;
            let mut governance_action_deposit = None;
            let mut drep_deposit = None;
            let mut drep_inactivity_period = None;

            let mut read = 0;
            while match len {
                cbor_event::Len::Len(n) => read < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                        0 => {
                            if minfee_a.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            minfee_a = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("minfee_a"))?,
                            );
                        }
                        1 => {
                            if minfee_b.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            minfee_b = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("minfee_b"))?,
                            );
                        }
                        2 => {
                            if max_block_body_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            max_block_body_size = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("max_block_body_size"))?,
                            );
                        }
                        3 => {
                            if max_tx_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            max_tx_size = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("max_tx_size"))?,
                            );
                        }
                        4 => {
                            if max_block_header_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            max_block_header_size = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("max_block_header_size"))?,
                            );
                        }
                        5 => {
                            if key_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            key_deposit = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("key_deposit"))?,
                            );
                        }
                        6 => {
                            if pool_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            pool_deposit = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("pool_deposit"))?,
                            );
                        }
                        7 => {
                            if max_epoch.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            max_epoch = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Epoch::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("max_epoch"))?,
                            );
                        }
                        8 => {
                            if n_opt.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(8)).into());
                            }
                            n_opt = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("n_opt"))?,
                            );
                        }
                        9 => {
                            if pool_pledge_influence.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(9)).into());
                            }
                            pool_pledge_influence = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Rational::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("pool_pledge_influence"))?,
                            );
                        }
                        10 => {
                            if expansion_rate.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(10)).into());
                            }
                            expansion_rate = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(UnitInterval::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("expansion_rate"))?,
                            );
                        }
                        11 => {
                            if treasury_growth_rate.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(11)).into());
                            }
                            treasury_growth_rate = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(UnitInterval::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("treasury_growth_rate"))?,
                            );
                        }
                        12 => {
                            if d.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(12)).into());
                            }
                            d = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(UnitInterval::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("d"))?,
                            );
                        }
                        13 => {
                            if extra_entropy.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(13)).into());
                            }
                            extra_entropy = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Nonce::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("extra_entropy"))?,
                            );
                        }
                        14 => {
                            if protocol_version.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(14)).into());
                            }
                            protocol_version = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(ProtocolVersion::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("protocol_version"))?,
                            );
                        }
                        16 => {
                            if min_pool_cost.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(16)).into());
                            }
                            min_pool_cost = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("min_pool_cost"))?,
                            );
                        }
                        17 => {
                            if ada_per_utxo_byte.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(17)).into());
                            }
                            ada_per_utxo_byte = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("ada_per_utxo_byte"))?,
                            );
                        }
                        18 => {
                            if cost_models.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(18)).into());
                            }
                            cost_models = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Costmdls::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("cost_models"))?,
                            );
                        }
                        19 => {
                            if execution_costs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(19)).into());
                            }
                            execution_costs = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(ExUnitPrices::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("execution_costs"))?,
                            );
                        }
                        20 => {
                            if max_tx_ex_units.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(20)).into());
                            }
                            max_tx_ex_units = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(ExUnits::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("max_tx_ex_units"))?,
                            );
                        }
                        21 => {
                            if max_block_ex_units.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(21)).into());
                            }
                            max_block_ex_units = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(ExUnits::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("max_block_ex_units"))?,
                            );
                        }
                        22 => {
                            if max_value_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(22)).into());
                            }
                            max_value_size = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("max_value_size"))?,
                            );
                        }
                        23 => {
                            if collateral_percentage.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(23)).into());
                            }
                            collateral_percentage = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("collateral_percentage"))?,
                            );
                        }
                        24 => {
                            if max_collateral_inputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(24)).into());
                            }
                            max_collateral_inputs = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("max_collateral_inputs"))?,
                            );
                        }
                        25 => {
                            if pool_voting_thresholds.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(25)).into());
                            }
                            pool_voting_thresholds = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(PoolVotingThresholds::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("pool_voting_thresholds"))?,
                            );
                        }
                        26 => {
                            if drep_voting_thresholds.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(26)).into());
                            }
                            drep_voting_thresholds = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(DrepVotingThresholds::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("drep_voting_thresholds"))?,
                            );
                        }
                        27 => {
                            if min_committee_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(27)).into());
                            }
                            min_committee_size = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("min_committee_size"))?,
                            );
                        }
                        28 => {
                            if committee_term_limit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(28)).into());
                            }
                            committee_term_limit = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Epoch::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("committee_term_limit"))?,
                            );
                        }
                        29 => {
                            if governance_action_validity_period.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(29)).into());
                            }
                            governance_action_validity_period = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Epoch::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("governance_action_validity_period"))?,
                            );
                        }
                        30 => {
                            if governance_action_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(30)).into());
                            }
                            governance_action_deposit = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("governance_action_deposit"))?,
                            );
                        }
                        31 => {
                            if drep_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(31)).into());
                            }
                            drep_deposit = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("drep_deposit"))?,
                            );
                        }
                        32 => {
                            if drep_inactivity_period.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(32)).into());
                            }
                            drep_inactivity_period = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Epoch::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("drep_inactivity_period"))?,
                            );
                        }
                        unknown_key => {
                            return Err(
                                DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()
                            )
                        }
                    },
                    CBORType::Text => match raw.text()?.as_str() {
                        unknown_key => {
                            return Err(DeserializeFailure::UnknownKey(Key::Str(
                                unknown_key.to_owned(),
                            ))
                                .into())
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::Len::Len(_) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => {
                        return Err(DeserializeFailure::UnexpectedKeyType(other_type).into())
                    }
                }
                read += 1;
            }
            read_len.finish()?;
            Ok(Self {
                minfee_a,
                minfee_b,
                max_block_body_size,
                max_tx_size,
                max_block_header_size,
                key_deposit,
                pool_deposit,
                max_epoch,
                n_opt,
                pool_pledge_influence,
                expansion_rate,
                treasury_growth_rate,
                d,
                extra_entropy,
                protocol_version,
                min_pool_cost,
                ada_per_utxo_byte,
                cost_models,
                execution_costs,
                max_tx_ex_units,
                max_block_ex_units,
                max_value_size,
                collateral_percentage,
                max_collateral_inputs,
                pool_voting_thresholds,
                drep_voting_thresholds,
                min_committee_size,
                committee_term_limit,
                governance_action_validity_period,
                governance_action_deposit,
                drep_deposit,
                drep_inactivity_period,
            })
        })()
            .map_err(|e| e.annotate("ProtocolParamUpdate"))
    }
}