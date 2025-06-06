//! This module provides the [`EXAMPLE_SERVER_LIST`] constant.

/// This constant represents the `example file` content of the [`SERVER_LIST_FILE`](crate::crate::generated_files::paths::SERVER_LIST_FILE) file.
pub const EXAMPLE_SERVER_LIST: &str = r#"# Restart the application to apply the changes made to the 'config/server_list.toml' file.

# This represents one Minecraft server
# The name in the brackets is also the name of the Minecraft server
# and the name of the folder the Minecraft server lies in. (here the Minecraft server lies in 'servers/myFirstServer')
# Because of the second use case, avoid using spaces or any special characters.
[myFirstServer]
# These are the args passed to the 'java' command.
# That means that this Minecraft server will be started using the command 'java -jar purpur-1.19.3-1876.jar nogui'
#
# Note: When specifying a ram limit like '-Xmx=4G', the Minecraft server will likely fail to start.
args = "-jar purpur-1.19.3-1933.jar nogui"
# This is a link from which the Minecraft server should be downloaded if none can be found.
# A download can be avoided by leaving this field empty. (For example: download_from = "")
download_from = "https://api.purpurmc.org/v2/purpur/1.19.3/1933/download"
# This is the type of the Minecraft server. Depending on what value got set,
# the application will register events like the joining of a player based on different log messages.
# See the 'config/server_types.toml' file for all available types.
server_type = "purpur"
# This is the amount of time the application should wait between restarts of this Minecraft server.
# If both the secs and nanos values are 0, no restarts will be performed.
[myFirstServer.restart_time]
secs = 86400
nanos = 0

[mySecondServer]
args = "-jar purpur-1.19.3-1933.jar nogui"
download_from = "https://api.purpurmc.org/v2/purpur/1.19.3/1933/download"
server_type = "purpur"
[mySecondServer.restart_time]
secs = 86400
nanos = 0"#;
