use super::util::Hash;
use scrypt::{scrypt, ScryptParams};

pub struct ScryptHasher {}

impl Hash for ScryptHasher {
    fn hash(input: &str, salt: &str, nonce: i32) -> Option<Vec<u8>> {
        let combined_input = format!("{}{}{}", input, salt, nonce);
        let mut derived_key = vec![0u8; 10];
        let params = ScryptParams::new(7, 8, 1).unwrap();
        let result = scrypt(
            combined_input.as_bytes(),
            salt.as_bytes(),
            &params,
            &mut derived_key,
        );

        match result {
            Ok(_) => Some(derived_key),

            Err(_) => {
                return None;
            }
        }
    }
}
