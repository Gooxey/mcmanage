use web_sys::{
    HtmlElement,
    HtmlInputElement,
};
use yew::prelude::*;

mod styles;

pub struct Console {
    log_last_item: NodeRef,
    input_ref: NodeRef,
    log: Vec<String>,
}

pub enum ConsoleMsg {
    Submit,
    FocusInput,
}

impl Component for Console {
    type Message = ConsoleMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            log_last_item: NodeRef::default(),
            input_ref: NodeRef::default(),
            log: vec![],
        }
    }

    fn update(&mut self, _: &Context<Self>, message: Self::Message) -> bool {
        match message {
            ConsoleMsg::Submit => {
                let value = self.input_ref.cast::<HtmlInputElement>().unwrap().value();
                if value == "" {
                    return false;
                }

                self.log.push(value);
                self.input_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .set_value("");

                self.log_last_item
                    .cast::<HtmlElement>()
                    .unwrap()
                    .scroll_into_view();

                true
            }
            ConsoleMsg::FocusInput => {
                self.input_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .focus()
                    .unwrap();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut line_counter = 0;
        html! {
            <div
                class={styles::console()}
                onclick={ctx.link().callback(|_| ConsoleMsg::FocusInput)}
            >
                <ul
                    class={classes!(styles::log(), "select")}
                >
                    {
                        for self.log.iter().map(|line| {
                            line_counter+=1;
                            html! { <li key={line_counter}>{ line }</li> }
                        })
                    }
                    <div ref={&self.log_last_item} style={"height: 1px;"}></div>
                </ul>

                <form
                    class={styles::input()}
                    onsubmit={ctx.link().callback(|e: SubmitEvent| {e.prevent_default(); ConsoleMsg::Submit})}
                >
                    <span>{">"}</span>
                    <input
                        ref={&self.input_ref}
                        class={styles::input_line()}
                    />
                </form>
            </div>
        }
    }
}
