//! This module provides a [`function_component`] which makes some css classes public to the whole website.

use use_css::{
    stylist::yew::Global,
    use_css,
};
use yew::prelude::*;

use_css!("global_style/global_classes");

/// Make some css classes public to the whole website.
///
/// # Public Classes
///
/// | Class               | Description                          |
/// |---------------------|--------------------------------------|
/// | `select`            | Allows the user to select text. |
/// | `nav_item`          | The style for a span in a navigation bar. |
/// | `nav_item_selected` | The style for a span in a navigation bar which is currently selected. This needs to be used with the `nav_item` style. |
#[function_component]
pub fn GlobalClasses() -> Html {
    html! {
        <>
            <Global css={style::classes()} />
        </>
    }
}
// var(--color-background)
