use common::status::Status;
use yew::prelude::*;
// use yew_hooks::prelude::*;

// use crate::globals::UPDATE_INTERVAL_SHORT;

mod styles;

#[derive(PartialEq, Properties)]
pub struct StatusCircleProps {
    pub server_name: String,
}

#[function_component]
pub fn StatusCircle(_props: &StatusCircleProps) -> Html {
    let status = use_state_eq(|| Status::Stopped);

    // let status_clone = status.clone();
    // let server_name_clone = props.server_name.clone();
    // FIXME agent for the status circle
    // use_interval(
    //     move || {
    //         get(
    //             &status_clone,
    //             &format!("api/server/info/get_status/{server_name_clone}"),
    //         );
    //     },
    //     UPDATE_INTERVAL_SHORT,
    // );

    html! {
        <div class={
            match *status {
                Status::Started => styles::online_circle(),
                Status::Stopped => styles::offline_circle(),
                _ => styles::standby_circle()
            }
        }></div>
    }
}
