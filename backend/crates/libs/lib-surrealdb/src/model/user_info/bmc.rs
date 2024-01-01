use serde::de::DeserializeOwned;
use surrealdb::sql::{self};

use crate::{
    ctx::Ctx,
    model::{Error, ModelManager, Result},
};

use super::{UserInfo, UserInfoCreated, UserInfoForCreate};

pub struct UserInfoBmc;

impl UserInfoBmc {
    pub async fn get<'de, E>(_ctx: &Ctx, mm: &ModelManager, id: sql::Uuid) -> Result<Option<E>>
    where
        E: DeserializeOwned,
    {
        let db = mm.db();
        let query = "SELECT * FROM user_info:$id LIMIT 1;";
        let mut response = db.query(query).bind(("id", id.to_string())).await?;

        let user_info: Option<E> = response.take(0)?;

        Ok(user_info)
    }

    pub async fn first_by_username<'de, E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: DeserializeOwned,
    {
        let db = mm.db();
        let query = "SELECT * FROM user_info WHERE username = $username LIMIT 1;";
        let mut response = db
            .query(query)
            .bind(("username", username.to_string()))
            .await?;

        let user_info_for_auth: Option<E> = response.take(0)?;

        Ok(user_info_for_auth)
    }

    pub async fn update_pwd(
        ctx: &Ctx,
        mm: &ModelManager,
        id: sql::Uuid,
        password: &str,
    ) -> Result<()> {
        let db = mm.db();
        let query =
            "UPDATE ONLY user_info:&id SET password = &password update_by = user_info:&update_by update_on = time::now();";
        let mut response = db
            .query(query)
            .bind(("id", id))
            .bind(("password", password))
            .bind(("update_by", ctx.user_id()))
            .await?;

        let _user_info: Option<UserInfo> = response.take(0)?;

        Ok(())
    }

    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        user_info_for_create: UserInfoForCreate,
    ) -> Result<UserInfo> {
        let db = mm.db();

        let user_id_create = ctx.user_id_thing();

        let user_info_created = UserInfoCreated {
            username: user_info_for_create.username,
            password: user_info_for_create.password,
            create_by: &user_id_create,
            update_by: &user_id_create,
        };

        let mut created: Vec<UserInfo> = db.create("user_info").content(user_info_created).await?;

        let user_info = created.pop().ok_or(Error::DataNotFound)?;

        Ok(user_info)
    }

    pub async fn validate_password(mm: &ModelManager, hash: &str, password: &str) -> Result<bool> {
        let db = mm.db();

        let query = "RETURN crypto::argon2::compare($hash, $pass)";

        let mut response = db
            .query(query)
            .bind(("hash", hash))
            .bind(("pass", password))
            .await?;

        response
            .take::<Option<bool>>(0)?
            .ok_or(Error::CannotComparePasswordFromDB)
    }
}

// TODO: Create Unit Test

// // region:    --- Tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use anyhow::Result;

//     fn test_first_ok_demo1() -> Result<()> {
// 		// -- Setup & Fixtures
// 		let mm = _dev_utils::init_test().await;
// 		let ctx = Ctx::root_ctx();
// 		let fx_username = "demo1";

// 		// -- Exec
// 		let user: UserInfo = UserBmc::first_by_username(&ctx, &mm, fx_username)
// 			.await?
// 			.context("Should have user 'demo1'")?;

// 		// -- Check
// 		assert_eq!(user.username, fx_username);

// 		Ok(())
//     }
// }
// // endregion: --- Tests
