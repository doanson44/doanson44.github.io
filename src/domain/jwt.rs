use serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct DecodedJwt {
    pub header: Value,
    pub payload: Value,
    pub signature: String,
}

pub fn decode_jwt(source: &str) -> Result<DecodedJwt, String> {
    let token = source.trim();
    let mut parts = token.split('.');
    let header_part = parts.next().unwrap_or_default();
    let payload_part = parts.next().unwrap_or_default();
    let signature = parts.next().unwrap_or_default();

    if header_part.is_empty()
        || payload_part.is_empty()
        || signature.is_empty()
        || parts.next().is_some()
    {
        return Err(
            "A JWT must contain exactly three non-empty segments separated by dots.".into(),
        );
    }

    let header = decode_json_segment(header_part, "header")?;
    let payload = decode_json_segment(payload_part, "payload")?;

    if !header.is_object() {
        return Err("JWT header must contain a JSON object.".into());
    }
    if !payload.is_object() {
        return Err("JWT payload must contain a JSON object.".into());
    }

    Ok(DecodedJwt {
        header,
        payload,
        signature: signature.to_string(),
    })
}

fn decode_json_segment(segment: &str, name: &str) -> Result<Value, String> {
    let bytes =
        decode_base64url(segment).map_err(|error| format!("Invalid JWT {name}: {error}"))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("JWT {name} is not valid JSON: {error}"))
}

fn decode_base64url(input: &str) -> Result<Vec<u8>, String> {
    if input.contains('=')
        || input
            .chars()
            .any(|c| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
    {
        return Err("segment contains invalid Base64URL characters".into());
    }

    if input.len() % 4 == 1 {
        return Err("segment has invalid Base64URL length".into());
    }

    let mut output = Vec::with_capacity(input.len() * 3 / 4);
    let mut buffer = 0u32;
    let mut bits = 0u8;

    for byte in input.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            _ => return Err("segment contains invalid Base64URL characters".into()),
        } as u32;

        buffer = (buffer << 6) | value;
        bits += 6;

        while bits >= 8 {
            bits -= 8;
            output.push((buffer >> bits) as u8);
            if bits > 0 {
                buffer &= (1 << bits) - 1;
            } else {
                buffer = 0;
            }
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

    #[test]
    fn decodes_valid_jwt() {
        let decoded = decode_jwt(SAMPLE).unwrap();
        assert_eq!(decoded.header["alg"], "HS256");
        assert_eq!(decoded.payload["name"], "John Doe");
        assert!(!decoded.signature.is_empty());
    }

    #[test]
    fn rejects_invalid_segment_count() {
        assert!(decode_jwt("one.two").is_err());
        assert!(decode_jwt("one.two.three.four").is_err());
    }

    #[test]
    fn rejects_invalid_base64url() {
        assert!(decode_jwt("@@@.eyJvayI6dHJ1ZX0.signature").is_err());
    }
}
