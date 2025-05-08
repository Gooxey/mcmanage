use std::sync::OnceLock;

use goolog::fatal;
use pg_embed::{postgres::{
    PgEmbed,
    PgSettings,
}, pg_fetch::{PG_V13, PgFetchSettings}, pg_enums::PgAuthMethod};
use rand::{Rng, distributions::Alphanumeric};
use tokio::sync::Mutex;

use crate::{config::Config, generated_files::paths::DATA_DIR};

const GOOLOG_CALLER: &str = "ConfigDB";

static DATABASE_SERVER: OnceLock<Mutex<PgEmbed>> = OnceLock::new();
pub static DATABASE_PASSWORD: OnceLock<String> = OnceLock::new();

/// Run a postgresql database for:
///     1. The [`Config`](crate::config::Config)
///     2. The [`ServerList`](crate::server_manager::server_list::ServerList)
pub async fn init_db_server() {
    if DATABASE_PASSWORD.set(
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    ).is_err() {
        fatal!("Cannot be initialized more than once.")
    }

    let mut database = PgEmbed::new(
        PgSettings {
            database_dir: DATA_DIR.clone(),
            port: Config::database_port().await,
            user: "mcmanage".into(),
            password: DATABASE_PASSWORD.get().unwrap_or_else(|| {
                fatal!("The password should have been set by now.")
            }).into(),
            auth_method: PgAuthMethod::Plain,
            persistent: true,
            timeout: None,
            migration_dir: None,
        },
        PgFetchSettings {
            version: PG_V13,
            ..Default::default()
        }
    )
    .await
    .unwrap_or_else(|error| {
        fatal!("Failed to configure. Error: {error}");
    });

    // start the database
    database.setup().await;
    database.start_db().await;

    // add missing databases
    if !database.database_exists("config").await.unwrap_or_else(|error| {
        fatal!("Could not check if the config database exists. Error: {error}")
    }) {
        database.create_database("config").await;
    }
    if !database.database_exists("server_types").await.unwrap_or_else(|error| {
        fatal!("Could not check if the server_list database exists. Error: {error}")
    }) {
        database.create_database("server_list").await;
    }
    if !database.database_exists("server_list").await.unwrap_or_else(|error| {
        fatal!("Could not check if the server_list database exists. Error: {error}")
    }) {
        database.create_database("server_list").await;
    }

    DATABASE_SERVER
        .set(database.into())
        .unwrap_or_else(|_| {
            fatal!("The database server should be initialized only once.")
        });
}

pub async fn stop_db_server() {
    DATABASE_SERVER
        .get()
        .unwrap_or_else(|| {
            fatal!("Not yet initialized.")
        })
        .lock()
        .await
        .stop_db()
        .await
        .unwrap_or_else(|error| {
            fatal!("Failed to stop. Error: {error}")
        })
}