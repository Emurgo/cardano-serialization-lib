use super::*;
use bech32::ToBase32;
use crate::legacy_address::ExtendedAddr;
use ed25519_bip32::XPub;

// returns (Number represented, bytes read) if valid encoding
// or None if decoding prematurely finished
fn variable_nat_decode(bytes: &[u8]) -> Option<(u64, usize)> {
    let mut output = 0u64;
    let mut bytes_read = 0;
    for byte in bytes {
        output = (output << 7) | (byte & 0x7F) as u64;
        bytes_read += 1;
        if (byte & 0x80) == 0 {
            return Some((output, bytes_read));
        }
    }
    None
}

fn variable_nat_encode(mut num: u64) -> Vec<u8> {
    let mut output = vec![num as u8 & 0x7F];
    num /= 128;
    while num > 0 {
        output.push((num & 0x7F) as u8 | 0x80);
        num /= 128;
    }
    output.reverse();
    output
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum StakeCredType {
    Key(Ed25519KeyHash),
    Script(ScriptHash),
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct StakeCredential(StakeCredType);

#[wasm_bindgen]
impl StakeCredential {
    pub fn from_keyhash(hash: &Ed25519KeyHash) -> Self {
        StakeCredential(StakeCredType::Key(hash.clone()))
    }

    pub fn from_scripthash(hash: &ScriptHash) -> Self {
        StakeCredential(StakeCredType::Script(hash.clone()))
    }

    pub fn to_keyhash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            StakeCredType::Key(hash) => Some(hash.clone()),
            StakeCredType::Script(_) => None,
        }
    }

    pub fn to_scripthash(&self) -> Option<ScriptHash> {
        match &self.0 {
            StakeCredType::Key(_) => None,
            StakeCredType::Script(hash) => Some(hash.clone()),
        }
    }

    pub fn kind(&self) -> u8 {
        match &self.0 {
            StakeCredType::Key(_) => 0,
            StakeCredType::Script(_) => 1,
        }
    }

    fn to_raw_bytes(&self) -> Vec<u8> {
        match &self.0 {
            StakeCredType::Key(hash) => hash.to_bytes(),
            StakeCredType::Script(hash) => hash.to_bytes(),
        }
    }
}

to_from_bytes!(StakeCredential);

impl cbor_event::se::Serialize for StakeCredential {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        match &self.0 {
            StakeCredType::Key(keyhash) => {
                serializer.write_unsigned_integer(0u64)?;
                serializer.write_bytes(keyhash.to_bytes())
            },
            StakeCredType::Script(scripthash) => {
                serializer.write_unsigned_integer(1u64)?;
                serializer.write_bytes(scripthash.to_bytes())
            },
        }
    }
}

impl Deserialize for StakeCredential {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            if let cbor_event::Len::Len(n) = len {
                if n != 2 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(2, len, "[id, hash]")).into())
                }
            }
            let cred_type = match raw.unsigned_integer()? {
                0 => StakeCredType::Key(Ed25519KeyHash::deserialize(raw)?),
                1 => StakeCredType::Script(ScriptHash::deserialize(raw)?),
                n => return Err(DeserializeFailure::FixedValueMismatch{
                    found: Key::Uint(n),
                    // TODO: change codegen to make FixedValueMismatch support Vec<Key> or ranges or something
                    expected: Key::Uint(0),
                }.into()),
            };
            if let cbor_event::Len::Indefinite = len {
                 if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }
            Ok(StakeCredential(cred_type))
        })().map_err(|e| e.annotate("StakeCredential"))
    }
}

#[derive(Debug, Clone)]
enum AddrType {
    Base(BaseAddress),
    Ptr(PointerAddress),
    Enterprise(EnterpriseAddress),
    Reward(RewardAddress),
    Byron(ByronAddress),
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct ByronAddress(pub (crate) ExtendedAddr);
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
    pub fn from_bytes(bytes: Vec<u8>) -> Result<ByronAddress, JsValue> {
        let mut raw = Deserializer::from(std::io::Cursor::new(bytes));
        let extended_addr = ExtendedAddr::deserialize(&mut raw)?;
        Ok(ByronAddress(extended_addr))
    }
    /// returns the byron protocol magic embedded in the address, or mainnet id if none is present
    /// note: for bech32 addresses, you need to use network_id instead
    pub fn byron_protocol_magic(&self) -> u32 {
        let mainnet_network_id = 764824073;
        match self.0.attributes.network_magic {
            Some(x) => x,
            None => mainnet_network_id, // mainnet is implied if omitted
        }
    }
    pub fn network_id(&self) -> u8 {
        // premise: during the Byron-era, we had one mainnet (764824073) and many many testnets
        // with each testnet getting a different protocol magic
        // in Shelley, this changes so that:
        // 1) all testnets use the same u8 protocol magic
        // 2) mainnet is re-mapped to a single u8 protocol magic

        // recall: in Byron mainnet, the network_id is omitted from the address to save a few bytes
        let mainnet_network_id = 764824073;
        // so here we return the mainnet id if none is found in the address

        match self.0.attributes.network_magic {
            // although mainnet should never be explicitly added, we check for it just in case
            Some(x) => if x == mainnet_network_id { 0b0001 } else { 0b000 },
            None => 0b0001, // mainnet is implied if omitted
        }
    }

    pub fn from_base58(s: &str) -> Result<ByronAddress, JsValue> {
        use std::str::FromStr;
        ExtendedAddr::from_str(s)
            .map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
            .map(ByronAddress)
    }

    // icarus-style address (Ae2)
    pub fn from_icarus_key(key: &Bip32PublicKey, network: u8) -> ByronAddress {
        let mut out = [0u8; 64];
        out.clone_from_slice(&key.as_bytes());

        // need to ensure we use None for mainnet since Byron-era addresses omitted the network id
        let mapped_network_id = if network == 0b0001 { None } else { Some(0b000 as u32) };
        ByronAddress(ExtendedAddr::new_simple(& XPub::from_bytes(out), mapped_network_id))
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
#[derive(Debug, Clone)]
pub struct Address(AddrType);

from_bytes!(Address, data, {
    Self::from_bytes_impl(data.as_ref())
});

// to/from_bytes() are the raw encoding without a wrapping CBOR Bytes tag
// while Serialize and Deserialize traits include that for inclusion with
// other CBOR types
#[wasm_bindgen]
impl Address {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match &self.0 {
            AddrType::Base(base) => {
                let header: u8 = (base.payment.kind() << 4)
                           | (base.stake.kind() << 5)
                           | (base.network & 0xF);
                buf.push(header);
                buf.extend(base.payment.to_raw_bytes());
                buf.extend(base.stake.to_raw_bytes());
            },
            AddrType::Ptr(ptr) => {
                let header: u8 = 0b0100_0000
                               | (ptr.payment.kind() << 4)
                               | (ptr.network & 0xF);
                buf.push(header);
                buf.extend(ptr.payment.to_raw_bytes());
                buf.extend(variable_nat_encode(ptr.stake.slot.into()));
                buf.extend(variable_nat_encode(ptr.stake.tx_index.into()));
                buf.extend(variable_nat_encode(ptr.stake.cert_index.into()));
            },
            AddrType::Enterprise(enterprise) => {
                let header: u8 = 0b0110_0000
                               | (enterprise.payment.kind() << 4)
                               | (enterprise.network & 0xF);
                buf.push(header);
                buf.extend(enterprise.payment.to_raw_bytes());
            },
            AddrType::Reward(reward) => {
                let header: u8 = 0b1110_0000
                                | (reward.payment.kind() << 4)
                                | (reward.network & 0xF);
                buf.push(header);
                buf.extend(reward.payment.to_raw_bytes());
            },
            AddrType::Byron(byron) => {
                buf.extend(byron.to_bytes())
            },
        }
        println!("to_bytes({:?}) = {:?}", self, buf);
        buf
    }

    fn from_bytes_impl(data: &[u8]) -> Result<Address, DeserializeError> {
        use std::convert::TryInto;
        println!("reading from: {:?}", data);
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
                let hash_bytes: [u8; HASH_LEN] = data[pos..pos+HASH_LEN].try_into().unwrap();
                let x = if header & (1 << bit)  == 0 {
                    StakeCredential::from_keyhash(&Ed25519KeyHash::from(hash_bytes))
                } else {
                    StakeCredential::from_scripthash(&ScriptHash::from(hash_bytes))
                };
                println!("read cred: {:?}", x);
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
                    AddrType::Base(BaseAddress::new(network, &read_addr_cred(4, 1), &read_addr_cred(5, 1 + HASH_LEN)))
                },
                // pointer
                0b0100 | 0b0101 => {
                    // header + keyhash + 3 natural numbers (min 1 byte each)
                    const PTR_ADDR_MIN_SIZE: usize = 1 + HASH_LEN + 1 + 1 + 1;
                    if data.len() < PTR_ADDR_MIN_SIZE {
                        // possibly more, but depends on how many bytes the natural numbers are for the pointer
                        return Err(cbor_event::Error::NotEnough(data.len(), PTR_ADDR_MIN_SIZE).into());
                    }
                    let mut byte_index = 1;
                    let payment_cred = read_addr_cred(4, 1);
                    byte_index += HASH_LEN;
                    let (slot, slot_bytes) = variable_nat_decode(&data[byte_index..])
                        .ok_or(DeserializeError::new("Address.Pointer.slot", DeserializeFailure::VariableLenNatDecodeFailed))?;
                    byte_index += slot_bytes;
                    let (tx_index, tx_bytes) = variable_nat_decode(&data[byte_index..])
                        .ok_or(DeserializeError::new("Address.Pointer.tx_index", DeserializeFailure::VariableLenNatDecodeFailed))?;
                    byte_index += tx_bytes;
                    let (cert_index, cert_bytes) = variable_nat_decode(&data[byte_index..])
                        .ok_or(DeserializeError::new("Address.Pointer.cert_index", DeserializeFailure::VariableLenNatDecodeFailed))?;
                    byte_index += cert_bytes;
                    if byte_index < data.len() {
                        return Err(cbor_event::Error::TrailingData.into());
                    }
                    AddrType::Ptr(
                        PointerAddress::new(
                            network,
                            &payment_cred,
                            &Pointer::new(
                                slot.try_into().map_err(|_| DeserializeError::new("Address.Pointer.slot", DeserializeFailure::CBOR(cbor_event::Error::ExpectedU32)))?,
                                tx_index.try_into().map_err(|_| DeserializeError::new("Address.Pointer.tx_index", DeserializeFailure::CBOR(cbor_event::Error::ExpectedU32)))?,
                                cert_index.try_into().map_err(|_| DeserializeError::new("Address.Pointer.cert_index", DeserializeFailure::CBOR(cbor_event::Error::ExpectedU32)))?)))
                },
                // enterprise
                0b0110 | 0b0111 => {
                    const ENTERPRISE_ADDR_SIZE: usize = 1 + HASH_LEN;
                    if data.len() < ENTERPRISE_ADDR_SIZE {
                        return Err(cbor_event::Error::NotEnough(data.len(), ENTERPRISE_ADDR_SIZE).into());
                    }
                    if data.len() > ENTERPRISE_ADDR_SIZE {
                        return Err(cbor_event::Error::TrailingData.into());
                    }
                    AddrType::Enterprise(EnterpriseAddress::new(network, &read_addr_cred(4, 1)))
                },
                // reward
                0b1110 | 0b1111 => {
                    const REWARD_ADDR_SIZE: usize = 1 + HASH_LEN;
                    if data.len() < REWARD_ADDR_SIZE {
                        return Err(cbor_event::Error::NotEnough(data.len(), REWARD_ADDR_SIZE).into());
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
                        Err(e) => return Err(cbor_event::Error::CustomError(e.as_string().unwrap_or_default()).into()),
                    }
                },
                _ => return Err(DeserializeFailure::BadAddressType(header).into()),
            };
            Ok(Address(addr))
        })().map_err(|e| e.annotate("Address"))
    }

    pub fn to_bech32(&self, prefix: Option<String>) -> String {
        bech32::encode(&prefix.unwrap_or("addr".to_string()), self.to_bytes().to_base32()).unwrap()
    }

    pub fn from_bech32(bech_str: &str) -> Result<Address, JsValue> {
        let (_hrp, u5data) = bech32::decode(bech_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let data: Vec<u8> = bech32::FromBase32::from_base32(&u5data).unwrap();
        Ok(Self::from_bytes_impl(data.as_ref())?)
    }

    pub fn network_id(&self) -> u8 {
        match &self.0 {
            AddrType::Base(a) => a.network,
            AddrType::Enterprise(a) => a.network,
            AddrType::Ptr(a) => a.network,
            AddrType::Reward(a) => a.network,
            AddrType::Byron(a) => a.network_id(),
        }
    }
}

impl cbor_event::se::Serialize for Address {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
    network: u8,
    payment: StakeCredential,
    stake: StakeCredential,
}

#[wasm_bindgen]
impl BaseAddress {
    pub fn new(network: u8, payment: &StakeCredential, stake: &StakeCredential) -> Self {
        Self {
            network,
            payment: payment.clone(),
            stake: stake.clone(),
        }
    }

    pub fn payment_cred(&self) -> StakeCredential {
        self.payment.clone()
    }

    pub fn stake_cred(&self) -> StakeCredential {
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
    network: u8,
    payment: StakeCredential,
}

#[wasm_bindgen]
impl EnterpriseAddress {
    pub fn new(network: u8, payment: &StakeCredential) -> Self {
        Self {
            network,
            payment: payment.clone(),
        }
    }

    pub fn payment_cred(&self) -> StakeCredential {
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
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct RewardAddress {
    network: u8,
    payment: StakeCredential,
}

#[wasm_bindgen]
impl RewardAddress {
    pub fn new(network: u8, payment: &StakeCredential) -> Self {
        Self {
            network,
            payment: payment.clone(),
        }
    }

    pub fn payment_cred(&self) -> StakeCredential {
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

// needed since we treat RewardAccount like RewardAddress
impl cbor_event::se::Serialize for RewardAddress {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
        })().map_err(|e| e.annotate("RewardAddress"))
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pointer {
    slot: Slot,
    tx_index: TransactionIndex,
    cert_index: CertificateIndex,
}

#[wasm_bindgen]
impl Pointer {
    pub fn new(slot: Slot, tx_index: TransactionIndex, cert_index: CertificateIndex) -> Self {
        Self {
            slot,
            tx_index,
            cert_index,
        }
    }

    pub fn slot(&self) -> Slot {
        self.slot.clone()
    }

    pub fn tx_index(&self) -> TransactionIndex {
        self.tx_index.clone()
    }

    pub fn cert_index(&self) -> CertificateIndex {
        self.cert_index.clone()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PointerAddress {
    network: u8,
    payment: StakeCredential,
    stake: Pointer,
}

#[wasm_bindgen]
impl PointerAddress {
    pub fn new(network: u8, payment: &StakeCredential, stake: &Pointer) -> Self {
        Self {
            network,
            payment: payment.clone(),
            stake: stake.clone(),
        }
    }

    pub fn payment_cred(&self) -> StakeCredential {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::*;

    #[test]
    fn variable_nat_encoding() {
        let cases = [
            0u64,
            127u64,
            128u64,
            255u64,
            256275757658493284u64
        ];
        for case in cases.iter() {
            let encoded = variable_nat_encode(*case);
            let decoded = variable_nat_decode(&encoded).unwrap().0;
            assert_eq!(*case, decoded);
        }
    }

    #[test]
    fn base_serialize_consistency() {
        let base = BaseAddress::new(
            5,
            &StakeCredential::from_keyhash(&Ed25519KeyHash::from([23; Ed25519KeyHash::BYTE_COUNT])),
            &StakeCredential::from_scripthash(&ScriptHash::from([42; ScriptHash::BYTE_COUNT])));
        let addr = base.to_address();
        let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
        assert_eq!(addr.to_bytes(), addr2.to_bytes());
    }

    #[test]
    fn ptr_serialize_consistency() {
        let ptr = PointerAddress::new(
            25,
            &StakeCredential::from_keyhash(&Ed25519KeyHash::from([23; Ed25519KeyHash::BYTE_COUNT])),
            &Pointer::new(2354556573, 127, 0));
        let addr = ptr.to_address();
        let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
        assert_eq!(addr.to_bytes(), addr2.to_bytes());
    }

    #[test]
    fn enterprise_serialize_consistency() {
        let enterprise = EnterpriseAddress::new(
            64,
            &StakeCredential::from_keyhash(&Ed25519KeyHash::from([23; Ed25519KeyHash::BYTE_COUNT])));
        let addr = enterprise.to_address();
        let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
        assert_eq!(addr.to_bytes(), addr2.to_bytes());
    }

    #[test]
    fn reward_serialize_consistency() {
        let reward = RewardAddress::new(
            9,
            &StakeCredential::from_scripthash(&ScriptHash::from([127; Ed25519KeyHash::BYTE_COUNT])));
        let addr = reward.to_address();
        let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
        assert_eq!(addr.to_bytes(), addr2.to_bytes());
    }

    fn root_key_12() -> Bip32PrivateKey {
        // test walk nut penalty hip pave soap entry language right filter choice
        let entropy = [0xdf, 0x9e, 0xd2, 0x5e, 0xd1, 0x46, 0xbf, 0x43, 0x33, 0x6a, 0x5d, 0x7c, 0xf7, 0x39, 0x59, 0x94];
        Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
    }

    fn root_key_15() -> Bip32PrivateKey {
        // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
        let entropy = [0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12];
        Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
    }

    fn root_key_24() -> Bip32PrivateKey {
        let entropy = [0x4e, 0x82, 0x8f, 0x9a, 0x67, 0xdd, 0xcf, 0xf0, 0xe6, 0x39, 0x1a, 0xd4, 0xf2, 0x6d, 0xdb, 0x75, 0x79, 0xf5, 0x9b, 0xa1, 0x4b, 0x6d, 0xd4, 0xba, 0xf6, 0x3d, 0xcf, 0xdb, 0x9d, 0x24, 0x20, 0xda];
        Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
    }

    fn harden(index: u32) -> u32 {
        index | 0x80_00_00_00
    }

    #[test]
    fn bech32_parsing() {
        let addr = Address::from_bech32("addr1u8pcjgmx7962w6hey5hhsd502araxp26kdtgagakhaqtq8sxy9w7g").unwrap();
        assert_eq!(addr.to_bech32(Some("stake".to_string())), "stake1u8pcjgmx7962w6hey5hhsd502araxp26kdtgagakhaqtq8squng76");
    }

    #[test]
    fn byron_magic_parsing() {
        // mainnet address w/ protocol magic omitted
        let addr = ByronAddress::from_base58("Ae2tdPwUPEZ4YjgvykNpoFeYUxoyhNj2kg8KfKWN2FizsSpLUPv68MpTVDo").unwrap();
        assert_eq!(addr.byron_protocol_magic(), 764824073);

        // original Byron testnet address
        let addr = ByronAddress::from_base58("2cWKMJemoBakg8XXW1XNFNZ8VFHVrBPfcoEc9amhL3BBMxJXdMiHmSyk3zRp2SDXHJcZr").unwrap();
        assert_eq!(addr.byron_protocol_magic(), 1097911063);
    }

    #[test]
    fn bip32_12_base() {
        let spend = root_key_12()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_12()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(0, &spend_cred, &stake_cred).to_address();
        assert_eq!(addr_net_0.to_bech32(None), "addr1qz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3jcu5d8ps7zex2k2xt3uqxgjqnnj83ws8lhrn648jjxtwqcyl47r");
        let addr_net_3 = BaseAddress::new(3, &spend_cred, &stake_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None), "addr1qw2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3jcu5d8ps7zex2k2xt3uqxgjqnnj83ws8lhrn648jjxtwqzhyupd");
    }

    #[test]
    fn bip32_12_enterprise() {
        let spend = root_key_12()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let addr_net_0 = EnterpriseAddress::new(0, &spend_cred).to_address();
        assert_eq!(addr_net_0.to_bech32(None), "addr1vz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzers6g8jlq");
        let addr_net_3 = EnterpriseAddress::new(3, &spend_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None), "addr1vw2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzers6h7glf");
    }

    #[test]
    fn bip32_12_pointer() {
        let spend = root_key_12()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let addr_net_0 = PointerAddress::new(0, &spend_cred, &Pointer::new(1, 2, 3)).to_address();
        assert_eq!(addr_net_0.to_bech32(None), "addr1gz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzerspqgpslhplej");
        let addr_net_3 = PointerAddress::new(3, &spend_cred, &Pointer::new(24157, 177, 42)).to_address();
        assert_eq!(addr_net_3.to_bech32(None), "addr1gw2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer5ph3wczvf2x4v58t");
    }

    #[test]
    fn bip32_15_base() {
        let spend = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(0, &spend_cred, &stake_cred).to_address();
        assert_eq!(addr_net_0.to_bech32(None), "addr1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qwmnp2v");
        let addr_net_3 = BaseAddress::new(3, &spend_cred, &stake_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None), "addr1qdu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2q5ggg4z");
    }

    #[test]
    fn bip32_15_enterprise() {
        let spend = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let addr_net_0 = EnterpriseAddress::new(0, &spend_cred).to_address();
        assert_eq!(addr_net_0.to_bech32(None), "addr1vpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5eg0yu80w");
        let addr_net_3 = EnterpriseAddress::new(3, &spend_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None), "addr1vdu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5eg0m9a08");
    }

    #[test]
    fn bip32_15_pointer() {
        let spend = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let addr_net_0 = PointerAddress::new(0, &spend_cred, &Pointer::new(1, 2, 3)).to_address();
        assert_eq!(addr_net_0.to_bech32(None), "addr1gpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5egpqgpsjej5ck");
        let addr_net_3 = PointerAddress::new(3, &spend_cred, &Pointer::new(24157, 177, 42)).to_address();
        assert_eq!(addr_net_3.to_bech32(None), "addr1gdu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5evph3wczvf27l8yfx");
    }

    #[test]
    fn bip32_15_byron() {
        let byron_key = root_key_15()
            .derive(harden(44))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let byron_addr = ByronAddress::from_icarus_key(&byron_key, 0b0001);
        assert_eq!(byron_addr.to_base58(), "Ae2tdPwUPEZHtBmjZBF4YpMkK9tMSPTE2ADEZTPN97saNkhG78TvXdp3GDk");
        assert!(ByronAddress::is_valid("Ae2tdPwUPEZHtBmjZBF4YpMkK9tMSPTE2ADEZTPN97saNkhG78TvXdp3GDk"));
        assert_eq!(byron_addr.network_id(), 0b0001);

        let byron_addr_2 = ByronAddress::from_address(&Address::from_bytes(byron_addr.to_bytes()).unwrap()).unwrap();
        assert_eq!(byron_addr.to_base58(), byron_addr_2.to_base58());
    }

    #[test]
    fn bip32_24_base() {
        let spend = root_key_24()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_24()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(0, &spend_cred, &stake_cred).to_address();
        assert_eq!(addr_net_0.to_bech32(None), "addr1qqy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmn8k8ttq8f3gag0h89aepvx3xf69g0l9pf80tqv7cve0l33su9wxrs");
        let addr_net_3 = BaseAddress::new(3, &spend_cred, &stake_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None), "addr1qvy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmn8k8ttq8f3gag0h89aepvx3xf69g0l9pf80tqv7cve0l33sxk40u7");
    }

    #[test]
    fn bip32_24_enterprise() {
        let spend = root_key_24()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let addr_net_0 = EnterpriseAddress::new(0, &spend_cred).to_address();
        assert_eq!(addr_net_0.to_bech32(None), "addr1vqy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqsg0y49");
        let addr_net_3 = EnterpriseAddress::new(3, &spend_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None), "addr1vvy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqshk74v");
    }

    #[test]
    fn bip32_24_pointer() {
        let spend = root_key_24()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let addr_net_0 = PointerAddress::new(0, &spend_cred, &Pointer::new(1, 2, 3)).to_address();
        assert_eq!(addr_net_0.to_bech32(None), "addr1gqy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqpqgpst4xf0c");
        let addr_net_3 = PointerAddress::new(3, &spend_cred, &Pointer::new(24157, 177, 42)).to_address();
        assert_eq!(addr_net_3.to_bech32(None), "addr1gvy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnyph3wczvf29j6huk");
    }
}
