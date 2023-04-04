//! This crate builds a yew app, which will be displayed as the interface for the [`MCManage network`](https://github.com/Gooxey/MCManage.git).


#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]


use yew::prelude::*;
use reqwasm::http::Request;
use fern::colors::{
    Color,
    ColoredLevelConfig,
};
use log::info;


/// The main component
pub struct App;
pub enum AppMsg {
    SendHello
}
impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, message: Self::Message) -> bool {
        match message {
            AppMsg::SendHello => {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get("http://127.0.0.1:8080/hello")
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    info!("Got response: {response}");
                });

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <span>{"Hello World!"}</span>
                <button
                    onclick={ctx.link().callback(|_| AppMsg::SendHello)}
                >{"Send Hello to main"}</button>
            </>
        }
    }
}


fn main() {
    console_log::init_with_level(log::Level::Info).unwrap_or_else(|erro| panic!("Failed to setup the logger. Error: {erro}"));
    yew::Renderer::<App>::new().render();
}
