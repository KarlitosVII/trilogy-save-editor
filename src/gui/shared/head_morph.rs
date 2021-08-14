use anyhow::Error;
use std::cell::{Ref, RefMut};
use yew::{prelude::*, utils::NeqAssign};
use yew_agent::{Bridge, Bridged};

use crate::gui::{components::Table, raw_ui::RawUiChildren, RcUi};
use crate::save_data::shared::appearance::HeadMorph as DataHeadMorph;
use crate::save_handler::{Request, Response, SaveHandler};

pub enum Msg {
    Import,
    HeadMorphImported(DataHeadMorph),
    Export,
    HeadMorphExported,
    RemoveHeadMorph,
    Error(Error),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub head_morph: RcUi<Option<RcUi<DataHeadMorph>>>,
    pub onnotification: Callback<&'static str>,
    pub onerror: Callback<Error>,
}

impl Props {
    fn head_morph(&self) -> Ref<'_, Option<RcUi<DataHeadMorph>>> {
        self.head_morph.borrow()
    }

    fn head_morph_mut(&mut self) -> RefMut<'_, Option<RcUi<DataHeadMorph>>> {
        self.head_morph.borrow_mut()
    }
}

pub struct HeadMorph {
    props: Props,
    link: ComponentLink<Self>,
    save_handle: Box<dyn Bridge<SaveHandler>>,
}

impl Component for HeadMorph {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let save_handle = SaveHandler::bridge(link.callback(|response| match response {
            Response::HeadMorphImported(head_morph) => Msg::HeadMorphImported(head_morph),
            Response::HeadMorphExported => Msg::HeadMorphExported,
            Response::Error(err) => Msg::Error(err),
            _ => unreachable!(),
        }));
        HeadMorph { props, link, save_handle }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Import => {
                self.save_handle.send(Request::ImportHeadMorph);
                false
            }
            Msg::HeadMorphImported(head_morph) => {
                *self.props.head_morph_mut() = Some(head_morph.into());
                self.props.onnotification.emit("Imported");
                true
            }
            Msg::Export => {
                if let Some(ref head_morph) = *self.props.head_morph() {
                    self.save_handle.send(Request::ExportHeadMorph(RcUi::clone(head_morph)));
                }
                false
            }
            Msg::HeadMorphExported => {
                self.props.onnotification.emit("Exported");
                false
            }
            Msg::RemoveHeadMorph => {
                self.props.head_morph_mut().take();
                true
            }
            Msg::Error(err) => {
                self.props.onerror.emit(err);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let head_morph = self.props.head_morph();
        let export_remove = head_morph.is_some().then(|| {
            html! {
                <>
                    <button class="button" onclick={self.link.callback(|_| Msg::Export)}>
                        {"Export"}
                    </button>
                    <span>{"-"}</span>
                    <button class="button" onclick={self.link.callback(|_| Msg::RemoveHeadMorph)}>
                        {"Remove head morph"}
                    </button>
                </>
            }
        });
        let raw = head_morph.as_ref().map(|head_morph| {
            html! {
                <Table title="Raw">
                    { for head_morph.children() }
                </Table>
            }
        });
        html! {
            <div class="flex-auto flex flex-col gap-1">
                <div class="flex items-center gap-2">
                    <button class="button" onclick={self.link.callback(|_| Msg::Import)}>
                        {"Import"}
                    </button>
                    { for export_remove }
                </div>
                <hr class="border-t border-default-border" />
                <div class="flex-auto h-0 overflow-y-auto">
                    { for raw }
                </div>
            </div>
        }
    }
}
