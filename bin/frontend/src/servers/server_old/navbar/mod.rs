use common::rest_api::server_data::ServerData;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::servers::server::server_route::ServerRoute;

mod styles;

#[derive(PartialEq, Properties)]
pub struct NavBarProps {
    pub server_data: ServerData,
}

#[function_component]
pub fn NavBar(props: &NavBarProps) -> Html {
    let navigator = use_navigator().unwrap();
    let current_route: ServerRoute = use_route().unwrap();

    html! {
        <ul class={styles::nav_bar()}>
            <span
                class={
                    if let ServerRoute::Console { .. } = current_route {
                        classes!("nav_item", "nav_item_selected")
                    } else {
                        classes!("nav_item")
                    }
                }
                onclick={
                    let navigator = navigator.clone();
                    let server_name = props.server_data.name.clone();

                    move |_| {
                        navigator.push(&ServerRoute::Console {server: server_name.clone()})
                    }
                }
            >{"Console"}</span>
            <span
                class={
                    if let ServerRoute::Settings{ .. } = current_route {
                        classes!("nav_item", "nav_item_selected")
                    } else {
                        classes!("nav_item")
                    }
                }
                onclick={
                    let navigator = navigator.clone();
                    let server_name = props.server_data.name.clone();

                    move |_| {
                        navigator.push(&ServerRoute::Settings {server: server_name.clone()})
                    }
                }
            >{"Settings"}</span>
        </ul>
    }
}
