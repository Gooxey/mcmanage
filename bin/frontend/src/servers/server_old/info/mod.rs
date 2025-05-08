use common::rest_api::server_data::ServerData;
use yew::prelude::*;

mod styles;

#[derive(PartialEq, Properties)]
pub struct InfoProps {
    pub server_name: String,
}

#[function_component]
pub fn Info(props: &InfoProps) -> Html {
    html! {
        <>
            <img src={"/img/logo.svg"} class={styles::image()} />

            <br />

            <div class={styles::name()}>
                <span>{&props.server_name}</span>
            </div>

            <br />

            <div>
                <span>{&props.server_name}</span>
                <br />
                <span>{&props.server_name}</span>
            </div>
        </>
    }
}
