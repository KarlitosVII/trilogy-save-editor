use std::cell::{Ref, RefMut};

use yew::prelude::*;

use crate::{
    gui::{
        components::{CallbackType, Helper, InputNumber, InputText, NumberType, Select, Table},
        raw_ui::RawUi,
        RcUi,
    },
    save_data::{
        mass_effect_1_le::{player::Player, squad::Henchman, Me1LeSaveData},
        shared::{
            player::{Notoriety, Origin},
            plot::PlotTable,
        },
    },
};

pub enum Msg {
    Gender(usize),
    Origin(usize),
    Notoriety(usize),
    Difficulty(usize),
    TalentPoints(CallbackType),
    ResetTalents(Option<&'static str>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub save_game: RcUi<Me1LeSaveData>,
}

impl Props {
    fn save_game(&self) -> Ref<'_, Me1LeSaveData> {
        self.save_game.borrow()
    }

    fn save_game_mut(&self) -> RefMut<'_, Me1LeSaveData> {
        self.save_game.borrow_mut()
    }
}

pub struct Me1LeGeneral;

impl Component for Me1LeGeneral {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        // TODO: ME1 LE class swap
        Me1LeGeneral
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Me1LeSaveData { player, squad, plot, .. } = &mut *ctx.props().save_game_mut();
        let (mut player, mut plot) = (player.borrow_mut(), plot.borrow_mut());
        match msg {
            Msg::Gender(gender) => {
                let gender = gender != 0;

                // Player
                *player.is_female_mut() = gender;

                // Plot
                if let Some(mut is_female) = plot.booleans_mut().get_mut(4639) {
                    *is_female = gender;
                }
                false
            }
            Msg::Origin(origin_idx) => {
                // Player
                *player.origin_mut() = Origin::from(origin_idx);

                // ME1 plot
                if let Some(origin) = plot.integers_mut().get_mut(1) {
                    *origin.borrow_mut() = origin_idx as i32;
                }
                false
            }
            Msg::Notoriety(notoriety_idx) => {
                // Player
                *player.notoriety_mut() = Notoriety::from(notoriety_idx);

                // ME1 plot
                if let Some(notoriety) = plot.integers_mut().get_mut(2) {
                    *notoriety.borrow_mut() = notoriety_idx as i32;
                }
                false
            }
            Msg::Difficulty(difficulty_idx) => {
                if let Some(difficulty) = player.game_options_mut().get_mut(0) {
                    *difficulty.borrow_mut() = difficulty_idx as i32;
                }
                false
            }
            Msg::ResetTalents(tag) => {
                let (talent_points, complex_talents) = if let Some(tag) = tag {
                    // Squad mate
                    let squad = squad.borrow();
                    let character = squad
                        .iter()
                        .find(|character| *character.borrow().tag() == tag)
                        .unwrap()
                        .borrow();
                    (RcUi::clone(&character.talent_points), RcUi::clone(&character.complex_talents))
                } else {
                    // Player
                    (RcUi::clone(&player.talent_points), RcUi::clone(&player.complex_talents))
                };

                for talent in complex_talents.borrow_mut().iter_mut() {
                    *talent_points.borrow_mut() += *talent.borrow().current_rank();
                    *talent.borrow_mut().current_rank_mut() = 0;
                }
                true
            }
            Msg::TalentPoints(CallbackType::Int(talent_points)) => {
                *player.talent_points_mut() = talent_points;
                true
            }
            _ => unreachable!(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let save_game = ctx.props().save_game();

        html! {
            <div class="flex divide-solid divide-x divide-default-border">
                <div class="flex-1 pr-1 flex flex-col gap-1">
                    { self.role_play(ctx, save_game.player()) }
                    { self.morality(save_game.plot()) }
                    { self.resources(save_game.player()) }
                </div>
                <div class="flex-1 pl-1 flex flex-col gap-1">
                    { self.general(ctx, save_game.player().game_options()) }
                    { self.gameplay(ctx, save_game.player()) }
                    { self.squad(ctx, save_game.squad()) }
                </div>
            </div>
        }
    }
}

impl Me1LeGeneral {
    fn role_play(&self, ctx: &Context<Self>, player: Ref<'_, Player>) -> Html {
        let link = ctx.link();
        let genders: &'static [&'static str] = &["Male", "Female"];
        html! {
            <Table title="Role-Play">
                { player.first_name.view("Name") }
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={genders}
                        current_idx={*player.is_female() as usize}
                        onselect={link.callback(Msg::Gender)}
                    />
                    {"Gender"}
                    <Helper text=
                        "If you change your gender, disable the head morph or import an appropriate one. \
                        Otherwise, Saren and his Geths will be the least of your worries..."
                    />
                </div>
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={Origin::variants()}
                        current_idx={*player.origin() as usize}
                        onselect={link.callback(Msg::Origin)}
                    />
                    {"Origin"}
                </div>
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={Notoriety::variants()}
                        current_idx={*player.notoriety() as usize}
                        onselect={link.callback(Msg::Notoriety)}
                    />
                    {"Notoriety"}
                </div>
                <InputText label="Identity Code" value={RcUi::clone(&player.face_code)} helper=
                    "If you change this you can display whatever you want in the menus \
                    in place of your `Identity Code`.\n\
                    This will NOT change your face, you have to edit your head morph \
                    or import one to do so."
                />
            </Table>
        }
    }

    fn morality(&self, plot: Ref<'_, PlotTable>) -> Html {
        html! {
            <Table title="Morality">
                { for plot.integers().get(47).map(|paragon| paragon.view("Paragon")) }
                { for plot.integers().get(46).map(|renegade| renegade.view("Renegade")) }
            </Table>
        }
    }

    fn resources(&self, player: Ref<'_, Player>) -> Html {
        let Player { credits, medigel, grenades, omnigel, .. } = &*player;
        html! {
            <Table title="Resources">
                { credits.view("Credits") }
                { medigel.view("Medigel") }
                { grenades.view("Grenades") }
                { omnigel.view("Omnigel") }
            </Table>
        }
    }

    fn general(&self, ctx: &Context<Self>, game_options: Ref<'_, Vec<RcUi<i32>>>) -> Html {
        let difficulty: &'static [&'static str] =
            &["Casual", "Normal", "Veteran", "Hardcore", "Insanity"];
        let current_difficulty =
            game_options.get(0).map(|d| *d.borrow() as usize).unwrap_or_default();
        html! {
            <Table title="General">
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={difficulty}
                        current_idx={current_difficulty}
                        onselect={ctx.link().callback(Msg::Difficulty)}
                    />
                    { "Difficulty" }
                </div>
            </Table>
        }
    }

    fn gameplay(&self, ctx: &Context<Self>, player: Ref<'_, Player>) -> Html {
        let Player { level, current_xp, .. } = &*player;

        html! {
            <Table title="Gameplay">
                <InputNumber
                    label="Level"
                    value={NumberType::Int(RcUi::clone(level))}
                    helper="Classic mode (1 - 60)"
                />
                { current_xp.view("Current XP") }
                <InputNumber
                    label="Talent Points"
                    value={NumberType::Int((*player.talent_points()).into())}
                    onchange={ctx.link().callback(Msg::TalentPoints)}
                />
                <button class="button" onclick={ctx.link().callback(|_| Msg::ResetTalents(None))}>
                    { "Reset player's talents" }
                </button>
            </Table>
        }
    }

    fn squad(&self, ctx: &Context<Self>, squad: Ref<'_, Vec<RcUi<Henchman>>>) -> Html {
        let characters = [
            ("hench_humanfemale", "Ashley"),
            ("hench_turian", "Garrus"),
            ("hench_humanmale", "Kaidan"),
            ("hench_asari", "Liara"),
            ("hench_quarian", "Tali"),
            ("hench_krogan", "Wrex"),
        ];

        let characters = squad.iter().filter_map(|character| {
            characters.iter().find_map(|&(tag, name)| {
                (*character.borrow().tag() == tag).then(|| {
                    html! {
                        <button class="button" onclick={ctx.link().callback(move |_| Msg::ResetTalents(Some(tag)))}>
                            { format!("Reset {}'s talents", name) }
                        </button>
                    }
                })
            })
        });

        html! {
            <Table title="Squad">
                { for characters }
            </Table>
        }
    }
}
