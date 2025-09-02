use std::io::Cursor;

use crate::{decode_variable_length_u64, encode_variable_length, Pointer};

#[test]
fn test_valid_pointer() {
    // Create a pointer with normal values
    let ptr = Pointer {
        slot: 12345u64.into(),
        tx_index: 67u64.into(),
        cert_index: 89u64.into(),
    };
    let encoded = ptr.to_bytes();

    let (result, _offset) = Pointer::from_bytes(&encoded).unwrap();
    
    assert_eq!(result.slot_bignum(), 12345u64.into());
    assert_eq!(result.tx_index_bignum(), 67u64.into());
    assert_eq!(result.cert_index_bignum(), 89u64.into());
}

#[test]
fn test_overflow_normalization() {
    // Create data with very large numbers that should trigger normalization
    let garbage_data = vec![
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F, // Huge number
        0x64, // 100
        0x59, // 89
    ];

    let (result, _offset) = Pointer::from_bytes(&garbage_data).unwrap();

    // Should be normalized to (0,0,0)
    assert_eq!(result.slot_bignum(), 0u64.into());
    assert_eq!(result.tx_index_bignum(), 0u64.into());
    assert_eq!(result.cert_index_bignum(), 0u64.into());
}

#[test]
fn test_overflow_without_normalization() {
    // Create data with very large numbers that should not trigger normalization bacuase after u64 overflow it would be less than u32::MAX
    let garbage_data = vec![
        0xFF, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F, // Huge number
        0x64, // 100
        0x59, // 89
    ];

    let (result, _offset) = Pointer::from_bytes(&garbage_data).unwrap();

    // Should be normalized to (0,0,0)
    assert_eq!(result.slot_bignum(), 0x7Fu64.into());
    assert_eq!(result.tx_index_bignum(), 100u64.into());
    assert_eq!(result.cert_index_bignum(), 89u64.into());
}

#[test]
fn test_large_values_normalization() {
    // Value larger than u32::MAX should trigger normalization
    let large_slot = (u32::MAX as u64) + 1000;
    let mut data = encode_variable_length(large_slot);
    data.extend_from_slice(&encode_variable_length(100));
    data.extend_from_slice(&encode_variable_length(200));

    let (result, _offset) = Pointer::from_bytes(&data).unwrap();

    // Should be normalized to (0,0,0) due to overflow
    assert_eq!(result.slot_bignum(), 0u64.into());
    assert_eq!(result.tx_index_bignum(), 0u64.into());
    assert_eq!(result.cert_index_bignum(), 0u64.into());
}

#[test]
fn test_max_valid_values() {
    // Test with maximum valid values that should NOT trigger normalization
    let ptr = Pointer {
        slot: (u32::MAX as u64).into(),
        tx_index: (u16::MAX as u64).into(),
        cert_index: (u16::MAX as u64).into(),
    };
    let encoded = ptr.to_bytes();

    let (result, _offset) = Pointer::from_bytes(&encoded).unwrap();
    
    assert_eq!(result.slot_bignum(), (u32::MAX as u64).into());
    assert_eq!(result.tx_index_bignum(), (u16::MAX as u64).into());
    assert_eq!(result.cert_index_bignum(), (u16::MAX as u64).into());
}

#[test]
fn test_encode_decode_roundtrip() {
    let original = Pointer {
        slot: 12345u64.into(),
        tx_index: 67u64.into(),
        cert_index: 89u64.into(),
    };
    
    let encoded = original.to_bytes();
    let (decoded, _offset) = Pointer::from_bytes(&encoded).unwrap();
    
    assert_eq!(original.slot_bignum(), decoded.slot_bignum());
    assert_eq!(original.tx_index_bignum(), decoded.tx_index_bignum());
    assert_eq!(original.cert_index_bignum(), decoded.cert_index_bignum());
}

#[test]
fn test_variable_length_encoding() {
    assert_eq!(encode_variable_length(0), vec![0]);
    assert_eq!(encode_variable_length(1), vec![1]);
    assert_eq!(encode_variable_length(127), vec![0x7F]);
    assert_eq!(encode_variable_length(128), vec![0x81, 0x00]);
    assert_eq!(encode_variable_length(255), vec![0x81, 0x7F]);
}

#[test]
fn test_variable_length_decoding() {
    let data = [0u8];
    let mut cursor = Cursor::new(&data[..]);
    assert_eq!(decode_variable_length_u64(&mut cursor, "test").unwrap(), 0);

    let data = [1u8];
    let mut cursor = Cursor::new(&data[..]);
    assert_eq!(decode_variable_length_u64(&mut cursor, "test").unwrap(), 1);

    let data = [0x7Fu8];
    let mut cursor = Cursor::new(&data[..]);
    assert_eq!(decode_variable_length_u64(&mut cursor, "test").unwrap(), 127);

    let data = [0x81u8, 0x00];
    let mut cursor = Cursor::new(&data[..]);
    assert_eq!(decode_variable_length_u64(&mut cursor, "test").unwrap(), 128);

    let data = [0x81u8, 0x7F];
    let mut cursor = Cursor::new(&data[..]);
    assert_eq!(decode_variable_length_u64(&mut cursor, "test").unwrap(), 255);
}

#[test]
fn test_insufficient_data_error() {
    let incomplete_data = vec![0x80]; // Missing terminating byte
    let result = Pointer::from_bytes(&incomplete_data);
    assert!(result.is_err());
    
    let empty_data = vec![];
    let result = Pointer::from_bytes(&empty_data);
    assert!(result.is_err());
}