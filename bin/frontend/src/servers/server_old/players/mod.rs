use common::rest_api::server_data::ServerData;
use yew::prelude::*;

mod styles;

#[derive(PartialEq, Properties)]
pub struct PlayersProps {
    pub server_data: ServerData,
}

#[function_component]
pub fn Players(props: &PlayersProps) -> Html {
    html! {
        <>
            <div class={styles::title_bar()}>
                <span class={styles::title()}>{"Players"}</span>
                <span class={styles::player_count()}>{format!("{}/{}", props.server_data.player_count, props.server_data.player_cap)}</span>
            </div>

            <br />

            <ul class={styles::player_list()}>
                <li>{"Gooxey"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"otehduisntoheudinthodeuitnhdoeuinthodeuintd"}</li>
                <li>{"Steve"}</li>
            </ul>
        </>
    }
}
