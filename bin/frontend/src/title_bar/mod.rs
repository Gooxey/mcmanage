use use_css::use_css;
use yew::prelude::*;
use yew_nested_router::{
    components::Link,
    prelude::use_router,
};

use crate::routes::{
    AppRoute,
};

use_css!("title_bar");

#[function_component]
pub fn TitleBar() -> Html {
    let router = use_router::<AppRoute>().unwrap();
    let current_route = router.active();

    html! {
        <>
            <div class={style::titlebar()}>
                <div class={style::left_container()}>
                    <img class={style::logo()} src={"/img/logo.svg"} />
                    <span class={style::title()}>{"MCManage"}</span>
                </div>

                <div class={style::middle_container()}>
                    <Link<AppRoute> target={AppRoute::Servers}>
                        <span
                            class={
                                match current_route {
                                    Some(AppRoute::Servers) |
                                    Some(AppRoute::Server {..}) => {
                                        classes!("nav_item", "nav_item_selected")
                                    }
                                    _ => {
                                        classes!("nav_item")
                                    }
                                }
                            }
                        >{"Servers"}</span>
                    </Link<AppRoute>>
                    <Link<AppRoute> target={AppRoute::Settings}>
                        <span
                            class={
                                if let Some(AppRoute::Settings) = current_route {
                                    classes!("nav_item", "nav_item_selected")
                                } else {
                                    classes!("nav_item")
                                }
                            }
                        >{"Settings"}</span>
                    </Link<AppRoute>>
                </div>

                <div class={style::right_container()}>
                    <span>{"Admin"}</span>
                    <img class={style::user_icon()} src={"/img/user-circle.svg"} />
                </div>
            </div>
            <div class={style::placeholder()}></div>
        </>
    }
}
