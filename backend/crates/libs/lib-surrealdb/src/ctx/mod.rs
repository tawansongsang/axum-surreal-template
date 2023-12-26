mod error;

use surrealdb::sql::Uuid;

use self::error::{Result, Error};

#[derive(Debug)]
pub struct Ctx {
    user_id: Uuid,
}

impl Ctx {
    pub fn root_ctx() -> Self {
        let root_uuid = Uuid(uuid::Uuid::nil());
        Ctx { user_id: root_uuid }
    }

    pub fn new(user_id: Uuid) -> Result<Self> {
        if user_id.is_nil() {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }
}