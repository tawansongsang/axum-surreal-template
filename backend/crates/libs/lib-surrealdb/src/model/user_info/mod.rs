pub mod bmc;

use surrealdb::sql;
use uuid::Uuid;

pub struct UserInfo {
    pub id: sql::Uuid,
    pub username: String,
}

pub struct UserInfoForCreate {
    pub username: String,
    pub password: String,
}

pub struct UserInfoForInsert {
    pub password: String,
}

pub struct UserInfoForLogin {
    pub username: String,
    pub password: String,

    // -- password and token info
    pub pwd: Option<String>, // encrypted, #_scheme_id_#....
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

pub struct UserInfoForAuth {
    pub username: String,
    pub password: String,

    // -- token info
    pub token_salt: Uuid,
}

pub trait UserInfoBy {}

impl UserInfoBy for UserInfo {}
impl UserInfoBy for UserInfoForLogin {}
impl UserInfoBy for UserInfoForAuth {}
