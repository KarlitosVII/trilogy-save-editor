use anyhow::Error;
use std::rc::Rc;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{
    database_service::{Database, DatabaseService, Request, Response, Type},
    gui::{
        components::{
            shared::{FloatPlotType, IntPlotType, PlotType, RawPlot},
            Tab, TabBar,
        },
        RcUi,
    },
    save_data::shared::plot::{BitVec, RawPlotDb},
};

pub enum Msg {
    PlotDb(Rc<RawPlotDb>),
    Error(Error),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub booleans: RcUi<BitVec>,
    pub integers: IntPlotType,
    pub floats: FloatPlotType,
    pub onerror: Callback<Error>,
}

pub struct Me2RawPlot {
    props: Props,
    _link: ComponentLink<Self>,
    _database_service: Box<dyn Bridge<DatabaseService>>,
    plot_db: Option<Rc<RawPlotDb>>,
}

impl Component for Me2RawPlot {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut database_service =
            DatabaseService::bridge(link.callback(|response| match response {
                Response::Database(Database::Me2RawPlot(db)) => Msg::PlotDb(db),
                Response::Error(err) => Msg::Error(err),
                _ => unreachable!(),
            }));

        database_service.send(Request::Database(Type::Me2RawPlot));

        Me2RawPlot { props, _link: link, _database_service: database_service, plot_db: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PlotDb(db) => {
                self.plot_db = Some(db);
                true
            }
            Msg::Error(err) => {
                self.props.onerror.emit(err);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        if let Some(ref plot_db) = self.plot_db {
            let (booleans, integers, floats) =
                (&self.props.booleans, &self.props.integers, &self.props.floats);

            html! {
                <TabBar>
                    <Tab title="Booleans">
                        <RawPlot plots=PlotType::Boolean(RcUi::clone(booleans)) plot_db=Rc::clone(plot_db) />
                    </Tab>
                    <Tab title="Integers">
                        <RawPlot plots=PlotType::Integer(integers.clone()) plot_db=Rc::clone(plot_db) />
                    </Tab>
                    <Tab title="Floats">
                        <RawPlot plots=PlotType::Float(floats.clone()) plot_db=Rc::clone(plot_db) />
                    </Tab>
                </TabBar>
            }
        } else {
            html! {
                <>
                    { "Loading database..." }
                    <hr class="border-t border-default-border" />
                </>
            }
        }
    }
}
