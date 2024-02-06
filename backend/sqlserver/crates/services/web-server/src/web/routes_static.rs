use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    routing::{any_service, MethodRouter},
};
use tower_http::services::ServeDir;

use crate::config::web_config;

// Note: Here we can just return a MethodRouter rather then a full Router
//       since ServeDir is a service.
pub fn serve_dir() -> MethodRouter {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Resource not found.")
    }

    any_service(
        ServeDir::new(&web_config().WEB_FOLDER).not_found_service(handle_404.into_service()),
    )
}
