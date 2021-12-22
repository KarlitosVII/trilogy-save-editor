use std::{
    cell::{Ref, RefMut},
    mem,
    rc::Rc,
};

use anyhow::Error;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::{
    gui::{
        components::{CallbackType, Helper, InputNumber, InputText, NumberType, Select, Table},
        raw_ui::RawUi,
        RcUi,
    },
    save_data::{
        mass_effect_1_le::{
            player::{Item, Me1LeClass, Player},
            player_class_db::{Me1LePlayerClass, Me1LePlayerClassDb},
            squad::Henchman,
            Me1LeSaveData,
        },
        shared::{
            player::{Notoriety, Origin},
            plot::PlotTable,
        },
    },
    services::database::{Database, DatabaseService, Request, Response, Type},
};

#[derive(Clone, RawUi)]
enum SoldierSpec {
    None = 119,
    ShockTrooper = 137,
    Commando = 141,
}

#[derive(Clone, RawUi)]
enum EngineerSpec {
    None = 122,
    Operative = 145,
    Medic = 149,
}

#[derive(Clone, RawUi)]
enum AdeptSpec {
    None = 126,
    Nemesis = 153,
    Bastion = 157,
}

#[derive(Clone, RawUi)]
enum InfiltratorSpec {
    None = 128,
    Commando = 142,
    Operative = 146,
}

#[derive(Clone, RawUi)]
enum SentinelSpec {
    None = 131,
    Medic = 150,
    Bastion = 158,
}

#[derive(Clone, RawUi)]
enum VanguardSpec {
    None = 134,
    ShockTrooper = 138,
    Nemesis = 154,
}

pub enum Msg {
    PlayerClassDb(Rc<Me1LePlayerClassDb>),
    Error(Error),
    Gender(usize),
    Origin(usize),
    Notoriety(usize),
    Difficulty(usize),
    TalentPoints(CallbackType),
    ResetTalents(Option<&'static str>),
    PlayerClass(usize),
    PlayerSpecialization(usize),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub save_game: RcUi<Me1LeSaveData>,
    pub onerror: Callback<Error>,
}

impl Props {
    fn save_game(&self) -> Ref<'_, Me1LeSaveData> {
        self.save_game.borrow()
    }

    fn save_game_mut(&self) -> RefMut<'_, Me1LeSaveData> {
        self.save_game.borrow_mut()
    }
}

pub struct Me1LeGeneral {
    _database_service: Box<dyn Bridge<DatabaseService>>,
    player_class_db: Option<Rc<Me1LePlayerClassDb>>,
}

impl Component for Me1LeGeneral {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut database_service =
            DatabaseService::bridge(ctx.link().callback(|response| match response {
                Response::Database(Database::Me1LePlayerClasses(db)) => Msg::PlayerClassDb(db),
                Response::Error(err) => Msg::Error(err),
                _ => unreachable!(),
            }));

        database_service.send(Request::Database(Type::Me1LePlayerClasses));

        Me1LeGeneral { _database_service: database_service, player_class_db: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Me1LeSaveData { player, squad, plot, .. } = &mut *ctx.props().save_game_mut();
        let (mut player, mut plot) = (player.borrow_mut(), plot.borrow_mut());
        match msg {
            Msg::PlayerClassDb(db) => {
                self.player_class_db = Some(db);
                true
            }
            Msg::Error(err) => {
                ctx.props().onerror.emit(err);
                false
            }
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
            Msg::PlayerClass(class_idx) => {
                if let Some(ref player_class_db) = self.player_class_db {
                    if let Some(new_class_data) = player_class_db.get(class_idx) {
                        let Me1LePlayerClass {
                            player_class,
                            localized_class_name,
                            auto_levelup_template_id,
                            simple_talents,
                            complex_talents,
                            armor,
                            omni_tool,
                            bio_amp,
                        } = new_class_data;

                        *player.player_class_mut() = player_class.clone();
                        *player.specialization_bonus_id_mut() = -1;
                        *player.localized_class_name_mut() = *localized_class_name;
                        *player.auto_levelup_template_id_mut() = *auto_levelup_template_id;

                        *player.simple_talents_mut() = simple_talents.clone();

                        // Complex talents
                        {
                            let mut current_complex_talents = player.complex_talents.borrow_mut();

                            const IGNORED_TALENTS: &[i32] = &[
                                108, // Charm
                                109, // Intimidate
                                259, // Spectre
                            ];

                            current_complex_talents.retain(|talent| {
                                let talent = talent.borrow();
                                let talent_id = talent.talent_id();
                                let is_ignored = IGNORED_TALENTS.contains(&talent_id);

                                // Reset non-ignored talent points before deleting it
                                if !is_ignored {
                                    *player.talent_points.borrow_mut() += *talent.current_rank();
                                }
                                is_ignored
                            });

                            // Append the new class talents
                            let mut new_complex_talents = complex_talents.clone();
                            current_complex_talents.append(&mut new_complex_talents);
                        }

                        // Gear
                        {
                            let inventory = player.inventory();
                            let equipment = inventory.equipment();

                            let mut unequipped_items: Vec<RcUi<Item>> = vec![];

                            let mut unequip_item_and_mods = |mut base_item: Item| {
                                if *base_item.item_id() == 0 {
                                    return;
                                }

                                // Mods
                                for detached_mod in base_item.attached_mods_mut().drain(..) {
                                    let detached_mod = detached_mod.borrow();

                                    let mut item = Item::default();
                                    item.item_id = detached_mod.item_id.clone();
                                    item.item_level = detached_mod.item_level.clone();
                                    item.manufacturer_id = detached_mod.manufacturer_id.clone();
                                    item.plot_conditional_id =
                                        detached_mod.plot_conditional_id.clone();
                                    *item.new_item_mut() = true;

                                    unequipped_items.push(item.into());
                                }

                                // Base item
                                *base_item.new_item_mut() = true;
                                unequipped_items.push(base_item.into());
                            };

                            // Armor
                            if let Some(current_armor) = equipment.get(1) {
                                let new_armor = armor.clone();
                                let old_armor =
                                    mem::replace(&mut *current_armor.borrow_mut(), new_armor);
                                unequip_item_and_mods(old_armor);
                            }

                            // Omni-Tool
                            if let Some(current_omni_tool) = equipment.get(3) {
                                let new_omni_tool = omni_tool.clone();
                                let old_omni_tool = mem::replace(
                                    &mut *current_omni_tool.borrow_mut(),
                                    new_omni_tool,
                                );
                                unequip_item_and_mods(old_omni_tool);
                            }

                            // Biotic Amp
                            if let Some(current_bio_amp) = equipment.get(4) {
                                let new_bio_amp = bio_amp.clone();
                                let old_bio_amp =
                                    mem::replace(&mut *current_bio_amp.borrow_mut(), new_bio_amp);
                                unequip_item_and_mods(old_bio_amp);
                            }

                            // Move the unequipped gear to the inventory
                            {
                                let mut inventory = inventory.inventory.borrow_mut();
                                inventory.append(&mut unequipped_items);
                            }
                        }
                    }
                }
                true
            }
            Msg::PlayerSpecialization(spec_idx) => {
                let specs = match *player.player_class() {
                    Me1LeClass::Soldier => [
                        SoldierSpec::None as i32,
                        SoldierSpec::ShockTrooper as i32,
                        SoldierSpec::Commando as i32,
                    ],
                    Me1LeClass::Engineer => [
                        EngineerSpec::None as i32,
                        EngineerSpec::Operative as i32,
                        EngineerSpec::Medic as i32,
                    ],
                    Me1LeClass::Adept => [
                        AdeptSpec::None as i32,
                        AdeptSpec::Nemesis as i32,
                        AdeptSpec::Bastion as i32,
                    ],
                    Me1LeClass::Infiltrator => [
                        InfiltratorSpec::None as i32,
                        InfiltratorSpec::Commando as i32,
                        InfiltratorSpec::Operative as i32,
                    ],
                    Me1LeClass::Sentinel => [
                        SentinelSpec::None as i32,
                        SentinelSpec::Medic as i32,
                        SentinelSpec::Bastion as i32,
                    ],
                    Me1LeClass::Vanguard => [
                        VanguardSpec::None as i32,
                        VanguardSpec::ShockTrooper as i32,
                        VanguardSpec::Nemesis as i32,
                    ],
                };

                let complex_talents = player.complex_talents.borrow_mut();
                let current_spec = complex_talents.iter().find(|talent| {
                    let talent = talent.borrow();
                    let talent_id = talent.talent_id();
                    specs.contains(&talent_id)
                });

                if let Some(current_spec) = current_spec {
                    let mut current_spec = current_spec.borrow_mut();

                    // Reset talent points
                    *player.talent_points.borrow_mut() += *current_spec.current_rank();

                    // Set new spec
                    let new_spec = specs[spec_idx];
                    let max_rank = if spec_idx == 0 { 6 } else { 12 };

                    *player.specialization_bonus_id.borrow_mut() =
                        if spec_idx == 0 { -1 } else { new_spec };
                    *current_spec.talent_id_mut() = new_spec;
                    *current_spec.current_rank_mut() = 0;
                    *current_spec.max_rank_mut() = max_rank;
                }
                true
            }
            _ => unreachable!(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref _player_class_db) = self.player_class_db {
            let save_game = ctx.props().save_game();

            html! {
                <div class="flex divide-solid divide-x divide-default-border">
                    <div class="flex-1 pr-1 flex flex-col gap-1">
                        { self.role_play(ctx, save_game.player()) }
                        { self.gameplay(ctx, save_game.player()) }
                    </div>
                    <div class="flex-1 pl-1 flex flex-col gap-1">
                        { self.general(ctx, save_game.player().game_options()) }
                        { self.morality(save_game.plot()) }
                        { self.resources(save_game.player()) }
                        { self.squad(ctx, save_game.squad()) }
                    </div>
                </div>
            }
        } else {
            html! {
                <p>{ "Loading database..." }</p>
            }
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

    fn gameplay(&self, ctx: &Context<Self>, player: Ref<'_, Player>) -> Html {
        let Player { level, current_xp, player_class, specialization_bonus_id, .. } = &*player;

        let player_class = player_class.borrow();
        let current_player_class = player_class.clone() as usize;
        let specialization_id = *specialization_bonus_id.borrow();

        let (specialization_variant, current_specialization) = match *player_class {
            Me1LeClass::Soldier => {
                let current = match specialization_id {
                    id if id == SoldierSpec::ShockTrooper as i32 => 1,
                    id if id == SoldierSpec::Commando as i32 => 2,
                    _ => 0,
                };
                (SoldierSpec::variants(), current)
            }
            Me1LeClass::Engineer => {
                let current = match specialization_id {
                    id if id == EngineerSpec::Operative as i32 => 1,
                    id if id == EngineerSpec::Medic as i32 => 2,
                    _ => 0,
                };
                (EngineerSpec::variants(), current)
            }
            Me1LeClass::Adept => {
                let current = match specialization_id {
                    id if id == AdeptSpec::Nemesis as i32 => 1,
                    id if id == AdeptSpec::Bastion as i32 => 2,
                    _ => 0,
                };
                (AdeptSpec::variants(), current)
            }
            Me1LeClass::Infiltrator => {
                let current = match specialization_id {
                    id if id == InfiltratorSpec::Commando as i32 => 1,
                    id if id == InfiltratorSpec::Operative as i32 => 2,
                    _ => 0,
                };
                (InfiltratorSpec::variants(), current)
            }
            Me1LeClass::Sentinel => {
                let current = match specialization_id {
                    id if id == SentinelSpec::Medic as i32 => 1,
                    id if id == SentinelSpec::Bastion as i32 => 2,
                    _ => 0,
                };
                (SentinelSpec::variants(), current)
            }
            Me1LeClass::Vanguard => {
                let current = match specialization_id {
                    id if id == VanguardSpec::ShockTrooper as i32 => 1,
                    id if id == VanguardSpec::Nemesis as i32 => 2,
                    _ => 0,
                };
                (VanguardSpec::variants(), current)
            }
        };

        html! {
            <Table title="Gameplay">
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={Me1LeClass::variants()}
                        current_idx={current_player_class}
                        onselect={ctx.link().callback(Msg::PlayerClass)}
                    />
                    { "Class" }
                    <Helper text=
                        "If you change your class: \n\
                        • Your talent points will be reset\n\
                        • Your specialization will be set to `None`\n\
                        • Your bonus talents will be removed\n\
                        • You will have to put them back manually after that"
                    />
                </div>
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={specialization_variant}
                        current_idx={current_specialization}
                        onselect={ctx.link().callback(Msg::PlayerSpecialization)}
                    />
                    { "Specialization" }
                </div>
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
