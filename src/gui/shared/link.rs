use wasm_bindgen::JsValue;
use web_sys::{PopStateEvent, PopStateEventInit};
use yew::prelude::*;
use yew::utils::NeqAssign;

pub enum Msg {
    Clicked,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub tab: String,
    pub children: Children,
}

pub struct Link {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Link {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Link { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                let window = yew::utils::window();

                let main_tab = JsValue::from_str(&self.props.tab);
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <button class="button link" onclick={self.link.callback(|_| Msg::Clicked)}>
                { self.props.children.clone() }
            </button>
        }
    }
}
