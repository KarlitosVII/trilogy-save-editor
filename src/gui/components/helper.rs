use web_sys::HtmlElement;
use yew::prelude::*;

pub enum Msg {
    Hover,
    Out,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: &'static str,
}

pub struct Helper {
    popup_ref: NodeRef,
    hovered: bool,
}

impl Component for Helper {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Helper { popup_ref: Default::default(), hovered: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        // Keep the popup in the viewport
        if let Some(popup) = self.popup_ref.cast::<HtmlElement>() {
            let viewport_width = yew::utils::document().document_element().unwrap().client_width();
            let client_rect = popup.get_bounding_client_rect();
            let width = client_rect.width() as i32;
            let left = client_rect.left() as i32;
            let right = left + width;

            if right > viewport_width - 30 {
                let _ =
                    popup.style().set_property("left", &format!("{}px", viewport_width - right));
                let _ = popup.style().set_property("top", "30px");
            } else {
                let _ = popup.style().set_property("left", "30px");
                let _ = popup.style().set_property("top", "10px");
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text = ctx.props().text.split_terminator('\n').map(|text| {
            html! { <p>{ text }</p> }
        });
        html! {
            <div class="relative">
                <div class="text-white/50 select-none"
                    onmouseover={ctx.link().callback(|_| Msg::Hover)}
                    onmouseout={ctx.link().callback(|_| Msg::Out)}
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
                        "w-[515px]",
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
