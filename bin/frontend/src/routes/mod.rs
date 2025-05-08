use yew::prelude::*;
use yew_nested_router::prelude::*;

#[macro_use]
mod routes;

routes! {
    app, AppRoute;
    server, ServerRoute;
}
