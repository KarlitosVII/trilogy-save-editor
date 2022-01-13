use std::cell::RefMut;

use indexmap::IndexMap;
use yew::prelude::*;

use crate::{
    gui::{
        components::{CheckBox, Table},
        raw_ui::RawUi,
    },
    save_data::{
        mass_effect_3::plot_db::PlotVariable as PlotVariableDb, shared::plot::BitVec, RcCell, RcRef,
    },
};

pub enum Msg {
    ChangeBool(usize, bool),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: Option<String>,
    pub booleans: RcRef<BitVec>,
    pub variables: RcRef<IndexMap<String, RcCell<i32>>>,
    pub plot_variable: PlotVariableDb,
}

impl Props {
    fn booleans_mut(&self) -> RefMut<'_, BitVec> {
        self.booleans.borrow_mut()
    }
}

pub struct PlotVariable {}

impl Component for PlotVariable {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut this = PlotVariable {};
        this.add_missing_plots(ctx);
        this
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeBool(idx, value) => {
                if let Some(mut plot) = ctx.props().booleans_mut().get_mut(idx) {
                    *plot = value;
                }
                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.add_missing_plots(ctx);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { title, booleans, variables, plot_variable, .. } = &ctx.props();
        let PlotVariableDb { booleans: bool_db, variables: var_db } = &plot_variable;

        let booleans = bool_db.iter().map(|(&idx, label)| match booleans.borrow().get(idx) {
            Some(value) => html! {
                <CheckBox
                    label={label.clone()}
                    value={RcCell::new(*value)}
                    onchange={ctx.link().callback(move |value| Msg::ChangeBool(idx, value))}
                />
            },
            None => Html::default(),
        });

        let variables = var_db.iter().map(|(db_key, label)| {
            let value = variables.borrow().iter().find_map(|(key, value)| {
                db_key.eq_ignore_ascii_case(key).then(|| RcCell::clone(value))
            });
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
    fn add_missing_plots(&mut self, ctx: &Context<Self>) {
        let Props { booleans, variables, plot_variable, .. } = &mut ctx.props();
        let PlotVariableDb { booleans: bool_db, variables: var_db } = &plot_variable;

        // Booleans
        if let Some(&max) = bool_db.keys().max() {
            let mut booleans = booleans.borrow_mut();
            if max >= booleans.len() {
                booleans.resize(max + 1, false);
            };
        }

        // Variables
        for db_key in var_db.keys().cloned() {
            let contains_key =
                variables.borrow().iter().any(|(key, _)| db_key.eq_ignore_ascii_case(key));
            if !contains_key {
                variables.borrow_mut().entry(db_key).or_default();
            }
        }
    }
}
