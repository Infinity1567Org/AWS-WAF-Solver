use aes_gcm::{
    aead::{Aead, AeadCore, Key, KeyInit, OsRng},
    Aes256Gcm,
};
use base64::prelude::*;
use crc32fast::hash;
pub struct Encryptor {}

impl Encryptor {
    pub fn calculate_checksum(json_body: &str) -> u32 {
        hash(json_body.as_bytes())
    }
    pub fn construct_payload_string(json_body: &str) -> String {
        let checksum = Self::calculate_checksum(json_body);
        let final_str = checksum.to_string() + "#" + json_body;
        final_str
    }

    pub fn encrypt(payload: &[u8], encryption_key: &str) -> String {
        let encryption_bytes = hex::decode(encryption_key).unwrap();
        let key = Key::<Aes256Gcm>::from_slice(encryption_bytes.as_slice());
        let cipher = Aes256Gcm::new(key);
        let iv = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&iv, payload).unwrap();
        let (ciphertext_bytes, tag) = ciphertext.split_at(ciphertext.len() - 16);

        BASE64_STANDARD.encode(iv.as_slice())
            + "::"
            + hex::encode(tag).as_str()
            + "::"
            + hex::encode(ciphertext_bytes).as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::Encryptor;
    use crate::payload::fingerprint::Fingerprint;
    use std::time::Instant;

    #[test]
    fn test_encryption() {
        let start = Instant::now();
        let payload = Fingerprint::new(
            String::from("https://huggingface.co/"),
            String::from(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0",
            ),
            String::from("https://huggingface.co/login"),
            r#"<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test Forms</title>
</head>
<body>
    <h1>Form Test</h1>

    <form id="form1">
        <label for="input1">Input 1:</label>
        <input type="text" id="input1" name="input1">
        <button type="submit">Submit Form 1</button>
    </form>

    <form id="form2">
        <label for="input2">Input 2:</label>
        <input type="text" id="input2" name="input2">
        <button type="submit">Submit Form 2</button>
    </form>

    <div>
        <p>This is a sample HTML document with two form elements for testing.</p>
    </div>
</body>
</html>
"#,
        );
        let serialized = serde_json::to_string(&payload).unwrap();
        // println!("{serialized}");
        let payload_string = Encryptor::construct_payload_string(&serialized);
        println!(
            "{}",
            Encryptor::encrypt(
                payload_string.as_bytes(),
                "93d9f6846b629edb2bdc4466af627d998496cb0c08f9cf043de68d6b25aa9693"
            )
        );
        println!("Time : {:?}", start.elapsed());
    }
}
