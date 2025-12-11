use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub struct CryptoUtils;

impl CryptoUtils {
    pub fn hash_password(password: &str) -> Result<String, String> {
        let argon2 = Argon2::default();
        let salt: SaltString = SaltString::generate(&mut OsRng);
        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => return Ok(hash.to_string()),
            Err(e) => {
                tracing::warn!("Failed to hash password: {}", e);
                return Err("failed to hash the pasword".to_string());
            }
        };
    }

    pub fn compare_password(hash_password: &str, password: &str) -> Result<(), String> {
        let argon2 = Argon2::default();
        let parsed_hash = match PasswordHash::new(password) {
            Ok(parsed_hash) => parsed_hash,
            Err(err) => {
                tracing::warn!("failed to parse hash {}", err.to_string());
                return Err("hash password failed".to_string());
            }
        };

        if let Err(err) = argon2.verify_password(hash_password.as_bytes(), &parsed_hash) {
            return Err(format!("Invalid password: {}", err));
        }
        return Ok(());
    }
}
