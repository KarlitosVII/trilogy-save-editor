use std::cell::Ref;

use yew::prelude::*;

use crate::{
    gui::{
        components::{Helper, InputText, Select, Table},
        raw_ui::RawUi,
        shared::{BonusPowerType, BonusPowers},
    },
    save_data::{
        mass_effect_2::{player::Player, Difficulty},
        shared::{
            player::{Notoriety, Origin},
            plot::PlotTable,
            EndGameState,
        },
        RcRef,
    },
};

use super::Me2Type;

#[derive(Clone, RawUi)]
enum Me2Class {
    Soldier,
    Engineer,
    Adept,
    Infiltrator,
    Sentinel,
    Vanguard,
}

impl Me2Class {
    fn names() -> &'static [&'static str] {
        &[
            "SFXGame.SFXPawn_PlayerSoldier",
            "SFXGame.SFXPawn_PlayerEngineer",
            "SFXGame.SFXPawn_PlayerAdept",
            "SFXGame.SFXPawn_PlayerInfiltrator",
            "SFXGame.SFXPawn_PlayerSentinel",
            "SFXGame.SFXPawn_PlayerVanguard",
        ]
    }
}

pub enum Msg {
    Gender(usize),
    Origin(usize),
    Notoriety(usize),
    PlayerClass(usize),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub save_game: Me2Type,
}

pub struct Me2General;

impl Component for Me2General {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Me2General
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (player, me1_plot, plot) = match ctx.props().save_game {
            Me2Type::Vanilla(ref me2) => {
                let me2 = me2.borrow();
                (RcRef::clone(&me2.player), RcRef::clone(&me2.me1_plot), RcRef::clone(&me2.plot))
            }
            Me2Type::Legendary(ref me2) => {
                let me2 = me2.borrow();
                (RcRef::clone(&me2.player), RcRef::clone(&me2.me1_plot), RcRef::clone(&me2.plot))
            }
        };
        let (mut player, mut me1_plot, mut plot) =
            (player.borrow_mut(), me1_plot.borrow_mut(), plot.borrow_mut());

        match msg {
            Msg::Gender(gender) => {
                let gender = gender != 0;

                // Player
                player.set_is_female(gender);

                // Plot
                // ME1
                if let Some(mut is_female) = me1_plot.booleans_mut().get_mut(4639) {
                    *is_female = gender;
                }
                // ME2
                if let Some(mut is_female) = plot.booleans_mut().get_mut(66) {
                    *is_female = gender;
                }

                false
            }
            Msg::Origin(origin_idx) => {
                // Player
                *player.origin_mut() = Origin::from(origin_idx);

                // ME1 imported
                match *player.origin() {
                    Origin::None => {}
                    Origin::Spacer => {
                        if let Some(mut spacer) = plot.booleans_mut().get_mut(1533) {
                            *spacer = true;
                        }
                        if let Some(mut colonist) = plot.booleans_mut().get_mut(1535) {
                            *colonist = false;
                        }
                        if let Some(mut eathborn) = plot.booleans_mut().get_mut(1534) {
                            *eathborn = false;
                        }
                    }
                    Origin::Colonist => {
                        if let Some(mut spacer) = plot.booleans_mut().get_mut(1533) {
                            *spacer = false;
                        }
                        if let Some(mut colonist) = plot.booleans_mut().get_mut(1535) {
                            *colonist = true;
                        }
                        if let Some(mut eathborn) = plot.booleans_mut().get_mut(1534) {
                            *eathborn = false;
                        }
                    }
                    Origin::Earthborn => {
                        if let Some(mut spacer) = plot.booleans_mut().get_mut(1533) {
                            *spacer = false;
                        }
                        if let Some(mut colonist) = plot.booleans_mut().get_mut(1535) {
                            *colonist = false;
                        }
                        if let Some(mut eathborn) = plot.booleans_mut().get_mut(1534) {
                            *eathborn = true;
                        }
                    }
                }

                // ME1 plot
                if let Some(me1_origin) = me1_plot.integers_mut().get_mut(1) {
                    me1_origin.set(origin_idx as i32);
                }

                false
            }
            Msg::Notoriety(notoriety_idx) => {
                // Player
                *player.notoriety_mut() = Notoriety::from(notoriety_idx);

                // ME1 imported
                match *player.notoriety() {
                    Notoriety::None => {}
                    Notoriety::Survivor => {
                        if let Some(mut survivor) = plot.booleans_mut().get_mut(1537) {
                            *survivor = true;
                        }
                        if let Some(mut war_hero) = plot.booleans_mut().get_mut(1538) {
                            *war_hero = false;
                        }
                        if let Some(mut ruthless) = plot.booleans_mut().get_mut(1539) {
                            *ruthless = false;
                        }
                    }
                    Notoriety::Warhero => {
                        if let Some(mut survivor) = plot.booleans_mut().get_mut(1537) {
                            *survivor = false;
                        }
                        if let Some(mut war_hero) = plot.booleans_mut().get_mut(1538) {
                            *war_hero = true;
                        }
                        if let Some(mut ruthless) = plot.booleans_mut().get_mut(1539) {
                            *ruthless = false;
                        }
                    }
                    Notoriety::Ruthless => {
                        if let Some(mut survivor) = plot.booleans_mut().get_mut(1537) {
                            *survivor = false;
                        }
                        if let Some(mut war_hero) = plot.booleans_mut().get_mut(1538) {
                            *war_hero = false;
                        }
                        if let Some(mut ruthless) = plot.booleans_mut().get_mut(1539) {
                            *ruthless = true;
                        }
                    }
                }

                // ME1 plot
                if let Some(me1_notoriety) = me1_plot.integers_mut().get_mut(2) {
                    me1_notoriety.set(notoriety_idx as i32);
                }

                false
            }
            Msg::PlayerClass(class_idx) => {
                *player.class_name_mut() = Me2Class::names()[class_idx].to_owned();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (difficulty, end_game_state, player, plot) = match ctx.props().save_game {
            Me2Type::Vanilla(ref me2) => {
                let me2 = me2.borrow();
                (
                    RcRef::clone(&me2.difficulty),
                    RcRef::clone(&me2.end_game_state),
                    RcRef::clone(&me2.player),
                    RcRef::clone(&me2.plot),
                )
            }
            Me2Type::Legendary(ref me2) => {
                let me2 = me2.borrow();
                (
                    RcRef::clone(&me2.difficulty),
                    RcRef::clone(&me2.end_game_state),
                    RcRef::clone(&me2.player),
                    RcRef::clone(&me2.plot),
                )
            }
        };

        html! {
            <div class="flex divide-solid divide-x divide-default-border">
                <div class="flex-1 pr-1 flex flex-col gap-1">
                    { Self::role_play(ctx, player.borrow()) }
                    { Self::morality(plot.borrow()) }
                    { Self::gameplay(ctx, player.borrow()) }
                    { Self::resources(player.borrow()) }
                </div>
                <div class="flex-1 pl-1 flex flex-col gap-1">
                    { Self::general(difficulty, end_game_state) }
                    { Self::bonus_powers(player.borrow()) }
                </div>
            </div>
        }
    }
}

impl Me2General {
    fn role_play(ctx: &Context<Self>, player: Ref<'_, Player>) -> Html {
        let link = ctx.link();
        let genders: &'static [&'static str] = &["Male", "Female"];
        html! {
            <Table title="Role-Play">
                { player.first_name.view("Name") }
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={genders}
                        current_idx={player.is_female() as usize}
                        onselect={link.callback(Msg::Gender)}
                    />
                    {"Gender"}
                    <Helper text=
                        "If you change your gender, disable the head morph or import an appropriate one. \
                        Otherwise, the Collectors will be the least of your worries..."
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
                <InputText label="Identity Code" value={RcRef::clone(&player.face_code)} helper=
                    "If you change this you can display whatever you want in the menus \
                    in place of your `Identity Code`.\n\
                    This will NOT change your face, you have to edit your head morph \
                    or import one to do so."
                />
            </Table>
        }
    }

    fn morality(plot: Ref<'_, PlotTable>) -> Html {
        html! {
            <Table title="Morality">
                { for plot.integers().get(2).map(|paragon| paragon.view("Paragon")) }
                { for plot.integers().get(3).map(|renegade| renegade.view("Renegade")) }
            </Table>
        }
    }

    fn gameplay(ctx: &Context<Self>, player: Ref<'_, Player>) -> Html {
        let Player { level, current_xp, talent_points, credits, medigel, .. } = &*player;

        let class_idx = Me2Class::names()
            .iter()
            .enumerate()
            .find_map(|(i, name)| player.class_name().eq_ignore_ascii_case(name).then(|| i))
            .unwrap_or_default();

        html! {
            <Table title="Gameplay">
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={Me2Class::variants()}
                        current_idx={class_idx}
                        onselect={ctx.link().callback(Msg::PlayerClass)}
                    />
                    {"Class"}
                </div>
                { level.view("Level") }
                { current_xp.view("Current XP") }
                { talent_points.view("Talent Points") }
                { credits.view("Credits") }
                { medigel.view("Medigel") }
            </Table>
        }
    }

    fn resources(player: Ref<'_, Player>) -> Html {
        let Player { eezo, iridium, palladium, platinum, probes, current_fuel, .. } = &*player;

        html! {
            <Table title="Resources">
                { eezo.view("Eezo") }
                { iridium.view("Iridium") }
                { palladium.view("Palladium") }
                { platinum.view("Platinum") }
                { probes.view("Probes") }
                { current_fuel.view("Current Fuel") }
            </Table>
        }
    }

    fn general(difficulty: RcRef<Difficulty>, end_game_state: RcRef<EndGameState>) -> Html {
        html! {
            <Table title="General">
                { difficulty.view("Difficulty") }
                { end_game_state.view("End Game Stage") }
            </Table>
        }
    }

    fn bonus_powers(player: Ref<'_, Player>) -> Html {
        let power_list: &'static [(&'static str, &'static str, &'static str)] = &[
            ("Slam", "SFXGameContent_Powers.SFXPower_Crush_Player", "Slam"),
            ("Barrier", "SFXGameContent_Powers.SFXPower_Barrier_Player", "Barrier"),
            ("WarpAmmo", "SFXGameContent_Powers.SFXPower_WarpAmmo_Player", "Warp Ammo"),
            (
                "Fortification",
                "SFXGameContent_Powers.SFXPower_Fortification_Player",
                "Fortification",
            ),
            (
                "ArmorPiercingAmmo",
                "SFXGameContent_Powers.SFXPower_ArmorPiercingAmmo_Player",
                "Armor Piercing Ammo",
            ),
            ("NeuralShock", "SFXGameContent_Powers.SFXPower_NeuralShock_Player", "Neural Shock"),
            ("ShieldJack", "SFXGameContent_Powers.SFXPower_ShieldJack_Player", "Energy Drain"),
            ("Reave", "SFXGameContent_Powers.SFXPower_Reave_Player", "Reave"),
            ("Dominate", "SFXGameContent_Powers.SFXPower_Dominate_Player", "Dominate"),
            (
                "AntiOrganicAmmo",
                "SFXGameContent_Powers.SFXPower_AntiOrganicAmmo_Player",
                "Shredder Ammo",
            ),
            (
                "GethShieldBoost",
                "SFXGameContent_Powers.SFXPower_GethShieldBoost_Player",
                "Geth Shield Boost",
            ),
            (
                "ZaeedUnique",
                "SFXGameContentDLC_HEN_VT.SFXPower_ZaeedUnique_Player",
                "Inferno Grenade",
            ),
            (
                "KasumiUnique",
                "SFXGameContentKasumi.SFXPower_KasumiUnique_Player",
                "Flashbang Grenade",
            ),
            ("StasisNew", "SFXGameContentLiara.SFXPower_StasisNew", "Stasis"),
        ];

        html! {
            <BonusPowers {power_list} powers={BonusPowerType::Me2(RcRef::clone(&player.powers))} helper=
                "You can use as many bonus powers as you want and customize your build \
                to your liking. The only restriction is the size of your screen !\n\
                If you want to remove a bonus power you need to reset your talents \
                `before` or you will lose some talent points.\n\
                Unlike Mass Effect 3, the game will never recalculate your points. \
                At level 30 you have `51` points to spend."
            />
        }
    }
}
