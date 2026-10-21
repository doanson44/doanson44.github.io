use serde::{Deserialize, Serialize};

/// All account-manager data is stored locally by the browser adapter.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub environments: Vec<Environment>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub url: String,
    pub description: String,
    pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub otp_mode: OtpMode,
    pub otp_secret: String,
    pub default_otp: String,
    pub notes: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OtpMode {
    #[default]
    None,
    Totp,
    Default,
}

/// Generates a six-digit RFC 6238 TOTP using HMAC-SHA1 and a 30-second period.
pub fn generate_totp(secret: &str, unix_seconds: u64) -> Result<String, String> {
    let key = decode_base32(secret)?;
    if key.is_empty() {
        return Err("Enter a Base32 secret key.".into());
    }

    let counter = unix_seconds / 30;
    let digest = hmac_sha1(&key, &counter.to_be_bytes());
    let offset = (digest[digest.len() - 1] & 0x0f) as usize;
    let binary = ((u32::from(digest[offset]) & 0x7f) << 24)
        | (u32::from(digest[offset + 1]) << 16)
        | (u32::from(digest[offset + 2]) << 8)
        | u32::from(digest[offset + 3]);
    Ok(format!("{:06}", binary % 1_000_000))
}

fn decode_base32(input: &str) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0u8;
    for byte in input.bytes().filter(|byte| *byte != b'=' && !byte.is_ascii_whitespace()) {
        let value = match byte.to_ascii_uppercase() {
            b'A'..=b'Z' => byte.to_ascii_uppercase() - b'A',
            b'2'..=b'7' => byte - b'2' + 26,
            _ => return Err("Secret key must use Base32 characters A-Z and 2-7.".into()),
        };
        buffer = (buffer << 5) | u32::from(value);
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            output.push((buffer >> bits) as u8);
            buffer &= (1u32 << bits).wrapping_sub(1);
        }
    }
    Ok(output)
}

/// Encodes arbitrary bytes as an unpadded Base32 secret.
pub fn encode_base32(input: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut output = String::new();
    let mut buffer = 0u32;
    let mut bits = 0u8;
    for byte in input {
        buffer = (buffer << 8) | u32::from(*byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            output.push(ALPHABET[((buffer >> bits) & 0x1f) as usize] as char);
        }
    }
    if bits > 0 {
        output.push(ALPHABET[((buffer << (5 - bits)) & 0x1f) as usize] as char);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::generate_totp;

    #[test]
    fn generates_rfc6238_six_digit_totp() {
        // Base32 for "12345678901234567890"; RFC vector at 59 seconds uses SHA-1.
        assert_eq!(generate_totp("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", 59).unwrap(), "287082");
    }

    #[test]
    fn rejects_invalid_secret_characters() {
        assert!(generate_totp("INVALID0", 0).is_err());
    }
}
