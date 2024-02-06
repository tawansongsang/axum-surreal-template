mod error;

use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: Option<i64>,
}

impl Ctx {
    pub fn root_ctx() -> Self {
        let root_id = None;
        Ctx { user_id: root_id }
    }

    pub fn new(user_id: Option<i64>) -> Result<Self> {
        if user_id.is_none() {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }

    pub fn user_id(&self) -> Option<i64> {
        self.user_id
    }
}
