//! This module provides the [`MCSERVER_TYPES_DEFAULT constant`](MCSERVER_TYPES_DEFAULT), which represents the default text in the `config/mcserver_types.toml` file.


/// This constant represents the default text in the `config/mcserver_types.toml` file.
pub const MCSERVER_TYPES_DEFAULT: &str =
r#"# Restart the application to apply the changes made to the 'config/mcserver_types.toml' file.

# This file describes all mcserver_types that can be used in the 'config/server_list.toml' file.
# They are very important for the application because it registers all events of a Minecraft server via the console.
#
# In case you want to define your own mcserver_type you need to follow fill all of the following fields:
#
# [new_name_here]               => This is the name of the mcserver_type.
# started = [""]                => This is a list of every string the started message has to contain.
# player_joined = [""]          => This is a list of every string the player-joined message has to contain.
# player_left = [""]            => This is a list of every string the player-left message has to contain.
# player_name_joined_pos = 2    => This is the position, starting from 0, of the player name in the player-left message.
#                                  For the message '[13:53:51 INFO]: Gooxey joined the game' the player name is at the position 2.
# player_name_left_pos = 2      => This is the position, starting from 0, of the player name in the player-joined message.
#                                  For the message '[13:53:51 INFO]: Gooxey left the game' the player name is at the position 2.


[vanilla]
started = ["] [Server thread/INFO]: Done (", ")! For help, type \"help\""]
player_joined = [" joined the game"]
player_left = ["left the game"]
player_name_joined_pos = 2
player_name_left_pos = 2

[purpur]
started = [" INFO]: Done (", ")! For help, type \"help\""]
player_joined = [" joined the game"]
player_left = ["left the game"]
player_name_joined_pos = 2
player_name_left_pos = 2"#;