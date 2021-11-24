use gloo_utils as utils;
use wasm_bindgen::JsValue;
use web_sys::{PopStateEvent, PopStateEventInit};
use yew::prelude::*;

pub enum Msg {
    Clicked,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tab: String,
    pub children: Children,
}

pub struct Link;

impl Component for Link {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Link
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                let window = utils::window();

                let main_tab = JsValue::from_str(&ctx.props().tab);
                // let history = window.history().expect("no history");
                // history.push_state(&main_tab, "").expect("push history");

                let mut state = PopStateEventInit::new();
                state.state(&main_tab);
                if let Ok(event) = PopStateEvent::new_with_event_init_dict("popstate", &state) {
                    let _ = window.dispatch_event(&event);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button class="button link" onclick={ctx.link().callback(|_| Msg::Clicked)}>
                { ctx.props().children.clone() }
            </button>
        }
    }
}
