use goohttp::*;

router! {
    actions {
        restart, put;
        start, put;
        stop, put;
    }
}