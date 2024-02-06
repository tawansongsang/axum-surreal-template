use std::borrow::Cow;

use crate::model::conditions::{
    create_eq_con, create_str_contain_con, create_time_le_con, create_time_me_con,
};

use serde::{Deserialize, Serialize};
// use surrealdb::sql::{Datetime, Thing};

use super::Filter;

pub mod bmc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task<'a> {
    // pub id: Thing, // FIXME: change to sql server id
    // pub project_id: Thing,
    pub title: Cow<'a, str>,
    pub done: bool,
    // -- Timestamps
    //    (creator and last modified user_id/time)
    // FIXME: change to sql server id and datetime
    // pub create_by: Thing,
    // pub create_on: Datetime,
    // pub update_by: Thing,
    // pub update_on: Datetime,
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
    // FIXME: change to sql server ID
    // pub create_by: &'a Thing,
    // pub update_by: &'a Thing,
}

#[derive(Deserialize, Default)]
pub struct TaskParamsForUpdate {
    pub title: Option<String>,
    pub done: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TaskRecord {
    // FIXME: change to sql server ID
    // pub id: Thing,
}

#[derive(Debug, Deserialize, Default)]
pub struct TaskFilter {
    pub id: Option<String>, //Option<Thing>
    // project_id: Option<String>, //Option<Thing>
    pub title: Option<String>,
    pub done: Option<bool>,

    pub create_by: Option<String>, //Option<Thing>
    // FIXME: change to sqlserver Datetime
    // pub start_create_on: Option<Datetime>,
    // pub end_create_on: Option<Datetime>,
    pub update_by: Option<String>, //Option<Thing>
                                   // FIXME: change to sqlserver Datetime
                                   // pub start_update_on: Option<Datetime>,
                                   // pub end_update_on: Option<Datetime>,
}

impl Filter for TaskFilter {
    fn gen_condition(&self) -> Vec<String> {
        let mut conditions = Vec::new();

        create_eq_con!(self, conditions, id, id);
        create_str_contain_con!(self, conditions, title, title);
        create_eq_con!(self, conditions, done, done);
        create_eq_con!(self, conditions, create_by, create_by);
        // create_time_me_con!(self, conditions, create_on, start_create_on);
        // create_time_le_con!(self, conditions, create_on, end_create_on);
        create_eq_con!(self, conditions, update_by, update_by);
        // create_time_me_con!(self, conditions, update_on, start_update_on);
        // create_time_le_con!(self, conditions, update_on, end_update_on);

        conditions
    }
}
