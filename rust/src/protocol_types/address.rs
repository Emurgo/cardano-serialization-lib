use crate::*;
use crate::legacy_address::ExtendedAddr;
use bech32::ToBase32;
use ed25519_bip32::XPub;

// returns (Number represented, bytes read) if valid encoding
// or None if decoding prematurely finished
pub(crate) fn variable_nat_decode(bytes: &[u8]) -> Option<(u64, usize)> {
    let mut output = 0u128;
    let mut bytes_read = 0;
    for byte in bytes {
        output = (output << 7) | (byte & 0x7F) as u128;
        if output > u64::MAX.into() {
            return None;
        }
        bytes_read += 1;
        if (byte & 0x80) == 0 {
            return Some((output as u64, bytes_read));
        }
    }
    None
}

pub(crate) fn variable_nat_encode(mut num: u64) -> Vec<u8> {
    let mut output = vec![num as u8 & 0x7F];
    num /= 128;
    while num > 0 {
        output.push((num & 0x7F) as u8 | 0x80);
        num /= 128;
    }
    output.reverse();
    output
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NetworkInfo {
    network_id: u8,
    protocol_magic: u32,
}
#[wasm_bindgen]
impl NetworkInfo {
    pub fn new(network_id: u8, protocol_magic: u32) -> Self {
        Self {
            network_id,
            protocol_magic,
        }
    }
    pub fn network_id(&self) -> u8 {
        self.network_id
    }
    pub fn protocol_magic(&self) -> u32 {
        self.protocol_magic
    }

    pub fn testnet_preview() -> NetworkInfo {
        NetworkInfo {
            network_id: 0b0000,
            protocol_magic: 2,
        }
    }
    pub fn testnet_preprod() -> NetworkInfo {
        NetworkInfo {
            network_id: 0b0000,
            protocol_magic: 1,
        }
    }
    pub fn mainnet() -> NetworkInfo {
        NetworkInfo {
            network_id: 0b0001,
            protocol_magic: 764824073,
        }
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum AddrType {
    Base(BaseAddress),
    Ptr(PointerAddress),
    Enterprise(EnterpriseAddress),
    Reward(RewardAddress),
    Byron(ByronAddress),
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct ByronAddress(pub(crate) ExtendedAddr);
#[wasm_bindgen]
impl ByronAddress {
    pub fn to_base58(&self) -> String {
        format!("{}", self.0)
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut addr_bytes = Serializer::new_vec();
        self.0.serialize(&mut addr_bytes).unwrap();
        addr_bytes.finalize()
    }
    pub fn from_bytes(bytes: Vec<u8>) -> Result<ByronAddress, JsError> {
        let mut raw = Deserializer::from(std::io::Cursor::new(bytes));
        let extended_addr = ExtendedAddr::deserialize(&mut raw)?;
        Ok(ByronAddress(extended_addr))
    }
    /// returns the byron protocol magic embedded in the address, or mainnet id if none is present
    /// note: for bech32 addresses, you need to use network_id instead
    pub fn byron_protocol_magic(&self) -> u32 {
        match self.0.attributes.protocol_magic {
            Some(x) => x,
            None => NetworkInfo::mainnet().protocol_magic(), // mainnet is implied if omitted
        }
    }
    pub fn attributes(&self) -> Vec<u8> {
        let mut attributes_bytes = Serializer::new_vec();
        self.0.attributes.serialize(&mut attributes_bytes).unwrap();
        attributes_bytes.finalize()
    }
    pub fn network_id(&self) -> Result<u8, JsError> {
        // premise: during the Byron-era, we had one mainnet (764824073) and many many testnets
        // with each testnet getting a different protocol magic
        // in Shelley, this changes so that:
        // 1) all testnets use the same u8 protocol magic
        // 2) mainnet is re-mapped to a single u8 protocol magic

        // recall: in Byron mainnet, the network_id is omitted from the address to save a few bytes
        // so here we return the mainnet id if none is found in the address

        // mainnet is implied if omitted
        let protocol_magic = self.byron_protocol_magic();
        match protocol_magic {
            magic if magic == NetworkInfo::mainnet().protocol_magic() => {
                Ok(NetworkInfo::mainnet().network_id())
            }
            magic if magic == NetworkInfo::testnet_preprod().protocol_magic() => {
                Ok(NetworkInfo::testnet_preprod().network_id())
            }
            magic if magic == NetworkInfo::testnet_preview().protocol_magic() => {
                Ok(NetworkInfo::testnet_preview().network_id())
            }
            _ => Err(JsError::from_str(
                &format! {"Unknown network {}", protocol_magic},
            )),
        }
    }

    pub fn from_base58(s: &str) -> Result<ByronAddress, JsError> {
        use std::str::FromStr;
        ExtendedAddr::from_str(s)
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
            .map(ByronAddress)
    }

    // icarus-style address (Ae2)
    pub fn icarus_from_key(key: &Bip32PublicKey, protocol_magic: u32) -> ByronAddress {
        let mut out = [0u8; 64];
        out.clone_from_slice(&key.as_bytes());

        // need to ensure we use None for mainnet since Byron-era addresses omitted the network id
        let filtered_protocol_magic = if protocol_magic == NetworkInfo::mainnet().protocol_magic() {
            None
        } else {
            Some(protocol_magic)
        };
        ByronAddress(ExtendedAddr::new_simple(
            &XPub::from_bytes(out),
            filtered_protocol_magic,
        ))
    }

    pub fn is_valid(s: &str) -> bool {
        use std::str::FromStr;
        match ExtendedAddr::from_str(s) {
            Ok(_v) => true,
            Err(_err) => false,
        }
    }

    pub fn to_address(&self) -> Address {
        Address(AddrType::Byron(self.clone()))
    }

    pub fn from_address(addr: &Address) -> Option<ByronAddress> {
        match &addr.0 {
            AddrType::Byron(byron) => Some(byron.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Address(pub(crate) AddrType);

from_bytes!(Address, data, { Self::from_bytes_impl(data.as_ref()) });

to_from_json!(Address);

impl serde::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bech32 = self
            .to_bech32(None)
            .map_err(|e| serde::ser::Error::custom(format!("to_bech32: {:?}", e)))?;
        serializer.serialize_str(&bech32)
    }
}

impl<'de> serde::de::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let bech32 = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Address::from_bech32(&bech32).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&bech32),
                &"bech32 address string",
            )
        })
    }
}

impl JsonSchema for Address {
    fn schema_name() -> String {
        String::from("Address")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

// to/from_bytes() are the raw encoding without a wrapping CBOR Bytes tag
// while Serialize and Deserialize traits include that for inclusion with
// other CBOR types
#[wasm_bindgen]
impl Address {
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn from_hex(hex_str: &str) -> Result<Address, JsError> {
        match hex::decode(hex_str) {
            Ok(data) => Ok(Self::from_bytes_impl(data.as_ref())?),
            Err(e) => Err(JsError::from_str(&e.to_string())),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match &self.0 {
            AddrType::Base(base) => {
                let header: u8 = ((base.payment.kind() as u8) << 4)
                    | ((base.stake.kind() as u8) << 5)
                    | (base.network & 0xF);
                buf.push(header);
                buf.extend(base.payment.to_raw_bytes());
                buf.extend(base.stake.to_raw_bytes());
            }
            AddrType::Ptr(ptr) => {
                let header: u8 =
                    0b0100_0000 | ((ptr.payment.kind() as u8) << 4) | (ptr.network & 0xF);
                buf.push(header);
                buf.extend(ptr.payment.to_raw_bytes());
                buf.extend(variable_nat_encode(from_bignum(&ptr.stake.slot)));
                buf.extend(variable_nat_encode(from_bignum(&ptr.stake.tx_index)));
                buf.extend(variable_nat_encode(from_bignum(&ptr.stake.cert_index)));
            }
            AddrType::Enterprise(enterprise) => {
                let header: u8 = 0b0110_0000
                    | ((enterprise.payment.kind() as u8) << 4)
                    | (enterprise.network & 0xF);
                buf.push(header);
                buf.extend(enterprise.payment.to_raw_bytes());
            }
            AddrType::Reward(reward) => {
                let header: u8 =
                    0b1110_0000 | ((reward.payment.kind() as u8) << 4) | (reward.network & 0xF);
                buf.push(header);
                buf.extend(reward.payment.to_raw_bytes());
            }
            AddrType::Byron(byron) => buf.extend(byron.to_bytes()),
        }
        buf
    }

    fn from_bytes_impl(data: &[u8]) -> Result<Address, DeserializeError> {
        use std::convert::TryInto;
        // header has 4 bits addr type discrim then 4 bits network discrim.
        // Copied from shelley.cddl:
        //
        // shelley payment addresses:
        // bit 7: 0
        // bit 6: base/other
        // bit 5: pointer/enterprise [for base: stake cred is keyhash/scripthash]
        // bit 4: payment cred is keyhash/scripthash
        // bits 3-0: network id
        //
        // reward addresses:
        // bits 7-5: 111
        // bit 4: credential is keyhash/scripthash
        // bits 3-0: network id
        //
        // byron addresses:
        // bits 7-4: 1000
        (|| -> Result<Self, DeserializeError> {
            let header = data[0];
            let network = header & 0x0F;
            const HASH_LEN: usize = Ed25519KeyHash::BYTE_COUNT;
            // should be static assert but it's maybe not worth importing a whole external crate for it now
            assert_eq!(ScriptHash::BYTE_COUNT, HASH_LEN);
            // checks the /bit/ bit of the header for key vs scripthash then reads the credential starting at byte position /pos/
            let read_addr_cred = |bit: u8, pos: usize| {
                let hash_bytes: [u8; HASH_LEN] = data[pos..pos + HASH_LEN].try_into().unwrap();
                let x = if header & (1 << bit) == 0 {
                    Credential::from_keyhash(&Ed25519KeyHash::from(hash_bytes))
                } else {
                    Credential::from_scripthash(&ScriptHash::from(hash_bytes))
                };
                x
            };
            let addr = match (header & 0xF0) >> 4 {
                // base
                0b0000 | 0b0001 | 0b0010 | 0b0011 => {
                    const BASE_ADDR_SIZE: usize = 1 + HASH_LEN * 2;
                    if data.len() < BASE_ADDR_SIZE {
                        return Err(cbor_event::Error::NotEnough(data.len(), BASE_ADDR_SIZE).into());
                    }
                    if data.len() > BASE_ADDR_SIZE {
                        return Err(cbor_event::Error::TrailingData.into());
                    }
                    AddrType::Base(BaseAddress::new(
                        network,
                        &read_addr_cred(4, 1),
                        &read_addr_cred(5, 1 + HASH_LEN),
                    ))
                }
                // pointer
                0b0100 | 0b0101 => {
                    // header + keyhash + 3 natural numbers (min 1 byte each)
                    const PTR_ADDR_MIN_SIZE: usize = 1 + HASH_LEN + 1 + 1 + 1;
                    if data.len() < PTR_ADDR_MIN_SIZE {
                        // possibly more, but depends on how many bytes the natural numbers are for the pointer
                        return Err(
                            cbor_event::Error::NotEnough(data.len(), PTR_ADDR_MIN_SIZE).into()
                        );
                    }
                    let mut byte_index = 1;
                    let payment_cred = read_addr_cred(4, 1);
                    byte_index += HASH_LEN;
                    let (slot, slot_bytes) =
                        variable_nat_decode(&data[byte_index..]).ok_or(DeserializeError::new(
                            "Address.Pointer.slot",
                            DeserializeFailure::VariableLenNatDecodeFailed,
                        ))?;
                    byte_index += slot_bytes;
                    let (tx_index, tx_bytes) =
                        variable_nat_decode(&data[byte_index..]).ok_or(DeserializeError::new(
                            "Address.Pointer.tx_index",
                            DeserializeFailure::VariableLenNatDecodeFailed,
                        ))?;
                    byte_index += tx_bytes;
                    let (cert_index, cert_bytes) =
                        variable_nat_decode(&data[byte_index..]).ok_or(DeserializeError::new(
                            "Address.Pointer.cert_index",
                            DeserializeFailure::VariableLenNatDecodeFailed,
                        ))?;
                    byte_index += cert_bytes;
                    if byte_index < data.len() {
                        return Err(cbor_event::Error::TrailingData.into());
                    }
                    AddrType::Ptr(PointerAddress::new(
                        network,
                        &payment_cred,
                        &Pointer::new_pointer(
                            &to_bignum(slot),
                            &to_bignum(tx_index),
                            &to_bignum(cert_index),
                        ),
                    ))
                }
                // enterprise
                0b0110 | 0b0111 => {
                    const ENTERPRISE_ADDR_SIZE: usize = 1 + HASH_LEN;
                    if data.len() < ENTERPRISE_ADDR_SIZE {
                        return Err(
                            cbor_event::Error::NotEnough(data.len(), ENTERPRISE_ADDR_SIZE).into(),
                        );
                    }
                    if data.len() > ENTERPRISE_ADDR_SIZE {
                        return Err(cbor_event::Error::TrailingData.into());
                    }
                    AddrType::Enterprise(EnterpriseAddress::new(network, &read_addr_cred(4, 1)))
                }
                // reward
                0b1110 | 0b1111 => {
                    const REWARD_ADDR_SIZE: usize = 1 + HASH_LEN;
                    if data.len() < REWARD_ADDR_SIZE {
                        return Err(
                            cbor_event::Error::NotEnough(data.len(), REWARD_ADDR_SIZE).into()
                        );
                    }
                    if data.len() > REWARD_ADDR_SIZE {
                        return Err(cbor_event::Error::TrailingData.into());
                    }
                    AddrType::Reward(RewardAddress::new(network, &read_addr_cred(4, 1)))
                }
                // byron
                0b1000 => {
                    // note: 0b1000 was chosen because all existing Byron addresses actually start with 0b1000
                    // Therefore you can re-use Byron addresses as-is
                    match ByronAddress::from_bytes(data.to_vec()) {
                        Ok(addr) => AddrType::Byron(addr),
                        Err(e) => {
                            return Err(cbor_event::Error::CustomError(
                                e.as_string().unwrap_or_default(),
                            )
                            .into())
                        }
                    }
                }
                _ => return Err(DeserializeFailure::BadAddressType(header).into()),
            };
            Ok(Address(addr))
        })()
        .map_err(|e| e.annotate("Address"))
    }

    pub fn to_bech32(&self, prefix: Option<String>) -> Result<String, JsError> {
        let final_prefix = match prefix {
            Some(prefix) => prefix,
            None => {
                // see CIP5 for bech32 prefix rules
                let prefix_header = match &self.0 {
                    AddrType::Reward(_) => "stake",
                    _ => "addr",
                };
                let prefix_tail = match self.network_id()? {
                    id if id == NetworkInfo::testnet_preprod().network_id() => "_test",
                    id if id == NetworkInfo::testnet_preview().network_id() => "_test",
                    _ => "",
                };
                format!("{}{}", prefix_header, prefix_tail)
            }
        };
        bech32::encode(&final_prefix, self.to_bytes().to_base32())
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
    }

    pub fn from_bech32(bech_str: &str) -> Result<Address, JsError> {
        let (_hrp, u5data) =
            bech32::decode(bech_str).map_err(|e| JsError::from_str(&e.to_string()))?;
        let data: Vec<u8> = bech32::FromBase32::from_base32(&u5data).unwrap();
        Ok(Self::from_bytes_impl(data.as_ref())?)
    }

    pub fn network_id(&self) -> Result<u8, JsError> {
        match &self.0 {
            AddrType::Base(a) => Ok(a.network),
            AddrType::Enterprise(a) => Ok(a.network),
            AddrType::Ptr(a) => Ok(a.network),
            AddrType::Reward(a) => Ok(a.network),
            AddrType::Byron(a) => a.network_id(),
        }
    }
}

impl cbor_event::se::Serialize for Address {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(self.to_bytes())
    }
}

impl Deserialize for Address {
    fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Self::from_bytes_impl(raw.bytes()?.as_ref())
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct BaseAddress {
    pub(crate) network: u8,
    pub(crate) payment: Credential,
    pub(crate) stake: Credential,
}

#[wasm_bindgen]
impl BaseAddress {
    pub fn new(network: u8, payment: &Credential, stake: &Credential) -> Self {
        Self {
            network,
            payment: payment.clone(),
            stake: stake.clone(),
        }
    }

    pub fn payment_cred(&self) -> Credential {
        self.payment.clone()
    }

    pub fn stake_cred(&self) -> Credential {
        self.stake.clone()
    }

    pub fn to_address(&self) -> Address {
        Address(AddrType::Base(self.clone()))
    }

    pub fn from_address(addr: &Address) -> Option<BaseAddress> {
        match &addr.0 {
            AddrType::Base(base) => Some(base.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct EnterpriseAddress {
    pub(crate) network: u8,
    pub(crate) payment: Credential,
}

#[wasm_bindgen]
impl EnterpriseAddress {
    pub fn new(network: u8, payment: &Credential) -> Self {
        Self {
            network,
            payment: payment.clone(),
        }
    }

    pub fn payment_cred(&self) -> Credential {
        self.payment.clone()
    }

    pub fn to_address(&self) -> Address {
        Address(AddrType::Enterprise(self.clone()))
    }

    pub fn from_address(addr: &Address) -> Option<EnterpriseAddress> {
        match &addr.0 {
            AddrType::Enterprise(enterprise) => Some(enterprise.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RewardAddress {
    pub(crate) network: u8,
    pub(crate) payment: Credential,
}

#[wasm_bindgen]
impl RewardAddress {
    pub fn new(network: u8, payment: &Credential) -> Self {
        Self {
            network,
            payment: payment.clone(),
        }
    }

    pub fn payment_cred(&self) -> Credential {
        self.payment.clone()
    }

    pub fn to_address(&self) -> Address {
        Address(AddrType::Reward(self.clone()))
    }

    pub fn from_address(addr: &Address) -> Option<RewardAddress> {
        match &addr.0 {
            AddrType::Reward(reward) => Some(reward.clone()),
            _ => None,
        }
    }
}

impl serde::Serialize for RewardAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bech32 = self
            .to_address()
            .to_bech32(None)
            .map_err(|e| serde::ser::Error::custom(format!("to_bech32: {:?}", e)))?;
        serializer.serialize_str(&bech32)
    }
}

impl<'de> serde::de::Deserialize<'de> for RewardAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let bech32 = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        match Address::from_bech32(&bech32)
            .ok()
            .map(|addr| RewardAddress::from_address(&addr))
        {
            Some(Some(ra)) => Ok(ra),
            _ => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&bech32),
                &"bech32 reward address string",
            )),
        }
    }
}

impl JsonSchema for RewardAddress {
    fn schema_name() -> String {
        String::from("RewardAddress")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

// needed since we treat RewardAccount like RewardAddress
impl cbor_event::se::Serialize for RewardAddress {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.to_address().serialize(serializer)
    }
}

impl Deserialize for RewardAddress {
    fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<Self, DeserializeError> {
            let bytes = raw.bytes()?;
            match Address::from_bytes_impl(bytes.as_ref())?.0 {
                AddrType::Reward(ra) => Ok(ra),
                _other_address => Err(DeserializeFailure::BadAddressType(bytes[0]).into()),
            }
        })()
        .map_err(|e| e.annotate("RewardAddress"))
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pointer {
    pub(crate) slot: BigNum,
    pub(crate) tx_index: BigNum,
    pub(crate) cert_index: BigNum,
}

#[wasm_bindgen]
impl Pointer {
    /// !!! DEPRECATED !!!
    /// This constructor uses outdated slot number format for the ttl value, tx_index and cert_index.
    /// Use `.new_pointer` instead
    #[deprecated(
        since = "10.1.0",
        note = "Underlying value capacity of ttl (BigNum u64) bigger then Slot32. Use new_pointer instead."
    )]
    pub fn new(slot: Slot32, tx_index: TransactionIndex, cert_index: CertificateIndex) -> Self {
        Self {
            slot: slot.into(),
            tx_index: tx_index.into(),
            cert_index: cert_index.into(),
        }
    }

    pub fn new_pointer(slot: &SlotBigNum, tx_index: &BigNum, cert_index: &BigNum) -> Self {
        Self {
            slot: slot.clone(),
            tx_index: tx_index.clone(),
            cert_index: cert_index.clone(),
        }
    }

    pub fn slot(&self) -> Result<u32, JsError> {
        self.slot.clone().try_into()
    }

    pub fn tx_index(&self) -> Result<u32, JsError> {
        self.tx_index.clone().try_into()
    }

    pub fn cert_index(&self) -> Result<u32, JsError> {
        self.cert_index.clone().try_into()
    }

    pub fn slot_bignum(&self) -> BigNum {
        self.slot.clone()
    }

    pub fn tx_index_bignum(&self) -> BigNum {
        self.tx_index.clone()
    }

    pub fn cert_index_bignum(&self) -> BigNum {
        self.cert_index.clone()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PointerAddress {
    pub(crate) network: u8,
    pub(crate) payment: Credential,
    pub(crate) stake: Pointer,
}

#[wasm_bindgen]
impl PointerAddress {
    pub fn new(network: u8, payment: &Credential, stake: &Pointer) -> Self {
        Self {
            network,
            payment: payment.clone(),
            stake: stake.clone(),
        }
    }

    pub fn payment_cred(&self) -> Credential {
        self.payment.clone()
    }

    pub fn stake_pointer(&self) -> Pointer {
        self.stake.clone()
    }

    pub fn to_address(&self) -> Address {
        Address(AddrType::Ptr(self.clone()))
    }

    pub fn from_address(addr: &Address) -> Option<PointerAddress> {
        match &addr.0 {
            AddrType::Ptr(ptr) => Some(ptr.clone()),
            _ => None,
        }
    }
}
