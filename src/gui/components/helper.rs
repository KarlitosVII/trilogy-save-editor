use web_sys::HtmlElement;
use yew::{prelude::*, utils::NeqAssign};

pub enum Msg {
    Hover,
    Out,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: &'static str,
}

pub struct Helper {
    props: Props,
    link: ComponentLink<Self>,
    popup_ref: NodeRef,
    hovered: bool,
}

impl Component for Helper {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Helper { props, link, popup_ref: Default::default(), hovered: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover => {
                self.hovered = true;
                true
            }
            Msg::Out => {
                self.hovered = false;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn rendered(&mut self, _first_render: bool) {
        // Keep the popup in the viewport
        if let Some(popup) = self.popup_ref.cast::<HtmlElement>() {
            let viewport_width = yew::utils::document().document_element().unwrap().client_width();
            let client_rect = popup.get_bounding_client_rect();
            let width = client_rect.width() as i32;
            let left = client_rect.left() as i32;
            let right = left + width;

            if right + 30 > viewport_width {
                let _ =
                    popup.style().set_property("left", &format!("{}px", viewport_width - right));
                let _ = popup.style().set_property("top", "30px");
            } else {
                let _ = popup.style().set_property("left", "30px");
                let _ = popup.style().set_property("top", "10px");
            }
        }
    }

    fn view(&self) -> Html {
        let text = self.props.text.split_terminator('\n').map(|text| {
            html! { <p>{ text }</p> }
        });
        html! {
            <div class="relative">
                <div class="text-white/50 select-none"
                    onmouseover={self.link.callback(|_| Msg::Hover)}
                    onmouseout={self.link.callback(|_| Msg::Out)}
                >
                    { "(?)" }
                </div>
                <div class={classes![
                        "flex",
                        "flex-col",
                        "gap-1",
                        "absolute",
                        "bg-popup/90",
                        "border",
                        "border-default-border",
                        "px-2",
                        "py-1",
                        "w-[480px]",
                        "z-30",
                        (!self.hovered).then(|| "hidden" ),
                    ]}
                    ref={self.popup_ref.clone()}
                >
                    { for text }
                </div>
            </div>
        }
    }
}
