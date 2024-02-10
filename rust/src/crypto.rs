use crate::chain_crypto as crypto;
use crate::impl_mockchain as chain;
use bech32::ToBase32;
use cbor_event::{de::Deserializer, se::Serializer};
use chain::key;
use crypto::bech32::Bech32 as _;
use std::fmt;
use std::fmt::Display;
use std::io::{BufRead, Seek, Write};
use std::str::FromStr;

use cryptoxide::blake2b::Blake2b;

use super::*;

pub(crate) fn blake2b224(data: &[u8]) -> [u8; 28] {
    let mut out = [0; 28];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

pub(crate) fn blake2b256(data: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

// All key structs were taken from js-chain-libs:
// https://github.com/Emurgo/js-chain-libs