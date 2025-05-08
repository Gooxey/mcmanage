use std::sync::Arc;
use chrono::prelude::*;
use goolog::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, pool::PoolConnection};
use tokio::sync::{Mutex, MutexGuard};

use crate::{generated_files::paths::CONFIG_DIR, mcmanage_error::MCManageError};

use self::server_data::ServerData;

use super::server::Server;

const GOOLOG_CALLER: &str = "ServerList";
static SERVER_LIST: Mutex<Option<Arc<ServerList>>> = Mutex::const_new(None);

// TODO load servers
// TODO start servers on load
// TODO add a note about one time usage (once stopped you can never init it again)
// TODO note about restart time being only 1 sec granularity
// TODO a note about panics to every function
// TODO update list on every change

mod server_data;

pub struct ServerList {
    pool: Pool<Postgres>,
    last_update: Mutex<DateTime<Utc>>,
    list: Arc<Vec<Server>>
}
// internal
impl ServerList {
    async fn server_list() -> Arc<Self> {
        SERVER_LIST
            .lock()
            .await
            .as_ref()
            .unwrap_or_else(|| {
                fatal!("You must first initialize the server list with the `ServerList::init()` function before doing anything.")
            })
            .clone()
    }
    async fn connection() -> PoolConnection<Postgres> {
        Self::server_list()
            .await
            .pool
            .acquire()
            .await
            .unwrap_or_else(|error| {
                fatal!("Could not acquire a connection to the server_list database. Error: {error}")
            })
    }
}

// actions
impl ServerList {
    pub async fn init() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:password@localhost/mcmanage")
            .await
            .unwrap_or_else(|error| {
                fatal!("Failed to connect to the sever list database. Error: {error}")
            });

        // database.execute("
        //     CREATE TABLE IF NOT EXISTS servers (
        //         id INTEGER PRIMARY KEY,
        //         name TEXT NOT NULL,
        //         args TEXT NOT NULL,
        //         download_from TEXT,
        //         restart_time INTEGER,
        //         server_type TEXT NOT NULL,
        //     )",
        //     ()
        // );

        let mut server_list = SERVER_LIST.lock().await;
        if server_list.is_some() {
            fatal!("The server list has already been initialized.")
        }
        *server_list = Some(
            Self {
                pool,
                last_update: Utc::now().into(),
                list: vec![].into()
            }.into()
        );

        info!("Initialized!");
    }
    pub async fn stop() {
        todo!("Close every server");

        // by dropping the server list, the database connection will be close automatically
        drop(SERVER_LIST
            .lock()
            .await
            .take()
            .unwrap_or_else(|| {
                fatal!("You must first initialize the server list with the `ServerList::init()` function before doing anything.")
            }));
    }
}

// // set/modify data
// impl ServerList {
//     pub async fn add(server: ServerData) -> Result<(), MCManageError>{
//         async fn next_primary_key(database: &MutexGuard<'_, Connection>) -> usize {
//             let primary_key: usize = database
//                 .prepare("
//                     SELECT MIN(ID) + 1
//                     FROM servers t1
//                     WHERE NOT EXISTS
//                     (
//                         SELECT 1 FROM servers t2
//                         WHERE ID = t1.ID + 1
//                     )
//                 ").unwrap_or_else(|error| {
//                     fatal!("Could not acquire the lowest missing primary key. Error: {error}")
//                 })
//                 .query_row((), |row| {
//                     row.get(0)
//                 }).unwrap_or_else(|error| {
//                     fatal!("Could not acquire the lowest missing primary key. Error: {error}")
//                 });

//             primary_key
//         }

//         let server_list = ServerList::server_list().await;
//         let database = server_list.database.lock().await;
//         let primary_key = next_primary_key(&database).await;

//         // database.execute(
//         //     "INSERT INTO servers (
//         //         id,
//         //         name,
//         //         args,
//         //         download_from,
//         //         restart_time,
//         //         server_type,
//         //     ) VALUES (
//         //         ?1,
//         //         ?2,
//         //         ?3,
//         //         ?4,
//         //         ?5,
//         //         ?6,
//         //     )",
//         //     server.to_add_params(primary_key)?
//         // ).unwrap_or_else(|error| {
//         //     fatal!("Could not add an server to the database. Error: {error}")
//         // });

//         *server_list.last_update.lock().await = Utc::now();

//         Ok(())
//     }
//     pub async fn change(id: usize, server: ServerData) -> Result<(), MCManageError> {
//         let server_list = ServerList::server_list().await;
//         let database = server_list.database.lock().await;

//         let mut sql_call = vec![
//             "UPDATE servers".into(),
//             "SET".into()
//         ];

//         if let Some(name) = server.name {
//             sql_call.push(format!("name = {name}"));
//         }
//         if let Some(args) = server.args {
//             sql_call.push(format!("args = {args}"));
//         }
//         if let Some(download_from) = server.download_from {
//             sql_call.push(format!("download_from = {download_from}"));
//         }
//         if let Some(restart_time) = server.restart_time {
//             let restart_time = restart_time.as_secs();
//             sql_call.push(format!("restart_time = {restart_time}"));
//         }
//         if let Some(server_type) = server.server_type {
//             sql_call.push(format!("server_type = {server_type}"));
//         }

//         let id;
//         if let Some(value) = server.id {
//             id  = value;
//         } else {
//             return Err(MCManageError::InvalidRequest("You need to specify the id of the server to change.".into()));
//         }
//         sql_call.push(format!("WHERE id = {id}"));

//         if let Err(error) = database.execute(&sql_call.join("\n"), ()) {
//             Err(MCManageError::InvalidRequest(format!("Could not change the server with the id `{id}`. Error: {error}")))
//         } else {
//             *server_list.last_update.lock().await = Utc::now();
//             Ok(())
//         }
//     }
//     pub async fn remove(id: usize) -> Result<(), MCManageError> {
//         let server_list = ServerList::server_list().await;
//         let database = server_list.database.lock().await;

//         if let Err(error) = database
//             .execute("
//                 DELETE FROM servers
//                     WHERE id = ?1
//             ",
//             params![id])
//         {
//             Err(MCManageError::InvalidRequest(format!("Could not remove the server with the id `{id}`. Error: {error}")))
//         } else {
//             *server_list.last_update.lock().await = Utc::now();
//             Ok(())
//         }
//     }
// }

// // get data
// impl ServerList {
//     pub async fn server_count() -> usize {
//         let server_list = ServerList::server_list().await;
//         server_list.list.len()
//     }
//     pub async fn servers<'a>() -> Arc<Vec<Server>> {
//         Self::server_list().await.list.clone()
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    fn t() {
        
    }
}