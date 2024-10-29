use super::scrypt::ScryptHasher;
use super::sha256::Sha256Hasher;
pub enum HashType {
    Sha256,
    Scrypt,
}
pub trait Hash {
    fn hash(input: &str, salt: &str, nonce: i32) -> Option<Vec<u8>>;
}

fn hex_to_binary(hex_char: char) -> &'static str {
    match hex_char {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Invalid hex character"),
    }
}

pub fn check_difficulty(difficulty: usize, hash: &str) -> bool {
    let required_bits = difficulty / 4;
    let binary_string: String = hash
        .chars()
        .take(required_bits)
        .map(|c| hex_to_binary(c.to_ascii_uppercase()))
        .collect();
    let target: isize = isize::from_str_radix(&binary_string[..difficulty], 2).unwrap();
    target == 0
}

pub async fn pow(
    hash_type: HashType,
    input: &str,
    salt: &str,
    difficulty: usize,
) -> Result<String, String> {
    let mut nonce = 0;

    loop {
        let key_result = match hash_type {
            HashType::Scrypt => ScryptHasher::hash(input, salt, nonce),
            HashType::Sha256 => Sha256Hasher::hash(input, salt, nonce),
        };
        match key_result {
            Some(derived_key) => {
                if check_difficulty(difficulty, hex::encode(&derived_key).as_str()) {
                    return Ok(nonce.to_string());
                }
                nonce += 1;
            }
            None => return Err(String::from("Could not calculate hash")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{pow, HashType};
    #[tokio::test]
    async fn test_sha256_pow() {
        let input = "eyJ2ZXJzaW9uIjoxLCJ1YmlkIjoiNWRiNDMxMGItMWI0Yi00NzhiLWI2ODEtYzIxYTFkM2E0Mjg2IiwiYXR0ZW1wdF9pZCI6ImI0MWQxMDZmLWU0MGYtNDg0Mi1iZDdlLTFmMTUwYjBlNjZkMCIsImNyZWF0ZV90aW1lIjoiMjAyNC0wNS0yOVQyMjoyMzozNy4wNjQ3MDQ5OTJaIiwiZGlmZmljdWx0eSI6NCwiY2hhbGxlbmdlX3R5cGUiOiJIYXNoY2FzaFNjcnlwdCJ9AA8222865";
        let salt = "AA822286";
        let difficulty = 8;
        let result = pow(HashType::Sha256, input, salt, difficulty).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "133");
    }

    #[tokio::test]
    async fn test_scrypt_pow() {
        let input = "eyJ2ZXJzaW9uIjoxLCJ1YmlkIjoiMGNkY2VlMzItY2E3Ni00ZTQ3LWJlNmYtNzRjMWFjMGNiMTY0IiwiYXR0ZW1wdF9pZCI6IjcyYTI4NDlhLTAxZjctNGZlMS1iMzMyLTIyODk3OGVmODVhYSIsImNyZWF0ZV90aW1lIjoiMjAyNC0wNS0zMFQwMDo1Nzo0OS4wODc4MDgzMTJaIiwiZGlmZmljdWx0eSI6NCwiY2hhbGxlbmdlX3R5cGUiOiJIYXNoY2FzaFNjcnlwdCJ9";
        let salt = "3763231C";
        let difficulty = 8;
        let result = pow(HashType::Scrypt, input, salt, difficulty).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "371");
    }
}
