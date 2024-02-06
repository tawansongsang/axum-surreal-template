// use lib_surrealdb::{
//     ctx::Ctx,
//     model::{
//         user_info::{
//             bmc::UserInfoBmc, UserInfo, UserInfoCreated, UserInfoForLogin, UserInfoForTest,
//         },
//         ModelManager,
//     },
// };

// #[tokio::main]
// async fn main() {
//     println!("hello from example");

//     let mm = ModelManager::new().await.unwrap();
//     let ctx = Ctx::root_ctx();
//     let username = "demo1";

//     // let user_info_for_login: Option<UserInfoForLogin> =
//     //     UserInfoBmc::first_by_username(&ctx, &mm, username)
//     //         .await
//     //         .unwrap();

//     // dbg!("{:?}", user_info_for_login);

//     let db = mm.db();
//     let query = "SELECT * FROM user_info WHERE username = $username LIMIT 1;";
//     let mut response = db
//         .query(query)
//         .bind(("username", username.to_string()))
//         .await
//         .unwrap();

//     // dbg!("{:?}", response);

//     let tmp: Option<UserInfoForLogin> = response.take(0).unwrap();
//     // let user_info: Option<UserInfoForLogin> = response.take(0).unwrap();

//     println!("{:?}", tmp);

//     // let id = tmp.unwrap().id.to_raw();
//     // println!("{:?}", id);
// }

fn main() {
    println!("Hello from surrealdb");
}
