//! This module provides the [`MCServerType struct`](MCServerType), which is used to read the `config/mcserver_types.json` file and provide the [`MCServer`](super::MCServer) with strings
//! corresponding to different situations, like a player joining or leaving.


use async_recursion::async_recursion;
use serde_json::Value;

use crate::{
    mcmanage_error::MCManageError,
    qol::load_json_file::{
        load_json_file,
        replace_with_valid_file
    }
};
use self::mcserver_types_default::MCSERVER_TYPES_DEFAULT;


mod tests;
pub mod mcserver_types_default;


/// With this struct, the [`MCServer`](super::MCServer) is able to interpret messages sent by a Minecraft server. \
/// To be exact, this struct is responsible for reading the `config/mcserver_types.json` file and providing the [`MCServer`](super::MCServer) with strings corresponding to 
/// different situations, like a player joining or leaving.
/// 
/// # Methods
/// 
/// | Method                                                                               | Description                                                  |
/// |--------------------------------------------------------------------------------------|--------------------------------------------------------------|
/// | [`new(...) -> Self`](MCServerType::new)                                              | Create a new [`MCServerType`](MCServerType).                 |
/// |                                                                                      |                                                              |
/// | [`get_started(...) -> Result<...>`](MCServerType::get_started)                       | Get this Minecraft server types started message.             |
/// | [`get_player_joined(...) -> Result<...>`](MCServerType::get_player_joined)           | Get this Minecraft server types player joined message.       |
/// | [`get_player_left(...) -> Result<...>`](MCServerType::get_player_left)               | Get this Minecraft server types player left message.         |
/// | [`get_player_name_joined(...) -> Result<...>`](MCServerType::get_player_name_joined) | Get the name of the player that joined in the line provided. |
/// | [`get_player_name_left(...) -> Result<...>`](MCServerType::get_player_name_left)     | Get the name of the player that left in the line provided.   |
#[derive(Clone)]
pub struct MCServerType {
    /// The type of the [`MCServer`](super::MCServer) holding this struct
    server_type: String,
    /// The name of the [`MCServer`](super::MCServer) holding this struct
    parent: String
}
impl MCServerType {
    /// Create a new [`MCServerType`].
    /// 
    /// # Parameters
    /// 
    /// | Parameter           | Description                                                                                                                                                                                      |
    /// |---------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `server_type: &str` | To see all available options see the `config/mcserver_types.json` file. To see the standard options see the [`MCSERVER_TYPES_DEFAULT constant`](mcserver_types_default::MCSERVER_TYPES_DEFAULT). |
    /// | `parent: &str`      | The name of the [`MCServer`](super::MCServer) this [`MCServerType`] was meant for.                                                                                                               |
    pub fn new(server_type: &str, parent: &str) -> Self {
        Self {
            server_type: server_type.to_string(),
            parent: parent.to_string()
        }
    }

    /// Get a message from the `config/mcserver_types.json` file, which can be found under this MCServer's type ( vanilla, purpur, etc. ) and its
    /// identifier ( started, player_joined, etc. ). \
    /// \
    /// This method only works if the message to get is a single string. For messages containing multiple strings, use the
    /// [`get_message_vector method`](Self::get_message_vector).
    fn get_message(&self, identifier: &str) -> Result<Value, MCManageError> {
        let mcserver_type_json = load_json_file(
            &self.parent,
            "config",
            "mcserver_types",
            MCSERVER_TYPES_DEFAULT,
            true
        )?;

        // get the json of a provided server type
        if let Some(server) = mcserver_type_json.get(&self.server_type) {
            if let Some(message) = server.get(identifier) {
                Ok(message.to_owned())
            } else {
                replace_with_valid_file(MCSERVER_TYPES_DEFAULT, "config", "mcserver_types");
                self.get_message(identifier)
            }
        } else {
            Err(MCManageError::NotFound)
        }
    }
    /// Get a message from the `config/mcserver_types.json` file, which can be found under this MCServer's type ( vanilla, purpur, etc. ) and its
    /// identifier ( started, player_joined, etc. ). \
    /// \
    /// This method is only useful if the message to be retrieved contains multiple strings. For messages containing a single string, use the
    /// [`get_message method`](Self::get_message).
    fn get_message_vector(&self, identifier: &str) -> Result<Vec<String>, MCManageError> {
        // convert the message got into a vector of strings and return it
        let mut final_vec: Vec<String> = vec![];
        if let Some (vec) = self.get_message(identifier)?.as_array() {
            for item in vec {
                if let Some(string) = item.as_str() {
                    final_vec.push(string.to_string());
                } else {
                    replace_with_valid_file(MCSERVER_TYPES_DEFAULT, "config", "mcserver_types");
                    return self.get_message_vector(identifier);
                }
            }
            Ok(final_vec)
        } else {
            replace_with_valid_file(MCSERVER_TYPES_DEFAULT, "config", "mcserver_types");
            self.get_message_vector(identifier)
        }
    }
    
    /// Get this Minecraft server types started message.
    pub async fn get_started(&self) -> Result<Vec<String>, MCManageError> {
        self.get_message_vector("started")
    }
    /// Get this Minecraft server types player joined message.
    pub async fn get_player_joined(&self) -> Result<Vec<String>, MCManageError> {
        self.get_message_vector("player_joined")
    }
    /// Get this Minecraft server types player left message.
    pub async fn get_player_left(&self) -> Result<Vec<String>, MCManageError> {
        self.get_message_vector("player_left")
    }

    /// Get the name of the player that joined in the line provided.
    #[async_recursion]
    pub async fn get_player_name_joined(&self, line: &str) -> Result<String, MCManageError> {
        let player_name_pos;
        if let Some(pos) = self.get_message("player_name_joined_pos")?.as_u64() {
            player_name_pos = pos;
        } else {
            replace_with_valid_file(MCSERVER_TYPES_DEFAULT, "config", "mcserver_types");
            return self.get_player_name_joined(line).await;
        }

        let mut i: u64 = 0;
        let mut line_iter = line.split(' ').map(String::from);
        loop {
            if i >= player_name_pos {
                break;
            }

            line_iter.next();

            i += 1;
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
        if let Some(pos) = self.get_message("player_name_left_pos")?.as_u64() {
            player_name_pos = pos;
        } else {
            replace_with_valid_file(MCSERVER_TYPES_DEFAULT, "config", "mcserver_types");
            return self.get_player_name_left(line).await;
        }

        let mut i: u64 = 0;
        let mut line_iter = line.split(' ').map(String::from);
        loop {
            if i >= player_name_pos {
                break;
            }

            line_iter.next();

            i += 1;
        }

        if let Some(player_name) = line_iter.next() {
            Ok(player_name)
        } else {
            Err(MCManageError::NotFound)
        }
    }
}