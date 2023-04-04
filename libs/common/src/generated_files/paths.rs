//! This module provides various statics describing the paths to specific files generated by the MCManage applications.

use std::{
    env::current_exe,
    fs::create_dir_all,
    path::{
        Component,
        Path,
        PathBuf,
    },
};

use lazy_static::lazy_static;
use tokio::fs;

use super::default_files::get_example_content;

lazy_static! {
    /// The path to the executable at `./`
    pub static ref ROOT_DIR: PathBuf = {
        /// This function will get the workspace's root path.
        fn get_workspace_root() -> PathBuf {
            let root_dir = std::process::Command::new(env!("CARGO"))
                .arg("locate-project")
                .arg("--workspace")
                .arg("--message-format=plain")
                .output()
                .expect("Failed to run the cargo command to get the path to the workspace's root directory.")
                .stdout;
            PathBuf::from(std::str::from_utf8(&root_dir).expect("The `root_dir` generated by cargo should be convertible to a string").trim())
                .parent()
                .expect("Failed to create the path to the testing directory.")
                .to_path_buf()
        }


        if cfg!(test) {
            let root_dir = get_workspace_root().join("testing_files");
            create_dir_all(root_dir.clone()).unwrap_or_else(|erro| panic!("Failed to create the testing directory at '{}'. Error: {erro}", root_dir.display()));

            root_dir
        } else {
            let root_dir = current_exe()
                .unwrap_or_else(|erro| panic!("Could not get the path to the executable. Error: {erro}"))
                .join("..");
            let mut root_dir = dunce::canonicalize(root_dir)
                .unwrap_or_else(|erro| panic!("Failed to canonicalize the ROOT_DIR path. Error: {erro}"));

            // this will prevent the root dir getting set to './target/debug' during development
            for directory in root_dir.components() {
                if directory == Component::Normal("target".as_ref()) {
                    root_dir = get_workspace_root();
                    break;
                }
            }

            root_dir
        }
    };

    /// The path to the config directory at `./config`
    pub static ref CONFIG_DIR: PathBuf = ROOT_DIR.join("config");
    /// The path to the config directory at `./config/config.toml`
    pub static ref CONFIG_FILE: PathBuf = CONFIG_DIR.join("config.toml");
    /// The path to the config directory at `./config/server_list.toml`
    pub static ref SERVER_LIST_FILE: PathBuf = CONFIG_DIR.join("server_list.toml");
    /// The path to the config directory at `./config/mcserver_types.toml`
    pub static ref MCSERVER_TYPES_FILE: PathBuf = CONFIG_DIR.join("mcserver_types.toml");

    /// The path to the logs directory at `./logs`
    pub static ref LOGS_DIR: PathBuf = ROOT_DIR.join("logs");
    /// The path to the server logs directory at `./logs/servers` \
    /// This directory is intended for log files from Minecraft servers.
    pub static ref SERVER_LOGS_DIR: PathBuf = LOGS_DIR.join("servers");
    /// The path to the log file at `./logs/mcmanage.log`
    pub static ref LOG_FILE: PathBuf = LOGS_DIR.join("mcmanage.log");

    /// The path to the share directory at `./share`
    static ref SHARE_DIR: PathBuf = ROOT_DIR.join("share");
    /// The path to the frontend directory at `./share/frontend`
    pub static ref FRONTEND_DIR: PathBuf = SHARE_DIR.join("frontend");

    /// The path to the servers directory at `./servers`
    pub static ref SERVERS_DIR: PathBuf = ROOT_DIR.join("servers");
}

/// This function will generate the [`struct@CONFIG_DIR`] if it does not exist and return a [`PathBuf`] pointing to a non-existing file located at:
///     1. '[`struct@CONFIG_DIR`]/invalid_`{file_name}`.toml', in case no other invalid file could be found.
///     2. '[`struct@CONFIG_DIR`]/invalid_`{file_name}`(`{counter starting at 1}`).toml', in case other invalid files could be found.
///
/// Note: `file_name` would be, for instance, for the static [`struct@SERVER_LIST_FILE`], `server_list`. \
/// \
/// # Panics
///
/// This function will panic in case the given static's file should not be renamed to an 'invalid file'.
pub async fn get_invalid_path(file_path: &Path) -> PathBuf {
    /// This function will return a [`PathBuf`] pointing to a non-existing file. \
    /// See the [`get_invalid_file`] functions documentation for more information on how the returned file path gets called.
    fn get_invalid_name(file_name: &str) -> PathBuf {
        let mut invalid_file_name;
        for i in 0.. {
            if i == 0 {
                invalid_file_name = CONFIG_DIR.join(format!("invalid_{file_name}.toml"));
            } else {
                invalid_file_name = CONFIG_DIR.join(format!("invalid_{file_name}({i}).toml"));
            }
            if !invalid_file_name.exists() {
                return invalid_file_name;
            }
        }
        panic!("A invalid file name should have been found after enough iterations.")
    }

    if !CONFIG_DIR.exists() {
        fs::create_dir_all(CONFIG_DIR.as_path())
            .await
            .unwrap_or_else(|erro| {
                panic!(
                    "An error occurred while trying to create a folder at '{}'. Error: {erro}",
                    CONFIG_DIR.display()
                )
            });
    }

    if file_path == CONFIG_FILE.as_path() {
        get_invalid_name("config")
    } else if file_path == MCSERVER_TYPES_FILE.as_path() {
        get_invalid_name("mcserver_types")
    } else {
        panic!("The given static's file should not be renamed to an 'invalid file'.")
    }
}
/// This function will generate the [`struct@CONFIG_DIR`] if it does not exist and return a [`PathBuf`] pointing to a non-existing file located at:
///     1. '[`struct@CONFIG_DIR`]/invalid_`{file_name}`.toml', in case no other invalid file could be found.
///     2. '[`struct@CONFIG_DIR`]/invalid_`{file_name}`(`{counter starting at 1}`).toml', in case other invalid files could be found.
///
/// Note: `file_name` would be, for instance, for the static [`struct@SERVER_LIST_FILE`], `server_list`. \
/// \
/// # Panics
///
/// This function will panic in case the given static's file should not be renamed to an 'invalid file'.
pub async fn get_example_path(file_path: &Path) -> PathBuf {
    if !CONFIG_DIR.exists() {
        fs::create_dir_all(CONFIG_DIR.as_path())
            .await
            .unwrap_or_else(|erro| {
                panic!(
                    "An error occurred while trying to create a folder at '{}'. Error: {erro}",
                    CONFIG_DIR.display()
                )
            });
    }

    get_example_content(file_path).0
}