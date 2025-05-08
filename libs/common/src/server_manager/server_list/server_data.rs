use std::time::Duration;

use crate::mcmanage_error::MCManageError;

pub struct ServerData {
    pub id: Option<usize>,

    pub name: Option<String>,
    /// These are the args passed to the 'java' command.
    /// That means that this Minecraft server will be started using the command 'java -jar purpur-1.19.3-1876.jar nogui'
    ///
    /// Note: When specifying a ram limit like '-Xmx=4G', the Minecraft server will likely fail to start.
    pub args: Option<String>,
    /// This is a link from which the Minecraft server should be downloaded if none can be found.
    /// A download can be avoided by leaving this field empty. (For example: download_from = "")
    pub download_from: Option<String>,
    /// This is the type of the Minecraft server. Depending on what value got set,
    /// the application will register events like the joining of a player based on different log messages.
    /// See the 'config/server_types.toml' file for all available types.
    pub server_type: Option<String>,
    /// This is the amount of time the application should wait between restarts of this Minecraft server.
    /// If both the secs and nanos values are 0, no restarts will be performed.
    pub restart_time: Option<Duration>,
}
// impl ServerData {
//     pub fn to_add_params(&self, id: usize) -> Result<(usize, String, String, Option<String>, String, Option<u64>), MCManageError> {
//         let name = self.name.clone().unwrap_or("Minecraft Server".into());
//         let args;
//         let server_type = self.server_type.clone().unwrap_or("vanilla".into());

//         if let Some(value) = self.args.clone() {
//             args = value;
//         } else {
//             return Err(MCManageError::InvalidRequest("Can not add a Minecraft server without any arguments specified.".into()));
//         }

//         Ok((
//             id.clone(),
//             name.clone(),
//             args.clone(),
//             self.download_from.clone(),
//             server_type.clone(),
//             self.restart_time.and_then(|restart_time| restart_time.as_secs().into()).clone()
//         ))
//     }
// }