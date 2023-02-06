use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub(crate) struct TxBuilderInput {
    pub(crate) input: TransactionInput,
    pub(crate) amount: Value, // we need to keep track of the amount in the inputs for input selection
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct InputWithScriptWitness {
    pub(crate) input: TransactionInput,
    pub(crate) witness: ScriptWitnessType
}

#[wasm_bindgen]
impl InputWithScriptWitness {
    pub fn new_with_native_script_witness(input: &TransactionInput, witness: &NativeScript) -> Self {
        Self {
            input: input.clone(),
            witness: ScriptWitnessType::NativeScriptWitness(witness.clone())
        }
    }

    pub fn new_with_plutus_witness(input: &TransactionInput, witness: &PlutusWitness) -> Self {
        Self {
            input: input.clone(),
            witness: ScriptWitnessType::PlutusScriptWitness(witness.clone())
        }
    }

    pub fn input(&self) -> TransactionInput {
        self.input.clone()
    }
}

#[wasm_bindgen]
pub struct InputsWithScriptWitness(Vec<InputWithScriptWitness>);

#[wasm_bindgen]
impl InputsWithScriptWitness {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, input: &InputWithScriptWitness) {
        self.0.push(input.clone());
    }

    pub fn get(&self, index: usize) -> InputWithScriptWitness {
        self.0[index].clone()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlutusScriptSourceEnum {
    Script(PlutusScript),
    RefInput(TransactionInput, ScriptHash, Option<Language>),
}

impl PlutusScriptSourceEnum {
    pub fn script_hash(&self) -> ScriptHash {
        match self {
            PlutusScriptSourceEnum::Script(script) => script.hash(),
            PlutusScriptSourceEnum::RefInput(_, script_hash, _) => script_hash.clone(),
        }
    }

    pub fn language(&self) -> Option<Language> {
        match self {
            PlutusScriptSourceEnum::Script(script) => Some(script.language_version()),
            PlutusScriptSourceEnum::RefInput(_, _, language) => language.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusScriptSource(pub(crate) PlutusScriptSourceEnum);

#[wasm_bindgen]
impl PlutusScriptSource {
    pub fn new(script: &PlutusScript) -> Self {
        Self(PlutusScriptSourceEnum::Script(script.clone()))
    }

    /// !!! DEPRECATED !!!
    /// This constructor has missed information about plutus script language vesrion. That can affect
    /// the script data hash calculation.
    /// Use `.new_ref_input_with_lang_ver` instead
    #[deprecated(
    since = "11.3.0",
    note = "This constructor has missed information about plutus script language vesrion. That can affect the script data hash calculation. Use `.new_ref_input_with_lang_ver` instead."
    )]
    pub fn new_ref_input(script_hash: &ScriptHash, input: &TransactionInput) -> Self {
        Self(PlutusScriptSourceEnum::RefInput(input.clone(),
                                              script_hash.clone(),
                                              None))
    }

    pub fn new_ref_input_with_lang_ver(script_hash: &ScriptHash, input: &TransactionInput, lang_ver: &Language) -> Self {
        Self(PlutusScriptSourceEnum::RefInput(input.clone(),
                                              script_hash.clone(),
                                              Some(lang_ver.clone())))
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
    datum: Option<DatumSourceEnum>,
    redeemer: Redeemer,
}

#[wasm_bindgen]
impl PlutusWitness {
    pub fn new(script: &PlutusScript, datum: &PlutusData, redeemer: &Redeemer) -> Self {
        Self {
            script: PlutusScriptSourceEnum::Script(script.clone()),
            datum: Some(DatumSourceEnum::Datum(datum.clone())),
            redeemer: redeemer.clone(),
        }
    }

    pub fn new_with_ref(script: &PlutusScriptSource, datum: &DatumSource, redeemer: &Redeemer) -> Self {
        Self {
            script: script.0.clone(),
            datum: Some(datum.0.clone()),
            redeemer: redeemer.clone(),
        }
    }

    pub fn new_without_datum(script: &PlutusScript, redeemer: &Redeemer) -> Self {
        Self {
            script: PlutusScriptSourceEnum::Script(script.clone()),
            datum: None,
            redeemer: redeemer.clone(),
        }
    }

    pub fn new_with_ref_without_datum(script: &PlutusScriptSource, redeemer: &Redeemer) -> Self {
        Self {
            script: script.0.clone(),
            datum: None,
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
            Some(DatumSourceEnum::Datum(datum)) => Some(datum.clone()),
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

    pub(crate) fn collect(&self) -> (PlutusScripts, Option<PlutusList>, Redeemers) {
        let mut used_scripts = BTreeSet::new();
        let mut used_datums = BTreeSet::new();
        let mut used_redeemers = BTreeSet::new();
        let mut s = PlutusScripts::new();
        let mut d : Option<PlutusList> = None;
        let mut r = Redeemers::new();
        self.0.iter().for_each(|w| {
            if let PlutusScriptSourceEnum::Script(script) = &w.script {
                if used_scripts.insert(script.clone()) {
                    s.add(script);
                }
            }
            if let Some(DatumSourceEnum::Datum(datum)) = &w.datum {
                if used_datums.insert(datum) {
                    match d {
                        Some(ref mut d) => d.add(datum),
                        None => d =  {
                            let mut initial_list = PlutusList::new();
                            initial_list.add(datum);
                            Some(initial_list)
                        }
                    }
                }
            }
            if used_redeemers.insert(w.redeemer.clone()) {
                r.add(&w.redeemer);
            }
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

impl ScriptWitnessType {
    pub fn script_hash(&self) -> ScriptHash {
        match self {
            ScriptWitnessType::NativeScriptWitness(script) => script.hash(),
            ScriptWitnessType::PlutusScriptWitness(script) => script.script.script_hash(),
        }
    }
}

// We need to know how many of each type of witness will be in the transaction so we can calculate the tx fee
#[derive(Clone, Debug)]
pub struct RequiredWitnessSet {
    vkeys: RequiredSignersSet,
    scripts: LinkedHashMap<ScriptHash, LinkedHashMap<TransactionInput, Option<ScriptWitnessType>>>,
    bootstraps: BTreeSet<Vec<u8>>,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TxInputsBuilder {
    inputs: BTreeMap<TransactionInput, (TxBuilderInput, Option<ScriptHash>)>,
    required_witnesses: RequiredWitnessSet,
}

pub(crate) fn get_bootstraps(inputs: &TxInputsBuilder) -> BTreeSet<Vec<u8>> {
    inputs.required_witnesses.bootstraps.clone()
}

#[wasm_bindgen]
impl TxInputsBuilder {
    pub fn new() -> Self {
        Self {
            inputs: BTreeMap::new(),
            required_witnesses: RequiredWitnessSet {
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
        self.required_witnesses.vkeys.insert(hash.clone());
    }

    #[deprecated(
    since = "11.2.0",
    note = "Use `.add_native_script_input` or `.add_plutus_script_input` instead."
    )]
    /// !!! DEPRECATED !!!
    /// This function can make a mistake in choosing right input index. Use `.add_native_script_input` or `.add_plutus_script_input` instead.
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
        self.insert_input_with_empty_witness(hash, input);
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
        let witness = ScriptWitnessType::NativeScriptWitness(script.clone());
        self.insert_input_with_witness(&hash, input, &witness);
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
        let witness = ScriptWitnessType::PlutusScriptWitness(witness.clone());
        self.insert_input_with_witness(&hash, input, &witness);
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
        self.required_witnesses.bootstraps.insert(hash.to_bytes());
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
        self.required_witnesses
            .scripts
            .values()
            .flat_map(|v| v.values())
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
            if let Some(script_wits) = self.required_witnesses.scripts.get_mut(&hash) {
                let mut tx_in = None;
                for script_wit in script_wits {
                    if script_wit.1.is_none() {
                        tx_in = Some(script_wit.0.clone());
                        break;
                    }
                }

                if let Some(tx_in) = tx_in {
                    let witness = ScriptWitnessType::NativeScriptWitness(s.clone());
                    self.insert_input_with_witness(&hash, &tx_in, &witness);
                }
            }
        });
        self.count_missing_input_scripts()
    }

    #[deprecated(
    since = "11.2.0",
    note = "This function can make a mistake in choosing right input index. Use `.add_required_script_input_witnesses` instead."
    )]
    /// !!! DEPRECATED !!!
    /// This function can make a mistake in choosing right input index. Use `.add_required_script_input_witnesses` instead.
    /// Try adding the specified scripts as witnesses for ALREADY ADDED script inputs
    /// Any scripts that don't match any of the previously added inputs will be ignored
    /// Returns the number of remaining required missing witness scripts
    /// Use `.count_missing_input_scripts` to find the number of still missing scripts
    pub fn add_required_plutus_input_scripts(&mut self, scripts: &PlutusWitnesses) -> usize {
        scripts.0.iter().for_each(|s: &PlutusWitness| {
            let hash = s.script.script_hash();
            if let Some(script_wits) = self.required_witnesses.scripts.get_mut(&hash) {
                let mut tx_in = None;
                for script_wit in script_wits {
                    if script_wit.1.is_none() {
                        tx_in = Some(script_wit.0.clone());
                        break;
                    }
                }

                if let Some(tx_in) = tx_in {
                    let witness = ScriptWitnessType::PlutusScriptWitness(s.clone());
                    self.insert_input_with_witness(&hash, &tx_in, &witness);
                }
            }
        });
        self.count_missing_input_scripts()
    }

    /// Try adding the specified scripts as witnesses for ALREADY ADDED script inputs
    /// Any scripts that don't match any of the previously added inputs will be ignored
    /// Returns the number of remaining required missing witness scripts
    /// Use `.count_missing_input_scripts` to find the number of still missing scripts
    pub fn add_required_script_input_witnesses(&mut self, inputs_with_wit: &InputsWithScriptWitness) -> usize {
        inputs_with_wit.0.iter().for_each(|input_with_wit: &InputWithScriptWitness| {
            let hash = input_with_wit.witness.script_hash();
            if let Some(script_wits) = self.required_witnesses.scripts.get_mut(&hash) {
                if script_wits.contains_key(&input_with_wit.input) {
                    script_wits.insert(input_with_wit.input.clone(), Some(input_with_wit.witness.clone()));
                }
            } });
        self.count_missing_input_scripts()
    }

    pub fn get_ref_inputs(&self) -> TransactionInputs {
        let mut inputs = Vec::new();
        for wintess in self.required_witnesses.scripts.iter()
            .flat_map(|(_, tx_wits)| tx_wits.values())
            .filter_map(|wit| wit.as_ref()) {
            if let ScriptWitnessType::PlutusScriptWitness(plutus_witness) = wintess {
                if let Some(DatumSourceEnum::RefInput(input)) = &plutus_witness.datum {
                    inputs.push(input.clone());
                }
                if let PlutusScriptSourceEnum::RefInput(input, _, _) = &plutus_witness.script {
                    inputs.push(input.clone());
                }
            }
        }

        TransactionInputs(inputs)
    }

    /// Returns a copy of the current script input witness scripts in the builder
    pub fn get_native_input_scripts(&self) -> Option<NativeScripts> {
        let mut scripts = NativeScripts::new();
        self.required_witnesses.scripts
            .values()
            .flat_map(|v| v)
            .for_each(|tx_in_with_wit| {
            if let Some(ScriptWitnessType::NativeScriptWitness(s)) = tx_in_with_wit.1 {
                scripts.add(&s);
            }
        });
        if scripts.len() > 0 {
            Some(scripts)
        } else {
            None
        }
    }

    pub(crate) fn get_used_plutus_lang_versions(&self) -> BTreeSet<Language> {
        let mut used_langs = BTreeSet::new();
        self.required_witnesses.scripts.values().for_each(|input_with_wit| {
            for (_, script_wit) in input_with_wit {
                if let Some(ScriptWitnessType::PlutusScriptWitness(PlutusWitness { script, .. })) = script_wit {
                    if let Some(lang) = script.language() {
                        used_langs.insert(lang);
                    }
                }
            }
        });
        used_langs
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
        let script_hash_index_map: BTreeMap<&TransactionInput, BigNum> = self
            .inputs
            .values()
            .enumerate()
            .fold(BTreeMap::new(), |mut m, (i, (tx_in, hash_option))| {
                if hash_option.is_some() {
                    m.insert(&tx_in.input, to_bignum(i as u64));
                }
                m
            });
        let mut scripts = PlutusWitnesses::new();
        self.required_witnesses.scripts
            .iter()
            .flat_map(|x| x.1)
            .for_each(|(hash, option)| {
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
        self.required_witnesses.vkeys.insert(key.clone());
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

    fn insert_input_with_witness(&mut self, script_hash: &ScriptHash, input: &TransactionInput, witness: &ScriptWitnessType) {
        let script_inputs =
            self.required_witnesses.scripts.entry(script_hash.clone()).or_insert(LinkedHashMap::new());
        script_inputs.insert(input.clone(), Some(witness.clone()));
    }

    fn insert_input_with_empty_witness(&mut self, script_hash: &ScriptHash, input: &TransactionInput) {
        let script_inputs =
            self.required_witnesses.scripts.entry(script_hash.clone()).or_insert(LinkedHashMap::new());
        script_inputs.insert(input.clone(), None);
    }
}

impl From<&TxInputsBuilder> for RequiredSignersSet {
    fn from(inputs: &TxInputsBuilder) -> Self {
        let mut set = inputs.required_witnesses.vkeys.clone();
        inputs
            .required_witnesses
            .scripts
            .values()
            .flat_map(|tx_wits| tx_wits.values())
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
