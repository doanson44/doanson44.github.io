use crate::domain::base64;

/// Application service coordinating Base64 encoding and decoding.
pub struct Base64Service;

impl Base64Service {
    /// Encode UTF-8 text as standard Base64.
    pub fn encode(input: &str) -> String {
        base64::encode(input)
    }

    /// Decode standard Base64 into UTF-8 text.
    pub fn decode(input: &str) -> Result<String, String> {
        base64::decode(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_encodes_and_decodes() {
        let encoded = Base64Service::encode("hello");
        assert_eq!(encoded, "aGVsbG8=");
        assert_eq!(Base64Service::decode(&encoded).unwrap(), "hello");
    }
}
