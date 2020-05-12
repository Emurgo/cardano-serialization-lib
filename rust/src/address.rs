use super::*;
use prelude::*;

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
enum AddrCredType {
    Key(Keyhash),
    Script(Scripthash),
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct AddrCred(AddrCredType);

#[wasm_bindgen]
impl AddrCred {
    pub fn from_keyhash(hash: Keyhash) -> Self {
        AddrCred(AddrCredType::Key(hash))
    }

    pub fn from_scripthash(hash: Scripthash) -> Self {
        AddrCred(AddrCredType::Script(hash))
    }

    pub fn to_keyhash(&self) -> Option<Keyhash> {
        match &self.0 {
            AddrCredType::Key(hash) => Some(hash.clone()),
            AddrCredType::Script(_) => None,
        }
    }

    pub fn to_scripthash(&self) -> Option<Scripthash> {
        match &self.0 {
            AddrCredType::Key(_) => None,
            AddrCredType::Script(hash) => Some(hash.clone()),
        }
    }

    pub fn kind(&self) -> u8 {
        match &self.0 {
            AddrCredType::Key(_) => 0,
            AddrCredType::Script(_) => 1,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match &self.0 {
            AddrCredType::Key(hash) => hash.0.clone(),
            AddrCredType::Script(hash) => hash.0.clone(),
        }
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum AddrType {
    Base(BaseAddress),
    Ptr(PointerAddress),
    Enterprise(EnterpriseAddress),
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Address(AddrType);

// to/from_bytes() are the raw encoding without a wrapping CBOR Bytes tag
// while Serialize and Deserialize traits include that for inclusion with
// other CBOR types
//#[wasm_bindgen]
impl Address {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match &self.0 {
            AddrType::Base(base) => {
                let header: u8 = (base.payment.kind() << 4)
                           | (base.stake.kind() << 5)
                           | (base.network & 0xF);
                buf.push(header);
                buf.extend(base.payment.to_bytes());
                buf.extend(base.stake.to_bytes());
            },
            AddrType::Ptr(ptr) => {
                let header: u8 = 0b0100_0000
                               | (ptr.payment.kind() << 4)
                               | (ptr.network & 0xF);
                buf.push(header);
                buf.extend(ptr.payment.to_bytes());
                buf.extend(variable_nat_encode(ptr.stake.slot));
                buf.extend(variable_nat_encode(ptr.stake.tx_index));
                buf.extend(variable_nat_encode(ptr.stake.cert_index));
            },
            AddrType::Enterprise(enterprise) => {
                let header: u8 = 0b0110_0000
                               | (enterprise.payment.kind() << 4)
                               | (enterprise.network & 0xF);
                buf.push(header);
                buf.extend(enterprise.payment.to_bytes());
            },
        }
        println!("to_bytes({:?}) = {:?}", self, buf);
        buf
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Self, JsValue> {
        Self::from_bytes_impl(data).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    fn from_bytes_impl(data: Vec<u8>) -> Result<Self, DeserializeError> {
        println!("reading from: {:?}", data);
        // header has 4 bytes addr type discrim then 4 bytes network discrim.
        // Copied from shelley.cddl:
        // bit 7: byron/shelley
        // bit 6: base/other
        // bit 5: pointer/enterprise [for base: stake cred is keyhash/scripthash]
        // bit 4: payment cred is keyhash/scripthash
        // bits 3-0 (for shelley addr): network id
        let header = data[0];
        let network = header & 0x0F;
        let hash_len = 28;
        let read_addr_cred = |bit: u8, pos: usize| {
            let x = if header & (1 << bit)  == 0{
                AddrCred::from_keyhash(Keyhash::new(data[pos..pos+hash_len].to_vec()))
            } else {
                AddrCred::from_scripthash(Scripthash::new(data[pos..pos+hash_len].to_vec()))
            };
            println!("read cred: {:?}", x);
            x
        };
        let addr = match (header & 0xF0) >> 4 {
            // base
            0b0000 | 0b0001 | 0b0010 | 0b0011 => {
                AddrType::Base(BaseAddress::new(network, read_addr_cred(4, 1), read_addr_cred(5, 1 + hash_len)))
            },
            // pointer
            0b0100 | 0b0101 => {
                let mut byte_index = 1;
                let payment_cred = read_addr_cred(4, 1);
                byte_index += hash_len;
                let (slot, slot_bytes) = variable_nat_decode(&data[byte_index..])
                    .ok_or(DeserializeError::new("Address.slot", DeserializeFailure::VariableLenNatDecodeFailed))?;
                byte_index += slot_bytes;
                let (tx_index, tx_bytes) = variable_nat_decode(&data[byte_index..])
                    .ok_or(DeserializeError::new("Address.tx_index", DeserializeFailure::VariableLenNatDecodeFailed))?;
                byte_index += tx_bytes;
                let (cert_index, cert_bytes) = variable_nat_decode(&data[byte_index..])
                    .ok_or(DeserializeError::new("Address.cert_index", DeserializeFailure::VariableLenNatDecodeFailed))?;
                byte_index += cert_bytes;
                // TODO: check trailing bytes or not? or return how many were read?
                AddrType::Ptr(PointerAddress::new(network, payment_cred, Pointer::new(slot, tx_index, cert_index)))
            },
            // enterprise
            0b0110 | 0b0111 => {
                AddrType::Enterprise(EnterpriseAddress::new(network, read_addr_cred(4, 1)))
            },
            // byron
            0b1000 => {
                unimplemented!()
            },
            // TODO: return error
            _ => unimplemented!(),
        };
        Ok(Address(addr))
    }
}

impl cbor_event::se::Serialize for Address {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(self.to_bytes())
    }
}

impl Deserialize for Address {
    fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Self::from_bytes_impl(raw.bytes()?)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct BaseAddress {
    network: u8,
    payment: AddrCred,
    stake: AddrCred,
}

#[wasm_bindgen]
impl BaseAddress {
    pub fn new(network: u8, payment: AddrCred, stake: AddrCred) -> Self {
        Self {
            network,
            payment,
            stake,
        }
    }

    pub fn payment_cred(&self) -> AddrCred {
        self.payment.clone()
    }

    pub fn stake_cred(&self) -> AddrCred {
        self.stake.clone()
    }

    pub fn to_address(&self) -> Address {
        Address(AddrType::Base(self.clone()))
    }
}


#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct EnterpriseAddress {
    network: u8,
    payment: AddrCred,
}

#[wasm_bindgen]
impl EnterpriseAddress {
    pub fn new(network: u8, payment: AddrCred) -> Self {
        Self {
            network,
            payment,
        }
    }

    pub fn payment_cred(&self) -> AddrCred {
        self.payment.clone()
    }

    pub fn to_address(&self) -> Address {
        Address(AddrType::Enterprise(self.clone()))
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pointer {
    slot: u64,
    tx_index: u64,
    cert_index: u64,
}

#[wasm_bindgen]
impl Pointer {
    pub fn new(slot: u64, tx_index: u64, cert_index: u64) -> Self {
        Self {
            slot,
            tx_index,
            cert_index,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PointerAddress {
    network: u8,
    payment: AddrCred,
    stake: Pointer,
}

#[wasm_bindgen]
impl PointerAddress {
    pub fn new(network: u8, payment: AddrCred, stake: Pointer) -> Self {
        Self {
            network,
            payment,
            stake,
        }
    }

    pub fn payment_cred(&self) -> AddrCred {
        self.payment.clone()
    }

    pub fn stake_ponter(&self) -> Pointer {
        self.stake.clone()
    }

    pub fn to_address(&self) -> Address {
        Address(AddrType::Ptr(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            AddrCred::from_keyhash(Keyhash::new(vec![23; 28])),
            AddrCred::from_scripthash(Scripthash::new(vec![42; 28])));
        let addr = base.to_address();
        let addr2 = Address::from_bytes_impl(addr.to_bytes()).unwrap();
        assert_eq!(addr.to_bytes(), addr2.to_bytes());
    }

    #[test]
    fn ptr_serialize_consistency() {
        let ptr = PointerAddress::new(
            25,
            AddrCred::from_keyhash(Keyhash::new(vec![23; 28])),
            Pointer::new(2354556573, 127, 0));
        let addr = ptr.to_address();
        let addr2 = Address::from_bytes_impl(addr.to_bytes()).unwrap();
        assert_eq!(addr.to_bytes(), addr2.to_bytes());
    }

    #[test]
    fn enterprise_serialize_consistency() {
        let enterprise = EnterpriseAddress::new(
            64,
            AddrCred::from_keyhash(Keyhash::new(vec![23; 28])));
        let addr = enterprise.to_address();
        let addr2 = Address::from_bytes_impl(addr.to_bytes()).unwrap();
        assert_eq!(addr.to_bytes(), addr2.to_bytes());
    }
}