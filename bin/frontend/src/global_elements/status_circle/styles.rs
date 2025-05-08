use stylist::{
    css,
    StyleSource,
};

pub fn online_circle() -> StyleSource {
    css!(
        "
        display: inline-block;

        height: 12px;
        width: 12px;

        margin-right: 4px;

        border-radius: 50%;

        background-color: var(--color-green);
    "
    )
}
pub fn standby_circle() -> StyleSource {
    css!(
        "
        display: inline-block;

        height: 12px;
        width: 12px;

        margin-right: 4px;

        border-radius: 50%;

        background-color: var(--color-yellow);
    "
    )
}
pub fn offline_circle() -> StyleSource {
    css!(
        "
        display: inline-block;

        height: 12px;
        width: 12px;

        margin-right: 4px;

        border-radius: 50%;

        background-color: var(--color-red);
    "
    )
}
