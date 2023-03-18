//! This module provides the [`SERVER_LIST_EXAMPLE_DEFAULT constant`](SERVER_LIST_EXAMPLE_DEFAULT), which represents the default text in the `servers/server_list_example.json` file.


/// This constant represents the default text in the `servers/server_list_example.json` file.
pub const SERVER_LIST_EXAMPLE_DEFAULT: &str = "{
    \"0\": {
        \"name\": \"myFirstServer\",
        \"arg\": \"-jar purpur-1.19.3-1876.jar nogui\",
        \"type\": \"purpur\" 
    },
    \"1\": {
        \"name\": \"mySecondServer\",
        \"arg\": \"-jar purpur-1.19.3-1876.jar nogui\",
        \"type\": \"purpur\" 
    }
}";