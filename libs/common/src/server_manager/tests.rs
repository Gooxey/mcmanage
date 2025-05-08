// #![cfg(test)]

// use tokio::{
//     fs::{
//         self,
//         OpenOptions,
//     },
//     io,
//     spawn,
// };

// use super::*;
// use crate::{
//     generated_files::{
//         default_files::get_example_content,
//         paths::{
//             CONFIG_DIR,
//             SERVER_LIST_FILE,
//         },
//     },
//     test_functions::*,
// };

// // TODO fix tests

// async fn test_start() {
//     start_test();
//     generate_server_list().await;
//     ServerManager::init().await;
// }
// // TODO use 2 servers here as soon as the properties of a server become configurable
// async fn generate_server_list() {
//     let content = r#"
//         [myFirstServer]
//         download_from = "https://api.purpurmc.org/v2/purpur/1.19.3/1933/download"
//         args = "-jar purpur-1.19.3-1933.jar nogui"
//         server_type = "purpur"
//         [myFirstServer.restart_time]
//         secs = 60
//         nanos = 0
//     "#;

//     fs::create_dir(CONFIG_DIR.as_path()).await.unwrap();
//     let mut server_list_file = OpenOptions::new()
//         .write(true)
//         .create_new(true)
//         .open(SERVER_LIST_FILE.as_path())
//         .await
//         .unwrap();
//     io::copy(&mut content.as_bytes(), &mut server_list_file)
//         .await
//         .unwrap();
// }

// // the following two functions will also test `get_server_parameter` and `generate_valid_server_list_file`
// #[tokio::test]
// async fn load_server_list_valid_file() {
//     test_start().await;

//     ServerManager::load_server_list().await;

//     assert_eq!(
//         server_manager().server_list.lock().await.0.len(),
//         1,
//         "The function should have captured one server."
//     );
//     cleanup();
// }
// #[tokio::test]
// async fn load_server_list_invalid_file() {
//     start_test();
//     let content = r#"
//         [0]
//         name = "myFirstServer"
//         args = "-jar purpur-1.19.3-1876.jar nogui"
//     }"#;

//     fs::create_dir(CONFIG_DIR.as_path()).await.unwrap();
//     let mut server_list_file = OpenOptions::new()
//         .write(true)
//         .create_new(true)
//         .open(SERVER_LIST_FILE.as_path())
//         .await
//         .unwrap();
//     io::copy(&mut content.as_bytes(), &mut server_list_file)
//         .await
//         .unwrap();

//     let server_manager = Arc::new(ServerManager {
//         main_thread: Arc::new(None.into()),
//         status: Status::Stopped.into(),

//         server_list: (vec![], Utc::now()).into(),
//         restart_times: vec![].into(),
//     });

//     spawn(async {
//         sleep(Duration::new(1, 0)).await;

//         let mut valid_file = OpenOptions::new()
//             .write(true)
//             .create(true)
//             .open(SERVER_LIST_FILE.as_path())
//             .await
//             .unwrap();
//         io::copy(
//             &mut get_example_content(&SERVER_LIST_FILE).1.as_bytes(),
//             &mut valid_file,
//         )
//         .await
//         .unwrap();

//         let mut server_list = load_toml(&SERVER_LIST_FILE, "Test", true).await.unwrap();
//         server_list.remove("mySecondServer");

//         fs::write(
//             SERVER_LIST_FILE.as_path(),
//             toml::to_string(&server_list).unwrap().as_bytes(),
//         )
//         .await
//         .unwrap();
//     });

//     server_manager.load_server_list().await;

//     OpenOptions::new()
//         .write(true)
//         .create_new(true)
//         .open(CONFIG_DIR.join("example_server_list.toml"))
//         .await
//         .unwrap_err();
//     cleanup();
// }

// #[tokio::test]
// async fn get_server() {
//     let server_manager = test_start().await;

//     server_manager.clone().impl_start(false).await.unwrap();

//     let server = server_manager
//         .get_server("myFirstServer")
//         .await
//         .unwrap();

//     assert_eq!(server.name(), "myFirstServer");

//     server_manager.impl_stop(false, false).await.unwrap();
//     cleanup();
// }
// // set the `src/config::Config::shutdown_time` field to 1min to test the shutdown of the own machine
// // the `src/config::AGREE_TO_EULA` const needs to be true
// // the `src/config::Config::server_restart_time` const needs to be 1min
// #[tokio::test]
// async fn main() {
//     // this is a test for almost every function in the ServerManager struct
//     let server_manager = test_start().await;

//     server_manager.clone().impl_start(false).await.unwrap();

//     let server = server_manager
//         .get_server("myFirstServer")
//         .await
//         .unwrap();
//     let start_time = Instant::now();
//     loop {
//         if Status::Restarting == server.status().await {
//             break;
//         }
//         if Instant::now() - start_time > Duration::new(100, 0) {
//             assert!(false, "The ServerManager took to long to restart.");
//         }
//         sleep(Duration::new(1, 0)).await;
//     }
//     loop {
//         if Status::Started == server.status().await {
//             break;
//         }
//         sleep(Duration::new(1, 0)).await;
//     }
//     server_manager.impl_stop(false, false).await.unwrap();
//     cleanup();
// }
