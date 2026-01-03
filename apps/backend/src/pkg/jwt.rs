use crate::pkg::error::AppError;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
    Long,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: i64,    // Expiration time
    pub iat: i64,    // Issued at
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    public_key: Vec<u8>,
    access_token_expiry: i64,  // in seconds
    refresh_token_expiry: i64, // in seconds
}

impl JwtService {
    /// Create a new JWT service using RS256 (RSA with SHA-256)
    /// Keys should be in PEM format (loaded from files)
    pub fn new(
        private_key: &[u8],
        public_key: &[u8],
        access_token_expiry: i64,
        refresh_token_expiry: i64,
    ) -> Result<Self, AppError> {
        let encoding_key = EncodingKey::from_rsa_pem(private_key).map_err(|e| {
            eprintln!("Failed to load private key: {}", e);
            AppError::InternalError("Failed to load private key".to_string())
        })?;

        let decoding_key = DecodingKey::from_rsa_pem(public_key).map_err(|e| {
            eprintln!("Failed to load public key: {}", e);
            AppError::InternalError("Failed to load public key".to_string())
        })?;

        Ok(Self {
            encoding_key,
            decoding_key,
            public_key: public_key.to_vec(),
            access_token_expiry,
            refresh_token_expiry,
        })
    }

    /// Get the public key in PEM format
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Generate an access token (short-lived)
    pub fn generate_token(&self, user_id: Uuid, length: i64) -> Result<String, AppError> {
        let now = chrono::Utc::now().timestamp();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + length,
            iat: now,
        };

        let mut header = Header::default();
        header.alg = jsonwebtoken::Algorithm::RS256;
        encode(&header, &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalError(format!("Failed to encode access token: {}", e)))
    }

    /// Validate and decode a token
    pub fn validate_token(&self, token: &str) -> Result<Claims, AppError> {
        let validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
    }

    pub fn get_length(&self, token_type: &TokenType) -> i64 {
        match token_type {
            TokenType::Access => self.access_token_expiry,
            TokenType::Refresh => self.refresh_token_expiry,
            TokenType::Long => self.refresh_token_expiry * 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation() {
        // Sample RSA keys for testing
        let private_key = b"-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA2a2rwplBCRf3a3xZGBl0jBc4ytfQPVMVpNBQXPkVVLOwvd8k
q0A/HRCqOu4A1N2gn4n2+GBt8BX2xvpRjNBXhNvMyVQ5zxRSemW6TvKq0hNUEo9B
bBJvRhHSbqMFjKG1j8GxNJvEWJHLyh+1J5pBSMCBrfCf7Ni6YH5b8M5TzFJLVEJ5
vkJRZxRBg2BVhDVBN8Nj6fVa9KqH3p4aGWdqkDPKMHGhF7T4Rc0BDkT3QFbkQvl3
yT1QvbLX0FhGNx9EUJrBNJdWXVVNgBVq1vZUxZgJqOAXTkMgIBpvFyZELl0P1Lkq
fEL3VQhKFZ9QrKgbL5QVTbPHJ4dRxKIEO0vVwQIDAQABAoIBAFzYHXGVMpFRdV5z
3qTBBcaB9Fq7MWNc0kXYAFE0fRDqhT6sT1KY0nqZGZO0lL+3H0lJ0q3Q6UDv1Ybk
0rBQSJfQP3iVJv5MgQm3bPVp3Sm6BFhGYDOx5P3c6n0LQXc6pOxnXhJMPqUEVdkP
w0DXLDzUVKGT3KVQX4cOJ1PnzQdQQXBKRLGGNUABNi7MXPjxbZqXvEoECKB2Vdx7
LDKfVaFLwWJwN6VZ0ZqXX3PN+FcHNJZKPt0Q2j0CKwMvHBF6hGRQBOTNBjBVvKQw
2sQRDVGq4xUmBEBGBZd6nzJEz1IkqHXi0P1NZ8VVqTvFR3kqVVTrDSPjDdP9Bkkx
gPRZ4AECgYEA7lh3K+jfHnqe2BnHvBTKZ+8yk/lKxu9c+Y8F0VqIvFfVZLYhXmDG
Z4xLDn6VlCuIHGKn5kYv8qQdFo8LmwfVbNBVZrYWcPAFVj9hV/GJXTDnlXJKbGn8
LHqbBZQhpG0ORCZ5rqLFEQSXV/j4Hx3MdH6aXvvp8P9sF/JKl1XZODkCgYEA6Fjs
m8VyFN8eVSqBKPKXVh7lXLYcF8cNV5Y0fRV5VgOWBFYKCKqQCKY6t9L3b3fGZKnH
B0hQ0SZlZqQYbxL6LlwMhvMKJJXzfY9m8xKdBXdQDVVLEA+7aVmTg9fKBVPBGVMl
0PNXGj0iP7MjVLBJtPELXq8BQVdTJHE8pHJFgAECgYAfNkKlU1lMEYkFYK4KqQQN
ZCXi8xW8hLKVwPUbZMcTlVBKs8GVHP6K5EuLRB7s7fqR1XRKzJvvMZUfj9pf7Z/L
iGWUq2K0Wej8DvKQV8rPqNpQdFM3IK5KVF3LqK0N3P5Zo2RKhR8A0UXEqI8ZqhSl
+P6X9Qw/K8BVaYZa9J4U4QKBgQDkYzcEhQ8iVfKbVPONY7C5rNZQSLYEVIjFLUIL
w8v5hWm2PsYb5s0K0kMZGlEFJqDKDpVN8jhYRwEt+NE7n2x+YPqLvRQDjVUFy1Gr
Hn6bWqrQeKB+qZxFDRDhKVNd+xBIBdVPTJJH0Ak6XVPG3QMYg1fFkLT3C8mG5wH1
xJAAQQKBgQDEqWlLvLZXPsxKfPXIQd3eXKbD7EFpCxPLWNPLgLoaMhDmLUHLKmK1
d8P7vZ5kLLkLLwqaVXf3K3Y7CwKYJEXN8vKXEQdq1FBVN5QE1xLRpqvLYQC2V9Vg
HK1JhZVpCvBgDh7VdFjEOqYPXxF7xXdQvvDL7xEvGfQnNS0VhLgQEA==
-----END RSA PRIVATE KEY-----";
        let public_key = b"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA2a2rwplBCRf3a3xZGBl0
jBc4ytfQPVMVpNBQXPkVVLOwvd8kq0A/HRCqOu4A1N2gn4n2+GBt8BX2xvpRjNBX
hNvMyVQ5zxRSemW6TvKq0hNUEo9BbBJvRhHSbqMFjKG1j8GxNJvEWJHLyh+1J5pB
SMCBrfCf7Ni6YH5b8M5TzFJLVEJ5vkJRZxRBg2BVhDVBN8Nj6fVa9KqH3p4aGWdq
kDPKMHGhF7T4Rc0BDkT3QFbkQvl3yT1QvbLX0FhGNx9EUJrBNJdWXVVNgBVq1vZUx
ZgJqOAXTkMgIBpvFyZELl0P1LkqfEL3VQhKFZ9QrKgbL5QVTbPHJ4dRxKIEO0vVwQ
IDAQAB
-----END PUBLIC KEY-----";

        let service = JwtService::new(private_key, public_key, 3600, 86400)
            .expect("Failed to create JWT service");

        let user_id = Uuid::new_v4();
        let token = service
            .generate_token(user_id, service.get_length(&TokenType::Access))
            .expect("Failed to generate token");
        let claims = service
            .validate_token(&token)
            .expect("Failed to validate token");
        assert_eq!(claims.sub, user_id.to_string());
        assert!(matches!(claims.exp, exp if exp > claims.iat));
    }
}
