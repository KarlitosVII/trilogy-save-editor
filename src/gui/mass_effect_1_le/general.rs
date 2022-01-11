use std::{
    cell::{Ref, RefMut},
    mem,
    rc::Rc,
};

use yew::{context::ContextHandle, prelude::*};

use crate::{
    gui::{
        components::{CallbackType, Helper, InputNumber, InputText, NumberType, Select, Table},
        mass_effect_1_le::bonus_talents::BonusTalents,
        raw_ui::RawUi,
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
        RcCell, RcRef,
    },
    services::database::Databases,
};

#[derive(Clone, RawUi)]
enum SoldierSpec {
    None,
    ShockTrooper,
    Commando,
}

impl SoldierSpec {
    fn ids() -> &'static [i32] {
        &[119, 137, 141]
    }
}

#[derive(Clone, RawUi)]
enum EngineerSpec {
    None,
    Operative,
    Medic,
}

impl EngineerSpec {
    fn ids() -> &'static [i32] {
        &[122, 145, 149]
    }
}

#[derive(Clone, RawUi)]
enum AdeptSpec {
    None,
    Nemesis,
    Bastion,
}

impl AdeptSpec {
    fn ids() -> &'static [i32] {
        &[126, 153, 157]
    }
}

#[derive(Clone, RawUi)]
enum InfiltratorSpec {
    None,
    Commando,
    Operative,
}

impl InfiltratorSpec {
    fn ids() -> &'static [i32] {
        &[128, 142, 146]
    }
}

#[derive(Clone, RawUi)]
enum SentinelSpec {
    None,
    Medic,
    Bastion,
}

impl SentinelSpec {
    fn ids() -> &'static [i32] {
        &[131, 150, 158]
    }
}

#[derive(Clone, RawUi)]
enum VanguardSpec {
    None,
    ShockTrooper,
    Nemesis,
}

impl VanguardSpec {
    fn ids() -> &'static [i32] {
        &[134, 138, 154]
    }
}

pub enum Msg {
    DatabaseLoaded(Databases),
    Gender(usize),
    Origin(usize),
    Notoriety(usize),
    Difficulty(usize),
    TalentPoints(CallbackType),
    ResetTalents(Option<&'static str>),
    PlayerClass(usize),
    PlayerSpecialization(usize),
    BonusTalent(Option<i32>),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub save_game: RcRef<Me1LeSaveData>,
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
    _db_handle: ContextHandle<Databases>,
    player_class_db: Option<Rc<Me1LePlayerClassDb>>,
}

impl Component for Me1LeGeneral {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (databases, _db_handle) = ctx
            .link()
            .context::<Databases>(ctx.link().callback(Msg::DatabaseLoaded))
            .expect("no database provider");

        Me1LeGeneral { _db_handle, player_class_db: databases.get_me1_le_player_classes() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Me1LeSaveData { player, squad, plot, .. } = &mut *ctx.props().save_game_mut();
        let (mut player, mut plot) = (player.borrow_mut(), plot.borrow_mut());
        match msg {
            Msg::DatabaseLoaded(dbs) => {
                self.player_class_db = dbs.get_me1_le_player_classes();
                true
            }
            Msg::Gender(gender) => {
                let gender = gender != 0;

                // Player
                player.set_is_female(gender);

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
                    origin.set(origin_idx as i32);
                }
                false
            }
            Msg::Notoriety(notoriety_idx) => {
                // Player
                *player.notoriety_mut() = Notoriety::from(notoriety_idx);

                // ME1 plot
                if let Some(notoriety) = plot.integers_mut().get_mut(2) {
                    notoriety.set(notoriety_idx as i32);
                }
                false
            }
            Msg::Difficulty(difficulty_idx) => {
                if let Some(difficulty) = player.game_options_mut().get_mut(0) {
                    difficulty.set(difficulty_idx as i32);
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
                    (
                        RcCell::clone(&character.talent_points),
                        RcRef::clone(&character.complex_talents),
                    )
                } else {
                    // Player
                    (RcCell::clone(&player.talent_points), RcRef::clone(&player.complex_talents))
                };

                for talent in complex_talents.borrow_mut().iter_mut() {
                    talent_points.update(|tp| tp + talent.borrow().current_rank());
                    talent.borrow_mut().set_current_rank(0);
                }
                true
            }
            Msg::TalentPoints(CallbackType::Int(talent_points)) => {
                player.set_talent_points(talent_points);
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
                            bonus_talents: _,
                        } = new_class_data;

                        *player.player_class_mut() = player_class.clone();
                        player.set_specialization_bonus_id(-1);
                        player.set_localized_class_name(*localized_class_name);
                        player.set_auto_levelup_template_id(*auto_levelup_template_id);

                        // Simple talents
                        {
                            let mut player_simple_talents = player.simple_talents_mut();

                            // New Game + things
                            const IGNORED_SIMPLES: &[i32] = &[262, 263, 266, 265, 267, 264];

                            player_simple_talents.retain(|talent| {
                                let talent = talent.borrow();
                                let talent_id = talent.talent_id();
                                IGNORED_SIMPLES.contains(&talent_id)
                            });

                            // Append the new class talents
                            let mut new_simple_talents = simple_talents.clone();
                            player_simple_talents.append(&mut new_simple_talents);
                        }

                        // Complex talents
                        let mut spent_talent_points = 0;
                        {
                            let mut player_complex_talents = player.complex_talents_mut();

                            const IGNORED_COMPLEXES: &[i32] = &[
                                108, // Charm
                                109, // Intimidate
                                259, // Spectre
                            ];

                            player_complex_talents.retain(|talent| {
                                let talent = talent.borrow();
                                let talent_id = talent.talent_id();
                                let is_ignored = IGNORED_COMPLEXES.contains(&talent_id);

                                // Reset non-ignored talent points before deleting it
                                if !is_ignored {
                                    spent_talent_points += talent.current_rank();
                                }
                                is_ignored
                            });

                            // Append the new class talents
                            let mut new_complex_talents = complex_talents.clone();
                            player_complex_talents.append(&mut new_complex_talents);
                        }

                        if spent_talent_points > 0 {
                            player.talent_points.update(|tp| tp + spent_talent_points);
                        }

                        // Gear
                        {
                            let inventory = player.inventory();
                            let equipment = inventory.equipment();

                            let mut unequipped_items: Vec<RcRef<Item>> = vec![];

                            let mut unequip_item_and_mods = |mut base_item: Item| {
                                if base_item.item_id() == 0 {
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
                                    item.set_new_item(true);

                                    unequipped_items.push(item.into());
                                }

                                // Base item
                                base_item.set_new_item(true);
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
                    Me1LeClass::Soldier => SoldierSpec::ids(),
                    Me1LeClass::Engineer => EngineerSpec::ids(),
                    Me1LeClass::Adept => AdeptSpec::ids(),
                    Me1LeClass::Infiltrator => InfiltratorSpec::ids(),
                    Me1LeClass::Sentinel => SentinelSpec::ids(),
                    Me1LeClass::Vanguard => VanguardSpec::ids(),
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
                    player.talent_points.update(|tp| tp + current_spec.current_rank());

                    // Set new spec
                    let new_spec = specs[spec_idx];
                    let max_rank = if spec_idx == 0 { 6 } else { 12 };

                    player.specialization_bonus_id.set(if spec_idx == 0 { -1 } else { new_spec });
                    current_spec.set_talent_id(new_spec);
                    current_spec.set_current_rank(0);
                    current_spec.set_max_rank(max_rank);
                }
                true
            }
            Msg::BonusTalent(has_talent_points) => {
                if let Some(talent_points) = has_talent_points {
                    player.talent_points.update(|tp| tp + talent_points);
                }

                true
            }
            _ => unreachable!(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref player_class_db) = self.player_class_db {
            let save_game = ctx.props().save_game();

            html! {
                <div class="flex divide-solid divide-x divide-default-border">
                    <div class="flex-1 pr-1 flex flex-col gap-1">
                        { Self::role_play(ctx, save_game.player()) }
                        { Self::gameplay(ctx, save_game.player()) }
                        { Self::bonus_talents(ctx, player_class_db, save_game.player()) }
                    </div>
                    <div class="flex-1 pl-1 flex flex-col gap-1">
                        { Self::general(ctx, save_game.player().game_options()) }
                        { Self::morality(save_game.plot()) }
                        { Self::resources(save_game.player()) }
                        { Self::squad(ctx, save_game.squad()) }
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
                <InputText label="Identity Code" value={RcRef::clone(&player.face_code)} helper=
                    "If you change this you can display whatever you want in the menus \
                    in place of your `Identity Code`.\n\
                    This will NOT change your face, you have to edit your head morph \
                    or import one to do so."
                />
            </Table>
        }
    }

    fn gameplay(ctx: &Context<Self>, player: Ref<'_, Player>) -> Html {
        let Player { level, current_xp, player_class, specialization_bonus_id, .. } = &*player;

        let player_class = player_class.borrow();
        let current_player_class = player_class.clone() as usize;
        let current_spec_id = specialization_bonus_id.get();

        let (spec_variants, spec_ids) = match *player_class {
            Me1LeClass::Soldier => (SoldierSpec::variants(), SoldierSpec::ids()),
            Me1LeClass::Engineer => (EngineerSpec::variants(), EngineerSpec::ids()),
            Me1LeClass::Adept => (AdeptSpec::variants(), AdeptSpec::ids()),
            Me1LeClass::Infiltrator => (InfiltratorSpec::variants(), InfiltratorSpec::ids()),
            Me1LeClass::Sentinel => (SentinelSpec::variants(), SentinelSpec::ids()),
            Me1LeClass::Vanguard => (VanguardSpec::variants(), VanguardSpec::ids()),
        };

        let current_spec_idx = spec_ids
            .iter()
            .enumerate()
            .find_map(|(i, &spec_id)| (current_spec_id == spec_id).then(|| i))
            .unwrap_or_default();

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
                        • Your gear will be unequipped\n\
                        • You will have to put them back manually after that"
                    />
                </div>
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={spec_variants}
                        current_idx={current_spec_idx}
                        onselect={ctx.link().callback(Msg::PlayerSpecialization)}
                    />
                    { "Specialization" }
                </div>
                <InputNumber
                    label="Level"
                    value={NumberType::Int(RcCell::clone(level))}
                    helper="Classic mode (1 - 60)"
                />
                { current_xp.view("Current XP") }
                <InputNumber
                    label="Talent Points"
                    value={NumberType::Int(player.talent_points().into())}
                    onchange={ctx.link().callback(Msg::TalentPoints)}
                />
                <button class="button" onclick={ctx.link().callback(|_| Msg::ResetTalents(None))}>
                    { "Reset player's talents" }
                </button>
            </Table>
        }
    }

    fn bonus_talents(
        ctx: &Context<Self>, class_db: &Rc<Me1LePlayerClassDb>, player: Ref<'_, Player>,
    ) -> Html {
        let player_class = player.player_class();
        let simple_talents = RcRef::clone(&player.simple_talents);
        let complex_talents = RcRef::clone(&player.complex_talents);

        let talent_list = class_db
            .iter()
            .find_map(|class| {
                (class.player_class == *player_class).then(|| RcRef::clone(&class.bonus_talents))
            })
            .unwrap_or_default();

        html! {
            <BonusTalents {talent_list} {simple_talents} {complex_talents} helper=
                "You can use as many bonus powers as you want and customize your build \
                to your liking.\n\
                The only restriction is that the game will only allow you to use around \
                5-6 offensive abilities for use in game, no matter how many abilities \
                you add. So, don't add talent points in abilities you're not going to use."
                onselect={ctx.link().callback(Msg::BonusTalent)}
            />
        }
    }

    fn general(ctx: &Context<Self>, game_options: Ref<'_, Vec<RcCell<i32>>>) -> Html {
        let difficulty: &'static [&'static str] =
            &["Casual", "Normal", "Veteran", "Hardcore", "Insanity"];
        let current_difficulty = game_options.get(0).map(|d| d.get() as usize).unwrap_or_default();
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

    fn morality(plot: Ref<'_, PlotTable>) -> Html {
        html! {
            <Table title="Morality">
                { for plot.integers().get(47).map(|paragon| paragon.view("Paragon")) }
                { for plot.integers().get(46).map(|renegade| renegade.view("Renegade")) }
            </Table>
        }
    }

    fn resources(player: Ref<'_, Player>) -> Html {
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

    fn squad(ctx: &Context<Self>, squad: Ref<'_, Vec<RcRef<Henchman>>>) -> Html {
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
