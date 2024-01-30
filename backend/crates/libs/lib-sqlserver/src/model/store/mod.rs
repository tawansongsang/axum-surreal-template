mod error;

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tiberius::{AuthMethod, Config};

pub use self::error::{Error, Result};

use crate::config::core_config;

pub type Db = Pool<ConnectionManager>;

fn build_config_db() -> Config {
    let mut config_db = Config::new();

    config_db.host(&core_config().DB_HOST);
    config_db.port(core_config().DB_PORT.clone());
    config_db.database(&core_config().DB_NAME);
    config_db.authentication(AuthMethod::sql_server(
        &core_config().DB_USERNAME,
        &core_config().DB_PASSWORD,
    ));

    config_db.trust_cert(); // on production, it is not a good idea to do this

    config_db
}

pub async fn new_db_pool() -> Result<Db> {
    let config = build_config_db();
    let mgr = ConnectionManager::new(config);
    let pool = Pool::builder().max_size(3).build(mgr).await?;

    Ok(pool)
}
