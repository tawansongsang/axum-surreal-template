use crate::{
    ctx::Ctx,
    model::{task::TaskRecord, Error, ModelManager, Result},
};

use tracing::debug;

use super::{Task, TaskForCreate, TaskParamsForCreate, TaskParamsForUpdate};
pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskParamsForCreate,
    ) -> Result<TaskRecord> {
        let db = mm.db();
        let user_id = ctx.user_id_thing().ok_or(Error::UserIdNotFound)?;

        let task_c = TaskForCreate {
            title: &task_c.title,
            create_by: &user_id,
            update_by: &user_id,
        };

        let mut created: Vec<TaskRecord> = db.create("task").content(task_c).await?;

        let task_record = created.pop().ok_or(Error::DataNotFoundFromCreated);

        task_record
    }

    pub async fn get<'a>(_ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<Task<'a>> {
        let db = mm.db();
        // let task_id = Thing::from(("task", id));

        let task: Result<Task<'a>> = db.select(("task", id)).await?.ok_or(Error::DataNotFound);

        task
    }

    pub async fn list<'a>(
        _ctx: &Ctx,
        mm: &ModelManager,
        // filter: &'a str,
        // list_options: &'a str,
    ) -> Result<Vec<Task<'a>>> {
        let db = mm.db();
        // TODO: Create function for query builder;
        // let sql = "
        //     SELECT * FROM task
        //     WHERE id=$id AND string::contains($title)
        //             AND done AND create_by = $create_by
        //             AND update_by = $update_by
        //             AND start_create_on = $start_create_on
        //             AND start_update_on = $start_update_on
        //     ORDER BY $order_bys
        //     LIMIT $limit START $offset
        // ";

        // let mut result = db.query(sql).bind(("title", "test")).await?;

        // let task: Vec<Task<'a>> = result.take(0)?;

        let tasks: Vec<Task> = db.select("task").await?;

        Ok(tasks)
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        task_id: &str,
        task_u: TaskParamsForUpdate,
    ) -> Result<()> {
        let db = mm.db();
        let task = Self::get(ctx, mm, task_id).await?;
        let user_id = ctx.user_id_thing().ok_or(Error::UserIdNotFound)?;
        let title = task_u.title.as_deref().unwrap_or(&task.title);

        let query = r#"UPDATE type::thing($table, $id) MERGE { 
            title: $title, 
            update_by: $update_by, 
            update_on: time::now(), 
        };"#;

        let mut response = db
            .query(query)
            .bind(("table", "task"))
            .bind(("id", task_id))
            .bind(("title", title))
            .bind(("update_by", user_id))
            .await?;

        let _record = response
            .take::<Option<TaskRecord>>(0)?
            .ok_or(Error::DataNotFoundFromUpdate)?;

        Ok(())
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, task_id: &str) -> Result<()> {
        let db = mm.db();
        let _task: Result<TaskRecord> = db
            .delete(("task", task_id))
            .await?
            .ok_or(Error::DataNotFoundFromDelete);

        Ok(())
    }
}

// TODO: Create Unit Test
