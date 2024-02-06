use lib_sqlserver::{
    ctx::Ctx,
    model::{
        task::{bmc::TaskBmc, Task, TaskFilter, TaskParamsForCreate, TaskParamsForUpdate},
        ModelManager,
    },
};

use crate::{
    params::{ParamsForCreate, ParamsForUpdate, ParamsIded},
    router::RpcRouter,
    rpc_router, ParamsList, Result,
};

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add..
        create_task,
        list_tasks,
        update_task,
        delete_task,
    )
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

pub async fn list_tasks<'a>(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<TaskFilter>,
) -> Result<Vec<Task<'a>>> {
    let tasks = TaskBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

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
