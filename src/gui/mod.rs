use yew::prelude::*;

use crate::{
    gui::{
        component::*,
        mass_effect_2::{Me2General, Me2Type},
    },
    save_data::mass_effect_2::Me2LeSaveGame,
    unreal,
};

pub mod component;
mod mass_effect_2;
mod raw_ui;

pub use raw_ui::*;

pub struct Gui {
    link: ComponentLink<Self>,
    save_game: RcUi<Me2LeSaveGame>,
}

impl Component for Gui {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let save_game = RcUi::new(
            unreal::Deserializer::from_bytes(include_bytes!("../../test/ME2LeSave.pcsav")).unwrap(),
        );
        Gui { link, save_game }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="h-screen flex flex-col font-mono text-base leading-tight text-white me2">
                <NavBar />
                <section class="flex flex-auto p-1">
                    <TabBar>
                        <Tab title="Général">
                            <Me2General save_game=Me2Type::Legendary(self.save_game.clone()) />
                        </Tab>
                        <Tab title="Cécé">
                            <Clipper list=(0..25000).collect::<Vec<_>>() />
                        </Tab>
                    </TabBar>
                </section>
            </div>
        }
    }
}
