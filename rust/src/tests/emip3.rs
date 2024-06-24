use crate::*;

#[test]
fn encryption() {
    let password = String::from("70617373776f7264");
    let salt = String::from("50515253c0c1c2c3c4c5c6c750515253c0c1c2c3c4c5c6c750515253c0c1c2c3");
    let nonce = String::from("50515253c0c1c2c3c4c5c6c7");
    let data = String::from("736f6d65206461746120746f20656e6372797074");
    let encrypted_data = encrypt_with_password(&password, &salt, &nonce, &data).unwrap();
    let decrypted_data = decrypt_with_password(&password, &encrypted_data).unwrap();
    assert_eq!(data, decrypted_data);
}
