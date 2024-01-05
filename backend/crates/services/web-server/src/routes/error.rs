use std::sync::Arc;

use crate::routes;
use axum::{http::StatusCode, response::IntoResponse};
use derive_more::From;
use lib_auth::token;
use lib_surrealdb::model;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use tracing::debug;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd {
        user_id: String,
    },
    LoginFailPwdNotMatching {
        user_id: String,
    },
    // -- CtxExtError
    #[from]
    CtxExt(routes::mw_auth::CtxExtError),

    // -- ReqStamp
    ReqStampNotInResponseExt,

    // -- Rpc
    RpcMethodUnknow(String),
    RpcMissingParams {
        rpc_method: String,
    },
    RpcFailJsonParams {
        rpc_method: String,
    },

    // -- Module
    #[from]
    Model(model::Error),
    #[from]
    Token(token::Error),
    #[from]
    Rpc(lib_rpc::Error),

    // -- Externals
    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // -- Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // -- Insert the Error into the response.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use routes::Error::*;

        let response = match self {
            // -- Login
            LoginFailUsernameNotFound
            | LoginFailUserHasNoPwd { .. }
            | LoginFailPwdNotMatching { .. } => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Model TODO: implement for better client response.
            Model(_) => (StatusCode::BAD_REQUEST, ClientError::ENTITY_NOT_FOUND),

            // -- Rpc TODO: implement for better client response.
            RpcMissingParams { .. } | RpcFailJsonParams { .. } | RpcMethodUnknow(_) => {
                (StatusCode::BAD_REQUEST, ClientError::ENTITY_NOT_FOUND)
            }

            // -- Fallback,
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        };

        response
    }
}

// region:    --- Client Error
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    // ENTITY_NOT_FOUND { entity: &'static str, id: i64 },
    ENTITY_NOT_FOUND,

    SERVICE_ERROR,
}
// endregion: --- Client Error
