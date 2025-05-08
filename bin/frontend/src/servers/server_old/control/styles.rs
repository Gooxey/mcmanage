use stylist::{
    css,
    StyleSource,
};

pub fn status() -> StyleSource {
    css!(
        "
        display: flex;

        align-items: center;

        font-size: larger;
    "
    )
}
pub fn button() -> StyleSource {
    css!(
        "
        height: calc(var(--font-size) * 2);
        width: 90%;

        align-self: center;

        margin: calc(var(--border-radius) / 2);

        border: none;
        border-radius: var(--border-radius);

        color: var(--color-text);
        font-size: var(--font-size);
    "
    )
}
pub fn server_disabled() -> StyleSource {
    css!(
        "
        background-color: var(--color-red);
    "
    )
}
pub fn server_enabled() -> StyleSource {
    css!(
        "
        background-color: var(--color-green);
    "
    )
}
pub fn reboot_available() -> StyleSource {
    css!(
        "
        background-color: var(--color-accent);
    "
    )
}
pub fn abort_available() -> StyleSource {
    css!(
        "
        background-color: var(--color-red);
    "
    )
}
pub fn unavailable() -> StyleSource {
    css!(
        "
        color: var(--color-secondary-text);
        background-color: var(--color-unavailable);
    "
    )
}
