use surrealdb::sql;

use crate::{model::{ModelManager, Result}, ctx::Ctx};

use super::UserInfoBy;

pub struct UserInfoBmc;

impl UserInfoBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: sql::Uuid) -> Result<E>
    where
        E: UserInfoBy,
    {
        let db = mm.db();
        todo!()
    }

    pub async fn first_by_username<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserInfoBy
    {
        let db = mm.db();
        todo!()
    }

    pub async fn update_pwd(
		ctx: &Ctx,
		mm: &ModelManager,
		id: sql::Uuid,
		pwd_clear: &str,
	) -> Result<()> {
        let db = mm.db();
        todo!()
    }
}