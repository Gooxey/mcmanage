use stylist::{
    css,
    StyleSource,
};

pub fn title_bar() -> StyleSource {
    css!(
        "
        display: flex;

        padding-right: var(--border-radius);
    "
    )
}
pub fn title() -> StyleSource {
    css!(
        "
        flex-grow: 1;

        font-size: larger;
        font-weight: bolder;
    "
    )
}
pub fn player_count() -> StyleSource {
    css!(
        "
        font-size: larger;
        color: var(--color-secondary-text);
    "
    )
}
pub fn player_list() -> StyleSource {
    css!(
        "
        margin: 0;
        padding: 0;
        padding-right: var(--border-radius);

        overflow-y: auto;
        overflow-wrap: break-word;

        list-style-type: none;


        li + li {
            margin-top: 8px;
        }
    "
    )
}
