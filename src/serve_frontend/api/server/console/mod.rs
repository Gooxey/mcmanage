use goohttp::*;

router! {
    console {
        get_log, get;
        latest_log, get;

        send_input, get;
    }
}