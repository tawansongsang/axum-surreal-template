use crate::{
    ctx::Ctx,
    model::{conditions, task::TaskRecord, Error, ListOptions, ModelManager, Result},
};

use super::{Task, TaskFilter, TaskForCreate, TaskParamsForCreate, TaskParamsForUpdate};

pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskParamsForCreate,
    ) -> Result<TaskRecord> {
        let db = mm.db();
        // FIXME: fixed fot sql server
        // let user_id = ctx.user_id_thing().ok_or(Error::UserIdNotFound)?;

        // let task_c = TaskForCreate {
        //     title: &task_c.title,
        //     create_by: &user_id,
        //     update_by: &user_id,
        // };

        // let mut created: Vec<TaskRecord> = db.create("task").content(task_c).await?;

        // let task_record = created.pop().ok_or(Error::DataNotFoundFromCreated);

        // task_record
        todo!()
    }

    pub async fn get<'a>(_ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<Task<'a>> {
        let db = mm.db();
        // let task_id = Thing::from(("task", id));

        // FIXME: fixed for sql server
        // let task: Result<Task<'a>> = db.select(("task", id)).await?.ok_or(Error::DataNotFound);

        // task
        todo!()
    }

    pub async fn list<'a>(
        _ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TaskFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Task<'a>>> {
        let db = mm.db();
        // FIXME: fixed for sql server
        // let conditions = conditions::gen_all_condition(filters, list_options);

        // let sql = format!("SELECT * FROM task {}", conditions);

        // let mut response = db.query(sql).await?;

        // let tasks = response.take::<Vec<Task>>(0)?;

        // Ok(tasks)
        todo!()
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        task_id: &str,
        task_u: TaskParamsForUpdate,
    ) -> Result<()> {
        let db = mm.db();
        let task = Self::get(ctx, mm, task_id).await?;
        // FIXME: fixed for sql server
        // let user_id = ctx.user_id_thing().ok_or(Error::UserIdNotFound)?;
        // let title = task_u.title.as_deref().unwrap_or(&task.title);

        // let sql = r#"UPDATE type::thing($table, $id) MERGE {
        //     title: $title,
        //     update_by: $update_by,
        //     update_on: time::now(),
        // };"#;

        // let mut response = db
        //     .query(sql)
        //     .bind(("table", "task"))
        //     .bind(("id", task_id))
        //     .bind(("title", title))
        //     .bind(("update_by", user_id))
        //     .await?;

        // let _record = response
        //     .take::<Option<TaskRecord>>(0)?
        //     .ok_or(Error::DataNotFoundFromUpdate)?;

        // Ok(())
        todo!()
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, task_id: &str) -> Result<()> {
        let db = mm.db();
        // FIXME: fixed for sql server
        // let _task: Result<TaskRecord> = db
        //     .delete(("task", task_id))
        //     .await?
        //     .ok_or(Error::DataNotFoundFromDelete);

        // Ok(())
        todo!()
    }
}

// TODO: Create Unit for multi create and delete all and list task Test
// region:    --- Tests
// #[cfg(test)]
// mod tests {
//     pub type Result<T> = core::result::Result<T, Error>;
//     pub type Error = Box<dyn std::error::Error>; // For tests.

//     use crate::model;

//     use super::*;
//     use serial_test::serial;

//     #[serial]
//     #[tokio::test]
//     async fn test_create_delete_first_task_ok() -> Result<()> {
//         // -- Setup & Fixtures
//         let mm = model::ModelManager::new().await?;
//         let ctx = Ctx::new(Some("user_info:iR1f8i7Wg7jipR3uhDhJ".to_string())).unwrap();
//         let fx_task_for_create = TaskParamsForCreate {
//             title: "Task Test OK".to_string(),
//         };

//         // -- Exec
//         let task_id = TaskBmc::create(&ctx, &mm, fx_task_for_create)
//             .await
//             .unwrap()
//             .id
//             .id
//             .to_raw();

//         let deleted = TaskBmc::delete(&ctx, &mm, &task_id).await.unwrap();

//         // -- Check
//         assert_eq!(deleted, ());

//         Ok(())
//     }
// }
// // endregion: --- Tests
