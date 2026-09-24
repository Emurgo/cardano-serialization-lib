use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NativeScriptKind {
    ScriptPubkey,
    ScriptAll,
    ScriptAny,
    ScriptNOfK,
    TimelockStart,
    TimelockExpiry,
}

#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub enum NativeScriptEnum {
    ScriptPubkey(ScriptPubkey),
    ScriptAll(ScriptAll),
    ScriptAny(ScriptAny),
    ScriptNOfK(ScriptNOfK),
    TimelockStart(TimelockStart),
    TimelockExpiry(TimelockExpiry),
}

#[wasm_bindgen]
#[derive(JsonSchema, serde::Deserialize)]
pub struct NativeScript(pub(crate) NativeScriptEnum);

to_from_bytes!(NativeScript);

#[wasm_bindgen]
impl NativeScript {
    pub fn hash(&self) -> ScriptHash {
        let mut bytes = Vec::with_capacity(self.to_bytes().len() + 1);
        bytes.extend_from_slice(&vec![ScriptHashNamespace::NativeScript as u8]);
        bytes.extend_from_slice(&self.to_bytes());
        ScriptHash::from(blake2b224(bytes.as_ref()))
    }

    pub fn new_script_pubkey(script_pubkey: &ScriptPubkey) -> Self {
        Self(NativeScriptEnum::ScriptPubkey(script_pubkey.clone()))
    }

    pub fn new_script_all(script_all: &ScriptAll) -> Self {
        Self(NativeScriptEnum::ScriptAll(script_all.clone()))
    }

    pub fn new_script_any(script_any: &ScriptAny) -> Self {
        Self(NativeScriptEnum::ScriptAny(script_any.clone()))
    }

    pub fn new_script_n_of_k(script_n_of_k: &ScriptNOfK) -> Self {
        Self(NativeScriptEnum::ScriptNOfK(script_n_of_k.clone()))
    }

    pub fn new_timelock_start(timelock_start: &TimelockStart) -> Self {
        Self(NativeScriptEnum::TimelockStart(timelock_start.clone()))
    }

    pub fn new_timelock_expiry(timelock_expiry: &TimelockExpiry) -> Self {
        Self(NativeScriptEnum::TimelockExpiry(timelock_expiry.clone()))
    }

    pub fn kind(&self) -> NativeScriptKind {
        match &self.0 {
            NativeScriptEnum::ScriptPubkey(_) => NativeScriptKind::ScriptPubkey,
            NativeScriptEnum::ScriptAll(_) => NativeScriptKind::ScriptAll,
            NativeScriptEnum::ScriptAny(_) => NativeScriptKind::ScriptAny,
            NativeScriptEnum::ScriptNOfK(_) => NativeScriptKind::ScriptNOfK,
            NativeScriptEnum::TimelockStart(_) => NativeScriptKind::TimelockStart,
            NativeScriptEnum::TimelockExpiry(_) => NativeScriptKind::TimelockExpiry,
        }
    }

    pub fn as_script_pubkey(&self) -> Option<ScriptPubkey> {
        match &self.0 {
            NativeScriptEnum::ScriptPubkey(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_script_all(&self) -> Option<ScriptAll> {
        match &self.0 {
            NativeScriptEnum::ScriptAll(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_script_any(&self) -> Option<ScriptAny> {
        match &self.0 {
            NativeScriptEnum::ScriptAny(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_script_n_of_k(&self) -> Option<ScriptNOfK> {
        match &self.0 {
            NativeScriptEnum::ScriptNOfK(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_timelock_start(&self) -> Option<TimelockStart> {
        match &self.0 {
            NativeScriptEnum::TimelockStart(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_timelock_expiry(&self) -> Option<TimelockExpiry> {
        match &self.0 {
            NativeScriptEnum::TimelockExpiry(x) => Some(x.clone()),
            _ => None,
        }
    }

    /// Returns a set of Ed25519KeyHashes
    /// contained within this script recursively on any depth level.
    /// The order of the keys in the result is not determined in any way.
    pub fn get_required_signers(&self) -> Ed25519KeyHashes {
        Ed25519KeyHashes::from(self)
    }

    pub fn to_json(&self) -> Result<String, JsError> {
        Ok(crate::stack_safe::stringify_json(
            &native_script_to_json_val(self),
            true,
        ))
    }

    pub fn from_json(json: &str) -> Result<NativeScript, JsError> {
        // Input stays on serde_json on purpose: its recursion limit (128 levels) turns
        // pathological depth into an error instead of a stack overflow, as in every release
        // through 16.0.0. Only the output side must be iterative, since chain data feeds it.
        serde_json::from_str(json).map_err(|e| JsError::from_str(&format!("from_json: {}", e)))
    }

    #[cfg(all(
        target_arch = "wasm32",
        not(target_os = "emscripten"),
        not(feature = "dont-expose-wasm")
    ))]
    pub fn to_js_value(&self) -> Result<JsValue, JsError> {
        let json = self.to_json()?;
        js_sys::JSON::parse(&json)
            .map_err(|e| JsError::from_str(&format!("to_js_value: {:?}", e)))
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct ScriptPubkey {
    pub(crate) addr_keyhash: Ed25519KeyHash,
}

impl_to_from!(ScriptPubkey);

#[wasm_bindgen]
impl ScriptPubkey {
    pub fn addr_keyhash(&self) -> Ed25519KeyHash {
        self.addr_keyhash.clone()
    }

    pub fn new(addr_keyhash: &Ed25519KeyHash) -> Self {
        Self {
            addr_keyhash: addr_keyhash.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct ScriptAll {
    pub(crate) native_scripts: NativeScripts,
}

impl_to_from!(ScriptAll);

#[wasm_bindgen]
impl ScriptAll {
    pub fn native_scripts(&self) -> NativeScripts {
        self.native_scripts.clone()
    }

    pub fn new(native_scripts: &NativeScripts) -> Self {
        Self {
            native_scripts: native_scripts.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct ScriptAny {
    pub(crate) native_scripts: NativeScripts,
}

impl_to_from!(ScriptAny);

#[wasm_bindgen]
impl ScriptAny {
    pub fn native_scripts(&self) -> NativeScripts {
        self.native_scripts.clone()
    }

    pub fn new(native_scripts: &NativeScripts) -> Self {
        Self {
            native_scripts: native_scripts.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct ScriptNOfK {
    pub(crate) n: u32,
    pub(crate) native_scripts: NativeScripts,
}

impl_to_from!(ScriptNOfK);

#[wasm_bindgen]
impl ScriptNOfK {
    pub fn n(&self) -> u32 {
        self.n
    }

    pub fn native_scripts(&self) -> NativeScripts {
        self.native_scripts.clone()
    }

    pub fn new(n: u32, native_scripts: &NativeScripts) -> Self {
        Self {
            n: n,
            native_scripts: native_scripts.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct TimelockStart {
    pub(crate) slot: SlotBigNum,
}

impl_to_from!(TimelockStart);

#[wasm_bindgen]
impl TimelockStart {
    /// !!! DEPRECATED !!!
    /// Returns a Slot32 (u32) value in case the underlying original BigNum (u64) value is within the limits.
    /// Otherwise will just raise an error.
    /// Use `.slot_bignum` instead
    #[deprecated(
        since = "10.1.0",
        note = "Possible boundary error. Use slot_bignum instead"
    )]
    pub fn slot(&self) -> Result<Slot32, JsError> {
        self.slot.try_into()
    }

    pub fn slot_bignum(&self) -> SlotBigNum {
        self.slot
    }

    /// !!! DEPRECATED !!!
    /// This constructor uses outdated slot number format.
    /// Use `.new_timelockstart` instead.
    #[deprecated(
        since = "10.1.0",
        note = "Underlying value capacity (BigNum u64) bigger then Slot32. Use new_bignum instead."
    )]
    pub fn new(slot: Slot32) -> Self {
        Self { slot: slot.into() }
    }

    pub fn new_timelockstart(slot: &SlotBigNum) -> Self {
        Self { slot: slot.clone() }
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct TimelockExpiry {
    pub(crate) slot: SlotBigNum,
}

impl_to_from!(TimelockExpiry);

#[wasm_bindgen]
impl TimelockExpiry {
    pub fn slot(&self) -> Result<Slot32, JsError> {
        self.slot.try_into()
    }

    pub fn slot_bignum(&self) -> SlotBigNum {
        self.slot
    }

    /// !!! DEPRECATED !!!
    /// This constructor uses outdated slot number format.
    /// Use `.new_timelockexpiry` instead
    #[deprecated(
        since = "10.1.0",
        note = "Underlying value capacity (BigNum u64) bigger then Slot32. Use new_bignum instead."
    )]
    pub fn new(slot: Slot32) -> Self {
        Self {
            slot: (slot.into()),
        }
    }

    pub fn new_timelockexpiry(slot: &SlotBigNum) -> Self {
        Self { slot: slot.clone() }
    }
}

impl NativeScript {
    fn take_children(&mut self, stack: &mut Vec<NativeScript>) {
        match &mut self.0 {
            NativeScriptEnum::ScriptAll(x) => {
                stack.extend(std::mem::take(&mut x.native_scripts.scripts));
            }
            NativeScriptEnum::ScriptAny(x) => {
                stack.extend(std::mem::take(&mut x.native_scripts.scripts));
            }
            NativeScriptEnum::ScriptNOfK(x) => {
                stack.extend(std::mem::take(&mut x.native_scripts.scripts));
            }
            _ => {}
        }
    }

    fn kind_index(&self) -> u8 {
        match &self.0 {
            NativeScriptEnum::ScriptPubkey(_) => 0,
            NativeScriptEnum::ScriptAll(_) => 1,
            NativeScriptEnum::ScriptAny(_) => 2,
            NativeScriptEnum::ScriptNOfK(_) => 3,
            NativeScriptEnum::TimelockStart(_) => 4,
            NativeScriptEnum::TimelockExpiry(_) => 5,
        }
    }

    pub(crate) fn from_script_all_moved(native_scripts: NativeScripts) -> Self {
        Self(NativeScriptEnum::ScriptAll(ScriptAll { native_scripts }))
    }

    pub(crate) fn from_script_any_moved(native_scripts: NativeScripts) -> Self {
        Self(NativeScriptEnum::ScriptAny(ScriptAny { native_scripts }))
    }

    pub(crate) fn from_script_n_of_k_moved(n: u32, native_scripts: NativeScripts) -> Self {
        Self(NativeScriptEnum::ScriptNOfK(ScriptNOfK { n, native_scripts }))
    }

    pub(crate) fn into_enum(self) -> NativeScriptEnum {
        let this = std::mem::ManuallyDrop::new(self);
        unsafe { std::ptr::read(&this.0) }
    }
}

impl Drop for NativeScript {
    fn drop(&mut self) {
        crate::stack_safe::iterative_drop(self, |node, stack| node.take_children(stack));
    }
}

enum NativeCloneKind {
    All { tag: Option<CborSetType> },
    Any { tag: Option<CborSetType> },
    NOfK { n: u32, tag: Option<CborSetType> },
}

struct NativeCloneFrame<'a> {
    kind: NativeCloneKind,
    rest: std::slice::Iter<'a, NativeScript>,
    acc: Vec<NativeScript>,
}

fn finish_native_clone(kind: NativeCloneKind, scripts: Vec<NativeScript>) -> NativeScript {
    let native_scripts = NativeScripts {
        scripts,
        cbor_tag_type: match &kind {
            NativeCloneKind::All { tag } | NativeCloneKind::Any { tag } | NativeCloneKind::NOfK { tag, .. } => {
                tag.clone()
            }
        },
    };
    match kind {
        NativeCloneKind::All { .. } => NativeScript::from_script_all_moved(native_scripts),
        NativeCloneKind::Any { .. } => NativeScript::from_script_any_moved(native_scripts),
        NativeCloneKind::NOfK { n, .. } => NativeScript::from_script_n_of_k_moved(n, native_scripts),
    }
}

impl Clone for NativeScript {
    fn clone(&self) -> Self {
        let mut stack: Vec<NativeCloneFrame<'_>> = Vec::new();
        let mut node = self;
        loop {
            let mut completed = match &node.0 {
                NativeScriptEnum::ScriptPubkey(x) => {
                    NativeScript(NativeScriptEnum::ScriptPubkey(x.clone()))
                }
                NativeScriptEnum::TimelockStart(x) => {
                    NativeScript(NativeScriptEnum::TimelockStart(x.clone()))
                }
                NativeScriptEnum::TimelockExpiry(x) => {
                    NativeScript(NativeScriptEnum::TimelockExpiry(x.clone()))
                }
                NativeScriptEnum::ScriptAll(x) => {
                    let kind = NativeCloneKind::All {
                        tag: x.native_scripts.cbor_tag_type.clone(),
                    };
                    let mut rest = x.native_scripts.scripts.iter();
                    if let Some(first) = rest.next() {
                        stack.push(NativeCloneFrame {
                            kind,
                            rest,
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    finish_native_clone(kind, Vec::new())
                }
                NativeScriptEnum::ScriptAny(x) => {
                    let kind = NativeCloneKind::Any {
                        tag: x.native_scripts.cbor_tag_type.clone(),
                    };
                    let mut rest = x.native_scripts.scripts.iter();
                    if let Some(first) = rest.next() {
                        stack.push(NativeCloneFrame {
                            kind,
                            rest,
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    finish_native_clone(kind, Vec::new())
                }
                NativeScriptEnum::ScriptNOfK(x) => {
                    let kind = NativeCloneKind::NOfK {
                        n: x.n,
                        tag: x.native_scripts.cbor_tag_type.clone(),
                    };
                    let mut rest = x.native_scripts.scripts.iter();
                    if let Some(first) = rest.next() {
                        stack.push(NativeCloneFrame {
                            kind,
                            rest,
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    finish_native_clone(kind, Vec::new())
                }
            };

            loop {
                match stack.last_mut() {
                    None => return completed,
                    Some(frame) => {
                        frame.acc.push(completed);
                        if let Some(next) = frame.rest.next() {
                            node = next;
                            break;
                        } else {
                            let frame = stack.pop().unwrap();
                            completed = finish_native_clone(frame.kind, frame.acc);
                        }
                    }
                }
            }
        }
    }
}

impl PartialEq for NativeScript {
    fn eq(&self, other: &Self) -> bool {
        let mut stack = vec![(self, other)];
        while let Some((a, b)) = stack.pop() {
            match (&a.0, &b.0) {
                (NativeScriptEnum::ScriptPubkey(x), NativeScriptEnum::ScriptPubkey(y)) => {
                    if x != y {
                        return false;
                    }
                }
                (NativeScriptEnum::TimelockStart(x), NativeScriptEnum::TimelockStart(y)) => {
                    if x != y {
                        return false;
                    }
                }
                (NativeScriptEnum::TimelockExpiry(x), NativeScriptEnum::TimelockExpiry(y)) => {
                    if x != y {
                        return false;
                    }
                }
                (NativeScriptEnum::ScriptAll(x), NativeScriptEnum::ScriptAll(y)) => {
                    if x.native_scripts.scripts.len() != y.native_scripts.scripts.len() {
                        return false;
                    }
                    stack.extend(
                        x.native_scripts
                            .scripts
                            .iter()
                            .zip(y.native_scripts.scripts.iter()),
                    );
                }
                (NativeScriptEnum::ScriptAny(x), NativeScriptEnum::ScriptAny(y)) => {
                    if x.native_scripts.scripts.len() != y.native_scripts.scripts.len() {
                        return false;
                    }
                    stack.extend(
                        x.native_scripts
                            .scripts
                            .iter()
                            .zip(y.native_scripts.scripts.iter()),
                    );
                }
                (NativeScriptEnum::ScriptNOfK(x), NativeScriptEnum::ScriptNOfK(y)) => {
                    if x.n != y.n
                        || x.native_scripts.scripts.len() != y.native_scripts.scripts.len()
                    {
                        return false;
                    }
                    stack.extend(
                        x.native_scripts
                            .scripts
                            .iter()
                            .zip(y.native_scripts.scripts.iter()),
                    );
                }
                _ => return false,
            }
        }
        true
    }
}

impl Eq for NativeScript {}

enum NativeCmpJob<'a> {
    Nodes(&'a NativeScript, &'a NativeScript),
    Lens(usize, usize),
    NOfK(u32, u32),
}

impl PartialOrd for NativeScript {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NativeScript {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut stack = vec![NativeCmpJob::Nodes(self, other)];
        while let Some(job) = stack.pop() {
            match job {
                NativeCmpJob::Lens(a, b) => match a.cmp(&b) {
                    Ordering::Equal => {}
                    other => return other,
                },
                NativeCmpJob::NOfK(a, b) => match a.cmp(&b) {
                    Ordering::Equal => {}
                    other => return other,
                },
                NativeCmpJob::Nodes(a, b) => {
                    match a.kind_index().cmp(&b.kind_index()) {
                        Ordering::Equal => {}
                        other => return other,
                    }
                    match (&a.0, &b.0) {
                        (NativeScriptEnum::ScriptPubkey(x), NativeScriptEnum::ScriptPubkey(y)) => {
                            match x.cmp(y) {
                                Ordering::Equal => {}
                                other => return other,
                            }
                        }
                        (NativeScriptEnum::TimelockStart(x), NativeScriptEnum::TimelockStart(y)) => {
                            match x.cmp(y) {
                                Ordering::Equal => {}
                                other => return other,
                            }
                        }
                        (
                            NativeScriptEnum::TimelockExpiry(x),
                            NativeScriptEnum::TimelockExpiry(y),
                        ) => match x.cmp(y) {
                            Ordering::Equal => {}
                            other => return other,
                        },
                        (NativeScriptEnum::ScriptAll(x), NativeScriptEnum::ScriptAll(y)) => {
                            push_script_vec_cmp(
                                &mut stack,
                                &x.native_scripts.scripts,
                                &y.native_scripts.scripts,
                            );
                        }
                        (NativeScriptEnum::ScriptAny(x), NativeScriptEnum::ScriptAny(y)) => {
                            push_script_vec_cmp(
                                &mut stack,
                                &x.native_scripts.scripts,
                                &y.native_scripts.scripts,
                            );
                        }
                        (NativeScriptEnum::ScriptNOfK(x), NativeScriptEnum::ScriptNOfK(y)) => {
                            stack.push(NativeCmpJob::Lens(
                                x.native_scripts.scripts.len(),
                                y.native_scripts.scripts.len(),
                            ));
                            let min_len = x
                                .native_scripts
                                .scripts
                                .len()
                                .min(y.native_scripts.scripts.len());
                            for i in (0..min_len).rev() {
                                stack.push(NativeCmpJob::Nodes(
                                    &x.native_scripts.scripts[i],
                                    &y.native_scripts.scripts[i],
                                ));
                            }
                            stack.push(NativeCmpJob::NOfK(x.n, y.n));
                        }
                        _ => {}
                    }
                }
            }
        }
        Ordering::Equal
    }
}

fn push_script_vec_cmp<'a>(
    stack: &mut Vec<NativeCmpJob<'a>>,
    xs: &'a [NativeScript],
    ys: &'a [NativeScript],
) {
    stack.push(NativeCmpJob::Lens(xs.len(), ys.len()));
    let min_len = xs.len().min(ys.len());
    for i in (0..min_len).rev() {
        stack.push(NativeCmpJob::Nodes(&xs[i], &ys[i]));
    }
}

impl fmt::Debug for NativeScript {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Non-recursive: a 10k spine must not overflow `{:?}`.
        match &self.0 {
            NativeScriptEnum::ScriptPubkey(x) => f
                .debug_tuple("NativeScript")
                .field(&format_args!("ScriptPubkey({:?})", x))
                .finish(),
            NativeScriptEnum::TimelockStart(x) => f
                .debug_tuple("NativeScript")
                .field(&format_args!("TimelockStart({:?})", x))
                .finish(),
            NativeScriptEnum::TimelockExpiry(x) => f
                .debug_tuple("NativeScript")
                .field(&format_args!("TimelockExpiry({:?})", x))
                .finish(),
            NativeScriptEnum::ScriptAll(x) => f
                .debug_tuple("NativeScript")
                .field(&format_args!(
                    "ScriptAll(len={})",
                    x.native_scripts.scripts.len()
                ))
                .finish(),
            NativeScriptEnum::ScriptAny(x) => f
                .debug_tuple("NativeScript")
                .field(&format_args!(
                    "ScriptAny(len={})",
                    x.native_scripts.scripts.len()
                ))
                .finish(),
            NativeScriptEnum::ScriptNOfK(x) => f
                .debug_tuple("NativeScript")
                .field(&format_args!(
                    "ScriptNOfK(n={}, len={})",
                    x.n,
                    x.native_scripts.scripts.len()
                ))
                .finish(),
        }
    }
}

enum NativeJsonKind {
    All,
    Any,
    NOfK { n: u32 },
}

struct NativeJsonFrame<'a> {
    kind: NativeJsonKind,
    rest: std::slice::Iter<'a, NativeScript>,
    acc: Vec<crate::stack_safe::JsonVal>,
}

fn native_script_to_json_val(root: &NativeScript) -> crate::stack_safe::JsonVal {
    use crate::stack_safe::JsonVal;
    let mut stack: Vec<NativeJsonFrame<'_>> = Vec::new();
    let mut node = root;
    loop {
        let mut completed = match &node.0 {
            NativeScriptEnum::ScriptPubkey(x) => tagged_object(
                "ScriptPubkey",
                vec![(
                    "addr_keyhash".to_string(),
                    JsonVal::String(x.addr_keyhash.to_hex()),
                )],
            ),
            NativeScriptEnum::TimelockStart(x) => tagged_object(
                "TimelockStart",
                vec![("slot".to_string(), JsonVal::String(x.slot.to_str()))],
            ),
            NativeScriptEnum::TimelockExpiry(x) => tagged_object(
                "TimelockExpiry",
                vec![("slot".to_string(), JsonVal::String(x.slot.to_str()))],
            ),
            NativeScriptEnum::ScriptAll(x) => {
                let mut rest = x.native_scripts.scripts.iter();
                if let Some(first) = rest.next() {
                    stack.push(NativeJsonFrame {
                        kind: NativeJsonKind::All,
                        rest,
                        acc: Vec::new(),
                    });
                    node = first;
                    continue;
                }
                tagged_scripts_object("ScriptAll", None, Vec::new())
            }
            NativeScriptEnum::ScriptAny(x) => {
                let mut rest = x.native_scripts.scripts.iter();
                if let Some(first) = rest.next() {
                    stack.push(NativeJsonFrame {
                        kind: NativeJsonKind::Any,
                        rest,
                        acc: Vec::new(),
                    });
                    node = first;
                    continue;
                }
                tagged_scripts_object("ScriptAny", None, Vec::new())
            }
            NativeScriptEnum::ScriptNOfK(x) => {
                let mut rest = x.native_scripts.scripts.iter();
                if let Some(first) = rest.next() {
                    stack.push(NativeJsonFrame {
                        kind: NativeJsonKind::NOfK { n: x.n },
                        rest,
                        acc: Vec::new(),
                    });
                    node = first;
                    continue;
                }
                tagged_scripts_object("ScriptNOfK", Some(x.n), Vec::new())
            }
        };

        loop {
            match stack.last_mut() {
                None => return completed,
                Some(frame) => {
                    frame.acc.push(completed);
                    if let Some(next) = frame.rest.next() {
                        node = next;
                        break;
                    } else {
                        let frame = stack.pop().unwrap();
                        completed = match frame.kind {
                            NativeJsonKind::All => {
                                tagged_scripts_object("ScriptAll", None, frame.acc)
                            }
                            NativeJsonKind::Any => {
                                tagged_scripts_object("ScriptAny", None, frame.acc)
                            }
                            NativeJsonKind::NOfK { n } => {
                                tagged_scripts_object("ScriptNOfK", Some(n), frame.acc)
                            }
                        };
                    }
                }
            }
        }
    }
}

fn tagged_object(tag: &str, fields: Vec<(String, crate::stack_safe::JsonVal)>) -> crate::stack_safe::JsonVal {
    crate::stack_safe::JsonVal::Object(vec![(
        tag.to_string(),
        crate::stack_safe::JsonVal::Object(fields),
    )])
}

fn tagged_scripts_object(
    tag: &str,
    n: Option<u32>,
    scripts: Vec<crate::stack_safe::JsonVal>,
) -> crate::stack_safe::JsonVal {
    let mut fields = Vec::new();
    if let Some(n) = n {
        fields.push(("n".to_string(), crate::stack_safe::JsonVal::Number(n.to_string())));
    }
    fields.push((
        "native_scripts".to_string(),
        crate::stack_safe::JsonVal::Array(scripts),
    ));
    tagged_object(tag, fields)
}

impl serde::Serialize for NativeScript {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let json = crate::stack_safe::stringify_json(&native_script_to_json_val(self), false);
        let raw = serde_json::value::RawValue::from_string(json).map_err(serde::ser::Error::custom)?;
        raw.serialize(serializer)
    }
}
