use yew::prelude::*;
use yew_router::prelude::*;

use crate::servers::{
    server_list::ServerList,
    server::console::Console,
    route::ServersRoute,
};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum ServerRoute {
    #[at("/servers/:server/console")]
    Console { server: String },
    #[at("/servers/:server/settings")]
    Settings { server: String },
    #[not_found]
    #[at("/servers/:server/404")]
    NotFound,
}
pub fn switch_server(route: ServerRoute, server_list: ServerList) -> Html {
    match route {
        ServerRoute::Console { server } => {
            for server_name in server_list.iter() {
                if *server_name == server {
                    return html! { <Console /> };
                }
            }

            html! { <Redirect<ServerRoute> to={ServerRoute::NotFound} /> }
        }
        ServerRoute::Settings { server } => {
            for server_name in server_list.iter() {
                if *server_name == server {
                    return html! {"Settings"};
                }
            }

            html! { <Redirect<ServerRoute> to={ServerRoute::NotFound} /> }
        }
        ServerRoute::NotFound => html! { <Redirect<ServersRoute> to={ServersRoute::NotFound} /> },
    }
}
