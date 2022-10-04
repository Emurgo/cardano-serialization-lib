use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub(crate) struct TxBuilderInput {
    pub(crate) input: TransactionInput,
    pub(crate) amount: Value, // we need to keep track of the amount in the inputs for input selection
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlutusScriptSourceEnum {
    Script(PlutusScript),
    RefInput(TransactionInput, ScriptHash),
}

impl PlutusScriptSourceEnum {
    pub fn script_hash(&self) -> ScriptHash {
        match self {
            PlutusScriptSourceEnum::Script(script) => script.hash(),
            PlutusScriptSourceEnum::RefInput(_, script_hash) => script_hash.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusScriptSource(PlutusScriptSourceEnum);

#[wasm_bindgen]
impl PlutusScriptSource {
    pub fn new(script: &PlutusScript) -> Self {
        Self(PlutusScriptSourceEnum::Script(script.clone()))
    }

    pub fn new_ref_input(script_hash: &ScriptHash, input: &TransactionInput) -> Self {
        Self(PlutusScriptSourceEnum::RefInput(input.clone(), script_hash.clone()))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DatumSourceEnum {
    Datum(PlutusData),
    RefInput(TransactionInput),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DatumSource(DatumSourceEnum);

#[wasm_bindgen]
impl DatumSource {
    pub fn new(datum: &PlutusData) -> Self {
        Self(DatumSourceEnum::Datum(datum.clone()))
    }

    pub fn new_ref_input(input: &TransactionInput) -> Self {
        Self(DatumSourceEnum::RefInput(input.clone()))
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusWitness {
    script: PlutusScriptSourceEnum,
    datum: DatumSourceEnum,
    redeemer: Redeemer,
}

#[wasm_bindgen]
impl PlutusWitness {
    pub fn new(script: &PlutusScript, datum: &PlutusData, redeemer: &Redeemer) -> Self {
        Self {
            script: PlutusScriptSourceEnum::Script(script.clone()),
            datum: DatumSourceEnum::Datum(datum.clone()),
            redeemer: redeemer.clone(),
        }
    }

    pub fn new_with_ref(script: &PlutusScriptSource, datum: &DatumSource, redeemer: &Redeemer) -> Self {
        Self {
            script: script.0.clone(),
            datum: datum.0.clone(),
            redeemer: redeemer.clone(),
        }
    }

    pub fn script(&self) -> Option<PlutusScript> {
        match &self.script {
            PlutusScriptSourceEnum::Script(script) => Some(script.clone()),
            _ => None,
        }
    }

    pub fn datum(&self) -> Option<PlutusData> {
        match &self.datum {
            DatumSourceEnum::Datum(datum) => Some(datum.clone()),
            _ => None,
        }
    }

    pub fn redeemer(&self) -> Redeemer {
        self.redeemer.clone()
    }

    fn clone_with_redeemer_index(&self, index: &BigNum) -> Self {
        Self {
            script: self.script.clone(),
            datum: self.datum.clone(),
            redeemer: self.redeemer.clone_with_index(index),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusWitnesses(pub(crate) Vec<PlutusWitness>);

#[wasm_bindgen]
impl PlutusWitnesses {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusWitness {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &PlutusWitness) {
        self.0.push(elem.clone());
    }

    pub(crate) fn collect(&self) -> (PlutusScripts, PlutusList, Redeemers) {
        let mut s = PlutusScripts::new();
        let mut d = PlutusList::new();
        let mut r = Redeemers::new();
        self.0.iter().for_each(|w| {
            if let PlutusScriptSourceEnum::Script(script) = &w.script {
                s.add(script);
            }
            if let DatumSourceEnum::Datum(datum) = &w.datum {
                d.add(datum);
            }
            r.add(&w.redeemer);
        });
        (s, d, r)
    }
}

impl From<Vec<PlutusWitness>> for PlutusWitnesses {
    fn from(scripts: Vec<PlutusWitness>) -> Self {
        Self(scripts)
    }
}

#[derive(Clone, Debug)]
pub enum ScriptWitnessType {
    NativeScriptWitness(NativeScript),
    PlutusScriptWitness(PlutusWitness),
}

// We need to know how many of each type of witness will be in the transaction so we can calculate the tx fee
#[derive(Clone, Debug)]
pub struct MockWitnessSet {
    vkeys: RequiredSignersSet,
    scripts: LinkedHashMap<ScriptHash, Option<ScriptWitnessType>>,
    bootstraps: BTreeSet<Vec<u8>>,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TxInputsBuilder {
    inputs: BTreeMap<TransactionInput, (TxBuilderInput, Option<ScriptHash>)>,
    input_types: MockWitnessSet,
}

pub(crate) fn get_bootstraps(inputs: &TxInputsBuilder) -> BTreeSet<Vec<u8>> {
    inputs.input_types.bootstraps.clone()
}

#[wasm_bindgen]
impl TxInputsBuilder {
    pub fn new() -> Self {
        Self {
            inputs: BTreeMap::new(),
            input_types: MockWitnessSet {
                vkeys: BTreeSet::new(),
                scripts: LinkedHashMap::new(),
                bootstraps: BTreeSet::new(),
            },
        }
    }

    fn push_input(&mut self, e: (TxBuilderInput, Option<ScriptHash>)) {
        self.inputs.insert(e.0.input.clone(), e);
    }

    /// We have to know what kind of inputs these are to know what kind of mock witnesses to create since
    /// 1) mock witnesses have different lengths depending on the type which changes the expecting fee
    /// 2) Witnesses are a set so we need to get rid of duplicates to avoid over-estimating the fee
    pub fn add_key_input(
        &mut self,
        hash: &Ed25519KeyHash,
        input: &TransactionInput,
        amount: &Value,
    ) {
        let inp = TxBuilderInput {
            input: input.clone(),
            amount: amount.clone(),
        };
        self.push_input((inp, None));
        self.input_types.vkeys.insert(hash.clone());
    }

    /// This method adds the input to the builder BUT leaves a missing spot for the witness native script
    ///
    /// After adding the input with this method, use `.add_required_native_input_scripts`
    /// and `.add_required_plutus_input_scripts` to add the witness scripts
    ///
    /// Or instead use `.add_native_script_input` and `.add_plutus_script_input`
    /// to add inputs right along with the script, instead of the script hash
    pub fn add_script_input(
        &mut self,
        hash: &ScriptHash,
        input: &TransactionInput,
        amount: &Value,
    ) {
        let inp = TxBuilderInput {
            input: input.clone(),
            amount: amount.clone(),
        };
        self.push_input((inp, Some(hash.clone())));
        if !self.input_types.scripts.contains_key(hash) {
            self.input_types.scripts.insert(hash.clone(), None);
        }
    }

    /// This method will add the input to the builder and also register the required native script witness
    pub fn add_native_script_input(
        &mut self,
        script: &NativeScript,
        input: &TransactionInput,
        amount: &Value,
    ) {
        let hash = script.hash();
        self.add_script_input(&hash, input, amount);
        self.input_types.scripts.insert(
            hash,
            Some(ScriptWitnessType::NativeScriptWitness(script.clone())),
        );
    }

    /// This method will add the input to the builder and also register the required plutus witness
    pub fn add_plutus_script_input(
        &mut self,
        witness: &PlutusWitness,
        input: &TransactionInput,
        amount: &Value,
    ) {
        let hash = witness.script.script_hash();

        self.add_script_input(&hash, input, amount);
        self.input_types.scripts.insert(
            hash,
            Some(ScriptWitnessType::PlutusScriptWitness(witness.clone())),
        );
    }

    pub fn add_bootstrap_input(
        &mut self,
        hash: &ByronAddress,
        input: &TransactionInput,
        amount: &Value,
    ) {
        let inp = TxBuilderInput {
            input: input.clone(),
            amount: amount.clone(),
        };
        self.push_input((inp, None));
        self.input_types.bootstraps.insert(hash.to_bytes());
    }

    /// Note that for script inputs this method will use underlying generic `.add_script_input`
    /// which leaves a required empty spot for the script witness (or witnesses in case of Plutus).
    /// You can use `.add_native_script_input` or `.add_plutus_script_input` directly to register the input along with the witness.
    pub fn add_input(&mut self, address: &Address, input: &TransactionInput, amount: &Value) {
        match &BaseAddress::from_address(address) {
            Some(addr) => {
                match &addr.payment_cred().to_keyhash() {
                    Some(hash) => return self.add_key_input(hash, input, amount),
                    None => (),
                }
                match &addr.payment_cred().to_scripthash() {
                    Some(hash) => return self.add_script_input(hash, input, amount),
                    None => (),
                }
            }
            None => (),
        }
        match &EnterpriseAddress::from_address(address) {
            Some(addr) => {
                match &addr.payment_cred().to_keyhash() {
                    Some(hash) => return self.add_key_input(hash, input, amount),
                    None => (),
                }
                match &addr.payment_cred().to_scripthash() {
                    Some(hash) => return self.add_script_input(hash, input, amount),
                    None => (),
                }
            }
            None => (),
        }
        match &PointerAddress::from_address(address) {
            Some(addr) => {
                match &addr.payment_cred().to_keyhash() {
                    Some(hash) => return self.add_key_input(hash, input, amount),
                    None => (),
                }
                match &addr.payment_cred().to_scripthash() {
                    Some(hash) => return self.add_script_input(hash, input, amount),
                    None => (),
                }
            }
            None => (),
        }
        match &ByronAddress::from_address(address) {
            Some(addr) => {
                return self.add_bootstrap_input(addr, input, amount);
            }
            None => (),
        }
    }

    /// Returns the number of still missing input scripts (either native or plutus)
    /// Use `.add_required_native_input_scripts` or `.add_required_plutus_input_scripts` to add the missing scripts
    pub fn count_missing_input_scripts(&self) -> usize {
        self.input_types
            .scripts
            .values()
            .filter(|s| s.is_none())
            .count()
    }

    /// Try adding the specified scripts as witnesses for ALREADY ADDED script inputs
    /// Any scripts that don't match any of the previously added inputs will be ignored
    /// Returns the number of remaining required missing witness scripts
    /// Use `.count_missing_input_scripts` to find the number of still missing scripts
    pub fn add_required_native_input_scripts(&mut self, scripts: &NativeScripts) -> usize {
        scripts.0.iter().for_each(|s: &NativeScript| {
            let hash = s.hash();
            if self.input_types.scripts.contains_key(&hash) {
                self.input_types.scripts.insert(
                    hash,
                    Some(ScriptWitnessType::NativeScriptWitness(s.clone())),
                );
            }
        });
        self.count_missing_input_scripts()
    }

    /// Try adding the specified scripts as witnesses for ALREADY ADDED script inputs
    /// Any scripts that don't match any of the previously added inputs will be ignored
    /// Returns the number of remaining required missing witness scripts
    /// Use `.count_missing_input_scripts` to find the number of still missing scripts
    pub fn add_required_plutus_input_scripts(&mut self, scripts: &PlutusWitnesses) -> usize {
        scripts.0.iter().for_each(|s: &PlutusWitness| {
            let hash = s.script.script_hash();
            if self.input_types.scripts.contains_key(&hash) {
                self.input_types.scripts.insert(
                    hash,
                    Some(ScriptWitnessType::PlutusScriptWitness(s.clone())),
                );
            }
        });
        self.count_missing_input_scripts()
    }

    pub fn get_ref_inputs(&self) -> TransactionInputs {
        let mut inputs = Vec::new();
        for wintess in self.input_types.scripts.iter()
            .filter_map(|(_, wit)| wit.as_ref() ) {
            if let ScriptWitnessType::PlutusScriptWitness(plutus_witness) = wintess {
                if let DatumSourceEnum::RefInput(input) = &plutus_witness.datum {
                    inputs.push(input.clone());
                }
                if let PlutusScriptSourceEnum::RefInput(input, _) = &plutus_witness.script {
                    inputs.push(input.clone());
                }
            }
        }

        TransactionInputs(inputs)
    }

    /// Returns a copy of the current script input witness scripts in the builder
    pub fn get_native_input_scripts(&self) -> Option<NativeScripts> {
        let mut scripts = NativeScripts::new();
        self.input_types.scripts.values().for_each(|option| {
            if let Some(ScriptWitnessType::NativeScriptWitness(s)) = option {
                scripts.add(&s);
            }
        });
        if scripts.len() > 0 {
            Some(scripts)
        } else {
            None
        }
    }

    /// Returns a copy of the current plutus input witness scripts in the builder.
    /// NOTE: each plutus witness will be cloned with a specific corresponding input index
    pub fn get_plutus_input_scripts(&self) -> Option<PlutusWitnesses> {
        /*
         * === EXPLANATION ===
         * The `Redeemer` object contains the `.index` field which is supposed to point
         * exactly to the index of the corresponding input in the inputs array. We want to
         * simplify and automate this as much as possible for the user to not have to care about it.
         *
         * For this we store the script hash along with the input, when it was registered, and
         * now we create a map of script hashes to their input indexes.
         *
         * The registered witnesses are then each cloned with the new correct redeemer input index.
         */
        let script_hash_index_map: BTreeMap<&ScriptHash, BigNum> = self
            .inputs
            .values()
            .enumerate()
            .fold(BTreeMap::new(), |mut m, (i, (_, hash_option))| {
                if let Some(hash) = hash_option {
                    m.insert(hash, to_bignum(i as u64));
                }
                m
            });
        let mut scripts = PlutusWitnesses::new();
        self.input_types.scripts.iter().for_each(|(hash, option)| {
            if let Some(ScriptWitnessType::PlutusScriptWitness(s)) = option {
                if let Some(idx) = script_hash_index_map.get(&hash) {
                    scripts.add(&s.clone_with_redeemer_index(&idx));
                }
            }
        });
        if scripts.len() > 0 {
            Some(scripts)
        } else {
            None
        }
    }

    pub(crate) fn iter(&self) -> impl std::iter::Iterator<Item = &TxBuilderInput> + '_ {
        self.inputs.values().map(|(i, _)| i)
    }

    pub fn len(&self) -> usize {
        self.inputs.len()
    }

    pub fn add_required_signer(&mut self, key: &Ed25519KeyHash) {
        self.input_types.vkeys.insert(key.clone());
    }

    pub fn add_required_signers(&mut self, keys: &RequiredSigners) {
        keys.0.iter().for_each(|k| self.add_required_signer(k));
    }

    pub fn total_value(&self) -> Result<Value, JsError> {
        let mut res = Value::zero();
        for (inp, _) in self.inputs.values() {
            res = res.checked_add(&inp.amount)?;
        }
        Ok(res)
    }

    pub fn inputs(&self) -> TransactionInputs {
        TransactionInputs(
            self.inputs
                .values()
                .map(|(ref tx_builder_input, _)| tx_builder_input.input.clone())
                .collect(),
        )
    }

    pub fn inputs_option(&self) -> Option<TransactionInputs> {
        if self.len() > 0 {
            Some(self.inputs())
        } else {
            None
        }
    }
}

impl From<&TxInputsBuilder> for RequiredSignersSet {
    fn from(inputs: &TxInputsBuilder) -> Self {
        let mut set = inputs.input_types.vkeys.clone();
        inputs
            .input_types
            .scripts
            .values()
            .for_each(|swt: &Option<ScriptWitnessType>| {
                if let Some(ScriptWitnessType::NativeScriptWitness(s)) = swt {
                    RequiredSignersSet::from(s).iter().for_each(|k| {
                        set.insert(k.clone());
                    });
                }
            });
        set
    }
}
