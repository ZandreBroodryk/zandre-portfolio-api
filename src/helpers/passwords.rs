use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use shuttle_runtime::SecretStore;

use crate::{Result, ServerError};

pub fn hash_password(password: String, secrets: SecretStore) -> Result<String> {
    let salt = secrets
        .get("SALT_PHRASE")
        .ok_or_else(|| ServerError::NoSaltPhrase)?;

    let salt =
        SaltString::encode_b64(salt.as_bytes()).map_err(|_| ServerError::SaltEncodingFail)?;

    let argon2 = get_argon();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|error| ServerError::HashFail(error.to_string()))?
        .to_string();

    return Ok(password_hash);
}

pub fn validate_password(password: String, hash: String) -> Result<()> {
    let argon2 = get_argon();
    let parsed_hash =
        PasswordHash::new(&hash).map_err(|error| ServerError::HashFail(error.to_string()))?;

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ServerError::AuthFailInvalidPassword)?;

    return Ok(());
}

fn get_argon() -> Argon2<'static> {
    let argon2 = Argon2::default();
    return argon2;
}
