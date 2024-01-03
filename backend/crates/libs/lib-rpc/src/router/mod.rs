mod into_params;

pub use into_params::{IntoDefaultParams, IntoParams};

use serde_json::Value;

/// The raw JSON-RPC request object, serving as the foundation for RPC routing.
pub struct RpcRequest {
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

pub struct RpcRouter {
    route_by_name: String, // TODO: Change to rpc router name
}

impl RpcRouter {
    pub fn new() -> Self {
        Self {
            route_by_name: "Test".to_string(), // TODO: Change login new()
        }
    }

    pub fn add_dyn() {
        todo!()
    }

    pub fn add() {
        todo!()
    }

    pub fn call() {
        todo!()
    }
}
