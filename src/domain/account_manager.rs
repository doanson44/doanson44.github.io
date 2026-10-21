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


/// Computes HMAC-SHA1 without relying on browser APIs or external crypto crates.
fn hmac_sha1(key: &[u8], message: &[u8]) -> [u8; 20] {
    const BLOCK_SIZE: usize = 64;
    let mut normalized_key = [0u8; BLOCK_SIZE];
    if key.len() > BLOCK_SIZE {
        normalized_key[..20].copy_from_slice(&sha1(key));
    } else {
        normalized_key[..key.len()].copy_from_slice(key);
    }

    let mut inner = Vec::with_capacity(BLOCK_SIZE + message.len());
    let mut outer = Vec::with_capacity(BLOCK_SIZE + 20);
    for byte in normalized_key {
        inner.push(byte ^ 0x36);
        outer.push(byte ^ 0x5c);
    }
    inner.extend_from_slice(message);
    outer.extend_from_slice(&sha1(&inner));
    sha1(&outer)
}

/// SHA-1 digest implementation used only by the RFC 6238 TOTP helper.
fn sha1(input: &[u8]) -> [u8; 20] {
    let bit_len = (input.len() as u64).wrapping_mul(8);
    let mut message = Vec::with_capacity(input.len() + 72);
    message.extend_from_slice(input);
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_be_bytes());

    let mut h0 = 0x67452301u32;
    let mut h1 = 0xEFCDAB89u32;
    let mut h2 = 0x98BADCFEu32;
    let mut h3 = 0x10325476u32;
    let mut h4 = 0xC3D2E1F0u32;

    for chunk in message.chunks_exact(64) {
        let mut words = [0u32; 80];
        for (index, bytes) in chunk.chunks_exact(4).enumerate() {
            words[index] = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        }
        for index in 16..80 {
            words[index] = (words[index - 3] ^ words[index - 8] ^ words[index - 14] ^ words[index - 16]).rotate_left(1);
        }

        let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);
        for (index, word) in words.iter().enumerate() {
            let (function, constant) = match index {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let temp = a.rotate_left(5)
                .wrapping_add(function)
                .wrapping_add(e)
                .wrapping_add(constant)
                .wrapping_add(*word);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut digest = [0u8; 20];
    for (chunk, word) in digest.chunks_exact_mut(4).zip([h0, h1, h2, h3, h4]) {
        chunk.copy_from_slice(&word.to_be_bytes());
    }
    digest
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
