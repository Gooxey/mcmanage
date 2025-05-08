use stylist::{
    css,
    StyleSource,
};

pub fn image() -> StyleSource {
    css!(
        "
        border-radius: var(--border-radius);
    "
    )
}
pub fn name() -> StyleSource {
    css!(
        "
        display: flex;

        font-size: larger;
        font-weight: bolder;

        height: var(--font-size);
    "
    )
}
