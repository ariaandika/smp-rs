use std::io;

#[derive(thiserror::Error, Debug)]
pub enum SetupError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("failed to bind tcp: {0}")]
    Tcp(io::Error),
    #[error("failed to get {0:?}: {1}")]
    Var(&'static str, std::env::VarError),
}

pub fn check_env(name: &'static str) -> Result<(), SetupError> {
    match std::env::var(name) {
        Ok(_) => Ok(()),
        Err(err) => Err(SetupError::Var(name, err)),
    }
}

