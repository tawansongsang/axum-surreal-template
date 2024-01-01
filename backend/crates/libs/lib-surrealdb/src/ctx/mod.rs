mod error;

use std::str::FromStr;

use surrealdb::sql::Thing;

use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: Option<String>,
}

impl Ctx {
    pub fn root_ctx() -> Self {
        let root_id = None;
        Ctx { user_id: root_id }
    }

    pub fn new(user_id: Option<String>) -> Result<Self> {
        if user_id.is_none() {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }

    pub fn user_id(&self) -> &Option<String> {
        &self.user_id
    }

    pub fn user_id_thing(&self) -> Option<Thing> {
        self.user_id
            .clone()
            .and_then(|id_str| Thing::from_str(&id_str).ok())
    }
}
