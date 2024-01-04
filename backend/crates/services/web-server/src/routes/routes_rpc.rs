use super::{mw_auth::CtxW, Error, Result};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use lib_rpc::rpcs::task_rpc::{create_task, list_tasks};
use lib_surrealdb::{
    ctx::Ctx,
    model::{task::TaskParamsForCreate, ModelManager},
};
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};
use tracing::debug;

#[derive(Clone)]
pub struct RpcState {
    pub mm: ModelManager,
}

#[derive(Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

/// JSON-RPC Request Body.
#[derive(Deserialize)]
struct RpcRequest {
    // jsonrpc: String, MUST be exactly "2",
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

// Axum router for '/api/rpc'
pub fn routes(rpc_state: RpcState) -> Router {
    // -- Builder the combined RpcRouter.
    // let rpc_router = RpcRouter::new()
    //     .extend(task_rpc::rpc_router());
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(rpc_state)
}

async fn rpc_handler(
    State(rpc_state): State<RpcState>,
    ctx: CtxW,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    let ctx = ctx.0;
    inner_rpc_handler(ctx, rpc_state.mm, rpc_req)
        .await
        .into_response()
}

async fn inner_rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params,
    } = rpc_req;

    debug!(
        "{:<12} - inner_rpc_handler - method: {rpc_method}",
        "HANDLER"
    );

    let result_json = match rpc_method.as_str() {
        // -- Task RPC methods.
        "create_task" => {
            let params = rpc_params.ok_or(Error::RpcMissingParams {
                rpc_method: "create_task".to_string(),
            })?;

            // 3.36 min
            let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
                //ParamsForCreate<TaskParamsForCreate<'a>>
                rpc_method: "create_task".to_string(),
            })?;

            create_task(ctx, mm, params).await.map(to_value)??
        }
        "list_tasks" => list_tasks(ctx, mm).await.map(to_value)??,
        "update_task" => todo!(),
        "delete_task" => todo!(),

        // -- Fallback as Err.
        _ => return Err(Error::RpcMethodUnknow(rpc_method)),
    };

    let body_response = json!({
        "id": rpc_id,
        "result": result_json,
    });

    Ok(Json(body_response))
}
