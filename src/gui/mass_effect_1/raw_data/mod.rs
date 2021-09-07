pub mod property;

use std::cell::Ref;

use yew::prelude::*;

use crate::gui::{components::Table, mass_effect_1::raw_data::property::Property, RcUi};
use crate::save_data::mass_effect_1::player::Player;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub player: RcUi<Player>,
}

impl Props {
    fn player(&self) -> Ref<'_, Player> {
        self.player.borrow()
    }
}

pub struct Me1RawData;

impl Component for Me1RawData {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Me1RawData {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let player = ctx.props().player();
        let object_id = player
            .objects
            .iter()
            .enumerate()
            .find_map(|(idx, object)| {
                (player.get_name(object.object_name_id) == "CurrentGame").then(|| idx as i32 + 1)
            })
            .unwrap_or_default();

        let properties = &player.get_data(object_id).properties;
        let len = properties.len();
        let take = if len > 0 { len - 1 } else { 0 };
        let properties = properties.iter().take(take).map(|property| {
            html! {
                <Property
                    player={RcUi::clone(&ctx.props().player)}
                    property={RcUi::clone(property)}
                />
            }
        });

        html! {
            <Table>
                { for properties }
            </Table>
        }
    }
}
