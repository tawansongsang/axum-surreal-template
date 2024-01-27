use crate::web::{self, remove_token_cookie};

use super::error::{Error, Result};

use axum::{extract::State, routing::post, Json, Router};
use lib_surrealdb::{
    ctx::Ctx,
    model::{
        user_info::{bmc::UserInfoBmc, UserInfoForCreate, UserInfoForLogin},
        ModelManager,
    },
};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/logout", post(api_logout_handler))
        .route("/api/register", post(api_register_handler))
        .with_state(mm)
}

// region:    --- Login
async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_login_handler", "HANLDER");

    let LoginPayload { username, password } = payload;
    let root_ctx = Ctx::root_ctx();

    // -- Get the user.
    let user: UserInfoForLogin = UserInfoBmc::first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;

    let user_id = user.id.id.to_raw();

    // -- Validate the password.
    let Some(hash) = user.password else {
        return Err(Error::LoginFailUserHasNoPwd { user_id });
    };

    let scheme_status = UserInfoBmc::validate_password(&mm, &hash, &password).await?;

    if !scheme_status {
        return Err(Error::LoginFailPwdNotMatching { user_id });
    }

    // -- Set web token
    web::set_token_cookie(&cookies, &user_id, user.token_salt)?;

    // -- Create the success body
    let body = Json(json!({
        "result": {
            "success": true,
        },
        "data": {
            "id": user_id,
            "email": user.username,
            "name": user.name,
            "role": user.role,
            "image": null,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
// endregion: --- Login

// region:    --- Logout
async fn api_logout_handler(
    cookies: Cookies,
    Json(payload): Json<LogoutPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_logout_handler", "HANLDER");

    let should_logout = payload.logout;

    if should_logout {
        remove_token_cookie(&cookies)?;
    }

    // -- Create the success body.
    let body = Json(json!({
        "resutl": {
            "logout": should_logout
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoutPayload {
    logout: bool,
}
// endregion: --- Logout

async fn api_register_handler(
    State(mm): State<ModelManager>,
    _cookies: Cookies,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_register_handler", "HANLDER");
    let root_ctx = Ctx::root_ctx();

    let RegisterPayload {
        username,
        name,
        password,
    } = payload;

    let user_info_for_create = UserInfoForCreate {
        username,
        name,
        password,
    };

    let _user_info_record = UserInfoBmc::create(&root_ctx, &mm, user_info_for_create).await?;

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    username: String,
    name: String,
    password: String,
}
