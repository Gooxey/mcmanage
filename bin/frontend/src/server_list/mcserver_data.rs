use common::status::Status;
use goolog::*;
use yew::{UseReducerHandle};

use crate::server_list::{
    common::get,
    inner_updating_server_list, ServerListAction,
};

use chrono::prelude::*;
use super::ServerList;
use derive_getters::Getters;

macro_rules! get_property {
    ($self:ident, $property:expr, $changed_data:ident) => {
        paste::paste! {
            info!($self.name, "Updating {}", std::stringify!($property));
            if let Some(new_data) = get(
                &mut $self.last_update.[< $property >] ,
                &format!("/api/server/info/get_{}/{}", std::stringify!($property), $self.name),
                &format!("/api/server/info/latest_{}/{}", std::stringify!($property), $self.name),
            )
            .await {
                $changed_data = true;
                $self.[< $property >] = new_data;
            }
        }
    };
}

#[derive(Default, PartialEq, Clone)]
struct LastUpdate {
    version: Option<DateTime<Utc>>,
    server_type: Option<DateTime<Utc>>,
    status: Option<DateTime<Utc>>,
    player_count: Option<DateTime<Utc>>,
    player_cap: Option<DateTime<Utc>>,
}

#[derive(PartialEq, Clone, Getters)]
pub struct ServerData {
    #[getter(skip)]
    server_list: UseReducerHandle<ServerList>,
    #[getter(skip)]
    last_update: LastUpdate,
    /// The Name of the Minecraft server.
    name: String,
    /// The version of the Minecraft server.
    version: String,
    /// The [`type`](ServerType) of the Minecraft server.
    server_type: String,
    /// The [`Status`] of the Minecraft server.
    status: Status,
    /// The number of players currently on the Minecraft server.
    player_count: u64,
    /// The maximum amount of players allowed on the Minecraft server.
    player_cap: u64,
}
impl ServerData {
    pub fn new(name: &String, server_list: UseReducerHandle<ServerList>) -> Self {
        Self {
            server_list,
            last_update: Default::default(),
            name: name.into(),
            version: Default::default(),
            server_type: Default::default(),
            status: Default::default(),
            player_count: Default::default(),
            player_cap: Default::default(),
        }
    }

    pub(super) async fn update(mut self) {
        info!(self.name, "updating...");

        let mut updating_server_list = inner_updating_server_list(&self.name).await;
        if updating_server_list.contains(&self.name) {
            fatal!(
                self.name,
                "Only one `ServerData` of the same server should be active at a time."
            )
        } else if updating_server_list.contains(&"master".into()) {
            // It makes no sense to update if the server list is updating
            return;
        }  else {
            updating_server_list.push(self.name.clone());
            drop(updating_server_list);
        }

        let mut changed_data = false;

        get_property!(self, version, changed_data);
        get_property!(self, server_type, changed_data);
        get_property!(self, status, changed_data);
        get_property!(self, player_count, changed_data);
        get_property!(self, player_cap, changed_data);

        info!(self.name, "done");

        if changed_data {
            self.server_list.dispatch(ServerListAction::SetServerData(self.clone()));
        }
    }
}