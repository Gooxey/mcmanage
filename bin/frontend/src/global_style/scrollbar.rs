//! This module provides a [`function_component`] which modifies the websites scrollbar.

use stylist::{
    css,
    yew::Global,
};
use yew::prelude::*;

/// A custom scrollbar
#[function_component]
pub fn ScrollBar() -> Html {
    html! {
        <Global css={css!("
            /* width */
            ::-webkit-scrollbar {
                width: 7px;
            }

            /* Track */
            ::-webkit-scrollbar-track {
                background: none;
            }

            /* Handle */
            ::-webkit-scrollbar-thumb {
                border-radius: 4px;
                background: var(--color-secondary-text);
            }

            /* Handle on hover */
            ::-webkit-scrollbar-thumb:hover {
                background: var(--color-text);
            }
        ")} />
    }
}
