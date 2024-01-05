use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

pub mod bmc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task<'a> {
    pub id: Thing,
    // pub project_id: Thing,
    pub title: Cow<'a, str>,
    pub done: bool,

    // -- Timestamps
    //    (creator and last modified user_id/time)
    pub create_by: Thing,
    pub create_on: Datetime,
    pub update_by: Thing,
    pub update_on: Datetime,
}

#[derive(Serialize, Deserialize)]
pub struct TaskParamsForCreate {
    pub title: String,
    // pub project_id: Thing,
}

#[derive(Serialize)]
pub struct TaskForCreate<'a> {
    pub title: &'a str,
    // pub project_id: Thing,
    pub create_by: &'a Thing,
    pub update_by: &'a Thing,
}

#[derive(Deserialize, Default)]
pub struct TaskParamsForUpdate {
    pub title: Option<String>,
    pub done: Option<bool>,
}

// #[derive(Serialize)]
// pub struct TaskForUpdate<'a> {
//     pub title: Option<&'a str>,
//     pub done: Option<bool>,
//     pub update_by: &'a Thing,
// }

#[derive(Debug, Deserialize)]
pub struct TaskRecord {
    pub id: Thing,
}

pub struct TaskFilter {
    pub id: Option<Thing>,
    // project_id: Option<Thing>,
    pub title: Option<String>,
    pub done: Option<bool>,

    pub create_by: Option<Thing>,
    pub start_create_on: Option<Datetime>,
    pub end_create_on: Option<Datetime>,
    pub update_by: Option<Thing>,
    pub start_update_on: Option<Datetime>,
    pub end_update_on: Option<Datetime>,
}
