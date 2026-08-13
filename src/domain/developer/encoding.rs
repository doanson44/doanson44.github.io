use sha2::{Digest, Sha256, Sha512};

pub fn url(input: &str, mode: &str) -> Result<String, String> {
    match mode.to_ascii_lowercase().as_str() {
        "encode" => Ok(input
            .bytes()
            .map(|b| {
                if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) {
                    (b as char).to_string()
                } else {
                    format!("%{b:02X}")
                }
            })
            .collect()),
        "decode" | "" => {
            let bytes = input.as_bytes();
            let mut out = Vec::with_capacity(bytes.len());
            let mut i = 0;
            while i < bytes.len() {
                if bytes[i] == b'%' {
                    if i + 2 >= bytes.len() {
                        return Err("Invalid percent-encoding.".into());
                    }
                    let hex = std::str::from_utf8(&bytes[i + 1..i + 3])
                        .map_err(|_| "Invalid percent-encoding.")?;
                    out.push(u8::from_str_radix(hex, 16).map_err(|_| "Invalid percent-encoding.")?);
                    i += 3;
                } else {
                    out.push(bytes[i]);
                    i += 1;
                }
            }
            String::from_utf8(out).map_err(|_| "Decoded data is not valid UTF-8.".into())
        }
        _ => Err("Mode must be encode or decode.".into()),
    }
}

pub fn hash(input: &str, algorithm: &str) -> Result<String, String> {
    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }
    match algorithm.to_ascii_uppercase().as_str() {
        "SHA-256" | "SHA256" | "" => Ok(hex(&Sha256::digest(input.as_bytes()))),
        "SHA-512" | "SHA512" => Ok(hex(&Sha512::digest(input.as_bytes()))),
        _ => Err("Supported algorithms: SHA-256 and SHA-512.".into()),
    }
}

pub fn number_base(input: &str, target: &str) -> Result<String, String> {
    let value = if let Some(v) = input.trim().strip_prefix("0x") {
        i128::from_str_radix(v, 16)
    } else if let Some(v) = input.trim().strip_prefix("0b") {
        i128::from_str_radix(v, 2)
    } else if let Some(v) = input.trim().strip_prefix("0o") {
        i128::from_str_radix(v, 8)
    } else {
        input.trim().parse::<i128>()
    }
    .map_err(|_| "Enter a valid integer (decimal, 0x hex, 0b binary, or 0o octal).")?;
    match target.to_ascii_lowercase().as_str() {
        "bin" | "binary" => Ok(format!("0b{value:b}")),
        "oct" | "octal" => Ok(format!("0o{value:o}")),
        "hex" | "hexadecimal" => Ok(format!("0x{value:X}")),
        "dec" | "decimal" | "" => Ok(value.to_string()),
        _ => Err("Target base must be binary, octal, decimal, or hex.".into()),
    }
}

pub fn html_entity(input: &str, mode: &str) -> Result<String, String> {
    if mode.eq_ignore_ascii_case("decode") {
        Ok(input
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
            .replace("&quot;", "\"")
            .replace("&#39;", "'"))
    } else if mode.eq_ignore_ascii_case("encode") || mode.is_empty() {
        Ok(input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;"))
    } else {
        Err("Mode must be encode or decode.".into())
    }
}

pub fn unicode_escape(input: &str, mode: &str) -> Result<String, String> {
    if mode.eq_ignore_ascii_case("escape") || mode.is_empty() {
        Ok(input
            .chars()
            .map(|c| {
                if c.is_ascii() {
                    c.to_string()
                } else {
                    format!("\\u{{{:X}}}", c as u32)
                }
            })
            .collect())
    } else if mode.eq_ignore_ascii_case("unescape") {
        let mut out = String::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' && chars.peek() == Some(&'u') {
                chars.next();
                if chars.next() != Some('{') {
                    return Err("Invalid Unicode escape.".into());
                }
                let mut hex = String::new();
                while let Some(&h) = chars.peek() {
                    chars.next();
                    if h == '}' {
                        break;
                    }
                    hex.push(h);
                }
                let value = u32::from_str_radix(&hex, 16).map_err(|_| "Invalid Unicode escape.")?;
                let decoded = char::from_u32(value).ok_or("Invalid Unicode code point.")?;
                out.push(decoded);
            } else {
                out.push(c);
            }
        }
        Ok(out)
    } else {
        Err("Mode must be escape or unescape.".into())
    }
}
