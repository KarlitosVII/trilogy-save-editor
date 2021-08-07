use std::cell::{Ref, RefMut};
use yew::prelude::*;
use yewtil::NeqAssign;

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

    fn save_game_mut(&mut self) -> RefMut<'_, Me1LeSaveData> {
        self.save_game.borrow_mut()
    }
}

pub struct Me1LeGeneral {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Me1LeGeneral {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Me1LeGeneral { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let Me1LeSaveData { player, squad, plot, .. } = &mut *self.props.save_game_mut();
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
                let (mut talent_points, mut complex_talents) = if let Some(tag) = tag {
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
            Msg::TalentPoints(CallbackType::Integer(talent_points)) => {
                *player.talent_points_mut() = talent_points;
                true
            }
            _ => unreachable!(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let save_game = &*self.props.save_game();

        html! {
            <div class="flex flex-row divide-solid divide-x divide-default-border">
                <div class="flex-1 pr-1 flex flex-col gap-1">
                    { self.role_play(save_game.player()) }
                    { self.morality(save_game.plot()) }
                    { self.resources(save_game.player()) }
                </div>
                <div class="flex-1 pl-1 flex flex-col gap-1">
                    { self.general(save_game.player().game_options()) }
                    { self.gameplay(save_game.player()) }
                    { self.squad(save_game.squad()) }
                </div>
            </div>
        }
    }
}

impl Me1LeGeneral {
    fn role_play(&self, player: Ref<'_, Player>) -> Html {
        let genders: &'static [&'static str] = &["Male", "Female"];
        html! {
            <Table title=String::from("Role-Play")>
                { player.first_name.view("Name") }
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options=genders
                        current_idx=*player.is_female() as usize
                        onselect=self.link.callback(Msg::Gender)
                    />
                    {"Gender"}
                    <Helper text=
                        "If you change your gender, disable the head morph or import an appropriate one. \
                        Otherwise, Saren and his Geths will be the least of your worries..."
                    />
                </div>
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options=Origin::variants()
                        current_idx=player.origin().clone() as usize
                        onselect=self.link.callback(Msg::Origin)
                    />
                    {"Origin"}
                </div>
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options=Notoriety::variants()
                        current_idx=player.notoriety().clone() as usize
                        onselect=self.link.callback(Msg::Notoriety)
                    />
                    {"Notoriety"}
                </div>
                <InputText label="Identity Code" value=RcUi::clone(&player.face_code) helper=
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
            <Table title=String::from("Morality")>
                { plot.integers().get(47).map(|paragon| paragon.view("Paragon")).unwrap_or_default() }
                { plot.integers().get(46).map(|renegade| renegade.view("Renegade")).unwrap_or_default() }
            </Table>
        }
    }

    fn resources(&self, player: Ref<'_, Player>) -> Html {
        let Player { credits, medigel, grenades, omnigel, .. } = &*player;
        html! {
            <Table title=String::from("Gameplay")>
                { credits.view("Credits") }
                { medigel.view("Medigel") }
                { grenades.view("Grenades") }
                { omnigel.view("Omnigel") }
            </Table>
        }
    }

    fn general(&self, game_options: Ref<'_, Vec<RcUi<i32>>>) -> Html {
        let difficulty: &'static [&'static str] =
            &["Casual", "Normal", "Veteran", "Hardcore", "Insanity"];
        let current_difficulty =
            game_options.get(0).map(|d| *d.borrow()).unwrap_or_default() as usize;
        html! {
            <Table title=String::from("General")>
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options=difficulty
                        current_idx=current_difficulty
                        onselect=self.link.callback(Msg::Difficulty)
                    />
                    { "Difficulty" }
                </div>
            </Table>
        }
    }

    fn gameplay(&self, player: Ref<'_, Player>) -> Html {
        let Player { level, current_xp, .. } = &*player;

        html! {
            <Table title=String::from("Gameplay")>
                { level.view("Level") }
                { current_xp.view("Current XP") }
                <InputNumber
                    label="Talent Points"
                    value=NumberType::Integer((*player.talent_points()).into())
                    onchange=self.link.callback(Msg::TalentPoints)
                />
                <button class="btn" onclick=self.link.callback(|_| Msg::ResetTalents(None))>
                    { "Reset player's talents" }
                </button>
            </Table>
        }
    }

    fn squad(&self, squad: Ref<'_, Vec<RcUi<Henchman>>>) -> Html {
        let characters = [
            ("hench_humanfemale", "Ashley"),
            ("hench_turian", "Garrus"),
            ("hench_humanmale", "Kaidan"),
            ("hench_asari", "Liara"),
            ("hench_quarian", "Tali"),
            ("hench_krogan", "Wrex"),
        ];

        let characters = characters.iter().filter_map(|&(tag, name)| {
            squad.iter().find_map(|character| {
                (*character.borrow().tag() == tag).then(|| {
                    html! {
                        <button class="btn" onclick=self.link.callback(move |_| Msg::ResetTalents(Some(tag)))>
                            { format!("Reset {}'s talents", name) }
                        </button>
                    }
                })
            })
        });

        html! {
            <Table title=String::from("Gameplay")>
                { for characters }
            </Table>
        }
    }
}
