use use_css::{
    stylist::yew::Global,
    use_css,
};
use yew::prelude::*;

use_css!("global_style");

use self::global_classes::GlobalClasses;

mod global_classes;

/// Implement some styles which are apply to the whole website.
#[function_component]
pub fn GlobalStyle() -> Html {
    html! {
        <>
            <Global css={style::colors()} />
            <Global css={style::sizes()} />
            <Global css={style::body_style()} />
            <Global css={style::scrollbar()} />
            <GlobalClasses />
        </>
    }
}
