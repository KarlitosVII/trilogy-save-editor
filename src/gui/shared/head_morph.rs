use anyhow::Error;
use std::cell::{Ref, RefMut};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::gui::{components::Table, raw_ui::RawUiChildren, RcUi};
use crate::save_data::shared::appearance::HeadMorph as DataHeadMorph;
use crate::services::save_handler::{Request, Response, SaveHandler};

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

    fn head_morph_mut(&self) -> RefMut<'_, Option<RcUi<DataHeadMorph>>> {
        self.head_morph.borrow_mut()
    }
}

pub struct HeadMorph {
    save_handler: Box<dyn Bridge<SaveHandler>>,
}

impl Component for HeadMorph {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // TODO: Import gibbed head morph
        let save_handler = SaveHandler::bridge(ctx.link().callback(|response| match response {
            Response::HeadMorphImported(head_morph) => Msg::HeadMorphImported(head_morph),
            Response::HeadMorphExported => Msg::HeadMorphExported,
            Response::Error(err) => Msg::Error(err),
            _ => unreachable!(),
        }));
        HeadMorph { save_handler }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Import => {
                self.save_handler.send(Request::ImportHeadMorph);
                false
            }
            Msg::HeadMorphImported(head_morph) => {
                *ctx.props().head_morph_mut() = Some(head_morph.into());
                ctx.props().onnotification.emit("Imported");
                true
            }
            Msg::Export => {
                if let Some(ref head_morph) = *ctx.props().head_morph() {
                    self.save_handler.send(Request::ExportHeadMorph(RcUi::clone(head_morph)));
                }
                false
            }
            Msg::HeadMorphExported => {
                ctx.props().onnotification.emit("Exported");
                false
            }
            Msg::RemoveHeadMorph => {
                ctx.props().head_morph_mut().take();
                true
            }
            Msg::Error(err) => {
                ctx.props().onerror.emit(err);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let head_morph = ctx.props().head_morph();
        let export_remove = head_morph.is_some().then(|| {
            html! {
                <>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Export)}>
                        {"Export"}
                    </button>
                    <span>{"-"}</span>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::RemoveHeadMorph)}>
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
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Import)}>
                        {"Import"}
                    </button>
                    { for export_remove }
                </div>
                <hr class="border-t border-default-border" />
                { for raw }
            </div>
        }
    }
}
