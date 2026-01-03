use std::fs;
use std::path::Path;

use crate::pkg::error::AppError;

const PRIVATE_KEY_PATH: &str = ".keys/private.pem";
const PUBLIC_KEY_PATH: &str = ".keys/public.pem";

/// Key management - generates or loads RSA keys for JWT signing
pub struct KeyManager;

impl KeyManager {
    /// Load or generate RSA key pair
    /// Returns (private_key_bytes, public_key_bytes)
    pub fn load_or_generate_keys() -> Result<(Vec<u8>, Vec<u8>), AppError> {
        // Try to load existing keys
        if Path::new(PRIVATE_KEY_PATH).exists() && Path::new(PUBLIC_KEY_PATH).exists() {
            println!("Loading existing RSA keys from disk...");
            let private_key = fs::read(PRIVATE_KEY_PATH).map_err(|e| {
                AppError::InternalError(format!("Failed to read private key: {}", e))
            })?;
            let public_key = fs::read(PUBLIC_KEY_PATH).map_err(|e| {
                AppError::InternalError(format!("Failed to read public key: {}", e))
            })?;
            return Ok((private_key, public_key));
        }

        // Generate new keys
        println!("Generating new RSA keys...");
        Self::generate_keys()
    }

    /// Generate new RSA key pair using openssl
    fn generate_keys() -> Result<(Vec<u8>, Vec<u8>), AppError> {
        use std::process::Command;

        // Create keys directory if it doesn't exist
        fs::create_dir_all(".keys").map_err(|e| {
            AppError::InternalError(format!("Failed to create .keys directory: {}", e))
        })?;

        // Generate private key (2048-bit RSA)
        let output = Command::new("openssl")
            .args(&["genrsa", "-out", PRIVATE_KEY_PATH, "2048"])
            .output()
            .map_err(|e| {
                AppError::InternalError(format!("Failed to execute openssl genrsa: {}", e))
            })?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::InternalError(format!(
                "openssl genrsa failed: {}",
                err
            )));
        }

        // Extract public key from private key
        let output = Command::new("openssl")
            .args(&[
                "rsa",
                "-in",
                PRIVATE_KEY_PATH,
                "-pubout",
                "-out",
                PUBLIC_KEY_PATH,
            ])
            .output()
            .map_err(|e| {
                AppError::InternalError(format!("Failed to execute openssl rsa: {}", e))
            })?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::InternalError(format!(
                "openssl rsa failed: {}",
                err
            )));
        }

        // Read the generated keys
        let private_key = fs::read(PRIVATE_KEY_PATH).map_err(|e| {
            AppError::InternalError(format!("Failed to read generated private key: {}", e))
        })?;
        let public_key = fs::read(PUBLIC_KEY_PATH).map_err(|e| {
            AppError::InternalError(format!("Failed to read generated public key: {}", e))
        })?;

        println!(
            "RSA keys generated successfully at {} and {}",
            PRIVATE_KEY_PATH, PUBLIC_KEY_PATH
        );
        println!("Keys are stored for persistence across restarts.");

        Ok((private_key, public_key))
    }
}
