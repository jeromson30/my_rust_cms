use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use bcrypt::{hash as bcrypt_hash, verify as bcrypt_verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, &'static str> {
    // Use Argon2id for new passwords (recommended for security)
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| "Failed to hash password")
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, &'static str> {
    // Check if it's an Argon2 hash (starts with $argon2)
    if hash.starts_with("$argon2") {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| "Failed to parse password hash")?;
        
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map(|_| true)
            .or_else(|_| Ok(false))
    }
    // Fall back to bcrypt for legacy passwords
    else if hash.starts_with("$2") {
        bcrypt_verify(password, hash).map_err(|_| "Failed to verify password")
    }
    else {
        Err("Unsupported password hash format")
    }
}
