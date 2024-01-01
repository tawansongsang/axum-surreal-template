use axum::http::{Method, Uri};
use lib_surrealdb::ctx::Ctx;

use lib_utils::time::{format_time, now_utc};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use tracing::debug;

use crate::routes::ClientError;
use crate::routes::{self, mw_stamp::ReqStamp, routes_rpc::RpcInfo};
use crate::Result;

pub async fn log_request(
    http_method: Method,
    uri: Uri,
    req_stamp: ReqStamp,
    rpc_info: Option<&RpcInfo>,
    ctx: Option<Ctx>,
    web_error: Option<&routes::Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    // -- Prep Error
    let error_type = web_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(web_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    // -- Prep Req Infomation
    let ReqStamp { uuid, time_in } = req_stamp;
    let now = now_utc();
    let duration = now - time_in;
    // -- duration_ms in milliseconds with microseconds precision.
    let duration_ms = (duration.as_seconds_f64() * 1_000_000.).floor() / 1_000.;

    let user_id = match ctx {
        None => None,
        Some(ctx) => ctx.user_id().as_ref().map(|s| s.to_string()),
    };
    // -- Create the RequestLogLine
    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: format_time(now).unwrap_or("error generate time stamp".to_string()), // Logline timestamp ("time_out")
        time_in: format_time(time_in).unwrap_or("error generate time in".to_string()),
        duration_ms,

        user_id,

        http_path: uri.to_string(),
        http_method: http_method.to_string(),

        rpc_id: rpc_info.and_then(|rpc| rpc.id.as_ref().map(|id| id.to_string())),
        rpc_method: rpc_info.map(|rpc| rpc.method.to_string()),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };

    debug!("REQUEST LOG LINE:\n{}", json!(log_line));

    // TODO: Send to cloud-watch or log monitor

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,      //uuid string formatted
    timestamp: String, // (Rfc3339)
    time_in: String,   // (Rfx3339)
    duration_ms: f64,

    // -- User and context attributes.
    user_id: Option<String>,

    // -- http request attributes.
    http_path: String,
    http_method: String,

    // -- rpc info.
    rpc_id: Option<String>,
    rpc_method: Option<String>,

    // -- Errors attributes.
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
