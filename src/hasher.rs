use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{Error, PasswordHasher, SaltString, rand_core::OsRng},
};

pub fn hash(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}

pub fn verify_hash(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash);
    if parsed_hash.is_err() {
        return false;
    }
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash.unwrap())
        .is_ok()
}
