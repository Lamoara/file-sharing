use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{Error, PasswordHasher, SaltString, rand_core::OsRng},
};

pub fn hash(data: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let data_hash = argon2.hash_password(data.as_bytes(), &salt)?;

    Ok(data_hash.to_string())
}

pub fn verify_hash(data: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash);
    if parsed_hash.is_err() {
        return false;
    }
    Argon2::default()
        .verify_password(data.as_bytes(), &parsed_hash.unwrap())
        .is_ok()
}
