mod error;

use once_cell::sync::Lazy;
use surrealdb::{Surreal, engine::remote::ws::{Client, Ws}, opt::auth::Root};

pub use self::error::{Error, Result};

use crate::core_config;

pub type Db = Lazy<Surreal<Client>>;

pub async fn new_db_pool() -> Result<Db> {
    let db: Db = Lazy::new(Surreal::init);
    let capacity = 3;

    // Connect to the database
    db.connect::<Ws>(&core_config().DB_URL).with_capacity(capacity).await?;

    // Log into the database
    db.signin(Root {
        username: &core_config().DB_USERNAME,
        password: &core_config().DB_PASSWORD,
    }).await?;

    db.use_ns(&core_config().DB_NAMESPACE).use_db(&core_config().DB_DBNAME).await?;

    Ok(db)
}