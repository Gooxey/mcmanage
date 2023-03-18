//! This module provides the [`MCSERVER_TYPES_DEFAULT constant`](MCSERVER_TYPES_DEFAULT), which represents the default text in the `config/mcserver_types.json` file.


/// This constant represents the default text in the `config/mcserver_types.json` file. \
///  \
/// Note: When specifying the position of the player names, you have to count from 0. ( For instance: '0, 1, 2, ...' )
pub const MCSERVER_TYPES_DEFAULT: &str = "{
    \"vanilla\": {
        \"started\": [\"] [Server thread/INFO]: Done (\", \")! For help, type \\\"help\\\"\"],
        \"player_joined\": [\" joined the game\"],
        \"player_left\": [\"left the game\"],
        \"player_name_joined_pos\": 2,
        \"player_name_left_pos\": 2
    },
    \"purpur\": {
        \"started\": [\" INFO]: Done (\", \")! For help, type \\\"help\\\"\"],
        \"player_joined\": [\" joined the game\"],
        \"player_left\": [\"left the game\"],
        \"player_name_joined_pos\": 2,
        \"player_name_left_pos\": 2
    }
}";