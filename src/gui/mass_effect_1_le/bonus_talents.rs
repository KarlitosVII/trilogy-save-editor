use yew::prelude::*;

use crate::{
    gui::components::Table,
    save_data::{
        mass_effect_1_le::player::{ComplexTalent, SimpleTalent},
        RcRef,
    },
};

const BONUS_TALENTS: &[(i32, &[i32], &str)] = &[
    (50, &[248], "Lift"),
    (49, &[247], "Throw"),
    (56, &[249], "Warp"),
    (57, &[250], "Singularity"),
    (63, &[251], "Barrier"),
    (64, &[252], "Stasis"),
    (86, &[254], "Damping"),
    (91, &[256], "Hacking"),
    (84, &[253], "Electronics"),
    (93, &[255], "Decryption"),
    (98, &[257], "First Aid"),
    (99, &[257, 258], "Medicine"),
    (15, &[244], "Shotguns"),
    (7, &[245], "Assault Rifles"),
    (21, &[246], "Sniper Rifles"),
];

pub enum Msg {
    ToggleBonusTalent(usize),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub talent_list: RcRef<Vec<i32>>,
    pub simple_talents: RcRef<Vec<RcRef<SimpleTalent>>>,
    pub complex_talents: RcRef<Vec<RcRef<ComplexTalent>>>,
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
            Msg::ToggleBonusTalent(talent_idx) => {
                let Props { simple_talents, complex_talents, onselect, .. } = &ctx.props();

                let (complex_id, simple_ids, _) = BONUS_TALENTS[talent_idx];

                let found = complex_talents.borrow().iter().enumerate().find_map(|(i, talent)| {
                    (talent.borrow().talent_id() == complex_id)
                        .then(|| (i, talent.borrow().current_rank()))
                });

                let callback = if let Some((idx, spent_points)) = found {
                    complex_talents.borrow_mut().remove(idx);

                    simple_talents.borrow_mut().retain(|talent| {
                        let talent_id = talent.borrow().talent_id();
                        !simple_ids.contains(&talent_id)
                    });

                    Some(spent_points)
                } else {
                    let mut complex = ComplexTalent::default();
                    complex.set_talent_id(complex_id);
                    complex.set_max_rank(12);
                    complex.set_level_offset(-1);
                    complex.set_levels_per_rank(1);
                    complex.set_visual_order(85);

                    complex_talents.borrow_mut().push(complex.into());

                    for &simple_id in simple_ids {
                        let mut simple = SimpleTalent::default();
                        simple.set_talent_id(simple_id);
                        simple.set_current_rank(1);

                        simple_talents.borrow_mut().push(simple.into());
                    }
                    None
                };

                onselect.emit(callback);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { talent_list, complex_talents, helper, .. } = &ctx.props();

        let selectables =
            BONUS_TALENTS.iter().enumerate().filter_map(|(i, &(complex_id, _, talent_label))| {
                talent_list.borrow().iter().any(|&filter| filter == complex_id).then(|| {
                    let selected = complex_talents
                        .borrow()
                        .iter()
                        .any(|talent| talent.borrow().talent_id() == complex_id);

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
                            onclick={ctx.link().callback(move |_| Msg::ToggleBonusTalent(i))}
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
