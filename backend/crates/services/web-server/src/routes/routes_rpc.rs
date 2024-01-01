use axum::Router;
use lib_surrealdb::model::ModelManager;
use serde_json::Value;

#[derive(Clone)]
pub struct RpcState {
    pub mm: ModelManager,
}

#[derive(Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

pub fn routes(rpc_state: RpcState) -> Router {
    todo!()
}
