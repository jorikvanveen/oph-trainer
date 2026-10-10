use std::env;

pub struct Config {
    pub bind_address: String,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Result<Self, env::VarError> {
        Ok(Self {
            bind_address: env_or_default("OPH_BIND_ADDRESS", "127.0.0.1:3000")?,
            database_url: env_or_default("OPH_DATABASE_URL", "sqlite://oph-trainer.db?mode=rwc")?,
        })
    }
}

fn env_or_default(name: &str, default: &str) -> Result<String, env::VarError> {
    match env::var(name) {
        Ok(value) => Ok(value),
        Err(env::VarError::NotPresent) => Ok(default.to_owned()),
        Err(error) => Err(error),
    }
}
