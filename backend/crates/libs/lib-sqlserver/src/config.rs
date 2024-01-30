use core::panic;
use std::sync::OnceLock;

// region:    --- Modules
use lib_utils::envs::{get_env, get_env_parse};
// endregion: --- Modules

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONFIG - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    // -- Db
    pub DB_HOST: String,
    pub DB_PORT: u16,
    pub DB_NAME: String,
    pub DB_USERNAME: String,
    pub DB_PASSWORD: String,
}

impl CoreConfig {
    fn load_from_env() -> lib_utils::envs::Result<CoreConfig> {
        Ok(CoreConfig {
            // -- Db
            DB_HOST: get_env("MSSQL_HOST")?,
            DB_PORT: match get_env_parse::<u16>("MSSQL_PORT") {
                Ok(port) => port,
                Err(_) => 1433,
            },
            DB_NAME: get_env("MSSQL_DBNAME")?,
            DB_USERNAME: get_env("MSSQL_USERNAME")?,
            DB_PASSWORD: get_env("MSSQL_PASSWORD")?,
        })
    }
}
