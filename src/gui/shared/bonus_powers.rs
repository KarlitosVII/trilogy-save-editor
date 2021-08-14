use yew::{prelude::*, utils::NeqAssign};

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

pub struct BonusPowers {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for BonusPowers {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        BonusPowers { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleBonusPower(power_class_name) => {
                match self.props.powers {
                    BonusPowerType::Me2(ref mut powers) => {
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
                            let mut power = Me2Power::default();
                            *power.power_class_name.borrow_mut() = power_class_name;
                            powers.borrow_mut().push(power.into());
                        }
                    }
                    BonusPowerType::Me3(ref mut powers) => {
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
                            let mut power = Me3Power::default();
                            *power.power_class_name.borrow_mut() = power_class_name;
                            powers.borrow_mut().push(power.into());
                        }
                    }
                }

                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let Props { power_list, powers, helper } = &self.props;

        let selectables = power_list.iter().map(|&(power_class_name, power_name)| {
            let selected = match powers {
                BonusPowerType::Me2(powers) => powers.borrow()
                .iter()
                .any(|power| power.borrow().power_class_name().eq_ignore_ascii_case(power_class_name)),
                BonusPowerType::Me3(powers) => powers.borrow()
                .iter()
                .any(|power| power.borrow().power_class_name().eq_ignore_ascii_case(power_class_name)),
            };

            html_nested! {
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
                    onclick={self.link.callback(move |_| Msg::ToggleBonusPower(power_class_name.to_owned()))}
                >
                    {power_name}
                </button>
            }
        });

        html! {
            <Table title={String::from("Bonus Powers")} helper={*helper}>
                { for selectables }
            </Table>
        }
    }
}
