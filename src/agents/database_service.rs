use anyhow::{Context as AnyhowContext, Error, Result};
use std::rc::Rc;
use yew::{
    format::Nothing,
    services::{
        fetch::{self, FetchTask},
        FetchService,
    },
    worker::{Agent, AgentLink, Context, HandlerId},
};

use crate::save_data::{
    // mass_effect_1::{item_db::Me1ItemDb, plot_db::Me1PlotDb},
    mass_effect_2::plot_db::Me2PlotDb,
    // mass_effect_3::plot_db::Me3PlotDb,
    shared::plot::RawPlotDb,
};

// lazy_static! {
// ME1
// static ref ME1_PLOT_DB: Result<Me1PlotDb> = {
//     let input = fs::read_to_string("databases/me1_plot_db.ron")?;
//     ron::from_str(&input).map_err(Error::from)
// }.context("Failed to parse `databases/me1_plot_db.ron`");

// static ref ME1_RAW_PLOT_DB: Result<RawPlotDb> = {
//     let input = fs::read_to_string("databases/me1_raw_plot_db.ron")?;
//     ron::from_str(&input).map_err(Error::from)
// }.context("Failed to parse `databases/me1_raw_plot_db.ron`");

// static ref ME1_ITEM_DB: Result<Me1ItemDb> = {
//     let input = fs::read_to_string("databases/me1_item_db.ron")?;
//     ron::from_str(&input).map_err(Error::from)
// }.context("Failed to parse `databases/me1_item_db.ron`");

// ME3
// static ref ME3_PLOT_DB: Result<Me3PlotDb> = {
//     let input = fs::read_to_string("databases/me3_plot_db.ron")?;
//     ron::from_str(&input).map_err(Error::from)
// }.context("Failed to parse `databases/me3_plot_db.ron`");

// static ref ME3_RAW_PLOT_DB: Result<RawPlotDb> = {
//     let input = fs::read_to_string("databases/me3_raw_plot_db.ron")?;
//     ron::from_str(&input).map_err(Error::from)
// }.context("Failed to parse `databases/me3_raw_plot_db.ron`");
// }

pub enum Type {
    Me2Plot,
    Me2RawPlot,
}

pub enum Database {
    Me2Plot(Rc<Me2PlotDb>),
    Me2RawPlot(Rc<RawPlotDb>),
}

pub enum Request {
    Database(Type),
}

pub enum Response {
    Database(Database),
    Error(Error),
}

pub enum Msg {
    DatabaseLoaded(HandlerId, Database),
    Error(HandlerId, Error),
}

#[derive(Default)]
struct Databases {
    me2_plot: Option<Rc<Me2PlotDb>>,
    me2_raw_plot: Option<Rc<RawPlotDb>>,
}

pub struct DatabaseService {
    link: AgentLink<Self>,
    tasks: Vec<FetchTask>,
    dbs: Databases,
}

impl Agent for DatabaseService {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link, tasks: Default::default(), dbs: Default::default() }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::DatabaseLoaded(who, Database::Me2Plot(db)) => {
                self.dbs.me2_plot = Some(Rc::clone(&db));
                self.respond_db(who, Database::Me2Plot(db));
            }
            Msg::DatabaseLoaded(who, Database::Me2RawPlot(db)) => {
                self.dbs.me2_raw_plot = Some(Rc::clone(&db));
                self.respond_db(who, Database::Me2RawPlot(db));
            }
            Msg::Error(who, err) => self.link.respond(who, Response::Error(err)),
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        let handle_request = || match msg {
            Request::Database(db_type) => {
                match db_type {
                    Type::Me2Plot => match self.dbs.me2_plot {
                        Some(ref db) => self.respond_db(who, Database::Me2Plot(Rc::clone(db))),
                        None => self.fetch(who, "/databases/me2_plot_db.ron", |response| {
                            let db = ron::from_str(&response)
                                .context("Failed to parse `databases/me2_plot_db.ron`")?;
                            Ok(Database::Me2Plot(Rc::new(db)))
                        })?,
                    },
                    Type::Me2RawPlot => match self.dbs.me2_raw_plot {
                        Some(ref db) => self.respond_db(who, Database::Me2RawPlot(Rc::clone(db))),
                        None => self.fetch(who, "/databases/me2_raw_plot_db.ron", |response| {
                            let db = ron::from_str(&response)
                                .context("Failed to parse `databases/me2_raw_plot_db.ron`")?;
                            Ok(Database::Me2RawPlot(Rc::new(db)))
                        })?,
                    },
                }
                Ok(())
            }
        };

        if let Err(err) = handle_request() {
            self.link.send_message(Msg::Error(who, err));
        }
    }
}

impl DatabaseService {
    fn respond_db(&self, who: HandlerId, db: Database) {
        self.link.respond(who, Response::Database(db))
    }

    fn fetch<F>(&mut self, who: HandlerId, path: &str, deserialize: F) -> Result<()>
    where
        F: Fn(String) -> Result<Database> + 'static,
    {
        let get_request = fetch::Request::get(path).body(Nothing)?;

        let task = FetchService::fetch(
            get_request,
            self.link.callback(move |response: fetch::Response<Result<String>>| {
                let handle_db = || deserialize(response.into_body()?);
                match handle_db() {
                    Ok(db) => Msg::DatabaseLoaded(who, db),
                    Err(err) => Msg::Error(who, err),
                }
            }),
        )?;
        self.tasks.push(task);
        Ok(())
    }
}
