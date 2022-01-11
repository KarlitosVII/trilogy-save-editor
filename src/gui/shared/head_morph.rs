use std::cell::{Ref, RefMut};

use yew::{context::ContextHandle, prelude::*};

use crate::{
    gui::{components::Table, raw_ui::RawUiChildren, RcUi},
    save_data::shared::appearance::HeadMorph as DataHeadMorph,
    services::save_handler::{Action, SaveHandler},
};

pub enum Msg {
    Import,
    HeadMorphImported(DataHeadMorph),
    Export,
    RemoveHeadMorph,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub head_morph: RcUi<Option<RcUi<DataHeadMorph>>>,
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
    _db_handle: ContextHandle<SaveHandler>,
    save_handler: SaveHandler,
}

impl Component for HeadMorph {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (save_handler, _db_handle) = ctx
            .link()
            .context::<SaveHandler>(Callback::noop())
            .expect("no save handler provider");

        HeadMorph { _db_handle, save_handler }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Import => {
                let callback = ctx.link().callback(Msg::HeadMorphImported);
                self.save_handler.action(Action::ImportHeadMorph(callback));
                false
            }
            Msg::HeadMorphImported(head_morph) => {
                *ctx.props().head_morph_mut() = Some(head_morph.into());
                true
            }
            Msg::Export => {
                if let Some(ref head_morph) = *ctx.props().head_morph() {
                    self.save_handler.action(Action::ExportHeadMorph(RcUi::clone(head_morph)));
                }
                false
            }
            Msg::RemoveHeadMorph => {
                ctx.props().head_morph_mut().take();
                true
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
