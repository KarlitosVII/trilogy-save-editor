use std::cell::{Ref, RefMut};

use yew::prelude::*;

use crate::{
    gui::{
        components::{Helper, InputText, Select, Table},
        raw_ui::RawUi,
        shared::{BonusPowerType, BonusPowers},
        RcUi,
    },
    save_data::{
        mass_effect_3::{player::Player, plot::PlotTable, Me3SaveGame},
        shared::player::{Notoriety, Origin},
    },
};

#[derive(Clone, RawUi)]
enum Me3Class {
    Soldier,
    SoldierNonCombat,
    Engineer,
    EngineerNonCombat,
    Adept,
    AdeptNonCombat,
    Infiltrator,
    InfiltratorNonCombat,
    Sentinel,
    SentinelNonCombat,
    Vanguard,
    VanguardNonCombat,
}

impl Me3Class {
    fn names() -> &'static [&'static str] {
        &[
            "SFXGame.SFXPawn_PlayerSoldier",
            "SFXGame.SFXPawn_PlayerSoldierNonCombat",
            "SFXGame.SFXPawn_PlayerEngineer",
            "SFXGame.SFXPawn_PlayerEngineerNonCombat",
            "SFXGame.SFXPawn_PlayerAdept",
            "SFXGame.SFXPawn_PlayerAdeptNonCombat",
            "SFXGame.SFXPawn_PlayerInfiltrator",
            "SFXGame.SFXPawn_PlayerInfiltratorNonCombat",
            "SFXGame.SFXPawn_PlayerSentinel",
            "SFXGame.SFXPawn_PlayerSentinelNonCombat",
            "SFXGame.SFXPawn_PlayerVanguard",
            "SFXGame.SFXPawn_PlayerVanguardNonCombat",
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
    pub save_game: RcUi<Me3SaveGame>,
}

impl Props {
    fn save_game(&self) -> Ref<'_, Me3SaveGame> {
        self.save_game.borrow()
    }

    fn save_game_mut(&self) -> RefMut<'_, Me3SaveGame> {
        self.save_game.borrow_mut()
    }
}

pub struct Me3General;

impl Component for Me3General {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Me3General
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Me3SaveGame { player, plot, .. } = &mut *ctx.props().save_game_mut();
        let (mut player, mut plot) = (player.borrow_mut(), plot.borrow_mut());
        match msg {
            Msg::Gender(gender) => {
                let gender = gender != 0;

                // Player
                *player.is_female_mut() = gender;

                // Plot
                // ME1
                if let Some(mut is_female) = plot.booleans_mut().get_mut(14639) {
                    *is_female = gender;
                }
                // ME2
                if let Some(mut is_female) = plot.booleans_mut().get_mut(66) {
                    *is_female = gender;
                }
                // ME3
                if let Some(mut is_female) = plot.booleans_mut().get_mut(17662) {
                    *is_female = gender;
                }

                // Loco / Lola
                let is_loco = plot.booleans().get(19578).map(|b| *b).unwrap_or_else(|| false);
                let is_lola = plot.booleans().get(19579).map(|b| *b).unwrap_or_else(|| false);

                if is_loco || is_lola {
                    if let Some(mut is_loco) = plot.booleans_mut().get_mut(19578) {
                        *is_loco = !gender;
                    }
                    if let Some(mut is_lola) = plot.booleans_mut().get_mut(19579) {
                        *is_lola = gender;
                    }
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
                if let Some(me1_origin) = plot.integers_mut().get_mut(&10001) {
                    *me1_origin.borrow_mut() = origin_idx as i32;
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
                if let Some(me1_notoriety) = plot.integers_mut().get_mut(&10002) {
                    *me1_notoriety.borrow_mut() = notoriety_idx as i32;
                }
                false
            }
            Msg::PlayerClass(class_idx) => {
                *player.class_name_mut() = Me3Class::names()[class_idx].to_owned();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let save_game = ctx.props().save_game();

        html! {
            <div class="flex divide-solid divide-x divide-default-border">
                <div class="flex-1 pr-1 flex flex-col gap-1">
                    { Self::role_play(ctx, save_game.player()) }
                    { Self::morality(save_game.plot()) }
                    { Self::gameplay(ctx, save_game.player()) }
                </div>
                <div class="flex-1 pl-1 flex flex-col gap-1">
                    { Self::general(&save_game) }
                    { Self::bonus_powers(save_game.player()) }
                </div>
            </div>
        }
    }
}

impl Me3General {
    fn role_play(ctx: &Context<Self>, player: Ref<'_, Player>) -> Html {
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
                        Otherwise, the Reapers will be the least of your worries..."
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

    fn morality(plot: Ref<'_, PlotTable>) -> Html {
        html! {
            <Table title="Morality">
                { for plot.integers().get(&10159).map(|paragon| paragon.view("Paragon")) }
                { for plot.integers().get(&10160).map(|renegade| renegade.view("Renegade")) }
                { for plot.integers().get(&10297).map(|renegade| renegade.view("Reputation")) }
                { for plot.integers().get(&10380).map(|renegade| renegade.view("Reputation Points")) }
            </Table>
        }
    }

    fn gameplay(ctx: &Context<Self>, player: Ref<'_, Player>) -> Html {
        let Player {
            level,
            current_xp,
            talent_points,
            credits,
            medigel,
            grenades,
            current_fuel,
            ..
        } = &*player;

        let class_idx = Me3Class::names()
            .iter()
            .enumerate()
            .find_map(|(i, name)| player.class_name().eq_ignore_ascii_case(name).then(|| i))
            .unwrap_or_default();

        html! {
            <Table title="Gameplay">
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={Me3Class::variants()}
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
                { grenades.view("Grenades") }
                { current_fuel.view("Current Fuel") }
            </Table>
        }
    }

    fn general(save_game: &Ref<Me3SaveGame>) -> Html {
        let Me3SaveGame { difficulty, end_game_state, conversation_mode, .. } = &**save_game;
        html! {
            <Table title="General">
                { difficulty.view("Difficulty") }
                { conversation_mode.view("Conversation Mode") }
                { end_game_state.view("End Game Stage") }
            </Table>
        }
    }

    fn bonus_powers(player: Ref<'_, Player>) -> Html {
        let power_list: &'static [(&'static str, &'static str, &'static str)] = &[
            ("EnergyDrain", "SFXGameContent.SFXPowerCustomAction_EnergyDrain", "Energy Drain"),
            (
                "ProtectorDrone",
                "SFXGameContent.SFXPowerCustomAction_ProtectorDrone",
                "Defense Drone",
            ),
            (
                "GethShieldBoost",
                "SFXGameContent.SFXPowerCustomAction_GethShieldBoost",
                "Defense Matrix",
            ),
            ("Decoy", "SFXGameContent.SFXPowerCustomAction_Decoy", "Decoy"),
            (
                "ArmorPiercingAmmo",
                "SFXGameContent.SFXPowerCustomAction_ArmorPiercingAmmo",
                "Armor Piercing Ammo",
            ),
            (
                "ProximityMine",
                "SFXGameContent.SFXPowerCustomAction_ProximityMine",
                "Proximity Mine",
            ),
            ("Barrier", "SFXGameContent.SFXPowerCustomAction_Barrier", "Barrier"),
            ("Reave", "SFXGameContent.SFXPowerCustomAction_Reave", "Reave"),
            (
                "InfernoGrenade",
                "SFXGameContent.SFXPowerCustomAction_InfernoGrenade",
                "Inferno Grenade",
            ),
            ("Marksman", "SFXGameContent.SFXPowerCustomAction_Marksman", "Marksman"),
            ("WarpAmmo", "SFXGameContent.SFXPowerCustomAction_WarpAmmo", "Warp Ammo"),
            ("Stasis", "SFXGameContent.SFXPowerCustomAction_Stasis", "Stasis"),
            ("Fortification", "SFXGameContent.SFXPowerCustomAction_Fortification", "Fortification"),
            ("Carnage", "SFXGameContent.SFXPowerCustomAction_Carnage", "Carnage"),
            ("Slam", "SFXGameContent.SFXPowerCustomAction_Slam", "Slam"),
            ("DarkChannel", "SFXGameContent.SFXPowerCustomAction_DarkChannel", "Dark Channel"),
            ("Dominate", "SFXGameContentDLC_Exp_Pack001.SFXPowerCustomAction_Dominate", "Dominate"),
            ("AriaLash", "SFXGameContentDLC_Exp_Pack002.SFXPowerCustomAction_AriaLash", "Lash"),
            ("Flare", "SFXGameContentDLC_Exp_Pack002.SFXPowerCustomAction_BioticFlare", "Flare"),
        ];

        html! {
            <BonusPowers {power_list} powers={BonusPowerType::Me3(RcUi::clone(&player.powers))} helper=
                "You can use as many bonus powers as you want and customize your build to your liking. \
                The only restriction is the size of your screen !"
            />
        }
    }
}
