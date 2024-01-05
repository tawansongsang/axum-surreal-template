use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For examples.

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "password": "demo1",
        }),
    );

    req_login.await?.print().await?;

    let mut task_ids = Vec::new();

    let req_create_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "create_task",
            "params": {
                "data": {
                    "title": "task A1"
                }
            }
        }),
    );
    let res = req_create_task.await?;
    let task1 = res.json_body()?;
    let task_id_1 = task1["result"]["id"]["id"]["String"].as_str().unwrap();
    task_ids.push(task_id_1);
    res.print().await?;

    let req_create_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 2,
            "method": "create_task",
            "params": {
                "data": {
                    "title": "task A2"
                }
            }
        }),
    );
    let res = req_create_task.await?;
    let task2 = res.json_body()?;
    let task_id_2 = task2["result"]["id"]["id"]["String"].as_str().unwrap();
    task_ids.push(task_id_2);
    res.print().await?;

    let req_create_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 3,
            "method": "create_task",
            "params": {
                "data": {
                    "title": "task A3"
                }
            }
        }),
    );
    let res = req_create_task.await?;
    let task3 = res.json_body()?;
    let task_id_3 = task3["result"]["id"]["id"]["String"].as_str().unwrap();
    task_ids.push(task_id_3);
    res.print().await?;

    let req_update_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": "update",
            "method": "update_task",
            "params": {
                "id": task_id_1,
                "data": {
                    "title": "task A1 updated"
                }
            }
        }),
    );
    req_update_task.await?.print().await?;

    let req_list_tasks = hc.do_post(
        "/api/rpc",
        json!({
            "id": 5,
            "method": "list_tasks"
        }),
    );
    req_list_tasks.await?.print().await?;

    for task_id in task_ids {
        let req_delete_task = hc.do_post(
            "/api/rpc",
            json!({
                "id": "delete",
                "method": "delete_task",
                "params": {
                    "id": task_id
                }

            }),
        );
        req_delete_task.await?.print().await?;
    }

    let req_logout = hc.do_post(
        "/api/logout",
        json!({
            "logout": true,
        }),
    );

    req_logout.await?.print().await?;

    Ok(())
}
