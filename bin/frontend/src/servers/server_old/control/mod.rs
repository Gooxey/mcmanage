use common::{
    rest_api::server_data::ServerData,
    status::Status,
};
use yew::prelude::*;

use crate::global_elements::status_circle::StatusCircle;

mod styles;

pub struct Control;

pub enum ControlMsg {
    UpdateStatus(Status),
}

#[derive(PartialEq, Properties)]
pub struct ControlProps {
    pub server_name: String,
}

impl Component for Control {
    type Message = ControlMsg;
    type Properties = ControlProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, message: Self::Message) -> bool {
        match message {
            ControlMsg::UpdateStatus(status) => {
                // self.server_data.status = status;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                // <div class={styles::status()}>
                //     <StatusCircle server_data={self.server_data.clone()} />
                //     <span>{format!("{:?}", self.server_data.status)}</span>
                // </div>

                // <br />

                // {
                //     match self.server_data.status {
                //         Status::Stopped => {
                //             html! {
                //                 <button
                //                     onclick={ctx.link().callback(|_| ControlMsg::UpdateStatus(Status::Started))}
                //                     class={classes!(styles::button(), styles::server_disabled())}
                //                 >{"Disabled"}</button>
                //             }
                //         }
                //         Status::Stopping => {
                //             html! {
                //                 <button
                //                     onclick={ctx.link().callback(|_| ControlMsg::UpdateStatus(Status::Started))}
                //                     class={classes!(styles::button(), styles::server_disabled())}
                //                 >{"Disabled"}</button>
                //             }
                //         }
                //         _ => {
                //             html! {
                //                 <button
                //                     onclick={ctx.link().callback(|_| ControlMsg::UpdateStatus(Status::Stopped))}
                //                     class={classes!(styles::button(), styles::server_enabled())}
                //                 >{"Enabled"}</button>
                //             }
                //         }
                //     }
                // }
                // {
                //     if let Status::Started = self.server_data.status {
                //         html! {
                //             <button
                //                 onclick={ctx.link().callback(|_| ControlMsg::UpdateStatus(Status::Restarting))}
                //                 class={classes!(styles::button(), styles::reboot_available())}
                //             >{"Reboot"}</button>
                //         }
                //     } else {
                //         html! {
                //             <button class={classes!(styles::button(), styles::unavailable())}>{"Reboot"}</button>
                //         }
                //     }
                // }
                // {
                //     if let Status::Stopped = self.server_data.status {
                //         html! {
                //             <button class={classes!(styles::button(), styles::unavailable())}>{"Abort"}</button>
                //         }
                //     } else {
                //         html! {
                //             <button
                //                 onclick={ctx.link().callback(|_| ControlMsg::UpdateStatus(Status::Stopped))}
                //                 class={classes!(styles::button(), styles::abort_available())}
                //             >{"Abort"}</button>
                //         }
                //     }
                // }
            </>
        }
    }
}
