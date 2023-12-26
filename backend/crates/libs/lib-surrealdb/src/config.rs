use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATEL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    // -- Db
    // pub DB_HOST: String,
    // pub DB_PORT: String,
    pub DB_URL: String,
    pub DB_NAMESPACE: String,
    pub DB_DBNAME: String,
    pub DB_USERNAME: String,
    pub DB_PASSWORD: String,
    // pub AUTHGODTOKEN: String,
}

impl CoreConfig {
    fn load_from_env() -> lib_utils::envs::Result<CoreConfig> {
        let db_host = get_env("SURREALDB_HOST")?;
        let db_port = get_env("SURREALDB_PORT")?;
        let db_url = format!("{}:{}", db_host, db_port);
        Ok(CoreConfig {
            // -- Db
            DB_URL: db_url,
            DB_NAMESPACE: get_env("SURREALDB_NAMESPACE")?,
            DB_DBNAME: get_env("SURREALDB_DBNAME")?,
            DB_USERNAME: get_env("SURREALDB_USERNAME")?,
            DB_PASSWORD: get_env("SURREALDB_PASSWORD")?,
            // // -- AuthGodToken
            // AUTHGODTOKEN: get_env("AUTHGODTOKEN")?,
        })
    }
}