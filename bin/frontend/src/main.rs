use goolog::*;
use title_bar::TitleBar;
use yew::{
    prelude::*,
    Renderer,
};
use yew_nested_router::prelude::*;

// use yew_router::prelude::*;
use crate::{
    global_style::GlobalStyle,
    globals::URL_ORIGIN,
};
use crate::{
    globals::ServerListContext,
    routes::AppRoute,
    server_list::ServerList,
};

// pub mod agents;
mod global_elements;
mod global_style;
pub mod globals;
pub mod servers;
// pub mod request;
pub mod routes;
pub mod server_list;
mod title_bar;

// TODO Add animations
// TODO Use only sizes defined in sizes.rs
// TODO add fade ins and outs (see titlebar)

#[function_component]
pub fn App() -> Html {
    let url_origin = web_sys::window()
        .unwrap_or_else(|| {
            fatal!(
                "Main",
                "The `window` struct should be available for all function components."
            )
        })
        .location()
        .origin()
        .unwrap_or_else(|_| fatal!("Main", "The URL origin should be a string."));
    URL_ORIGIN.get_or_init(|| url_origin);

    let server_list = use_reducer_eq(ServerList::default);

    html! {
        <Router<AppRoute>>
            <GlobalStyle />

            <TitleBar />
            <ContextProvider<ServerListContext> context={server_list}>
                { AppRoute::render() }
            </ContextProvider<ServerListContext>>
        </Router<AppRoute>>
    }
}

fn main() {
    init_logger(None, None);

    Renderer::<App>::new().render();
}
