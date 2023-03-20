//! This module provides the [`SERVER_LIST_EXAMPLE_DEFAULT constant`](SERVER_LIST_EXAMPLE_DEFAULT), which represents the default text in the `servers/server_list_example.toml` file.


/// This constant represents the default text in the `servers/server_list_example.toml` file.
pub const SERVER_LIST_EXAMPLE_DEFAULT: &str =
r#"# Restart the application to apply the changes made to the 'config/server_list.toml' file.

# This represents one Minecraft server
# The name in the brackets is also the name of the Minecraft server
# and the name of the folder the Minecraft server lies in. (here the Minecraft server lies in 'servers/myFirstServer')
# Because of the second use case, avoid using spaces or any special characters.
[myFirstServer]
# These are the args passed to the 'java' command.
# That means that this Minecraft server will be started using the command 'java -jar purpur-1.19.3-1876.jar nogui'
#
# Note: When specifying a ram limit like '-Xmx=4G', the Minecraft server will likely fail to start.
args = "-jar purpur-1.19.3-1876.jar nogui"
# This is the type of the Minecraft server. Depending on what value got set,
# the application will register events like the joining of a player based on different log messages.
# See the 'config/mcserver_types.toml' file for all available types.
type = "purpur"

[mySecondServer]
args = "-jar purpur-1.19.3-1876.jar nogui"
type = "purpur""#;