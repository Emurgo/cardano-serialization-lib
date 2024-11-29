use crate::{CredType, Credential, Ed25519KeyHash, GovernanceActionId, JsError, ScriptHash, TransactionHash};
use bech32::{ToBase32, FromBase32};
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
pub(crate) enum GovIdType {
    CCHot = 0x0,
    CCCold = 0x1,
    DRep = 0x2,
}

impl TryFrom<u8> for GovIdType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(GovIdType::CCHot),
            0x1 => Ok(GovIdType::CCCold),
            0x2 => Ok(GovIdType::DRep),
            _ => Err("Invalid KeyType"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum CredentialType {
    KeyHash = 0x2,
    ScriptHash = 0x3,
}

impl TryFrom<u8> for CredentialType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x2 => Ok(CredentialType::KeyHash),
            0x3 => Ok(CredentialType::ScriptHash),
            _ => Err("Invalid CredentialType"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum GovernanceIdentifier {
    GovCredential {
        gov_id_type: GovIdType,
        credential: Credential
    },
    GovAction(GovernanceActionId),
}

impl GovernanceIdentifier {
    pub(crate) fn encode(&self) -> Vec<u8> {
        match self {
            GovernanceIdentifier::GovCredential {
                gov_id_type,
                credential
            } => {
                let (cred_type, cred_bytes) = match &credential.0 {
                    CredType::Key(key_hash) => (CredentialType::KeyHash, key_hash.to_bytes()),
                    CredType::Script(script_hash) => (CredentialType::ScriptHash, script_hash.to_bytes()),
                };
                let header = ((*gov_id_type as u8) << 4) | (cred_type as u8);
                let mut bytes = vec![header];
                bytes.extend_from_slice(&cred_bytes);
                bytes
            }
            GovernanceIdentifier::GovAction(gov_action_id) => {
                let mut bytes = gov_action_id.transaction_id.to_bytes();
                let index = gov_action_id.index().to_be_bytes();
                bytes.extend_from_slice(&index);
                bytes
            }
        }
    }

    pub(crate) fn decode(prefix: &str, bytes: &[u8]) -> Result<Self, JsError> {
        match prefix {
            "drep" | "cc_hot" | "cc_cold" => {
                if bytes.len() < 1 {
                    return Err(JsError::from_str("Invalid data length"));
                }
                let header = bytes[0];
                let gov_id_type = GovIdType::try_from(header >> 4).
                    map_err(|_| JsError::from_str("Invalid GovIdType"))?;
                let credential_type = CredentialType::try_from(header & 0x0F)
                    .map_err(|_| JsError::from_str("Invalid CredentialType"))?;
                let credential_bytes = bytes[1..].to_vec();
                let credential = match credential_type {
                    CredentialType::KeyHash => {
                        Credential::from_keyhash(&Ed25519KeyHash::from_bytes(credential_bytes)
                            .map_err(|_| JsError::from_str("Invalid key hash"))?)
                    }
                    CredentialType::ScriptHash => {
                        Credential::from_scripthash(&ScriptHash::from_bytes(credential_bytes)
                            .map_err(|_| JsError::from_str("Invalid script hash"))?)
                    }
                };
                Ok(GovernanceIdentifier::GovCredential {
                    gov_id_type,
                    credential,
                })
            }
            "gov_action" => {
                if bytes.len() < 33 {
                    return Err(JsError::from_str("Invalid data length"));
                }
                let tx_id = bytes[0..32].to_vec();
                let index_bytes = &bytes[32..];
                let index = match index_bytes.len() {
                    1 => u16::from(index_bytes[0]),
                    2 => u16::from_be_bytes([index_bytes[0], index_bytes[1]]),
                    _ => return Err(JsError::from_str("Invalid index length")),
                };
                let tx_hash = TransactionHash::from_bytes(tx_id)
                    .map_err(|_| JsError::from_str("Invalid transaction hash"))?;
                let governance_action_id = GovernanceActionId::new(&tx_hash, index.into());
                Ok(GovernanceIdentifier::GovAction(governance_action_id))
            }
            _ => Err(JsError::from_str("Unknown prefix")),
        }
    }

    pub(crate) fn to_bech32(&self) -> Result<String, JsError> {
        let (prefix, data) = match self {
            GovernanceIdentifier::GovCredential { gov_id_type, .. } => {
                let prefix = match gov_id_type {
                    GovIdType::CCHot => "cc_hot",
                    GovIdType::CCCold => "cc_cold",
                    GovIdType::DRep => "drep",
                };
                (prefix, self.encode())
            }
            GovernanceIdentifier::GovAction { .. } => ("gov_action", self.encode()),
        };
        let bech32_data = data.to_base32();
        bech32::encode(prefix, bech32_data).map_err(|e| JsError::from_str(&e.to_string()))
    }

    pub(crate) fn from_bech32(s: &str) -> Result<Self, JsError> {
        let (prefix, data) = bech32::decode(s).map_err(|e| JsError::from_str(&e.to_string()))?;
        let bytes = Vec::<u8>::from_base32(&data).map_err(|e| JsError::from_str(&e.to_string()))?;
        GovernanceIdentifier::decode(&prefix, &bytes)
    }
}