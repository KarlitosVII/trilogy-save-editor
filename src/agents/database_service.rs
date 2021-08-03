use anyhow::{Context as AnyhowContext, Error, Result};
use std::{collections::HashMap, rc::Rc};
use yew::{
    format::Nothing,
    services::{
        fetch::{self, FetchTask},
        FetchService,
    },
    worker::{Agent, AgentLink, Context, HandlerId},
};

use crate::save_data::{
    mass_effect_1::plot_db::Me1PlotDb, mass_effect_2::plot_db::Me2PlotDb, shared::plot::RawPlotDb,
};

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

pub enum Type {
    Me1Plot,
    Me2Plot,
    Me2RawPlot,
}

pub enum Database {
    Me1Plot(Rc<Me1PlotDb>),
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
    DatabaseLoaded(usize, HandlerId, Database),
    Error(Option<usize>, HandlerId, Error),
}

#[derive(Default)]
struct Databases {
    me1_plot: Option<Rc<Me1PlotDb>>,
    me2_plot: Option<Rc<Me2PlotDb>>,
    me2_raw_plot: Option<Rc<RawPlotDb>>,
}

pub struct DatabaseService {
    link: AgentLink<Self>,
    task_counter: usize,
    tasks: HashMap<usize, FetchTask>,
    dbs: Databases,
}

impl Agent for DatabaseService {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link, task_counter: 0, tasks: Default::default(), dbs: Default::default() }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::DatabaseLoaded(task_id, who, db) => {
                self.tasks.remove(&task_id);
                match db {
                    Database::Me1Plot(ref db) => {
                        self.dbs.me1_plot = Some(Rc::clone(db));
                    }
                    Database::Me2Plot(ref db) => {
                        self.dbs.me2_plot = Some(Rc::clone(db));
                    }
                    Database::Me2RawPlot(ref db) => {
                        self.dbs.me2_raw_plot = Some(Rc::clone(db));
                    }
                }
                self.respond_db(who, db);
            }
            Msg::Error(task_id, who, err) => {
                if let Some(task_id) = task_id {
                    self.tasks.remove(&task_id);
                }
                self.link.respond(who, Response::Error(err));
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        let handle_request = || match msg {
            Request::Database(db_type) => {
                match db_type {
                    Type::Me1Plot => match self.dbs.me1_plot {
                        Some(ref db) => self.respond_db(who, Database::Me1Plot(Rc::clone(db))),
                        None => self.fetch(who, "/databases/me1_plot_db.ron", |response| {
                            let db = ron::from_str(&response)?;
                            Ok(Database::Me1Plot(Rc::new(db)))
                        })?,
                    },
                    Type::Me2Plot => match self.dbs.me2_plot {
                        Some(ref db) => self.respond_db(who, Database::Me2Plot(Rc::clone(db))),
                        None => self.fetch(who, "/databases/me2_plot_db.ron", |response| {
                            let db = ron::from_str(&response)?;
                            Ok(Database::Me2Plot(Rc::new(db)))
                        })?,
                    },
                    Type::Me2RawPlot => match self.dbs.me2_raw_plot {
                        Some(ref db) => self.respond_db(who, Database::Me2RawPlot(Rc::clone(db))),
                        None => self.fetch(who, "/databases/me2_raw_plot_db.ron", |response| {
                            let db = ron::from_str(&response)?;
                            Ok(Database::Me2RawPlot(Rc::new(db)))
                        })?,
                    },
                }
                Ok::<_, Error>(())
            }
        };

        if let Err(err) = handle_request().context("Error while loading database") {
            self.link.send_message(Msg::Error(None, who, err));
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

        let path = path.to_owned();
        let task_id = self.task_counter;
        let callback = self.link.callback(move |response: fetch::Response<Result<String>>| {
            let handle_db = || deserialize(response.into_body()?);
            match handle_db().context(format!("Failed to parse `{}`", path)) {
                Ok(db) => Msg::DatabaseLoaded(task_id, who, db),
                Err(err) => Msg::Error(Some(task_id), who, err),
            }
        });

        let task = FetchService::fetch(get_request, callback)?;
        self.tasks.insert(task_id, task);
        self.task_counter += 1;
        Ok(())
    }
}
