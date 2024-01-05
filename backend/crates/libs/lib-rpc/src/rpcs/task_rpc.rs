use lib_surrealdb::{
    ctx::Ctx,
    model::{
        task::{bmc::TaskBmc, Task, TaskParamsForCreate, TaskParamsForUpdate},
        ModelManager,
    },
};

use crate::{
    params::{ParamsForCreate, ParamsForUpdate, ParamsIded},
    router::RpcRouter,
    Result,
};

pub fn rpc_router() -> RpcRouter {
    todo!()
}

pub async fn create_task<'a>(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<TaskParamsForCreate>,
) -> Result<Task<'a>> {
    let ParamsForCreate { data } = params;

    let task_id = TaskBmc::create(&ctx, &mm, data).await?;
    let task = TaskBmc::get(&ctx, &mm, &task_id.id.id.to_raw()).await?;

    Ok(task)
}

pub async fn list_tasks<'a>(ctx: Ctx, mm: ModelManager) -> Result<Vec<Task<'a>>> {
    // let tasks = TaskBmc::list(&ctx, &mm, filter, limit)?;
    let tasks = TaskBmc::list(&ctx, &mm).await?;

    Ok(tasks)
}

pub async fn update_task<'a>(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<TaskParamsForUpdate>,
) -> Result<Task<'a>> {
    let ParamsForUpdate { id, data } = params;

    TaskBmc::update(&ctx, &mm, &id, data).await?;

    let task = TaskBmc::get(&ctx, &mm, &id).await?;

    Ok(task)
}

pub async fn delete_task<'a>(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Task<'a>> {
    let ParamsIded { id } = params;

    let task = TaskBmc::get(&ctx, &mm, &id).await?;
    TaskBmc::delete(&ctx, &mm, &id).await?;

    Ok(task)
}
