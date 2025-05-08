use stylist::{
    css,
    StyleSource,
};

pub fn console() -> StyleSource {
    css!(
        "
        width: 100%;
        height: 100%;

        margin-top: var(--border-radius);
    ",
    )
}
pub fn log() -> StyleSource {
    css!(
        "
        height: calc(100% - 60px);

        margin: 0;
        padding: var(--border-radius);
        padding-top: 0;

        list-style-type: none;

        overflow-y: auto;
        overflow-wrap: break-word;
    "
    )
}
pub fn input() -> StyleSource {
    css!(
        "
        height: 25px;
    "
    )
}
pub fn input_line() -> StyleSource {
    css!(
        "
        width: calc(100% - 18px);

        color: inherit;
        background-color: inherit;

        font-size: inherit;
        font-family: inherit;

        padding: 0;
        margin-left: 3px;

        border: none;

        :focus-visible {
            outline: none;
        }
    "
    )
}
