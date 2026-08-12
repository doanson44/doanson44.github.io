//! Base64 encoding and decoding for UTF-8 text.

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encode UTF-8 text as standard Base64.
pub fn encode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut output = String::with_capacity(bytes.len().div_ceil(3) * 4);

    for chunk in bytes.chunks(3) {
        let first = chunk[0] as usize;
        let second = chunk.get(1).copied().unwrap_or(0) as usize;
        let third = chunk.get(2).copied().unwrap_or(0) as usize;

        output.push(ALPHABET[first >> 2] as char);
        output.push(ALPHABET[((first & 0x03) << 4) | (second >> 4)] as char);
        output.push(if chunk.len() > 1 {
            ALPHABET[((second & 0x0f) << 2) | (third >> 6)] as char
        } else {
            '='
        });
        output.push(if chunk.len() > 2 {
            ALPHABET[third & 0x3f] as char
        } else {
            '='
        });
    }

    output
}

/// Decode standard Base64 into UTF-8 text.
///
/// # Errors
/// Returns an error when the input has invalid length, invalid characters,
/// invalid padding, or does not contain valid UTF-8 bytes.
pub fn decode(input: &str) -> Result<String, String> {
    let compact: Vec<u8> = input
        .bytes()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect();
    if compact.is_empty() {
        return Ok(String::new());
    }
    if !compact.len().is_multiple_of(4) {
        return Err("Invalid Base64: length must be a multiple of 4.".into());
    }

    let mut bytes = Vec::with_capacity(compact.len() / 4 * 3);
    for (index, chunk) in compact.chunks(4).enumerate() {
        let is_last = index == compact.len() / 4 - 1;
        let first = value(chunk[0])?;
        let second = value(chunk[1])?;
        let third = if chunk[2] == b'=' {
            0
        } else {
            value(chunk[2])?
        };
        let fourth = if chunk[3] == b'=' {
            0
        } else {
            value(chunk[3])?
        };

        if chunk[0] == b'=' || chunk[1] == b'=' {
            return Err("Invalid Base64: padding appears too early.".into());
        }
        if chunk[2] == b'=' && chunk[3] != b'=' {
            return Err("Invalid Base64: invalid padding.".into());
        }
        if !is_last && (chunk[2] == b'=' || chunk[3] == b'=') {
            return Err("Invalid Base64: padding is only allowed in the final group.".into());
        }
        if chunk[2] == b'=' && second & 0x0f != 0 {
            return Err("Invalid Base64: non-zero padding bits.".into());
        }
        if chunk[3] == b'=' && chunk[2] != b'=' && third & 0x03 != 0 {
            return Err("Invalid Base64: non-zero padding bits.".into());
        }

        bytes.push(first << 2 | second >> 4);
        if chunk[2] != b'=' {
            bytes.push((second & 0x0f) << 4 | third >> 2);
        }
        if chunk[3] != b'=' {
            bytes.push((third & 0x03) << 6 | fourth);
        }
    }

    String::from_utf8(bytes).map_err(|_| "Decoded Base64 is not valid UTF-8 text.".into())
}

fn value(byte: u8) -> Result<u8, String> {
    match byte {
        b'A'..=b'Z' => Ok(byte - b'A'),
        b'a'..=b'z' => Ok(byte - b'a' + 26),
        b'0'..=b'9' => Ok(byte - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(format!("Invalid Base64 character: '{}'.", byte as char)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_ascii() {
        assert_eq!(encode("Hello, world!"), "SGVsbG8sIHdvcmxkIQ==");
    }

    #[test]
    fn encodes_utf8() {
        assert_eq!(encode("Xin chào, Son!"), "WGluIGNow6BvLCBTb24h");
    }

    #[test]
    fn decodes_ascii() {
        assert_eq!(decode("SGVsbG8sIHdvcmxkIQ==").unwrap(), "Hello, world!");
    }

    #[test]
    fn decodes_utf8() {
        assert_eq!(decode("WGluIGNow6BvLCBTb24h").unwrap(), "Xin chào, Son!");
    }

    #[test]
    fn ignores_whitespace() {
        assert_eq!(decode("SGVs\nbG8=").unwrap(), "Hello");
    }

    #[test]
    fn rejects_invalid_input() {
        assert!(decode("not base64!").is_err());
        assert!(decode("SGVsbG8").is_err());
    }
}
