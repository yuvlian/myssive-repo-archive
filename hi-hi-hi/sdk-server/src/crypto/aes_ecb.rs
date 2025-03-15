use serde::Serialize;
use soft_aes::aes::aes_enc_ecb;
use std::error::Error;

const AES_KEY: [u8; 32] = [
    54, 51, 52, 57, 102, 54, 54, 100, 54, 56, 102, 51, 54, 100, 55, 48, 57, 54, 57, 101, 97, 53,
    98, 101, 54, 98, 101, 98, 57, 97, 97, 50,
];

pub fn encrypt_ecb<T: Serialize>(data: T) -> Result<String, Box<dyn Error>> {
    let json = serde_json::to_string(&data)?;
    let encrypted = aes_enc_ecb(json.as_bytes(), &AES_KEY, Some("PKCS7"))
        .map_err(|e| format!("Encryption failed: {:?}", e))?;
    Ok(rbase64::encode(&encrypted))
}
