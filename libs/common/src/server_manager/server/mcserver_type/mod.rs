//! This module provides the [`ServerType struct`](ServerType), which is used to read the `config/server_types.toml` file and provide the [`Server`](super::Server) with strings
//! corresponding to different situations, like a player joining or leaving.

use std::sync::Arc;

use async_recursion::async_recursion;
use tokio::{
    sync::Mutex,
    time::sleep,
};
use toml::Value;

use goolog::*;

use crate::{
    config::Config,
    generated_files::{
        load_toml_file::{
            load_toml_replace,
            replace_with_valid_file,
        },
        paths::MCSERVER_TYPES_FILE,
    },
    mcmanage_error::MCManageError,
};

mod tests;

/// With this struct, the [`Server`](super::Server) is able to interpret messages sent by a Minecraft server. \
/// To be exact, this struct is responsible for reading the `config/server_types.toml` file and providing the [`Server`](super::Server) with strings corresponding to
/// different situations, like a player joining or leaving.
///
/// # Methods
///
/// | Method                                                                               | Description                                                  |
/// |--------------------------------------------------------------------------------------|--------------------------------------------------------------|
/// | [`new(...) -> Self`](ServerType::new)                                              | Create a new [`ServerType`](ServerType).                 |
/// |                                                                                      |                                                              |
/// | [`get_started(...) -> Result<...>`](ServerType::get_started)                       | Get this Minecraft server types started message.             |
/// | [`get_player_joined(...) -> Result<...>`](ServerType::get_player_joined)           | Get this Minecraft server types player joined message.       |
/// | [`get_player_left(...) -> Result<...>`](ServerType::get_player_left)               | Get this Minecraft server types player left message.         |
/// | [`get_player_name_joined(...) -> Result<...>`](ServerType::get_player_name_joined) | Get the name of the player that joined in the line provided. |
/// | [`get_player_name_left(...) -> Result<...>`](ServerType::get_player_name_left)     | Get the name of the player that left in the line provided.   |
#[derive(Clone)]
pub struct ServerType {
    /// The type of the [`Server`](super::Server) holding this struct
    server_type: String,
    /// The name of the [`Server`](super::Server) holding this struct
    parent: String,
}
impl ServerType {
    /// Create a new [`ServerType`].
    ///
    /// # Parameters
    ///
    /// | Parameter           | Description                                                                        |
    /// |---------------------|------------------------------------------------------------------------------------|
    /// | `server_type: &str` | To see all available options see the `config/server_types.toml` file.            |
    /// | `parent: &str`      | The name of the [`Server`](super::Server) this [`ServerType`] was meant for. |
    pub fn new(server_type: &str, parent: &str) -> Self {
        Self {
            server_type: server_type.to_string(),
            parent: parent.to_string(),
        }
    }

    /// Get a message from the `config/server_types.toml` file, which can be found under this Server's type ( vanilla, purpur, etc. ) and its
    /// identifier ( started, player_joined, etc. ). \
    /// \
    /// This method only works if the message to get is a single string. For messages containing multiple strings, use the
    /// [`get_message_vector method`](Self::get_message_vector).
    #[async_recursion]
    async fn get_message(&self, identifier: &str) -> Value {
        let server_type_toml = load_toml_replace(&MCSERVER_TYPES_FILE, &self.parent, true).await;

        // get the toml of a provided server type
        if let Some(server) = server_type_toml.get(&self.server_type) {
            if let Some(message) = server.get(identifier) {
                message.to_owned()
            } else {
                replace_with_valid_file(&MCSERVER_TYPES_FILE).await;
                self.get_message(identifier).await
            }
        } else {
            error!(
                self.parent,
                "Could not find the server_type {} in the config/server_types.toml file.",
                self.server_type
            );
            error!(
                self.parent,
                "This Server will now be blocked until the server_type {} got added.",
                self.server_type
            );
            error!(
                self.parent,
                "In case you change the server_type for {}, restart this application.",
                self.parent
            );

            loop {
                let server_type_toml =
                    load_toml_replace(&MCSERVER_TYPES_FILE, &self.parent, true).await;

                if server_type_toml.get(&self.server_type).is_some() {
                    return self.get_message(identifier).await;
                }

                // TODO check if the server's type got changed

                sleep(Config::cooldown().await).await;
            }
        }
    }
    /// Get a message from the `config/server_types.toml` file, which can be found under this Server's type ( vanilla, purpur, etc. ) and its
    /// identifier ( started, player_joined, etc. ). \
    /// \
    /// This method is only useful if the message to be retrieved contains multiple strings. For messages containing a single string, use the
    /// [`get_message method`](Self::get_message).
    #[async_recursion]
    async fn get_message_vector(&self, identifier: &str) -> Vec<String> {
        // convert the message got into a vector of strings and return it
        if let Some(vec) = self.get_message(identifier).await.as_array() {
            let mut final_vec: Vec<String> = vec![];
            for item in vec {
                if let Some(string) = item.as_str() {
                    final_vec.push(string.to_string());
                } else {
                    replace_with_valid_file(&MCSERVER_TYPES_FILE).await;
                    return self.get_message_vector(identifier).await;
                }
            }
            final_vec
        } else {
            warn!(self.parent, "Could not find the parameter {identifier} in the config/server_list.toml file. A valid file will be generated.");
            replace_with_valid_file(&MCSERVER_TYPES_FILE).await;
            self.get_message_vector(identifier).await
        }
    }

    /// Get this Minecraft server types started message.
    pub async fn get_started(&self) -> Vec<String> {
        self.get_message_vector("started").await
    }
    /// Get this Minecraft server types player joined message.
    pub async fn get_player_joined(&self) -> Vec<String> {
        self.get_message_vector("player_joined").await
    }
    /// Get this Minecraft server types player left message.
    pub async fn get_player_left(&self) -> Vec<String> {
        self.get_message_vector("player_left").await
    }

    /// Get the name of the player that joined in the line provided.
    #[async_recursion]
    pub async fn get_player_name_joined(&self, line: &str) -> Result<String, MCManageError> {
        let player_name_pos: u64;
        if let Some(pos) = self
            .get_message("player_name_joined_pos")
            .await
            .as_integer()
        {
            if let Ok(pos) = pos.try_into() {
                player_name_pos = pos;
            } else {
                replace_with_valid_file(&MCSERVER_TYPES_FILE).await;
                return self.get_player_name_joined(line).await;
            }
        } else {
            replace_with_valid_file(&MCSERVER_TYPES_FILE).await;
            return self.get_player_name_joined(line).await;
        }

        let mut line_iter = line.split(' ').map(String::from);
        for i in 0.. {
            if i >= player_name_pos {
                break;
            }

            line_iter.next();
        }

        if let Some(player_name) = line_iter.next() {
            Ok(player_name)
        } else {
            Err(MCManageError::NotFound)
        }
    }
    /// Get the name of the player that left in the line provided.
    #[async_recursion]
    pub async fn get_player_name_left(&self, line: &str) -> Result<String, MCManageError> {
        let player_name_pos;
        if let Some(pos) = self.get_message("player_name_left_pos").await.as_integer() {
            if let Ok(pos) = pos.try_into() {
                player_name_pos = pos;
            } else {
                replace_with_valid_file(&MCSERVER_TYPES_FILE).await;
                return self.get_player_name_left(line).await;
            }
        } else {
            replace_with_valid_file(&MCSERVER_TYPES_FILE).await;
            return self.get_player_name_left(line).await;
        }

        let mut line_iter = line.split(' ').map(String::from);
        for i in 0.. {
            if i >= player_name_pos {
                break;
            }

            line_iter.next();
        }

        if let Some(player_name) = line_iter.next() {
            Ok(player_name)
        } else {
            Err(MCManageError::NotFound)
        }
    }
}
impl ToString for ServerType {
    fn to_string(&self) -> String {
        self.server_type.clone()
    }
}
