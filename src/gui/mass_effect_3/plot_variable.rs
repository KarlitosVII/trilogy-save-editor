use indexmap::IndexMap;
use std::cell::RefMut;
use yew::{prelude::*, utils::NeqAssign};

use crate::{
    gui::{
        components::{CheckBox, Table},
        raw_ui::RawUi,
        RcUi,
    },
    save_data::{mass_effect_3::plot_db::PlotVariable as PlotVariableDb, shared::plot::BitVec},
};

pub enum Msg {
    ChangeBool(usize, bool),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: Option<String>,
    pub booleans: RcUi<BitVec>,
    pub variables: RcUi<IndexMap<String, RcUi<i32>>>,
    pub plot_variable: PlotVariableDb,
}

impl Props {
    fn booleans_mut(&mut self) -> RefMut<'_, BitVec> {
        self.booleans.borrow_mut()
    }
}

pub struct PlotVariable {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for PlotVariable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut this = PlotVariable { props, link };
        this.add_missing_plots();
        this
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeBool(idx, value) => {
                if let Some(mut plot) = self.props.booleans_mut().get_mut(idx) {
                    *plot = value;
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.neq_assign(props) {
            self.add_missing_plots();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let Props { title, booleans, variables, plot_variable, .. } = &self.props;
        let PlotVariableDb { booleans: bool_db, variables: var_db } = &plot_variable;

        let booleans = bool_db.iter().map(|(&idx, label)| match booleans.borrow().get(idx) {
            Some(value) => html! {
                <CheckBox
                    label={label.to_owned()}
                    value={RcUi::new(*value)}
                    onchange={self.link.callback(move |value| Msg::ChangeBool(idx, value))}
                />
            },
            None => Html::default(),
        });

        let variables = var_db.iter().map(|(db_key, label)| {
            let value = variables
                .borrow()
                .iter()
                .find_map(|(key, value)| unicase::eq(key, db_key).then(|| RcUi::clone(value)));
            match value {
                Some(value) => value.view(label),
                None => Html::default(),
            }
        });

        html! {
            <Table title={title.clone()} opened={title.is_none()}>
                { for booleans }
                { for variables }
            </Table>
        }
    }
}

impl PlotVariable {
    fn add_missing_plots(&mut self) {
        let Props { booleans, variables, plot_variable, .. } = &mut self.props;
        let PlotVariableDb { booleans: bool_db, variables: var_db } = &plot_variable;

        // Booleans
        if let Some(&max) = bool_db.keys().max() {
            let mut booleans = booleans.borrow_mut();
            if max >= booleans.len() {
                booleans.resize(max + 1, Default::default());
            };
        }

        // Variables
        for db_key in var_db.keys().cloned() {
            let contains_key = variables.borrow().iter().any(|(key, _)| unicase::eq(key, &db_key));
            if !contains_key {
                variables.borrow_mut().entry(db_key).or_default();
            }
        }
    }
}
