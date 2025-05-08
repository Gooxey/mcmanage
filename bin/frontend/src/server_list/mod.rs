use std::{
    collections::HashMap,
    rc::Rc,
    sync::{OnceLock},
};

use chrono::prelude::*;
use goolog::*;
use tokio::sync::{
    Mutex,
    MutexGuard,
};
use yew::{
    Reducible,
    UseReducerHandle,
};

use self::{
    common::get,
    server_data::ServerData,
};

mod server_data;
#[macro_use]
mod common;

// TODO do the server data

static UPDATING_SERVER_LIST: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
async fn inner_updating_server_list<'a>(caller: &str) -> MutexGuard<'a, Vec<String>> {
    UPDATING_SERVER_LIST
        .get_or_init(|| {
            fatal!(
                caller,
                "The `UPDATING_MCSERVER_LIST` should have been set at the first initialization."
            )
        })
        .lock()
        .await
}

pub enum ServerListAction {
    SetHandle(UseReducerHandle<ServerList>),
    Update,
    UpdateServer(String),
    SetListData {
        last_update: DateTime<Utc>,
        list: Vec<String>,
        servers: HashMap<String, ServerData>,
    },
    SetServerData(ServerData)
}

pub struct ServerList {
    handle: Option<UseReducerHandle<Self>>,
    last_update: Option<DateTime<Utc>>,
    list: Vec<String>,
    servers: HashMap<String, ServerData>,
}
impl ServerList {
    pub fn list(&self) -> &Vec<String> {
        &self.list
    }
    pub fn server(&self, server: &str) -> Option<&ServerData> {
        self.servers.get(server)
    }

    async fn update(
        handle: UseReducerHandle<Self>,
        mut last_update: Option<DateTime<Utc>>,
        mut list: Vec<String>,
        mut servers: HashMap<String, ServerData>,
    ) {
        fn compare_server_lists(
            list: &Vec<String>,
            new_list: &Vec<String>,
        ) -> (Vec<String>, Vec<String>) {
            let mut to_remove = vec![];
            let mut to_add = vec![];

            for server in list {
                if !new_list.contains(server) {
                    to_remove.push(server.to_owned())
                }
            }
            for server in new_list {
                if !list.contains(server) {
                    to_add.push(server.to_owned())
                }
            }

            (to_remove, to_add)
        }

        let mut updating_server_list = inner_updating_server_list("ServerList").await;
        if updating_server_list.contains(&"master".into()) {
            fatal!(
                "ServerList",
                "Only one `ServerList` should be active at a time."
            )
        } else {
            updating_server_list.push("master".into());
            drop(updating_server_list);
        }

        if let Some(new_list) = get(
            &mut last_update,
            "/api/server/info/get_list",
            "/api/server/info/latest_list",
        )
        .await
        {
            let (to_remove, to_add) = compare_server_lists(&list, &new_list);

            // update both lists
            for server in to_remove {
                let server_index = list
                    .binary_search(&server)
                    .expect("The server to_remove should be in the list.");
                list.remove(server_index);
                servers.remove(&server);
            }
            for server in to_add {
                list.push(server.clone());
                servers.insert(server.clone(), ServerData::new(&server, handle.clone()));
            }

            handle.dispatch(ServerListAction::SetListData {
                last_update: last_update
                    .unwrap_or_else(|| {
                        fatal!("ServerList", "The `last_update` should be `Some(...)`")
                    })
                    .clone(),
                list: list.clone(),
                servers: servers.clone(),
            })
        }

        let mut updating_server_list = inner_updating_server_list("ServerList").await;
        let master_index = updating_server_list
            .binary_search(&"master".into())
            .expect("The master string should be in the UPDATING_MCSERVER_LIST.");
        updating_server_list.remove(master_index);
    }
}
impl Reducible for ServerList {
    type Action = ServerListAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ServerListAction::SetHandle(handle) => Self {
                handle: handle.into(),
                last_update: self.last_update.clone(),
                list: self.list.clone(),
                servers: self.servers.clone(),
            }
            .into(),
            ServerListAction::Update => {
                let handle;
                if let Some(value) = self.handle.clone() {
                    handle = value
                } else {
                    fatal!(
                        "ServerList",
                        "Cannot attempt a list update if the handle to this MCSeverList is not set."
                    )
                }
                wasm_bindgen_futures::spawn_local(Self::update(
                    handle,
                    self.last_update.clone(),
                    self.list.clone(),
                    self.servers.clone(),
                ));

                self
            }
            ServerListAction::UpdateServer(server) => {
                info!("ServerList", "Update Server start");

                let server = self.server(&server).unwrap_or_else(|| {
                    fatal!("ServerList", "Cannot update a server which is not registered.")
                });

                wasm_bindgen_futures::spawn_local(ServerData::update(server.clone()));

                self
            }
            ServerListAction::SetListData {
                last_update,
                list,
                servers,
            } => Self {
                handle: self.handle.clone(),
                last_update: last_update.into(),
                list,
                servers,
            }
            .into(),
            ServerListAction::SetServerData(server_data) => Self {
                handle: self.handle.clone(),
                last_update: self.last_update.clone(),
                list: self.list.clone(),
                servers: {
                    info!("ServerList", "set data Server");
                    let server = server_data.name().clone();
                    let mut servers = self.servers.clone();
                    *servers.get_mut(&server).unwrap_or_else(|| {
                        fatal!("ServerList", "Cannot set the data for a server which is not registered.")
                    }) = server_data;

                    servers
                },
            }
            .into(),
        }
    }
}
impl Default for ServerList {
    fn default() -> Self {
        UPDATING_SERVER_LIST.get_or_init(|| vec![].into());

        Self {
            handle: Default::default(),
            last_update: Default::default(),
            list: Default::default(),
            servers: Default::default(),
        }
    }
}
impl PartialEq for ServerList {
    fn eq(&self, other: &Self) -> bool {
        self.last_update == other.last_update
            && self.list == other.list
            && self.servers == other.servers
    }
}
