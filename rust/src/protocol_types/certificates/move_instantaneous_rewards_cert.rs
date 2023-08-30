use crate::*;
use std::vec::Vec;

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
pub struct MoveInstantaneousRewardsCert {
    pub(crate) move_instantaneous_reward: MoveInstantaneousReward,
}

impl_to_from!(MoveInstantaneousRewardsCert);

#[wasm_bindgen]
impl MoveInstantaneousRewardsCert {
    pub fn move_instantaneous_reward(&self) -> MoveInstantaneousReward {
        self.move_instantaneous_reward.clone()
    }

    pub fn new(move_instantaneous_reward: &MoveInstantaneousReward) -> Self {
        Self {
            move_instantaneous_reward: move_instantaneous_reward.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Copy,
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
pub enum MIRPot {
    Reserves,
    Treasury,
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
pub enum MIREnum {
    ToOtherPot(Coin),
    ToStakeCredentials(MIRToStakeCredentials),
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub enum MIRKind {
    ToOtherPot,
    ToStakeCredentials,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct MIRToStakeCredentials {
    pub(crate) rewards: linked_hash_map::LinkedHashMap<StakeCredential, DeltaCoin>,
}

impl_to_from!(MIRToStakeCredentials);

#[wasm_bindgen]
impl MIRToStakeCredentials {
    pub fn new() -> Self {
        Self {
            rewards: linked_hash_map::LinkedHashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.rewards.len()
    }

    pub fn insert(&mut self, cred: &StakeCredential, delta: &DeltaCoin) -> Option<DeltaCoin> {
        self.rewards.insert(cred.clone(), delta.clone())
    }

    pub fn get(&self, cred: &StakeCredential) -> Option<DeltaCoin> {
        self.rewards.get(cred).map(|v| v.clone())
    }

    pub fn keys(&self) -> StakeCredentials {
        StakeCredentials(
            self.rewards
                .iter()
                .map(|(k, _v)| k.clone())
                .collect::<Vec<StakeCredential>>(),
        )
    }
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
struct StakeToCoinJson {
    stake_cred: StakeCredential,
    amount: DeltaCoin,
}

impl serde::Serialize for MIRToStakeCredentials {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let vec = self
            .rewards
            .iter()
            .map(|(k, v)| StakeToCoinJson {
                stake_cred: k.clone(),
                amount: v.clone(),
            })
            .collect::<Vec<StakeToCoinJson>>();
        vec.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for MIRToStakeCredentials {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let map = Vec::<StakeToCoinJson>::deserialize(deserializer)?
            .into_iter()
            .map(|v| (v.stake_cred, v.amount));

        Ok(Self {
            rewards: map.collect(),
        })
    }
}

impl JsonSchema for MIRToStakeCredentials {
    fn schema_name() -> String {
        String::from("MIRToStakeCredentials")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
       Vec::<StakeToCoinJson>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        Vec::<StakeToCoinJson>::is_referenceable()
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
pub struct MoveInstantaneousReward {
    pub(crate) pot: MIRPot,
    pub(crate) variant: MIREnum,
}

impl_to_from!(MoveInstantaneousReward);

#[wasm_bindgen]
impl MoveInstantaneousReward {
    pub fn new_to_other_pot(pot: MIRPot, amount: &Coin) -> Self {
        Self {
            pot,
            variant: MIREnum::ToOtherPot(amount.clone()),
        }
    }

    pub fn new_to_stake_creds(pot: MIRPot, amounts: &MIRToStakeCredentials) -> Self {
        Self {
            pot,
            variant: MIREnum::ToStakeCredentials(amounts.clone()),
        }
    }

    pub fn pot(&self) -> MIRPot {
        self.pot
    }

    pub fn kind(&self) -> MIRKind {
        match &self.variant {
            MIREnum::ToOtherPot(_) => MIRKind::ToOtherPot,
            MIREnum::ToStakeCredentials(_) => MIRKind::ToStakeCredentials,
        }
    }

    pub fn as_to_other_pot(&self) -> Option<Coin> {
        match &self.variant {
            MIREnum::ToOtherPot(amount) => Some(amount.clone()),
            MIREnum::ToStakeCredentials(_) => None,
        }
    }

    pub fn as_to_stake_creds(&self) -> Option<MIRToStakeCredentials> {
        match &self.variant {
            MIREnum::ToOtherPot(_) => None,
            MIREnum::ToStakeCredentials(amounts) => Some(amounts.clone()),
        }
    }
}
