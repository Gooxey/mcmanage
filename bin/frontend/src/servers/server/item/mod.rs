use goolog::fatal;
use use_css::use_css;
use yew::prelude::*;
use yew_nested_router::components::Link;

use crate::{routes::{
    ServerRoute, AppRoute,
}, globals::ServerListContext, server_list::ServerListAction};

use_css!("servers/server/item");

#[derive(PartialEq, Properties)]
pub struct ItemProps {
    pub server: String,
}

#[function_component]
pub fn Item(props: &ItemProps) -> Html {
    let server_list = use_context::<ServerListContext>().unwrap_or_else(|| {
        fatal!("ServerItem", "The server list should be available through the context.")
    });
    server_list.dispatch(ServerListAction::UpdateServer(props.server.clone()));

    html! {
        <Link<AppRoute> target={AppRoute::Server { server: "Hello".into(), target: ServerRoute::default() }}>
            <div
                class={style::container()}
            >
                <img
                    class={style::image()}
                    src={"/img/logo.svg"}
                />

                <span class={style::name()}>{&props.server}</span>

                <div class={style::data()}>
                    <span>{&props.server}</span>
                    <span>{&props.server}</span>
                </div>

                <div class={style::stats()}>
                    // <div>
                    //     <StatusCircle server_name={props.server_name.clone()} />
                    //     <span>{format!("{:?}", server_data.status)}</span>
                    // </div>
                    // <span>{format!("{}/{}", server_data.player_count, server_data.player_cap)}</span>
                </div>
            </div>
        </Link<AppRoute>>
    }
}
