use common::rest_api::server_data::ServerData;
use yew::prelude::*;
use yew_router::Switch;

use self::{
    control::Control,
    info::Info,
    navbar::NavBar,
    players::Players,
};
use crate::servers::{
    server_list::ServerList,
    server::server_route::{
        switch_server,
        ServerRoute,
    },
};

mod console;
mod control;
mod info;
pub mod server_route;
mod navbar;
mod players;
mod styles;

#[derive(PartialEq, Properties)]
pub struct ServerProps {
    pub server_name: String,
}

#[function_component]
pub fn Server(props: &ServerProps) -> Html {
    let server_list = use_context::<ServerList>().unwrap();

    html! {
        <div class={styles::container()}>
            <div class={styles::info()}>
                <Info server_name={props.server_name.clone()} />
            </div>
            <div class={styles::control()}>
                <Control server_name={props.server_name.clone()} />
            </div>

            <div class={styles::navbar()}>
                // <NavBar server_name={props.server_name.clone()} />
            </div>
            <div class={styles::content()}>
                <Switch<ServerRoute> render={move |route| {switch_server(route, server_list.clone())}} />
            </div>

            <div class={styles::players()}>
                // <Players server_name={props.server_name.clone()} />
            </div>
        </div>
    }
}
