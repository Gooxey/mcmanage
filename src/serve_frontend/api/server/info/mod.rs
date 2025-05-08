use goohttp::*;

router! {
    info {
        get_list, get;
        get_version, get, ":server";
        get_server_type, get, ":server";
        get_status, get, ":server";
        get_player_count, get, ":server";
        get_player_cap, get, ":server";

        latest_list, get;
        latest_version, get, ":server";
        latest_server_type, get, ":server";
        latest_status, get, ":server";
        latest_player_count, get, ":server";
        latest_player_cap, get, ":server";
    }
}