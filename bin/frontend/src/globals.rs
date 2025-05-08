use std::sync::OnceLock;

use yew::UseReducerHandle;

use crate::server_list::ServerList;

/// Use this one for updating non critical info like the Server list
pub const UPDATE_INTERVAL: u32 = 10_000; // 10 sec
/// Use this one for updating critical info like the Server log
pub const UPDATE_INTERVAL_SHORT: u32 = 1000; // 1 sec

/// The origin part of the current URL.
pub static URL_ORIGIN: OnceLock<String> = OnceLock::new();

pub type ServerListContext = UseReducerHandle<ServerList>;
