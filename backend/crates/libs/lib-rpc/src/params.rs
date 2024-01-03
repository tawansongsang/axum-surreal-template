//! Base constructs for the typed RPC Params that will be used in their respective
//! rpc handler functions (e.g., `project_rpc::create_project` and `project_rpc::list_projects`).
//!
//! Most of these base constructs use generics for their respective data elements, allowing
//! each rpc handler function to receive the exact desired type.
//!
//! `IntoParams` or `IntoDefaultParams` are implemented to ensure these Params conform to the
//! `RpcRouter` (i.e., `rpc::router`) model.

use crate::router::IntoParams;

use lib_surrealdb::sql::Thing;
use serde::{de::DeserializeOwned, Deserialize};
use serde_with::serde_as;

/// Params structure for any RPC Create call.
#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    pub data: D,
}

impl<D> IntoParams for ParamsForCreate<D> where D: DeserializeOwned + Send {}

// TODO: implement ParamsList
/// Params structure for any RPC Update call.
#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    pub id: Thing,
    pub data: D,
}

impl<D> IntoParams for ParamsForUpdate<D> where D: DeserializeOwned + Send {}

/// Params structure for any RPC Update call.
#[derive(Deserialize)]
pub struct ParamsIded {
    pub id: Thing,
}
impl IntoParams for ParamsIded {}

// /// Params structure for any RPC List call.
// #[serde_as]
// #[derive(Deserialize, Default)]
// pub struct ParamsList<F>
// where
//     F: DeserializeOwned,
// {
//     #[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
//     pub filers: Option<Vec<F>>,
//     pub list_options: Option<ListOptions>, // ListOptions from modql TODO: understand modql and implement with owned.
// }
