use std::rc::Rc;

use anyhow::Error;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::gui::{
    components::{Tab, TabBar},
    shared::{FloatPlotType, IntPlotType, PlotType, RawPlot},
    RcUi,
};
use crate::save_data::shared::plot::{BitVec, RawPlotDb};
use crate::services::database::{Database, DatabaseService, Request, Response, Type};

pub enum Msg {
    PlotDb(Rc<RawPlotDb>),
    Error(Error),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub booleans: RcUi<BitVec>,
    pub integers: IntPlotType,
    pub floats: FloatPlotType,
    pub onerror: Callback<Error>,
}

pub struct Me2RawPlot {
    _database_service: Box<dyn Bridge<DatabaseService>>,
    plot_db: Option<Rc<RawPlotDb>>,
}

impl Component for Me2RawPlot {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut database_service =
            DatabaseService::bridge(ctx.link().callback(|response| match response {
                Response::Database(Database::Me2RawPlot(db)) => Msg::PlotDb(db),
                Response::Error(err) => Msg::Error(err),
                _ => unreachable!(),
            }));

        database_service.send(Request::Database(Type::Me2RawPlot));

        Me2RawPlot { _database_service: database_service, plot_db: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PlotDb(db) => {
                self.plot_db = Some(db);
                true
            }
            Msg::Error(err) => {
                ctx.props().onerror.emit(err);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref plot_db) = self.plot_db {
            let (booleans, integers, floats) =
                (&ctx.props().booleans, &ctx.props().integers, &ctx.props().floats);

            html! {
                <TabBar>
                    <Tab title="Booleans">
                        <RawPlot plots={PlotType::Boolean(RcUi::clone(booleans))} plot_db={Rc::clone(plot_db)} />
                    </Tab>
                    <Tab title="Integers">
                        <RawPlot plots={PlotType::Int(integers.clone())} plot_db={Rc::clone(plot_db)} />
                    </Tab>
                    <Tab title="Floats">
                        <RawPlot plots={PlotType::Float(floats.clone())} plot_db={Rc::clone(plot_db)} />
                    </Tab>
                </TabBar>
            }
        } else {
            html! {
                <>
                    <p>{ "Loading database..." }</p>
                    <hr class="border-t border-default-border" />
                </>
            }
        }
    }
}
