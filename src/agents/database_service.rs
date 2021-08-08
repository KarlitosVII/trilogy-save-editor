use anyhow::{Context as AnyhowContext, Error, Result};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::save_data::{
    mass_effect_1::plot_db::Me1PlotDb, mass_effect_2::plot_db::Me2PlotDb,
    mass_effect_3::plot_db::Me3PlotDb, shared::plot::RawPlotDb,
};

// static ref ME1_ITEM_DB: Result<Me1ItemDb> = {
//     let input = fs::read_to_string("databases/me1_item_db.ron")?;
//     ron::from_str(&input).map_err(Error::from)
// }.context("Failed to parse `databases/me1_item_db.ron`");

pub enum Type {
    Me1Plot,
    Me1RawPlot,
    Me2Plot,
    Me2RawPlot,
    Me3Plot,
    Me3RawPlot,
}

pub enum Database {
    Me1Plot(Rc<Me1PlotDb>),
    Me1RawPlot(Rc<RawPlotDb>),
    Me2Plot(Rc<Me2PlotDb>),
    Me2RawPlot(Rc<RawPlotDb>),
    Me3Plot(Rc<Me3PlotDb>),
    Me3RawPlot(Rc<RawPlotDb>),
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
    me1_plot: Option<Rc<Me1PlotDb>>,
    me1_raw_plot: Option<Rc<RawPlotDb>>,
    me2_plot: Option<Rc<Me2PlotDb>>,
    me2_raw_plot: Option<Rc<RawPlotDb>>,
    me3_plot: Option<Rc<Me3PlotDb>>,
    me3_raw_plot: Option<Rc<RawPlotDb>>,
}

pub struct DatabaseService {
    link: AgentLink<Self>,
    dbs: Databases,
}

impl Agent for DatabaseService {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link, dbs: Default::default() }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::DatabaseLoaded(who, db) => {
                match db {
                    Database::Me1Plot(ref db) => {
                        self.dbs.me1_plot = Some(Rc::clone(db));
                    }
                    Database::Me1RawPlot(ref db) => {
                        self.dbs.me1_raw_plot = Some(Rc::clone(db));
                    }
                    Database::Me2Plot(ref db) => {
                        self.dbs.me2_plot = Some(Rc::clone(db));
                    }
                    Database::Me2RawPlot(ref db) => {
                        self.dbs.me2_raw_plot = Some(Rc::clone(db));
                    }
                    Database::Me3Plot(ref db) => {
                        self.dbs.me3_plot = Some(Rc::clone(db));
                    }
                    Database::Me3RawPlot(ref db) => {
                        self.dbs.me3_raw_plot = Some(Rc::clone(db));
                    }
                }
                self.respond_db(who, db);
            }
            Msg::Error(who, err) => {
                self.link.respond(who, Response::Error(err));
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::Database(db_type) => match db_type {
                Type::Me1Plot => match self.dbs.me1_plot {
                    Some(ref db) => self.respond_db(who, Database::Me1Plot(Rc::clone(db))),
                    None => self.fetch(who, "/databases/me1_plot_db.ron", |response| {
                        let db = ron::from_str(&response)?;
                        Ok(Database::Me1Plot(Rc::new(db)))
                    }),
                },
                Type::Me1RawPlot => match self.dbs.me1_raw_plot {
                    Some(ref db) => self.respond_db(who, Database::Me1RawPlot(Rc::clone(db))),
                    None => self.fetch(who, "/databases/me1_raw_plot_db.ron", |response| {
                        let db = ron::from_str(&response)?;
                        Ok(Database::Me1RawPlot(Rc::new(db)))
                    }),
                },
                Type::Me2Plot => match self.dbs.me2_plot {
                    Some(ref db) => self.respond_db(who, Database::Me2Plot(Rc::clone(db))),
                    None => self.fetch(who, "/databases/me2_plot_db.ron", |response| {
                        let db = ron::from_str(&response)?;
                        Ok(Database::Me2Plot(Rc::new(db)))
                    }),
                },
                Type::Me2RawPlot => match self.dbs.me2_raw_plot {
                    Some(ref db) => self.respond_db(who, Database::Me2RawPlot(Rc::clone(db))),
                    None => self.fetch(who, "/databases/me2_raw_plot_db.ron", |response| {
                        let db = ron::from_str(&response)?;
                        Ok(Database::Me2RawPlot(Rc::new(db)))
                    }),
                },
                Type::Me3Plot => match self.dbs.me3_plot {
                    Some(ref db) => self.respond_db(who, Database::Me3Plot(Rc::clone(db))),
                    None => self.fetch(who, "/databases/me3_plot_db.ron", |response| {
                        let db = ron::from_str(&response)?;
                        Ok(Database::Me3Plot(Rc::new(db)))
                    }),
                },
                Type::Me3RawPlot => match self.dbs.me3_raw_plot {
                    Some(ref db) => self.respond_db(who, Database::Me3RawPlot(Rc::clone(db))),
                    None => self.fetch(who, "/databases/me3_raw_plot_db.ron", |response| {
                        let db = ron::from_str(&response)?;
                        Ok(Database::Me3RawPlot(Rc::new(db)))
                    }),
                },
            },
        }
    }
}

impl DatabaseService {
    fn respond_db(&self, who: HandlerId, db: Database) {
        self.link.respond(who, Response::Database(db))
    }

    fn fetch<F>(&mut self, who: HandlerId, path: &'static str, deserialize: F)
    where
        F: Fn(String) -> Result<Database> + 'static,
    {
        let link = self.link.clone();
        spawn_local(async move {
            let handle_db = async {
                let response =
                    reqwest::get(format!("http://127.0.0.1:8080{}", path)).await?.text().await?;
                deserialize(response)
            };
            let msg = match handle_db.await.context(format!("Failed to parse `{}`", path)) {
                Ok(db) => Msg::DatabaseLoaded(who, db),
                Err(err) => Msg::Error(who, err),
            };
            link.send_message(msg);
        });
    }
}
