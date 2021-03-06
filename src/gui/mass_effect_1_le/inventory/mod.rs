use std::{cell::Ref, rc::Rc};

use yew::{context::ContextHandle, prelude::*};

use crate::{
    gui::components::{Select, Table},
    save_data::{
        mass_effect_1_le::{
            item_db::{DbItem, Me1ItemDb},
            player::{Inventory, Item, ItemLevel, Player},
            squad::Henchman,
        },
        RcRef,
    },
    services::database::Databases,
};

mod item_select;
pub use self::item_select::*;

pub enum Msg {
    DatabaseLoaded(Databases),
    ChangeItem(RcRef<Item>, DbItem),
    ChangeItemLevel(RcRef<Item>, usize),
    RemoveItem(RcRef<Vec<RcRef<Item>>>, usize),
    AddItem(RcRef<Vec<RcRef<Item>>>),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub player: RcRef<Player>,
    pub squad: RcRef<Vec<RcRef<Henchman>>>,
}

impl Props {
    fn player(&self) -> Ref<'_, Player> {
        self.player.borrow()
    }

    fn squad(&self) -> Ref<'_, Vec<RcRef<Henchman>>> {
        self.squad.borrow()
    }
}

pub struct Me1LeInventory {
    _db_handle: ContextHandle<Databases>,
    item_db: Option<Rc<Me1ItemDb>>,
}

impl Component for Me1LeInventory {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (databases, _db_handle) = ctx
            .link()
            .context::<Databases>(ctx.link().callback(Msg::DatabaseLoaded))
            .expect("no database provider");

        Me1LeInventory { _db_handle, item_db: databases.get_me1_item_db() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DatabaseLoaded(dbs) => {
                self.item_db = dbs.get_me1_item_db();
                true
            }
            Msg::ChangeItem(item, new_item) => {
                let mut item = item.borrow_mut();
                item.set_item_id(new_item.item_id);
                item.set_manufacturer_id(new_item.manufacturer_id);
                false
            }
            Msg::ChangeItemLevel(item, item_level) => {
                let mut item = item.borrow_mut();
                *item.item_level_mut() = ItemLevel::from(item_level);
                false
            }
            Msg::RemoveItem(item_list, idx) => {
                item_list.borrow_mut().remove(idx);
                true
            }
            Msg::AddItem(item_list) => {
                item_list.borrow_mut().push(Default::default());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.item_db.is_some() {
            let player = ctx.props().player();
            html! {
                <div class="flex divide-solid divide-x divide-default-border">
                    <div class="flex-1 pr-1 min-w-0">
                        { self.player(ctx, player.inventory()) }
                        { self.squad(ctx, ctx.props().squad()) }
                    </div>
                    <div class="flex-1 flex flex-col gap-1 pl-1 min-w-0">
                        { self.inventory(ctx, player.inventory()) }
                    </div>
                </div>
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

impl Me1LeInventory {
    fn item_view(&self, ctx: &Context<Self>, item: &RcRef<Item>) -> Html {
        html! {
            <div class="flex items-center gap-1 min-w-0">
                {self.item_view_no_flex(ctx, item)}
            </div>
        }
    }

    fn item_view_no_flex(&self, ctx: &Context<Self>, item: &RcRef<Item>) -> Html {
        let current_item = DbItem {
            item_id: item.borrow().item_id(),
            manufacturer_id: item.borrow().manufacturer_id(),
        };
        let current_level = *item.borrow().item_level() as usize;
        let onselect_item = {
            let item = RcRef::clone(item);
            ctx.link().callback(move |new_item| Msg::ChangeItem(RcRef::clone(&item), new_item))
        };
        let onselect_level = {
            let item = RcRef::clone(item);
            ctx.link().callback(move |idx| Msg::ChangeItemLevel(RcRef::clone(&item), idx))
        };
        html! {
            <>
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
            </>
        }
    }

    fn player(&self, ctx: &Context<Self>, inventory: Ref<'_, Inventory>) -> Html {
        let equipment = inventory.equipment();
        let equipment = equipment.iter().map(|item| self.item_view(ctx, item));

        let quick_slots = inventory.quick_slots();
        let quick_slots = quick_slots.iter().map(|item| self.item_view(ctx, item));
        html! {
            <div class="flex flex-col gap-1">
                <div>
                    <p>{"Player"}</p>
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

    fn squad(&self, ctx: &Context<Self>, inventory: Ref<'_, Vec<RcRef<Henchman>>>) -> Html {
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
            let equipment = equipment.iter().map(|item| self.item_view(ctx, item));

            let quick_slots = henchman.quick_slots();
            let quick_slots = quick_slots.iter().map(|item| self.item_view(ctx, item));
            html! {
                <div class="flex flex-col gap-1 mt-1">
                    <div>
                        <p>{ name }</p>
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

    fn inventory(&self, ctx: &Context<Self>, player_inventory: Ref<'_, Inventory>) -> Html {
        let link = ctx.link();
        let inventory_add = {
            let inventory = RcRef::clone(&player_inventory.inventory);
            link.callback(move |_| Msg::AddItem(RcRef::clone(&inventory)))
        };
        let buy_pack_add = {
            let buy_pack = RcRef::clone(&player_inventory.buy_pack);
            link.callback(move |_| Msg::AddItem(RcRef::clone(&buy_pack)))
        };

        let item_remove_view = |item_list, idx, item| {
            let item = self.item_view_no_flex(ctx, item);

            html! {
                <div class="flex items-center gap-1 min-w-0">
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
                            onclick={link.callback(move |_| Msg::RemoveItem(RcRef::clone(&item_list), idx))}
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
            item_remove_view(RcRef::clone(&player_inventory.buy_pack), idx, item)
        });

        let inventory2 = player_inventory.inventory();
        let inventory = inventory2.iter().enumerate().map(|(idx, item)| {
            item_remove_view(RcRef::clone(&player_inventory.inventory), idx, item)
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
