mod conditions;
mod error;
mod store;
pub mod task;
pub mod user_info;

use tracing::info;

use self::store::{new_db_pool, Db};

pub use self::conditions::*;
pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    /// Return the surrealdb pool reference.
    /// (Only for the model layer)
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }

    pub async fn test_connection(&self) {
        info!(
            "{:<12} - test_connection - {}",
            "STARTUP", "trying to use pool for connecting to surrealdb server"
        );

        let db = self.db();
        let sql = "RETURN 'OK'";
        let mut response = db.query(sql).await.unwrap().check().unwrap();

        let info_root: Option<String> = response.take(0).unwrap();

        info!(
            "{:<12} - test_connection {}: \n{:?}",
            "STARTUP", "testing query, result is:", info_root
        );
    }
}
