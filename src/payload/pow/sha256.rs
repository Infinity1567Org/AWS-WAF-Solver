use super::util::Hash;
use sha2::{Digest, Sha256};
pub struct Sha256Hasher {}
impl Hash for Sha256Hasher {
    fn hash(input: &str, salt: &str, nonce: i32) -> Option<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", input, salt, nonce).as_str());
        Some(hasher.finalize().to_vec())
    }
}
