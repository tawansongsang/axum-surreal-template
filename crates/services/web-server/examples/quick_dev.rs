use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For examples.

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1@demo.com",
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
    // res.print().await?;

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
    // res.print().await?;

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
    // res.print().await?;

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
    let res = req_update_task.await?;
    // res.print().await?;

    // let req_list_tasks = hc.do_post(
    //     "/api/rpc",
    //     json!({
    //         "id": 5,
    //         "method": "list_tasks"
    //     }),
    // );
    // let res = req_list_tasks.await?;
    // res.print().await?;

    let req_list_tasks = hc.do_post(
        "/api/rpc",
        json!({
            "id": 5,
            "method": "list_tasks",
            "params": {
                "filters": {
                    // "id": "task:Ay5RJ87J5dSkChl9KlSD",
                    // "title": "2",
                    // "done": false,
                    // "create_by": "user_info:iR1f8i7Wg7jipR3uhDhJ",
                    "start_create_on": "2024-01-07T00:00:00Z",
                    // "end_create_on": "2024-01-07T05:00:00Z",
                    // "update_by": "user_info:iR1f8i7Wg7jipR3uhDhJ",
                    // "start_update_on": "2024-01-07T06:00:00Z",
                    // "end_update_on": "2024-01-08T08:00:00Z",
                },
                "list_options": {
                    "start": 0,
                    "limit": 100,
                    "orderby": [
                        {
                            "field": "create_on",
                            "is_desc": true,
                        },
                        // {
                        //     "field": "update_on",
                        //     "is_desc": false,
                        // }
                    ]
                }
            }
        }),
    );
    let res = req_list_tasks.await?;
    // res.print().await?;

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
        let res = req_delete_task.await?;
        // res.print().await?;
    }

    let req_logout = hc.do_post(
        "/api/logout",
        json!({
            "logout": true,
        }),
    );
    let res = req_logout.await?;
    res.print().await?;

    let req_register = hc.do_post(
        "/api/register",
        json!({
            "username": "demo2@demo.com",
            "name": "demo2",
            "password": "demo2",
        }),
    );
    let res = req_register.await?;
    res.print().await?;

    Ok(())
}
