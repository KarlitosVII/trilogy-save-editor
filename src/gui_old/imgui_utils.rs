use imgui::{
    im_str, sys, ChildWindow, Condition, ImStr, ImString, TabItem, TreeNodeFlags, TreeNodeToken, Ui,
};
use std::marker::PhantomData;

use super::Gui;

impl<'ui> Gui<'ui> {
    pub fn set_next_item_open(&self, is_open: bool) {
        unsafe {
            sys::igSetNextItemOpen(is_open, Condition::FirstUseEver as i32);
        }
    }
}

// Table
pub struct Table<'a> {
    ident: &'a ImStr,
    column: i32,
}

impl<'a> Table<'a> {
    pub fn new(ident: &ImStr, column: i32) -> Table {
        Table { ident, column }
    }

    fn begin_with_flags<'ui>(self, flags: u32, ui: &Ui<'ui>) -> Option<TableToken<'ui>> {
        unsafe {
            sys::igBeginTable(
                self.ident.as_ptr(),
                self.column,
                flags as i32,
                [0.0, 0.0].into(),
                0.0,
            )
        }
        .then(|| TableToken::new(ui))
    }

    pub fn begin<'ui>(self, ui: &Ui<'ui>) -> Option<TableToken<'ui>> {
        const FLAGS: u32 = sys::ImGuiTableFlags_RowBg
            | sys::ImGuiTableFlags_BordersOuterH
            | sys::ImGuiTableFlags_BordersOuterV;

        self.begin_with_flags(FLAGS, ui)
    }

    pub fn build<T, F>(self, ui: &Ui<'_>, f: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        self.begin(ui).map(|_token| f())
    }

    pub fn begin_columns<'ui>(column: i32, ui: &Ui<'ui>) -> Option<TableToken<'ui>> {
        const FLAGS: u32 = sys::ImGuiTableFlags_BordersInnerV;

        Table { ident: im_str!("columns"), column }.begin_with_flags(FLAGS, ui)
    }

    pub fn next_row() {
        unsafe {
            sys::igTableNextRow(sys::ImGuiTableRowFlags_None as i32, 19.0);
            sys::igTableNextColumn();
        }
    }

    pub fn next_column() -> bool {
        unsafe { sys::igTableNextColumn() }
    }
}

#[must_use]
pub struct TableToken<'ui>(PhantomData<Ui<'ui>>);
impl<'ui> TableToken<'ui> {
    pub fn new(_: &Ui<'ui>) -> Self {
        Self(PhantomData)
    }
}

impl Drop for TableToken<'_> {
    fn drop(&mut self) {
        unsafe { sys::igEndTable() }
    }
}

// TreeNode
pub struct TreeNode<'a> {
    ident: &'a str,
    label: Option<&'a str>,
}

impl<'a> TreeNode<'a> {
    pub fn new(ident: &str) -> TreeNode {
        let mut rsplit = ident.rsplit("##");
        let ident = rsplit.next().unwrap();
        let label = rsplit.next();
        TreeNode { ident, label }
    }

    pub fn push<'ui>(self, ui: &Ui<'ui>) -> Option<TreeNodeToken<'ui>> {
        let TreeNode { ident, label } = self;

        const FLAGS: TreeNodeFlags = TreeNodeFlags::SPAN_AVAIL_WIDTH;
        ui.align_text_to_frame_padding();
        match label {
            Some(label) => imgui::TreeNode::new(&ImString::new(ident))
                .label(&ImString::new(label))
                .flags(FLAGS)
                .push(ui),
            None => imgui::TreeNode::new(&ImString::new(ident)).flags(FLAGS).push(ui),
        }
    }

    pub fn build<T, F>(self, ui: &Ui<'_>, f: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        self.push(ui).map(|_token| f())
    }
}

// TabScroll
pub struct TabScroll<'a> {
    ident: &'a ImStr,
}

impl<'a> TabScroll<'a> {
    pub fn new(ident: &ImStr) -> TabScroll {
        TabScroll { ident }
    }

    pub fn build<T, F>(self, ui: &Ui<'_>, f: F) -> Option<Option<T>>
    where
        F: FnOnce() -> T,
    {
        TabItem::new(self.ident).build(ui, || ChildWindow::new(im_str!("scroll")).build(ui, || f()))
    }
}
