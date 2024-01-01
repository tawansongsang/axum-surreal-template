pub mod bmc;

use serde::{Deserialize, Serialize};
use surrealdb::sql;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub id: sql::Thing,
    pub username: String,

    // -- pwd and token info
    pub password: String,
    pub password_salt: Uuid,
    pub token_salt: Uuid,
    pub create_by: sql::Thing,
    pub create_on: sql::Datetime,
    pub update_by: sql::Thing,
    pub update_on: sql::Datetime,
}

#[derive(Debug, Serialize)]
pub struct UserInfoForCreate {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfoCreated<'a> {
    pub username: String,
    pub password: String,
    pub create_by: &'a Option<sql::Thing>,
    pub update_by: &'a Option<sql::Thing>,
}

// #[derive(Debug, Deserialize)]
// pub struct UserInfoForUpdatePWD {
//     pub id: sql::Thing,
//     pub password: String,
//     pub update_by: sql::Thing,
// }

#[derive(Debug, Deserialize)]
pub struct UserInfoForLogin {
    pub id: sql::Thing,
    pub username: String,
    pub password: Option<String>, // encrypted, #_scheme_id_#....
    pub password_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoForAuth {
    pub id: sql::Thing,
    pub username: String,

    // -- token info
    pub token_salt: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoForTest {
    pub id: sql::Thing,
    pub username: String,
    pub password: Option<String>,
    pub password_salt: Uuid,
    pub token_salt: Uuid,
}
