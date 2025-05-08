use gloo_timers::callback::Interval;
use goolog::fatal;
use yew::prelude::*;

use crate::{
    globals::{
        ServerListContext,
        UPDATE_INTERVAL,
    },
    server_list::ServerListAction,
    servers::server::item::Item,
};

mod server;

#[function_component]
pub fn Servers() -> Html {
    let server_list = use_context::<ServerListContext>().unwrap_or_else(|| {
        fatal!(
            "Servers",
            "The server list should be available through the context."
        )
    });

    use_effect_with_deps(
        {
            let server_list = server_list.clone();
            move |_| {
                // initial load
                server_list.dispatch(ServerListAction::SetHandle(server_list.clone()));
                server_list.dispatch(ServerListAction::Update);

                let interval = Interval::new(UPDATE_INTERVAL, move || {
                    server_list.dispatch(ServerListAction::Update)
                });

                // prevent dropping of the interval by dropping it manually once it is done
                move || drop(interval)
            }
        },
        (),
    );

    html! {
        <>
            {
                for server_list.list().iter().map(|server| {
                    html! {
                        // {server}
                        <Item server={server.clone()} />
                    }
                })
            }
        </>
    }
}
