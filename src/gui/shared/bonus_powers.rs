use yew::prelude::*;

use crate::gui::{components::Table, RcUi};
use crate::save_data::{
    mass_effect_2::player::Power as Me2Power, mass_effect_3::player::Power as Me3Power,
};

#[derive(Clone)]
pub enum BonusPowerType {
    Me2(RcUi<Vec<RcUi<Me2Power>>>),
    Me3(RcUi<Vec<RcUi<Me3Power>>>),
}

impl PartialEq for BonusPowerType {
    fn eq(&self, other: &BonusPowerType) -> bool {
        match (self, other) {
            (BonusPowerType::Me2(me2_powers), BonusPowerType::Me2(other)) => me2_powers == other,
            (BonusPowerType::Me3(me3_powers), BonusPowerType::Me3(other)) => me3_powers == other,
            _ => false,
        }
    }
}

pub enum Msg {
    ToggleBonusPower(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub power_list: &'static [(&'static str, &'static str)], // TODO: Add power name
    pub powers: BonusPowerType,
    pub helper: Option<&'static str>,
}

pub struct BonusPowers;

impl Component for BonusPowers {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        BonusPowers
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleBonusPower(power_class_name) => {
                match ctx.props().powers {
                    BonusPowerType::Me2(ref powers) => {
                        let idx = powers.borrow().iter().enumerate().find_map(|(i, power)| {
                            power
                                .borrow()
                                .power_class_name()
                                .eq_ignore_ascii_case(&power_class_name)
                                .then(|| i)
                        });

                        if let Some(idx) = idx {
                            powers.borrow_mut().remove(idx);
                        } else {
                            let power = Me2Power::default();
                            *power.power_class_name.borrow_mut() = power_class_name;
                            powers.borrow_mut().push(power.into());
                        }
                    }
                    BonusPowerType::Me3(ref powers) => {
                        let idx = powers.borrow().iter().enumerate().find_map(|(i, power)| {
                            power
                                .borrow()
                                .power_class_name()
                                .eq_ignore_ascii_case(&power_class_name)
                                .then(|| i)
                        });

                        if let Some(idx) = idx {
                            powers.borrow_mut().remove(idx);
                        } else {
                            let power = Me3Power::default();
                            *power.power_class_name.borrow_mut() = power_class_name;
                            powers.borrow_mut().push(power.into());
                        }
                    }
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { power_list, powers, helper } = &ctx.props();

        let selectables = power_list.iter().map(|&(power_class_name, power_name)| {
            let selected = match powers {
                BonusPowerType::Me2(powers) => powers.borrow()
                .iter()
                .any(|power| power.borrow().power_class_name().eq_ignore_ascii_case(power_class_name)),
                BonusPowerType::Me3(powers) => powers.borrow()
                .iter()
                .any(|power| power.borrow().power_class_name().eq_ignore_ascii_case(power_class_name)),
            };

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
                    onclick={ctx.link().callback(move |_| Msg::ToggleBonusPower(power_class_name.to_owned()))}
                >
                    {power_name}
                </button>
            }
        });

        html! {
            <Table title="Bonus Powers" helper={*helper}>
                { for selectables }
            </Table>
        }
    }
}
