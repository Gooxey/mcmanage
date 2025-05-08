use stylist::{
    css,
    StyleSource,
};

pub fn container() -> StyleSource {
    css!(
        "
        display: grid;

        grid-template:
            \"info navbar players\"
            \"info content players\"
            \"control content players\";
        grid-template-columns: 250px auto 250px;
        grid-template-rows: 50px 1fr min-content;
        grid-gap: var(--margin-container);

        height: calc(100% - (var(--margin-container) + 85px));
        width: calc(100% - var(--margin-container) * 2);

        margin: var(--margin-container);
        margin-top: 0;
    "
    )
}

pub fn info() -> StyleSource {
    css!(
        "
        grid-area: info;

        display: flex;
        flex-direction: column;

        padding: var(--border-radius);

        background-color: var(--color-primary);
        border-radius: var(--border-radius);
    "
    )
}
pub fn control() -> StyleSource {
    css!(
        "
        grid-area: control;

        display: flex;
        flex-direction: column;

        padding: var(--border-radius);

        background-color: var(--color-primary);
        border-radius: var(--border-radius);
    "
    )
}
pub fn navbar() -> StyleSource {
    css!(
        "
        grid-area: navbar;

        display: flex;

        align-items: center;

        background-color: var(--color-primary);
        border-radius: var(--border-radius);
    "
    )
}
pub fn content() -> StyleSource {
    css!(
        "
        grid-area: content;

        background-color: var(--color-primary);
        border-radius: var(--border-radius);
    "
    )
}
pub fn players() -> StyleSource {
    css!(
        "
        display: flex;
        flex-direction: column;

        grid-area: players;

        padding: var(--border-radius);
        padding-right: 0;

        background-color: var(--color-primary);
        border-radius: var(--border-radius);
    "
    )
}
