use std::cell::Ref;

use yew::prelude::*;

use crate::save_data::{
    mass_effect_1::{
        data::{Data, Property as DataProperty},
        player::Player,
    },
    shared::plot::PlotTable,
    List,
};
use crate::{
    gui::{
        components::{Select, Table},
        mass_effect_1::property::Property,
        raw_ui::RawUi,
    },
    save_data::{mass_effect_1::data::StructType, RcRef},
};

pub enum Msg {
    Difficulty(usize),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub player: RcRef<Player>,
    pub plot: RcRef<PlotTable>,
}

impl Props {
    fn player(&self) -> Ref<'_, Player> {
        self.player.borrow()
    }

    fn plot(&self) -> Ref<'_, PlotTable> {
        self.plot.borrow()
    }
}

pub struct Me1General;

impl Component for Me1General {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Me1General
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Difficulty(new_difficulty_idx) => {
                let player = ctx.props().player();

                // Worst thing ever >>
                // Find current game
                // Then find game options
                // Then find difficulty option
                let value = player
                    .objects
                    .iter()
                    .enumerate()
                    .find_map(|(i, object)| {
                        let object_name = player.get_name(object.object_name_id);
                        (object_name == "CurrentGame").then(|| player.get_data(i as i32 + 1))
                    })
                    .and_then(|current_game| {
                        let m_game_options =
                            Self::find_property(ctx, &current_game.properties, "m_GameOptions")?
                                .borrow();
                        match *m_game_options {
                            DataProperty::Struct {
                                struct_type: StructType::Properties(ref properties),
                                ..
                            } => Some(properties),
                            _ => None,
                        }
                        .and_then(|properties| {
                            Self::find_property(ctx, properties, "m_nCombatDifficulty").and_then(
                                |p| match *p.borrow() {
                                    DataProperty::Int { ref value, .. } => {
                                        Some(RcRef::clone(value))
                                    }
                                    _ => None,
                                },
                            )
                        })
                    });

                // Then set new difficulty
                if let Some(value) = value {
                    *value.borrow_mut() = new_difficulty_idx as i32;
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! { for Self::try_view(ctx) }
    }
}

impl Me1General {
    fn try_view(ctx: &Context<Self>) -> Option<Html> {
        let player = ctx.props().player();

        let current_game = player.objects.iter().enumerate().find_map(|(i, object)| {
            let object_name = player.get_name(object.object_name_id);
            (object_name == "CurrentGame").then(|| player.get_data(i as i32 + 1))
        })?;

        let m_player = {
            let object_id = Self::find_object_id(ctx, &current_game.properties, "m_Player")?;
            player.get_data(object_id)
        };

        let m_squad = {
            let object_id = Self::find_object_id(ctx, &m_player.properties, "m_Squad")?;
            player.get_data(object_id)
        };

        let m_inventory = {
            let object_id = Self::find_object_id(ctx, &m_squad.properties, "m_Inventory")?;
            player.get_data(object_id)
        };

        Some(html! {
            <div class="flex divide-solid divide-x divide-default-border">
                <div class="flex-1 pr-1 flex flex-col gap-1">
                    { Self::role_play(ctx, m_player) }
                    { Self::gameplay(ctx, m_player, m_squad) }
                    { Self::morality(ctx) }
                </div>
                <div class="flex-1 pl-1 flex flex-col gap-1">
                    { for Self::general(ctx, &current_game.properties) }
                    { Self::resources(ctx, m_inventory) }
                </div>
            </div>
        })
    }

    fn role_play(ctx: &Context<Self>, m_player: &Data) -> Html {
        let name = Self::find_property(ctx, &m_player.properties, "m_FirstName")
            .map(|p| Self::view_property(ctx, p, "Name"));
        let gender = Self::find_property(ctx, &m_player.properties, "m_Gender")
            .map(|p| Self::view_property(ctx, p, "Gender"));
        let origin = Self::find_property(ctx, &m_player.properties, "m_BackgroundOrigin")
            .map(|p| Self::view_property(ctx, p, "Origin"));
        let notoriety = Self::find_property(ctx, &m_player.properties, "m_BackgroundNotoriety")
            .map(|p| Self::view_property(ctx, p, "Notoriety"));

        html! {
            <Table title="Role-Play">
                { for name }
                { for gender }
                { for origin }
                { for notoriety }
            </Table>
        }
    }

    fn gameplay(ctx: &Context<Self>, m_player: &Data, m_squad: &Data) -> Html {
        let class = Self::find_property(ctx, &m_player.properties, "m_ClassBase")
            .map(|p| Self::view_property(ctx, p, "Class"));
        let level = Self::find_property(ctx, &m_player.properties, "m_XPLevel")
            .map(|p| Self::view_property(ctx, p, "Level"));
        let curent_xp = Self::find_property(ctx, &m_squad.properties, "m_nSquadExperience")
            .map(|p| Self::view_property(ctx, p, "Current XP"));

        html! {
            <Table title="Gameplay">
                { for class }
                { for level }
                { for curent_xp }
            </Table>
        }
    }

    fn morality(ctx: &Context<Self>) -> Html {
        let plot = ctx.props().plot();
        html! {
            <Table title="Morality">
                { for plot.integers().get(47).map(|paragon| paragon.view("Paragon")) }
                { for plot.integers().get(46).map(|renegade| renegade.view("Renegade")) }
            </Table>
        }
    }

    fn general(ctx: &Context<Self>, current_game: &List<RcRef<DataProperty>>) -> Option<Html> {
        let difficulty: &'static [&'static str] =
            &["Casual", "Normal", "Veteran", "Hardcore", "Insanity"];

        let m_game_options = Self::find_property(ctx, current_game, "m_GameOptions")?.borrow();
        let difficulty = match *m_game_options {
            DataProperty::Struct {
                struct_type: StructType::Properties(ref properties), ..
            } => Some(properties),
            _ => None,
        }
        .and_then(|properties| {
            Self::find_property(ctx, properties, "m_nCombatDifficulty").and_then(|p| {
                match *p.borrow() {
                    DataProperty::Int { ref value, .. } => Some(*value.borrow() as usize),
                    _ => None,
                }
            })
        })
        .map(|current_idx| {
            html! {
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={difficulty}
                        {current_idx}
                        onselect={ctx.link().callback(Msg::Difficulty)}
                    />
                    { "Difficulty" }
                </div>
            }
        });

        Some(html! {
            <Table title="General">
                { for difficulty }
            </Table>
        })
    }

    fn resources(ctx: &Context<Self>, m_inventory: &Data) -> Html {
        let credits = Self::find_property(ctx, &m_inventory.properties, "m_nResourceCredits")
            .map(|p| Self::view_property(ctx, p, "Credits"));
        let medigel = Self::find_property(ctx, &m_inventory.properties, "m_fResourceMedigel")
            .map(|p| Self::view_property(ctx, p, "Medigel"));
        let grenades = Self::find_property(ctx, &m_inventory.properties, "m_nResourceGrenades")
            .map(|p| Self::view_property(ctx, p, "Grenades"));
        let omnigel = Self::find_property(ctx, &m_inventory.properties, "m_fResourceSalvage")
            .map(|p| Self::view_property(ctx, p, "Omnigel"));

        html! {
            <Table title="Resources">
                { for credits }
                { for medigel }
                { for grenades }
                { for omnigel }
            </Table>
        }
    }

    fn view_property(ctx: &Context<Self>, property: &RcRef<DataProperty>, label: &str) -> Html {
        let player = &ctx.props().player;
        html! {
            <Property
                player={RcRef::clone(player)}
                property={RcRef::clone(property)}
                label={label.to_owned()}
            />
        }
    }

    fn find_property<'a>(
        ctx: &Context<Self>, properties: &'a List<RcRef<DataProperty>>, property_name: &str,
    ) -> Option<&'a RcRef<DataProperty>> {
        let player = ctx.props().player();
        properties.iter().find_map(|property| match *property.borrow() {
            DataProperty::Array { name_id, .. }
            | DataProperty::Bool { name_id, .. }
            | DataProperty::Byte { name_id, .. }
            | DataProperty::Float { name_id, .. }
            | DataProperty::Int { name_id, .. }
            | DataProperty::Name { name_id, .. }
            | DataProperty::Object { name_id, .. }
            | DataProperty::Str { name_id, .. }
            | DataProperty::StringRef { name_id, .. }
            | DataProperty::Struct { name_id, .. }
            | DataProperty::None { name_id, .. } => {
                (player.get_name(name_id) == property_name).then(|| property)
            }
        })
    }

    fn find_object_id(
        ctx: &Context<Self>, properties: &List<RcRef<DataProperty>>, property_name: &str,
    ) -> Option<i32> {
        Self::find_property(ctx, properties, property_name).and_then(|property| {
            match *property.borrow() {
                DataProperty::Object { object_id, .. } => Some(object_id),
                _ => None,
            }
        })
    }
}
