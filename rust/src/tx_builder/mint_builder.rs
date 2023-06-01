use super::*;
use crate::plutus::Redeemer;
use crate::tx_builder::script_structs::PlutusScriptSourceEnum;
use crate::tx_builder::script_structs::PlutusScriptSource;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum MintWitnessEnum {
    Plutus(PlutusScriptSourceEnum, Redeemer),
    NativeScript(NativeScript),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[wasm_bindgen]
pub struct MintWitness(MintWitnessEnum);

#[wasm_bindgen]
impl MintWitness {
    pub fn new_native_script(native_script: &NativeScript) -> MintWitness {
        MintWitness(MintWitnessEnum::NativeScript(native_script.clone()))
    }

    pub fn new_plutus_script(plutus_script: &PlutusScriptSource, redeemer: &Redeemer) -> MintWitness {
        MintWitness(MintWitnessEnum::Plutus(plutus_script.0.clone(), redeemer.clone()))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct NativeMints {
    script: NativeScript,
    mints: BTreeMap<AssetName, Int>
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PlutusMints {
    script: PlutusScriptSourceEnum,
    redeemer_mints: BTreeMap<Redeemer, BTreeMap<AssetName, Int>>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ScriptMint {
    Plutus(PlutusMints),
    Native(NativeMints),
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MintBuilder {
    mints: BTreeMap<PolicyID, ScriptMint>
}

#[wasm_bindgen]
impl MintBuilder {
    pub fn new() -> MintBuilder {
        MintBuilder {
            mints: BTreeMap::new(),
        }
    }

    pub fn add_asset(&mut self, mint: &MintWitness, asset_name: &AssetName, amount: &Int) {
        self.update_mint_value(mint, asset_name, amount, false);
    }

    pub fn set_asset(&mut self, mint: &MintWitness, asset_name: &AssetName, amount: &Int) {
        self.update_mint_value(mint, asset_name, amount, true);
    }

    fn update_mint_value(&mut self, mint: &MintWitness, asset_name: &AssetName, amount: &Int, overwrite: bool) {
        match &mint.0 {
            MintWitnessEnum::NativeScript(native_script) => {
                let script_mint = self.mints.entry(native_script.hash()).or_insert(ScriptMint::Native(NativeMints {
                    script: native_script.clone(),
                    mints: BTreeMap::new(),
                }));
                match script_mint {
                    ScriptMint::Native(native_mints) => {
                        let mint = native_mints.mints.entry(asset_name.clone()).or_insert(Int::new(&BigNum::zero()));
                        if overwrite {
                            mint.0 = amount.0;
                        } else {
                            mint.0 += amount.0;
                        }
                    },
                    _ => {},
                }
            },
            MintWitnessEnum::Plutus(plutus_script, redeemer) => {
                let script_mint = self.mints.entry(plutus_script.script_hash()).or_insert(ScriptMint::Plutus(PlutusMints {
                    script: plutus_script.clone(),
                    redeemer_mints: BTreeMap::new(),
                }));
                match script_mint {
                    ScriptMint::Plutus(plutus_mints) => {
                        let redeemer_mints = plutus_mints.redeemer_mints.entry(redeemer.clone()).or_insert(BTreeMap::new());
                        let mint = redeemer_mints.entry(asset_name.clone()).or_insert(Int::new(&BigNum::zero()));
                        if overwrite {
                            mint.0 = amount.0;
                        } else {
                            mint.0 += amount.0;
                        }
                    },
                    _ => {},
                }
            },
        }
    }

    pub fn build(&self) -> Mint {
        let mut mint = Mint::new();
        for (_, script_mint) in self.mints.iter() {
            match script_mint {
                ScriptMint::Native(native_mints) => {
                    let mut mint_asset = MintAssets::new();
                    for (asset_name, amount) in &native_mints.mints {
                        mint_asset.insert(asset_name, amount.clone());
                    }
                    mint.insert(&native_mints.script.hash(), &mint_asset);
                },
                ScriptMint::Plutus(plutus_mints) => {
                    for (_, redeemer_mints) in &plutus_mints.redeemer_mints {
                        let mut mint_asset = MintAssets::new();
                        for (asset_name, amount) in redeemer_mints {
                            mint_asset.insert(asset_name, amount.clone());

                        }
                        mint.insert(&plutus_mints.script.script_hash(), &mint_asset);
                    }
                }
            }
        }
        mint
    }

    pub fn get_native_scripts(&self) -> NativeScripts {
        let mut native_scripts = Vec::new();
        for script_mint in self.mints.values() {
            match script_mint {
                ScriptMint::Native(native_mints) => {
                    native_scripts.push(native_mints.script.clone());
                },
                _ => {},
            }
        }
        NativeScripts(native_scripts)
    }

    pub fn get_plutus_witnesses(&self) -> PlutusWitnesses {
        let mut plutus_witnesses = Vec::new();
        for script_mint in self.mints.values() {
            match script_mint {
                ScriptMint::Plutus(plutus_mints) => {
                    for (redeemer, _) in &plutus_mints.redeemer_mints {
                        plutus_witnesses.push(
                            PlutusWitness::new_with_ref_without_datum(
                                &PlutusScriptSource(plutus_mints.script.clone()),
                                redeemer)
                        );
                    }
                },
                _ => {},
            }
        }
        PlutusWitnesses(plutus_witnesses)
    }

    pub fn get_ref_inputs(&self) -> TransactionInputs {
        let mut reference_inputs = Vec::new();
        for script_mint in self.mints.values() {
            match script_mint {
                ScriptMint::Plutus(plutus_mints) => {
                    if let PlutusScriptSourceEnum::RefInput(ref_input, _, _) = &plutus_mints.script {
                        reference_inputs.push(ref_input.clone());
                    }
                },
                _ => {},
            }
        }
        TransactionInputs(reference_inputs)
    }

    pub fn get_redeeemers(&self) -> Result<Redeemers, JsError> {
        let tag = RedeemerTag::new_mint();
        let mut redeeemers = Vec::new();
        let mut index = BigNum::zero();
        for (_, script_mint) in &self.mints {
            match script_mint {
                ScriptMint::Plutus(plutus_mints) => {
                    for (redeemer, _) in &plutus_mints.redeemer_mints {
                        redeeemers.push(redeemer.clone_with_index_and_tag(&index, &tag));
                        index = index.checked_add(&BigNum::one())?;
                    }
                },
                _ => {
                    index = index.checked_add(&BigNum::one())?;
                },
            }
        }
        Ok(Redeemers(redeeemers))
    }

    pub fn has_plutus_scripts(&self) -> bool {
        for script_mint in self.mints.values() {
            match script_mint {
                ScriptMint::Plutus(_) => {
                    return true;
                },
                _ => {},
            }
        }
        false
    }

    pub fn has_native_scripts(&self) -> bool {
        for script_mint in self.mints.values() {
            match script_mint {
                ScriptMint::Native(_) => {
                    return true;
                },
                _ => {},
            }
        }
        false
    }

    pub(crate) fn get_used_plutus_lang_versions(&self) -> BTreeSet<Language> {
        let mut used_langs = BTreeSet::new();
        for (_, script_mint) in &self.mints {
            match script_mint {
                ScriptMint::Plutus(plutus_mints) => {
                    if let Some(lang) = plutus_mints.script.language() {
                        used_langs.insert(lang);
                    }
                },
                _ => {},
            }
        }
        used_langs
    }
}