use std::{env::args, io};

use argon2::{
    password_hash::{rand_core::OsRng, SaltString, PasswordHasher},
    Argon2,
};

fn main() -> Result<(), io::Error> {
    let Some(passwd) = args().skip(1).next() else {
        return Err(io::Error::new(io::ErrorKind::Other, "input required"));
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let result = argon.hash_password(passwd.as_bytes(), &salt).map_err(|e|io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    println!("{}", result.to_string());
    Ok(())
}

