use crate::servers::Servers;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[target(index)]
    Index,
    Servers,
    #[target(rename = "servers")]
    Server {
        server: String,
        #[target(nested, default)]
        target: ServerRoute
    },
    Settings,
}
impl AppRoute {
    pub fn render() -> Html {
        #[function_component]
        fn AvoidIndex() -> Html {
            let router = use_router::<AppRoute>().unwrap();
            router.push(AppRoute::Servers);

            html! { "Error: You should have been redirected." }
        }

        html! {
            <Switch<Self> render={|target|match target {
                Self::Index => html! { <AvoidIndex /> },
                Self::Servers => html! { <Servers /> },
                Self::Server { server, target } => ServerRoute::render(server, target),
                Self::Settings => html! { "Settings" },
            }}/>
        }
    }
}
