use crate::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum MintWitnessEnum {
    Plutus(PlutusScriptSourceEnum, Redeemer),
    NativeScript(NativeScriptSourceEnum),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[wasm_bindgen]
pub struct MintWitness(MintWitnessEnum);

#[wasm_bindgen]
impl MintWitness {
    pub fn new_native_script(native_script: &NativeScriptSource) -> MintWitness {
        MintWitness(MintWitnessEnum::NativeScript(native_script.0.clone()))
    }

    pub fn new_plutus_script(
        plutus_script: &PlutusScriptSource,
        redeemer: &Redeemer,
    ) -> MintWitness {
        MintWitness(MintWitnessEnum::Plutus(
            plutus_script.0.clone(),
            redeemer.clone(),
        ))
    }

    pub(crate) fn script_hash(&self) -> ScriptHash {
        match &self.0 {
            MintWitnessEnum::NativeScript(native_script) => native_script.script_hash(),
            MintWitnessEnum::Plutus(plutus_script, _) => plutus_script.script_hash(),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct NativeMints {
    script: NativeScriptSourceEnum,
    mints: BTreeMap<AssetName, Int>,
}

impl NativeMints {

    #[allow(dead_code)]
    fn script_hash(&self) -> PolicyID {
        match &self.script {
            NativeScriptSourceEnum::NativeScript(script, _) => script.hash(),
            NativeScriptSourceEnum::RefInput(_, script_hash, _) => script_hash.clone(),
        }
    }

    fn ref_input(&self) -> Option<&TransactionInput> {
        match &self.script {
            NativeScriptSourceEnum::RefInput(input, _, _) => Some(input),
            _ => None,
        }
    }

    fn native_script(&self) -> Option<&NativeScript> {
        match &self.script {
            NativeScriptSourceEnum::NativeScript(script, _) => Some(script),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PlutusMints {
    script: PlutusScriptSourceEnum,
    redeemer: Redeemer,
    mints: BTreeMap<AssetName, Int>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ScriptMint {
    Plutus(PlutusMints),
    Native(NativeMints),
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MintBuilder {
    mints: BTreeMap<PolicyID, ScriptMint>,
}

#[wasm_bindgen]
impl MintBuilder {
    pub fn new() -> MintBuilder {
        MintBuilder {
            mints: BTreeMap::new(),
        }
    }

    pub fn add_asset(
        &mut self,
        mint: &MintWitness,
        asset_name: &AssetName,
        amount: &Int,
    ) -> Result<(), JsError> {
        if amount.0 == 0 {
            return Err(JsError::from_str("Mint cannot be zero."));
        }
        self.update_mint_value(mint, asset_name, amount, false)?;
        Ok(())
    }

    pub fn set_asset(
        &mut self,
        mint: &MintWitness,
        asset_name: &AssetName,
        amount: &Int,
    ) -> Result<(), JsError> {
        if amount.0 == 0 {
            return Err(JsError::from_str("Mint cannot be zero."));
        }
        self.update_mint_value(mint, asset_name, amount, true)?;
        Ok(())
    }

    fn update_mint_value(
        &mut self,
        mint_witness: &MintWitness,
        asset_name: &AssetName,
        amount: &Int,
        overwrite: bool,
    ) -> Result<(), JsError> {
        if amount.0 == 0 {
            return Err(JsError::from_str("Mint cannot be zero."));
        }
        let script_mint = self.mints.get(&mint_witness.script_hash());
        Self::validate_mint_witness(mint_witness, script_mint)?;

        match &mint_witness.0 {
            MintWitnessEnum::NativeScript(native_script) => {
                let script_mint =
                    self.mints
                        .entry(native_script.script_hash())
                        .or_insert(ScriptMint::Native(NativeMints {
                            script: native_script.clone(),
                            mints: BTreeMap::new(),
                        }));
                match script_mint {
                    ScriptMint::Native(native_mints) => {
                        let mint = native_mints
                            .mints
                            .entry(asset_name.clone())
                            .or_insert(Int::new(&BigNum::zero()));
                        if overwrite {
                            mint.0 = amount.0;
                        } else {
                            mint.0 += amount.0;
                        }
                    }
                    _ => {}
                }
            }
            MintWitnessEnum::Plutus(plutus_script, redeemer) => {
                let script_mint =
                    self.mints
                        .entry(plutus_script.script_hash())
                        .or_insert(ScriptMint::Plutus(PlutusMints {
                            script: plutus_script.clone(),
                            redeemer: redeemer.clone(),
                            mints: BTreeMap::new(),
                        }));
                match script_mint {
                    ScriptMint::Plutus(plutus_mints) => {
                        let mint = plutus_mints
                            .mints
                            .entry(asset_name.clone())
                            .or_insert(Int::new(&BigNum::zero()));
                        if overwrite {
                            mint.0 = amount.0;
                        } else {
                            mint.0 += amount.0;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn validate_mint_witness(
        mint_witness: &MintWitness,
        current_script_mint: Option<&ScriptMint>,
    ) -> Result<(), JsError> {
        if let Some(current_script_mint) = current_script_mint {
            match &mint_witness.0 {
                MintWitnessEnum::NativeScript(native_script) => {
                    if let ScriptMint::Native(native_mints) = current_script_mint {
                        Self::validate_native_source_type(&native_mints.script, &native_script)?;
                    } else {
                        return Err(JsError::from_str(
                            &BuilderError::MintBuilderDifferentScriptType.as_str(),
                        ));
                    }
                }
                MintWitnessEnum::Plutus(plutus_script, redeemer) => {
                    if let ScriptMint::Plutus(plutus_mints) = current_script_mint {
                        Self::validate_plutus_script_source_type(
                            &plutus_mints.script,
                            &plutus_script,
                        )?;
                        if !plutus_mints.redeemer.partially_eq(redeemer) {
                            return Err(JsError::from_str(
                                &BuilderError::MintBuilderDifferentRedeemerDataAndExUnits(
                                    plutus_mints.redeemer.to_json()?,
                                    redeemer.to_json()?,
                                )
                                .as_str(),
                            ));
                        }
                    } else {
                        return Err(JsError::from_str(
                            &BuilderError::MintBuilderDifferentScriptType.as_str(),
                        ));
                    }
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    fn validate_native_source_type(
        current_script_source: &NativeScriptSourceEnum,
        input_script_source: &NativeScriptSourceEnum,
    ) -> Result<(), JsError> {
        match current_script_source {
            NativeScriptSourceEnum::NativeScript(_, _) => {
                if let NativeScriptSourceEnum::NativeScript(_, _) = input_script_source {
                    Ok(())
                } else {
                    Err(JsError::from_str(
                        &BuilderError::MintBuilderDifferentWitnessTypeNonRef.as_str(),
                    ))
                }
            }
            NativeScriptSourceEnum::RefInput(_, _, _) => {
                if let NativeScriptSourceEnum::RefInput(_, _, _) = input_script_source {
                    Ok(())
                } else {
                    Err(JsError::from_str(
                        &BuilderError::MintBuilderDifferentWitnessTypeRef.as_str(),
                    ))
                }
            }
        }
    }

    fn validate_plutus_script_source_type(
        current_script_source: &PlutusScriptSourceEnum,
        input_script_source: &PlutusScriptSourceEnum,
    ) -> Result<(), JsError> {
        match current_script_source {
            PlutusScriptSourceEnum::RefInput(_, _) => {
                if let PlutusScriptSourceEnum::RefInput(_, _) = input_script_source {
                    Ok(())
                } else {
                    Err(JsError::from_str(
                        &BuilderError::MintBuilderDifferentWitnessTypeRef.as_str(),
                    ))
                }
            }
            PlutusScriptSourceEnum::Script(_, _) => {
                if let PlutusScriptSourceEnum::Script(_, _) = input_script_source {
                    Ok(())
                } else {
                    Err(JsError::from_str(
                        &BuilderError::MintBuilderDifferentWitnessTypeNonRef.as_str(),
                    ))
                }
            }
        }
    }

    pub(crate) fn build_unchecked(&self) -> Mint {
        let mut mint = Mint::new();
        for (policy, script_mint) in self.mints.iter() {
            let mut mint_asset = MintAssets::new();
            match script_mint {
                ScriptMint::Native(native_mints) => {
                    for (asset_name, amount) in &native_mints.mints {
                        mint_asset.insert_unchecked(asset_name, amount.clone());
                    }
                }
                ScriptMint::Plutus(plutus_mints) => {
                    for (asset_name, amount) in &plutus_mints.mints {
                        mint_asset.insert_unchecked(asset_name, amount.clone());
                    }
                }
            }
            mint.insert(&policy, &mint_asset);
        }
        mint
    }

    pub fn build(&self) -> Result<Mint, JsError> {
        let mut mint = Mint::new();
        for (policy, script_mint) in &self.mints {
            let mut mint_asset = MintAssets::new();
            match script_mint {
                ScriptMint::Native(native_mints) => {
                    for (asset_name, amount) in &native_mints.mints {
                        mint_asset.insert(asset_name, amount.clone())?;
                    }
                }
                ScriptMint::Plutus(plutus_mints) => {
                    for (asset_name, amount) in &plutus_mints.mints {
                        mint_asset.insert(asset_name, amount.clone())?;
                    }
                }
            }
            mint.insert(&policy, &mint_asset);
        }
        Ok(mint)
    }

    pub fn get_native_scripts(&self) -> NativeScripts {
        let mut native_scripts = Vec::new();
        for script_mint in self.mints.values() {
            match script_mint {
                ScriptMint::Native(native_mints) => {
                    if let Some(script) = native_mints.native_script() {
                        native_scripts.push(script.clone());
                    }
                }
                _ => {}
            }
        }
        NativeScripts(native_scripts)
    }

    pub fn get_plutus_witnesses(&self) -> PlutusWitnesses {
        let mut plutus_witnesses = Vec::new();
        let tag = RedeemerTag::new_mint();
        for (index, (_, script_mint)) in self.mints.iter().enumerate() {
            match script_mint {
                ScriptMint::Plutus(plutus_mints) => {
                    plutus_witnesses.push(PlutusWitness::new_with_ref_without_datum(
                        &PlutusScriptSource(plutus_mints.script.clone()),
                        &plutus_mints.redeemer.clone_with_index_and_tag(
                            &BigNum::from(index),
                            &tag,
                        ),
                    ));
                }
                _ => {}
            }
        }
        PlutusWitnesses(plutus_witnesses)
    }

    pub fn get_ref_inputs(&self) -> TransactionInputs {
        let mut reference_inputs = Vec::new();
        for script_mint in self.mints.values() {
            match script_mint {
                ScriptMint::Plutus(plutus_mints) => {
                    if let PlutusScriptSourceEnum::RefInput(ref_script, _) = &plutus_mints.script {
                        reference_inputs.push(ref_script.input_ref.clone());
                    }
                }
                ScriptMint::Native(native_mints) => {
                    if let Some(input) = native_mints.ref_input() {
                        reference_inputs.push(input.clone());
                    }
                }
            }
        }
        TransactionInputs::from_vec(reference_inputs)
    }

    pub fn get_redeemers(&self) -> Result<Redeemers, JsError> {
        let tag = RedeemerTag::new_mint();
        let mut redeeemers = Vec::new();
        let mut index = BigNum::zero();
        for (_, script_mint) in &self.mints {
            match script_mint {
                ScriptMint::Plutus(plutus_mints) => {
                    redeeemers.push(plutus_mints.redeemer.clone_with_index_and_tag(&index, &tag));
                    index = index.checked_add(&BigNum::one())?;
                }
                _ => {
                    index = index.checked_add(&BigNum::one())?;
                }
            }
        }
        Ok(Redeemers::from(redeeemers))
    }

    pub fn has_plutus_scripts(&self) -> bool {
        for script_mint in self.mints.values() {
            match script_mint {
                ScriptMint::Plutus(_) => {
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    pub fn has_native_scripts(&self) -> bool {
        for script_mint in self.mints.values() {
            match script_mint {
                ScriptMint::Native(_) => {
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    pub(crate) fn get_used_plutus_lang_versions(&self) -> BTreeSet<Language> {
        let mut used_langs = BTreeSet::new();
        for (_, script_mint) in &self.mints {
            match script_mint {
                ScriptMint::Plutus(plutus_mints) => {
                    used_langs.insert(plutus_mints.script.language());
                }
                _ => {}
            }
        }
        used_langs
    }

    //return only ref inputs that are script refs with added size
    //used for calculating the fee for the transaction
    //another ref input and also script ref input without size are filtered out
    pub(crate) fn get_script_ref_inputs_with_size(
        &self,
    ) -> impl Iterator<Item = (&TransactionInput, usize)> {
        self.mints.iter().filter_map(|(_, script_mint)| {
            if let ScriptMint::Plutus(plutus_mints) = script_mint {
                if let PlutusScriptSourceEnum::RefInput(script_ref, _) = &plutus_mints.script {
                    return Some((&script_ref.input_ref, script_ref.script_size));
                }
            }
            None
        })
    }
}
