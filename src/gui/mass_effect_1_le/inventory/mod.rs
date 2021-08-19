use std::{cell::Ref, rc::Rc};

use anyhow::Error;
use yew::{prelude::*, utils::NeqAssign};
use yew_agent::{Bridge, Bridged};

use crate::gui::{
    components::{Select, Table},
    RcUi,
};
use crate::save_data::mass_effect_1_le::{
    item_db::{DbItem, Me1ItemDb},
    player::{Inventory, Item, ItemLevel, Player},
    squad::Henchman,
};
use crate::services::database::{Database, DatabaseService, Request, Response, Type};

mod item_select;
pub use self::item_select::*;

pub enum Msg {
    ItemDb(Rc<Me1ItemDb>),
    Error(Error),
    ChangeItem(RcUi<Item>, DbItem),
    ChangeItemLevel(RcUi<Item>, usize),
    RemoveItem(RcUi<Vec<RcUi<Item>>>, usize),
    AddItem(RcUi<Vec<RcUi<Item>>>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub player: RcUi<Player>,
    pub squad: RcUi<Vec<RcUi<Henchman>>>,
    pub onerror: Callback<Error>,
}

impl Props {
    fn player(&self) -> Ref<'_, Player> {
        self.player.borrow()
    }

    fn squad(&self) -> Ref<'_, Vec<RcUi<Henchman>>> {
        self.squad.borrow()
    }
}

pub struct Me1LeInventory {
    props: Props,
    link: ComponentLink<Self>,
    _database_service: Box<dyn Bridge<DatabaseService>>,
    item_db: Option<Rc<Me1ItemDb>>,
}

impl Component for Me1LeInventory {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut database_service =
            DatabaseService::bridge(link.callback(|response| match response {
                Response::Database(Database::Me1ItemDb(db)) => Msg::ItemDb(db),
                Response::Error(err) => Msg::Error(err),
                _ => unreachable!(),
            }));

        database_service.send(Request::Database(Type::Me1ItemDb));

        Me1LeInventory { props, link, _database_service: database_service, item_db: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ItemDb(db) => {
                self.item_db = Some(db);
                true
            }
            Msg::Error(err) => {
                self.props.onerror.emit(err);
                false
            }
            Msg::ChangeItem(mut item, new_item) => {
                let mut item = item.borrow_mut();
                *item.item_id_mut() = new_item.item_id;
                *item.manufacturer_id_mut() = new_item.manufacturer_id;
                false
            }
            Msg::ChangeItemLevel(mut item, item_level) => {
                let mut item = item.borrow_mut();
                *item.item_level_mut() = ItemLevel::from(item_level);
                false
            }
            Msg::RemoveItem(mut item_list, idx) => {
                item_list.borrow_mut().remove(idx);
                true
            }
            Msg::AddItem(mut item_list) => {
                item_list.borrow_mut().push(Default::default());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        if self.item_db.is_some() {
            let player = self.props.player();
            html! {
                <div class="flex divide-solid divide-x divide-default-border">
                    <div class="flex-1 flex flex-col gap-1 pr-1 min-w-0">
                        { self.player(player.inventory()) }
                        { self.squad(self.props.squad()) }
                    </div>
                    <div class="flex-1 flex flex-col gap-1 pl-1 min-w-0">
                        { self.inventory(player.inventory()) }
                    </div>
                </div>
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

impl Me1LeInventory {
    fn item_view(&self, item: RcUi<Item>) -> Html {
        let current_item = DbItem {
            item_id: *item.borrow().item_id(),
            manufacturer_id: *item.borrow().manufacturer_id(),
        };
        let current_level = *item.borrow().item_level() as usize;
        let onselect_item = {
            let item = RcUi::clone(&item);
            self.link.callback(move |new_item| Msg::ChangeItem(RcUi::clone(&item), new_item))
        };
        let onselect_level =
            self.link.callback(move |idx| Msg::ChangeItemLevel(RcUi::clone(&item), idx));
        html! {
            <div class="flex-auto flex items-center gap-1 min-w-0">
                <ItemSelect
                    item_db={Rc::clone(self.item_db.as_ref().unwrap())}
                    {current_item}
                    onselect={onselect_item}
                />
                <Select
                    options={ItemLevel::variants()}
                    current_idx={current_level}
                    onselect={onselect_level}
                    sized=false
                />
            </div>
        }
    }

    fn player(&self, inventory: Ref<'_, Inventory>) -> Html {
        let equipment = inventory.equipment();
        let equipment = equipment.iter().map(|item| self.item_view(RcUi::clone(item)));

        let quick_slots = inventory.quick_slots();
        let quick_slots = quick_slots.iter().map(|item| self.item_view(RcUi::clone(item)));
        html! {
            <div class="flex flex-col gap-1">
                <div>
                    {"Player"}
                    <hr class="border-t border-default-border" />
                </div>
                <Table title="Equipement">
                    { for equipment }
                </Table>
                <Table title="Quick slots">
                    { for quick_slots }
                </Table>
            </div>
        }
    }

    fn squad(&self, inventory: Ref<'_, Vec<RcUi<Henchman>>>) -> Html {
        let squad = inventory.iter().map(|henchman| {
            let henchman = henchman.borrow();

            let name = match henchman.tag().as_str() {
                "hench_asari" => "Liara",
                "hench_humanfemale" => "Ashley",
                "hench_humanmale" => "Kaidan",
                "hench_krogan" => "Wrex",
                "hench_quarian" => "Tali",
                "hench_turian" => "Garrus",
                _ => "Jenkins",
            };

            let equipment = henchman.equipment();
            let equipment = equipment.iter().map(|item| self.item_view(RcUi::clone(item)));

            let quick_slots = henchman.quick_slots();
            let quick_slots = quick_slots.iter().map(|item| self.item_view(RcUi::clone(item)));
            html! {
                <div class="flex flex-col gap-1">
                    <div>
                        { name }
                        <hr class="border-t border-default-border" />
                    </div>
                    <Table title="Equipement">
                        { for equipment }
                    </Table>
                    <Table title="Quick slots">
                        { for quick_slots }
                    </Table>
                </div>
            }
        });

        html! { for squad }
    }

    fn inventory(&self, player_inventory: Ref<'_, Inventory>) -> Html {
        let inventory_add = {
            let inventory = RcUi::clone(&player_inventory.inventory);
            self.link.callback(move |_| Msg::AddItem(RcUi::clone(&inventory)))
        };
        let buy_pack_add = {
            let buy_pack = RcUi::clone(&player_inventory.buy_pack);
            self.link.callback(move |_| Msg::AddItem(RcUi::clone(&buy_pack)))
        };

        let item_remove_view = |item_list, idx, item| {
            let item = self.item_view(RcUi::clone(item));

            html! {
                <div class="flex gap-1">
                    <div class="py-px">
                        <a class={classes![
                                "rounded-none",
                                "select-none",
                                "hover:bg-theme-hover",
                                "active:bg-theme-active",
                                "bg-theme-bg",
                                "px-1",
                                "py-0",
                                "cursor-pointer",
                            ]}
                            onclick={self.link.callback(move |_| Msg::RemoveItem(RcUi::clone(&item_list), idx))}
                        >
                            {"remove"}
                        </a>
                    </div>
                    { item }
                </div>
            }
        };

        let buy_pack = player_inventory.buy_pack();
        let buy_pack = buy_pack.iter().enumerate().map(|(idx, item)| {
            item_remove_view(RcUi::clone(&player_inventory.buy_pack), idx, item)
        });

        let inventory2 = player_inventory.inventory();
        let inventory = inventory2.iter().enumerate().map(|(idx, item)| {
            item_remove_view(RcUi::clone(&player_inventory.inventory), idx, item)
        });
        html! {
            <>
                <Table title="Inventory">
                    { for inventory }
                    <button class="rounded-none hover:bg-theme-hover active:bg-theme-active bg-theme-bg px-1"
                        onclick={inventory_add}
                    >
                        {"add"}
                    </button>
                </Table>
                <Table title="Buy pack items">
                    { for buy_pack }
                    <button class="rounded-none hover:bg-theme-hover active:bg-theme-active bg-theme-bg px-1"
                        onclick={buy_pack_add}
                    >
                        {"add"}
                    </button>
                </Table>
            </>
        }
    }
}
