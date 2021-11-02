//! HD Payload
//!
//! The HD Payload is an Address attribute stored along the address
//! in encrypted form.
//!
//! This use chacha20poly1305 to auth-encrypt a BIP39 derivation
//! path, which is then stored in the address. The owner of the
//! symmetric key used to encrypt, can then decrypt the address
//! payload and find the derivation path associated with it.
//!
use cryptoxide::chacha20poly1305::ChaCha20Poly1305;
use cryptoxide::hmac::Hmac;
use cryptoxide::pbkdf2::pbkdf2;
use cryptoxide::sha2::Sha512;
use cryptoxide::ed25519;
use cryptoxide::mac::Mac;
use cryptoxide::curve25519::{sc_reduce};

use std::{
    fmt,
    io::{BufRead, Write},
    ops::Deref,
};

use cbor_event::{
    self,
    de::Deserializer,
    se::{self, Serializer},
    cbor
};

use crate::chain_crypto as crypto;
use crate::chain_crypto::algorithms::legacy_daedalus::{SEED_SIZE};
use ed25519_bip32::{XPRV_SIZE};

const NONCE: &'static [u8] = b"serokellfore";
const SALT: &'static [u8] = b"address-hashing";
const TAG_LEN: usize = 16;

#[derive(Debug)]
pub enum Error {
    InvalidHDKeySize(usize),
    CannotDecrypt,
    NotEnoughEncryptedData,
    /// this relates to the issue that addresses with the payload data
    /// can have an infinite length (as long as it fits in the max block size
    /// and max transaction size).
    PayloadIsTooLarge(usize),
    CborError(cbor_event::Error),
}
impl From<cbor_event::Error> for Error {
    fn from(e: cbor_event::Error) -> Self {
        Error::CborError(e)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidHDKeySize(sz) => {
                write!(f, "Invalid size for an HDKey, expecting {} bytes", sz)
            }
            Error::CannotDecrypt => write!(f, "Cannot decrypt HDPayload with given HDKey"),
            Error::NotEnoughEncryptedData => {
                write!(f, "Invalid HDPayload, expecting at least {} bytes", TAG_LEN)
            }
            Error::CborError(_) => write!(f, "HDPayload decrypted but invalid value"),
            Error::PayloadIsTooLarge(len) => write!(
                f,
                "HDPayload is too large to be valid. Its size {} is beyond the max size ({} bytes)",
                len, MAX_PAYLOAD_SIZE
            ),
        }
    }
}
impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&::std::error::Error> {
        match self {
            Error::CborError(ref err) => Some(err),
            _ => None,
        }
    }
}

/// This is the max size we accept to try to decrypt a HDPayload.
/// This is due to avoid trying to decrypt content that are way beyond
/// reasonable size.
pub const MAX_PAYLOAD_SIZE: usize = 48;

pub type Result<T> = ::std::result::Result<T, Error>;

/// A derivation path of HD wallet derivation indices which uses a CBOR encoding
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "generic-serialization", derive(Serialize, Deserialize))]
pub struct Path(Vec<u32>);
impl Deref for Path {
    type Target = [u32];
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl AsRef<[u32]> for Path {
    fn as_ref(&self) -> &[u32] {
        self.0.as_ref()
    }
}
impl Path {
    pub fn new(v: Vec<u32>) -> Self {
        Path(v)
    }
    fn from_cbor(bytes: &[u8]) -> Result<Self> {
        let cursor = std::io::Cursor::new(bytes);
        let mut raw = Deserializer::from(cursor);
        Ok(cbor_event::de::Deserialize::deserialize(&mut raw)?)
    }
    fn cbor(&self) -> Vec<u8> {
        cbor!(self).expect("Serialize the given Path in cbor")
    }
}
impl cbor_event::se::Serialize for Path {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        se::serialize_indefinite_array(self.0.iter(), serializer)
    }
}
impl cbor_event::Deserialize for Path {
    fn deserialize<R: BufRead>(reader: &mut Deserializer<R>) -> cbor_event::Result<Self> {
        Ok(Path(reader.deserialize()?))
    }
}

pub const HDKEY_SIZE: usize = 32;

/// The key to encrypt and decrypt HD payload
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "generic-serialization", derive(Serialize, Deserialize))]
pub struct HDKey([u8; HDKEY_SIZE]);
impl AsRef<[u8]> for HDKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
impl HDKey {
    /// Create a new `HDKey` from an extended public key
    pub fn new(root_pub: &crypto::PublicKey::<crypto::LegacyDaedalus>) -> Self {
        let mut mac = Hmac::new(Sha512::new(), root_pub.as_ref());
        let mut result = [0; HDKEY_SIZE];
        let iters = 500;
        pbkdf2(&mut mac, &SALT[..], iters, &mut result);
        HDKey(result)
    }

    /// create a `HDKey` by taking ownership of the given bytes
    pub fn from_bytes(bytes: [u8; HDKEY_SIZE]) -> Self {
        HDKey(bytes)
    }
    /// create a `HDKey` from the given slice
    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        if bytes.len() == HDKEY_SIZE {
            let mut v = [0u8; HDKEY_SIZE];
            v[0..HDKEY_SIZE].clone_from_slice(bytes);
            Ok(HDKey::from_bytes(v))
        } else {
            Err(Error::InvalidHDKeySize(bytes.len()))
        }
    }

    pub fn encrypt(&self, input: &[u8]) -> Vec<u8> {
        let mut ctx = ChaCha20Poly1305::new(self.as_ref(), &NONCE[..], &[]);

        let len = input.len();

        let mut out: Vec<u8> = vec![0; len];
        let mut tag = [0; TAG_LEN];

        ctx.encrypt(&input, &mut out[0..len], &mut tag);
        out.extend_from_slice(&tag[..]);
        out
    }

    pub fn decrypt(&self, input: &[u8]) -> Result<Vec<u8>> {
        if input.len() <= TAG_LEN {
            return Err(Error::NotEnoughEncryptedData);
        };
        let len = input.len() - TAG_LEN;
        if len >= MAX_PAYLOAD_SIZE {
            return Err(Error::PayloadIsTooLarge(len));
        }

        let mut ctx = ChaCha20Poly1305::new(self.as_ref(), &NONCE[..], &[]);

        let mut out: Vec<u8> = vec![0; len];

        if ctx.decrypt(&input[..len], &mut out[..], &input[len..]) {
            Ok(out)
        } else {
            Err(Error::CannotDecrypt)
        }
    }

    pub fn encrypt_path(&self, derivation_path: &Path) -> HDAddressPayload {
        let input = derivation_path.cbor();
        let out = self.encrypt(&input);

        HDAddressPayload::from_vec(out)
    }

    pub fn decrypt_path(&self, payload: &HDAddressPayload) -> Result<Path> {
        let out = self.decrypt(payload.as_ref())?;
        Path::from_cbor(&out)
    }
}

/// The address attributes payload, that should contains an encrypted derivation path with a MAC tag
///
/// It's however possible to store anything in this attributes, including
/// non encrypted information.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "generic-serialization", derive(Serialize, Deserialize))]
pub struct HDAddressPayload(Vec<u8>);
impl AsRef<[u8]> for HDAddressPayload {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
impl HDAddressPayload {
    pub fn from_vec(v: Vec<u8>) -> Self {
        HDAddressPayload(v)
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        HDAddressPayload::from_vec(bytes.iter().cloned().collect())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
impl cbor_event::se::Serialize for HDAddressPayload {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        se::serialize_cbor_in_cbor(self.0.as_slice(), serializer)
    }
}
impl cbor_event::de::Deserialize for HDAddressPayload {
    fn deserialize<R: BufRead>(reader: &mut Deserializer<R>) -> cbor_event::Result<Self> {
        let inner_cbor = reader.bytes()?;
        let inner_cbor = std::io::Cursor::new(inner_cbor);
        let mut inner_cbor = Deserializer::from(inner_cbor);
        Ok(HDAddressPayload::from_bytes(&mut inner_cbor.bytes()?))
    }
}
impl Deref for HDAddressPayload {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
impl fmt::Debug for HDAddressPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt() {
        let bytes = vec![42u8; MAX_PAYLOAD_SIZE - 1];
        let sk = crypto::derive::legacy_daedalus_from_bip39_entropy(&[0; SEED_SIZE]).unwrap();
        let pk = sk.to_public();

        let key = HDKey::new(&pk);
        let payload = key.encrypt(&bytes);
        assert_eq!(bytes, key.decrypt(&payload).unwrap())
    }

    #[test]
    fn decrypt_too_small() {
        const TOO_SMALL_PAYLOAD: usize = TAG_LEN - 1;
        let bytes = vec![42u8; TOO_SMALL_PAYLOAD];
        let sk = crypto::derive::legacy_daedalus_from_bip39_entropy(&[0; SEED_SIZE]).unwrap();
        let pk = sk.to_public();

        let key = HDKey::new(&pk);
        match key.decrypt(&bytes).unwrap_err() {
            Error::NotEnoughEncryptedData => {}
            err => assert!(
                false,
                "expecting Error::NotEnoughEncryptedData but got {:#?}",
                err
            ),
        }
    }
    #[test]
    fn decrypt_too_large() {
        const TOO_LARGE_PAYLOAD: usize = 2 * MAX_PAYLOAD_SIZE;
        let bytes = vec![42u8; TOO_LARGE_PAYLOAD];
        let sk = crypto::derive::legacy_daedalus_from_bip39_entropy(&[0; SEED_SIZE]).unwrap();
        let pk = sk.to_public();

        let key = HDKey::new(&pk);
        match key.decrypt(&bytes).unwrap_err() {
            Error::PayloadIsTooLarge(len) => assert_eq!(len, TOO_LARGE_PAYLOAD - TAG_LEN),
            err => assert!(
                false,
                "expecting Error::PayloadIsTooLarge({}) but got {:#?}",
                TOO_LARGE_PAYLOAD - TAG_LEN,
                err
            ),
        }
    }

    #[test]
    fn path_cbor_encoding() {
        let path = Path::new(vec![0, 1, 2]);
        let cbor = path.cbor();
        assert_eq!(path, Path::from_cbor(cbor.as_ref()).unwrap());
    }

    #[test]
    fn hdpayload() {
        let path = Path::new(vec![0, 1, 2]);
        let sk = crypto::derive::legacy_daedalus_from_bip39_entropy(&[0; SEED_SIZE]).unwrap();
        let pk = sk.to_public();

        let key = HDKey::new(&pk);
        let payload = key.encrypt_path(&path);
        assert_eq!(path, key.decrypt_path(&payload).unwrap())
    }

    #[test]
    fn unit1() {
        let key = HDKey::from_bytes([0u8; 32]);
        let dat = [0x9f, 0x00, 0x01, 0x0ff];
        let expected = [
            0xda, 0xac, 0x4a, 0x55, 0xfc, 0xa7, 0x48, 0xf3, 0x2f, 0xfa, 0xf4, 0x9e, 0x2b, 0x41,
            0xab, 0x86, 0xf3, 0x54, 0xdb, 0x96,
        ];
        let got = key.encrypt(&dat[..]);
        assert_eq!(&expected[..], &got[..])
    }

    #[test]
    fn unit2() {
        let path = Path::new(vec![0, 1]);
        let expected = [0x9f, 0x00, 0x01, 0x0ff];
        let cbor = path.cbor();
        assert_eq!(&expected[..], &cbor[..])
    }

    struct GoldenTest {
        xprv_key: [u8; XPRV_SIZE],
        hdkey: [u8; HDKEY_SIZE],
        payload: &'static [u8],
        addressing: [u32; 2],
    }

    const GOLDEN_TESTS: &'static [GoldenTest] = &[
        GoldenTest {
            xprv_key: [
                32, 15, 90, 64, 107, 113, 208, 132, 181, 199, 158, 192, 82, 246, 119, 189, 80, 23,
                31, 95, 219, 198, 94, 39, 18, 166, 174, 186, 139, 177, 243, 82, 202, 175, 171, 241,
                217, 208, 101, 229, 20, 60, 84, 114, 214, 1, 73, 40, 25, 142, 239, 22, 239, 146,
                66, 82, 121, 206, 22, 120, 24, 45, 126, 66, 208, 108, 114, 200, 223, 219, 60, 98,
                75, 118, 2, 56, 104, 230, 68, 215, 229, 31, 241, 136, 165, 71, 176, 231, 189, 125,
                179, 211, 163, 66, 186, 210,
            ],
            hdkey: [
                96, 3, 72, 241, 97, 26, 53, 38, 110, 107, 149, 105, 139, 250, 203, 125, 73, 152,
                12, 195, 158, 54, 84, 69, 99, 239, 234, 122, 177, 179, 59, 200,
            ],
            payload: &[
                0x33, 0x1c, 0xd6, 0xc3, 0x02, 0x5d, 0x59, 0xa1, 0x6a, 0x5f, 0x82, 0x9e, 0xd7, 0xf2,
                0x4c, 0xf8, 0x74, 0xf3, 0xab, 0x50,
            ],
            addressing: [0, 0],
        },
        GoldenTest {
            xprv_key: [
                32, 15, 90, 64, 107, 113, 208, 132, 181, 199, 158, 192, 82, 246, 119, 189, 80, 23,
                31, 95, 219, 198, 94, 39, 18, 166, 174, 186, 139, 177, 243, 82, 202, 175, 171, 241,
                217, 208, 101, 229, 20, 60, 84, 114, 214, 1, 73, 40, 25, 142, 239, 22, 239, 146,
                66, 82, 121, 206, 22, 120, 24, 45, 126, 66, 208, 108, 114, 200, 223, 219, 60, 98,
                75, 118, 2, 56, 104, 230, 68, 215, 229, 31, 241, 136, 165, 71, 176, 231, 189, 125,
                179, 211, 163, 66, 186, 210,
            ],
            hdkey: [
                96, 3, 72, 241, 97, 26, 53, 38, 110, 107, 149, 105, 139, 250, 203, 125, 73, 152,
                12, 195, 158, 54, 84, 69, 99, 239, 234, 122, 177, 179, 59, 200,
            ],
            payload: &[
                0x33, 0x06, 0x56, 0x3c, 0x02, 0xd0, 0x2f, 0x38, 0x1e, 0x78, 0xdf, 0x84, 0x04, 0xc3,
                0x50, 0x56, 0x76, 0xd5, 0x5e, 0x45, 0x71, 0x93, 0xe7, 0x4a, 0x34, 0xb6, 0x90, 0xec,
            ],
            addressing: [0x80000000, 0x80000000],
        },
    ];

    fn run_golden_test(golden_test: &GoldenTest) {
        let xprv = crypto::SecretKey::<crypto::LegacyDaedalus>::from_binary(&golden_test.xprv_key).unwrap();
        let hdkey = HDKey::from_bytes(golden_test.hdkey);
        let payload = HDAddressPayload::from_bytes(golden_test.payload);
        let path = Path::new(Vec::from(&golden_test.addressing[..]));

        let our_hdkey = HDKey::new(&xprv.to_public());
        assert_eq!(hdkey, our_hdkey);

        let our_payload = hdkey.encrypt_path(&path);
        assert_eq!(payload, our_payload);

        let our_path = hdkey.decrypt_path(&payload).unwrap();
        assert_eq!(path, our_path);
    }

    #[test]
    fn golden_tests() {
        for golden_test in GOLDEN_TESTS {
            run_golden_test(golden_test)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "generic-serialization", derive(Serialize, Deserialize))]
enum DerivationType {
    Soft(u32),
    Hard(u32),
}

fn to_type(index: u32) -> DerivationType {
    if index >= 0x80000000 {
        DerivationType::Hard(index)
    } else {
        DerivationType::Soft(index)
    }
}

pub fn derive_sk_legacy_daedalus(xprv: &crypto::SecretKey<crypto::LegacyDaedalus>, index: u32) -> crypto::SecretKey<crypto::LegacyDaedalus> {
    fn be32(i: u32) -> [u8; 4] {
        [(i >> 24) as u8, (i >> 16) as u8, (i >> 8) as u8, i as u8]
    }

    fn add_28_mul8_v1(x: &[u8], y: &[u8]) -> [u8; 32] {
        assert!(x.len() == 32);
        assert!(y.len() == 32);
    
        let yfe8 = {
            let mut acc = 0;
            let mut out = [0u8; 64];
            for i in 0..32 {
                out[i] = (y[i] << 3) + (acc & 0x8);
                acc = y[i] >> 5;
            }
            out
        };
    
        let mut r32 = [0u8; 32];
        let mut r = [0u8; 64];
        let mut carry = 0u16;
        for i in 0..32 {
            let v = x[i] as u16 + yfe8[i] as u16 + carry;
            r[i] = v as u8;
            carry = v >> 8;
        }
        if carry > 0 {
            r[32] = carry as u8;
        }
        sc_reduce(&mut r);
        r32.clone_from_slice(&r[0..32]);
        r32
    }
    
    fn add_256bits_v1(x: &[u8], y: &[u8]) -> [u8; 32] {
        assert!(x.len() == 32);
        assert!(y.len() == 32);
    
        let mut out = [0u8; 32];
        for i in 0..32 {
            let r = x[i].wrapping_add(y[i]);
            out[i] = r;
        }
        out
    }

    fn mk_xprv(out: &mut [u8; XPRV_SIZE], kl: &[u8], kr: &[u8], cc: &[u8]) {
        assert!(kl.len() == 32);
        assert!(kr.len() == 32);
        assert!(cc.len() == 32);
    
        out[0..32].clone_from_slice(kl);
        out[32..64].clone_from_slice(kr);
        out[64..96].clone_from_slice(cc);
    }

    fn mk_public_key(extended_secret: &[u8]) -> [u8; 32] {
        assert!(extended_secret.len() == 64);
        ed25519::to_public(extended_secret)
    }
    
    /*
     * If so (hardened child):
     *    let Z = HMAC-SHA512(Key = cpar, Data = 0x00 || ser256(left(kpar)) || ser32(i)).
     *    let I = HMAC-SHA512(Key = cpar, Data = 0x01 || ser256(left(kpar)) || ser32(i)).
     * If not (normal child):
     *    let Z = HMAC-SHA512(Key = cpar, Data = 0x02 || serP(point(kpar)) || ser32(i)).
     *    let I = HMAC-SHA512(Key = cpar, Data = 0x03 || serP(point(kpar)) || ser32(i)).
     **/

    let ekey = &xprv.as_ref()[0..64];
    let kl = &ekey[0..32];
    let kr = &ekey[32..64];
    let chaincode = &xprv.as_ref()[64..96];

    let mut zmac = Hmac::new(Sha512::new(), &chaincode);
    let mut imac = Hmac::new(Sha512::new(), &chaincode);
    let seri = be32(index);
    match to_type(index) {
        DerivationType::Soft(_) => {
            let pk = mk_public_key(ekey);
            zmac.input(&[0x2]);
            zmac.input(&pk);
            zmac.input(&seri);
            imac.input(&[0x3]);
            imac.input(&pk);
            imac.input(&seri);
        }
        DerivationType::Hard(_) => {
            zmac.input(&[0x0]);
            zmac.input(ekey);
            zmac.input(&seri);
            imac.input(&[0x1]);
            imac.input(ekey);
            imac.input(&seri);
        }
    };

    let mut zout = [0u8; 64];
    zmac.raw_result(&mut zout);
    let zl = &zout[0..32];
    let zr = &zout[32..64];

    // left = kl + 8 * trunc28(zl)
    let left = add_28_mul8_v1(kl, zl);
    // right = zr + kr
    let right = add_256bits_v1(kr, zr);

    let mut iout = [0u8; 64];
    imac.raw_result(&mut iout);
    let cc = &iout[32..];

    let mut out = [0u8; XPRV_SIZE];
    mk_xprv(&mut out, &left, &right, cc);

    imac.reset();
    zmac.reset();

    crypto::SecretKey::from_binary(&out).unwrap()
}

#[cfg(test)]
#[cfg(feature = "with-bench")]
mod bench {
    use test;

    #[bench]
    fn decrypt_fail(b: &mut test::Bencher) {
        let path = Path::new(vec![0, 1]);
        let sk = crypto::derive::legacy_daedalus_from_bip39_entropy(&[0; SEED_SIZE]).unwrap();
        let pk = sk.to_public();

        let key = HDKey::new(&pk);
        let payload = key.encrypt_path(&path);

        let sk = crypto::derive::legacy_daedalus_from_bip39_entropy(&[1; SEED_SIZE]).unwrap();
        let pk = sk.to_public();
        let key = HDKey::new(&pk);
        b.iter(|| {
            let _ = key.decrypt(&payload);
        })
    }

    #[bench]
    fn decrypt_ok(b: &mut test::Bencher) {
        let path = Path::new(vec![0, 1]);
        let sk = crypto::derive::legacy_daedalus_from_bip39_entropy(&[0; SEED_SIZE]).unwrap();
        let pk = sk.to_public();

        let key = HDKey::new(&pk);
        let payload = key.encrypt_path(&path);

        b.iter(|| {
            let _ = key.decrypt(&payload);
        })
    }

    #[bench]
    fn decrypt_with_cbor(b: &mut test::Bencher) {
        let path = Path::new(vec![0, 1]);
        let sk = crypto::derive::legacy_daedalus_from_bip39_entropy(&[0; SEED_SIZE]).unwrap();
        let pk = sk.to_public();

        let key = HDKey::new(&pk);
        let payload = key.encrypt_path(&path);

        b.iter(|| {
            let _ = key.decrypt_path(&payload);
        })
    }
}
