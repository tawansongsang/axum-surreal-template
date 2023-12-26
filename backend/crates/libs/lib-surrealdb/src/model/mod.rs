mod error;
mod store;
pub mod user_info;

use self::store::{Db, new_db_pool};

use self::error::Result;

// #[derive(Clone)]
pub struct ModelManager {
    db: Db
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
}