use std::rc::Rc;

use anyhow::{Context as ErrorContext, Error, Result};
use yew::{prelude::*, ContextProvider};

use crate::{
    save_data::{
        mass_effect_1::plot_db::Me1PlotDb, mass_effect_1_le::item_db::Me1ItemDb,
        mass_effect_1_le::player_class_db::Me1LePlayerClassDb, mass_effect_2::plot_db::Me2PlotDb,
        mass_effect_3::plot_db::Me3PlotDb, shared::plot::RawPlotDb,
    },
    services::rpc,
};

pub enum Type {
    Me1LePlayerClasses,
    Me1Plot,
    Me1RawPlot,
    Me1Items,
    Me2Plot,
    Me2RawPlot,
    Me3Plot,
    Me3RawPlot,
}

pub enum Database {
    Me1LePlayerClasses(Me1LePlayerClassDb),
    Me1Plot(Me1PlotDb),
    Me1RawPlot(RawPlotDb),
    Me1Items(Me1ItemDb),
    Me2Plot(Me2PlotDb),
    Me2RawPlot(RawPlotDb),
    Me3Plot(Me3PlotDb),
    Me3RawPlot(RawPlotDb),
}

pub enum Msg {
    LoadDatabase(Type),
    DatabaseLoaded(Box<Database>),
    Error(Error),
}

#[derive(Clone, Default)]
pub struct Databases {
    me1_le_player_classes: Option<Rc<Me1LePlayerClassDb>>,
    me1_plot: Option<Rc<Me1PlotDb>>,
    me1_raw_plot: Option<Rc<RawPlotDb>>,
    me1_item_db: Option<Rc<Me1ItemDb>>,
    me2_plot: Option<Rc<Me2PlotDb>>,
    me2_raw_plot: Option<Rc<RawPlotDb>>,
    me3_plot: Option<Rc<Me3PlotDb>>,
    me3_raw_plot: Option<Rc<RawPlotDb>>,
    load_callback: Callback<Type>,
}

impl Databases {
    pub fn get_me1_le_player_classes(self) -> Option<Rc<Me1LePlayerClassDb>> {
        if self.me1_le_player_classes.is_none() {
            self.load_database(Type::Me1LePlayerClasses);
        }
        self.me1_le_player_classes
    }

    pub fn get_me1_plot(self) -> Option<Rc<Me1PlotDb>> {
        if self.me1_plot.is_none() {
            self.load_database(Type::Me1Plot);
        }
        self.me1_plot
    }

    pub fn get_me1_raw_plot(self) -> Option<Rc<RawPlotDb>> {
        if self.me1_raw_plot.is_none() {
            self.load_database(Type::Me1RawPlot);
        }
        self.me1_raw_plot
    }

    pub fn get_me1_item_db(self) -> Option<Rc<Me1ItemDb>> {
        if self.me1_item_db.is_none() {
            self.load_database(Type::Me1Items);
        }
        self.me1_item_db
    }

    pub fn get_me2_plot(self) -> Option<Rc<Me2PlotDb>> {
        if self.me2_plot.is_none() {
            self.load_database(Type::Me2Plot);
        }
        self.me2_plot
    }

    pub fn get_me2_raw_plot(self) -> Option<Rc<RawPlotDb>> {
        if self.me2_raw_plot.is_none() {
            self.load_database(Type::Me2RawPlot);
        }
        self.me2_raw_plot
    }

    pub fn get_me3_plot(self) -> Option<Rc<Me3PlotDb>> {
        if self.me3_plot.is_none() {
            self.load_database(Type::Me3Plot);
        }
        self.me3_plot
    }

    pub fn get_me3_raw_plot(self) -> Option<Rc<RawPlotDb>> {
        if self.me3_raw_plot.is_none() {
            self.load_database(Type::Me3RawPlot);
        }
        self.me3_raw_plot
    }

    fn load_database(&self, db_type: Type) {
        self.load_callback.emit(db_type);
    }
}

impl PartialEq for Databases {
    fn eq(&self, other: &Self) -> bool {
        let Databases {
            me1_le_player_classes,
            me1_plot,
            me1_raw_plot,
            me1_item_db,
            me2_plot,
            me2_raw_plot,
            me3_plot,
            me3_raw_plot,
            load_callback: _,
        } = self;
        me1_le_player_classes.is_some() == other.me1_le_player_classes.is_some()
            && me1_plot.is_some() == other.me1_plot.is_some()
            && me1_raw_plot.is_some() == other.me1_raw_plot.is_some()
            && me1_item_db.is_some() == other.me1_item_db.is_some()
            && me2_plot.is_some() == other.me2_plot.is_some()
            && me2_raw_plot.is_some() == other.me2_raw_plot.is_some()
            && me3_plot.is_some() == other.me3_plot.is_some()
            && me3_raw_plot.is_some() == other.me3_raw_plot.is_some()
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub onerror: Callback<Error>,
}

#[derive(Clone)]
pub struct DatabaseProvider {
    dbs: Databases,
}

impl Component for DatabaseProvider {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let load_callback = ctx.link().callback(Msg::LoadDatabase);
        let dbs = Databases { load_callback, ..Default::default() };
        Self { dbs }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadDatabase(db_type) => {
                match db_type {
                    Type::Me1LePlayerClasses => {
                        Self::load_db(ctx, "databases/me1_le_player_class_db.ron", |response| {
                            let db = ron::from_str(&response)?;
                            Ok(Database::Me1LePlayerClasses(db))
                        })
                    }
                    Type::Me1Plot => Self::load_db(ctx, "databases/me1_plot_db.ron", |response| {
                        let db = ron::from_str(&response)?;
                        Ok(Database::Me1Plot(db))
                    }),
                    Type::Me1RawPlot => {
                        Self::load_db(ctx, "databases/me1_raw_plot_db.ron", |response| {
                            let db = ron::from_str(&response)?;
                            Ok(Database::Me1RawPlot(db))
                        })
                    }
                    Type::Me1Items => Self::load_db(ctx, "databases/me1_item_db.ron", |response| {
                        let db = ron::from_str(&response)?;
                        Ok(Database::Me1Items(db))
                    }),
                    Type::Me2Plot => Self::load_db(ctx, "databases/me2_plot_db.ron", |response| {
                        let db = ron::from_str(&response)?;
                        Ok(Database::Me2Plot(db))
                    }),
                    Type::Me2RawPlot => {
                        Self::load_db(ctx, "databases/me2_raw_plot_db.ron", |response| {
                            let db = ron::from_str(&response)?;
                            Ok(Database::Me2RawPlot(db))
                        })
                    }
                    Type::Me3Plot => Self::load_db(ctx, "databases/me3_plot_db.ron", |response| {
                        let db = ron::from_str(&response)?;
                        Ok(Database::Me3Plot(db))
                    }),
                    Type::Me3RawPlot => {
                        Self::load_db(ctx, "databases/me3_raw_plot_db.ron", |response| {
                            let db = ron::from_str(&response)?;
                            Ok(Database::Me3RawPlot(db))
                        })
                    }
                }
                false
            }
            Msg::DatabaseLoaded(db) => {
                match *db {
                    Database::Me1LePlayerClasses(db) => {
                        self.dbs.me1_le_player_classes = Some(db.into());
                    }
                    Database::Me1Plot(db) => {
                        self.dbs.me1_plot = Some(db.into());
                    }
                    Database::Me1RawPlot(db) => {
                        self.dbs.me1_raw_plot = Some(db.into());
                    }
                    Database::Me1Items(db) => {
                        self.dbs.me1_item_db = Some(db.into());
                    }
                    Database::Me2Plot(db) => {
                        self.dbs.me2_plot = Some(db.into());
                    }
                    Database::Me2RawPlot(db) => {
                        self.dbs.me2_raw_plot = Some(db.into());
                    }
                    Database::Me3Plot(db) => {
                        self.dbs.me3_plot = Some(db.into());
                    }
                    Database::Me3RawPlot(db) => {
                        self.dbs.me3_raw_plot = Some(db.into());
                    }
                }
                true
            }
            Msg::Error(err) => {
                ctx.props().onerror.emit(err);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ContextProvider<Databases> context={self.dbs.clone()}>
                { ctx.props().children.clone() }
            </ContextProvider<Databases>>
        }
    }
}

impl DatabaseProvider {
    fn load_db<F>(ctx: &Context<Self>, path: &'static str, deserialize: F)
    where
        F: Fn(String) -> Result<Database> + 'static,
    {
        ctx.link().send_future(async move {
            let handle_db = async {
                let rpc_file = rpc::load_database(path).await?;
                let file = String::from_utf8(rpc_file.file.decode()?)?;
                deserialize(file)
            };
            match handle_db.await.context(format!("Failed to parse `/{}`", path)) {
                Ok(db) => Msg::DatabaseLoaded(Box::new(db)),
                Err(err) => Msg::Error(err),
            }
        });
    }
}
