use crate::domain::jwt::{decode_jwt, DecodedJwt};

pub struct JwtService;

impl JwtService {
    pub fn decode(source: &str) -> Result<DecodedJwt, String> {
        decode_jwt(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_decodes_jwt() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.signature";
        let result = JwtService::decode(token).unwrap();
        assert_eq!(result.header["typ"], "JWT");
    }
}
