use surrealdb::sql::Thing;

use crate::{
    ctx::Ctx,
    model::{
        task::{TaskForUpdate, TaskRecord},
        Error, ModelManager, Result,
    },
};

use super::{Task, TaskForCreate, TaskParamsForCreate, TaskParamsForUpdate};
pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskParamsForCreate<'_>,
    ) -> Result<TaskRecord> {
        let db = mm.db();
        let id = ctx.user_id().ok_or(Error::UserIdNotFound)?;
        let user_id = Thing::from(("task", id));

        let task_c = TaskForCreate {
            title: task_c.title,
            create_by: &user_id,
            update_by: &user_id,
        };

        let mut created: Vec<TaskRecord> = db.create("task").content(task_c).await?;

        let task_record = created.pop().ok_or(Error::DataNotFoundFromCreated);

        task_record
    }

    pub async fn get<'a>(_ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<Task<'a>> {
        let db = mm.db();
        let task_id = Thing::from(("task", id));

        let task: Result<Task<'a>> = db.select(task_id).await?.ok_or(Error::DataNotFound);

        task
    }

    pub async fn list<'a>(
        _ctx: &Ctx,
        mm: &ModelManager,
        filter: &'a str,
        list_options: &'a str,
    ) -> Result<Vec<Task<'a>>> {
        let db = mm.db();
        // TODO: Create function for query builder;
        let sql = "
            SELECT * FROM task
            WHERE id=$id AND string::contains($title)
                    AND done AND create_by = $create_by
                    AND update_by = $update_by
                    AND start_create_on = $start_create_on
                    AND start_update_on = $start_update_on
            ORDER BY $order_bys 
            LIMIT $limit START $offset
        ";

        let mut result = db.query(sql).bind(("title", "test")).await?;

        let task: Vec<Task<'a>> = result.take(0)?;

        todo!()
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        task_id: &str,
        task_u: TaskParamsForUpdate<'_>,
    ) -> Result<()> {
        let db = mm.db();
        let id = ctx.user_id().ok_or(Error::UserIdNotFound)?;
        let user_id = Thing::from(("task", id));
        let task_u = TaskForUpdate {
            title: task_u.title,
            done: task_u.done,
            update_by: &user_id,
        };

        let _task: Result<TaskRecord> = db
            .update(("task", task_id))
            .content(task_u)
            .await?
            .ok_or(Error::DataNotFoundFromUpdate);

        Ok(())
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, task_id: &str) -> Result<()> {
        let db = mm.db();
        let _task: Result<TaskRecord> = db
            .delete(("task", task_id))
            .await?
            .ok_or(Error::DataNotFoundFromDelete);

        Ok(())
    }
}

// TODO: Create Unit Test
