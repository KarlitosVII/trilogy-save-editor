use yew::prelude::*;

use crate::{
    gui::{components::Table, RcUi},
    save_data::mass_effect_1_le::player::ComplexTalent,
};

pub enum Msg {
    ToggleBonusTalent(i32),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub talent_list: RcUi<Vec<i32>>,
    pub player_talents: RcUi<Vec<RcUi<ComplexTalent>>>,
    pub helper: Option<&'static str>,
    pub onselect: Callback<Option<i32>>,
}

pub struct BonusTalents;

impl Component for BonusTalents {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        BonusTalents
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleBonusTalent(talent_id) => {
                let Props { player_talents, onselect, .. } = &ctx.props();
                let found = player_talents.borrow().iter().enumerate().find_map(|(i, talent)| {
                    (*talent.borrow().talent_id() == talent_id)
                        .then(|| (i, *talent.borrow().current_rank()))
                });

                let callback = if let Some((idx, spent_points)) = found {
                    player_talents.borrow_mut().remove(idx);
                    Some(spent_points)
                } else {
                    let mut talent = ComplexTalent::default();
                    *talent.talent_id_mut() = talent_id;
                    *talent.max_rank_mut() = 12;
                    *talent.level_offset_mut() = -1;
                    *talent.levels_per_rank_mut() = 1;
                    *talent.visual_order_mut() = 85;

                    player_talents.borrow_mut().push(talent.into());
                    None
                };

                onselect.emit(callback);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { talent_list, player_talents, helper, .. } = &ctx.props();

        const BONUS_TALENTS: &[(i32, &str)] = &[
            (50, "Lift"),
            (49, "Throw"),
            (56, "Warp"),
            (57, "Singularity"),
            (63, "Barrier"),
            (64, "Stasis"),
            (86, "Damping"),
            (91, "Hacking"),
            (84, "Electronics"),
            (93, "Decryption"),
            (98, "First Aid"),
            (99, "Medicine"),
            (15, "Shotguns"),
            (7, "Assault Rifles"),
            (21, "Sniper Rifles"),
        ];

        let selectables = BONUS_TALENTS.iter().filter_map(|&(talent_id, talent_label)| {
            talent_list.borrow().iter().any(|&filter| filter == talent_id).then(|| {
                let selected = player_talents
                    .borrow()
                    .iter()
                    .any(|talent| *talent.borrow().talent_id() == talent_id);

                html! {
                    <button
                        class={classes![
                            "rounded-none",
                            "hover:bg-theme-hover",
                            "active:bg-theme-active",
                            "px-1",
                            "w-full",
                            "text-left",
                            selected.then(|| "bg-theme-bg"),
                        ]}
                        onclick={ctx.link().callback(move |_| Msg::ToggleBonusTalent(talent_id))}
                    >
                        {talent_label}
                    </button>
                }
            })
        });

        html! {
            <Table title="Bonus Talents" helper={*helper}>
                { for selectables }
            </Table>
        }
    }
}
