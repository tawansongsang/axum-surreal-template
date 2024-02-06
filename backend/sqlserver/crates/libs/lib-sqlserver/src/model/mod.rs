mod conditions;
mod error;
mod store;
pub mod task;
pub mod user_info;

pub use self::conditions::*;
pub use self::error::{Error, Result};

use self::store::{new_db_pool, Db};

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Contructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }

    // TODO: create test_connection
}
