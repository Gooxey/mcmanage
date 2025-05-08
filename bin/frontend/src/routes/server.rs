use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Target, Default)]
pub enum ServerRoute {
    #[default]
    Console,
    Settings,
}
impl ServerRoute {
    pub fn render(server: String, target: Self) -> Html {

        html! {
            match target {
                Self::Console => html!{ format!("{server} Console") },
                Self::Settings => html!{ format!("{server} Settings") },
            }
        }
    }
}