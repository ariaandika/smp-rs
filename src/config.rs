use anyhow::Context;


pub fn host() -> String {
    match std::env::var("HOST") {
        Ok(host) => host,
        Err(_) => "0.0.0.0:3000".to_owned(),
    }
}

pub fn env(name: &str) -> String {
    std::env::var(name).expect("env is not asserted")
}

pub fn env_assert(name: &str) -> anyhow::Result<String> {
    std::env::var(name).with_context(||format!("failed to get {name:?} env"))
}

