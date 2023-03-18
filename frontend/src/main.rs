//! This crate builds a yew app, which will be displayed as the interface for the [`MCManage network`](https://github.com/Gooxey/MCManage.git).


#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]


use yew::prelude::*;


/// The main component
pub struct App;
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            {"Hello World!"}
        }
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
