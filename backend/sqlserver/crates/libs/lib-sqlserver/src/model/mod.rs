mod conditions;
mod error;
mod store;
pub mod task;
pub mod user_info;

pub use self::conditions::*;
use tracing::log::info;

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

    pub async fn test_connection(&self) {
        info!(
            "{:<12} - test_connection - {}",
            "STARTUP", "trying to use pool for connecting to sql server"
        );

        let mut client = self.db().get().await.unwrap();

        let stream = client
            .simple_query("SELECT * FROM @@VERSION;")
            .await
            .unwrap();

        let result = stream
            .into_first_result()
            .await
            .unwrap()
            .into_iter()
            .map(|row| {
                let val: &str = row.get(0).unwrap();
                String::from(val)
            })
            .collect::<Vec<_>>();

        info!(
            "{:<12} - test_connection {}: \n{:?}",
            "STARTUP", "Sql Server Version: ", result
        );
    }
}
