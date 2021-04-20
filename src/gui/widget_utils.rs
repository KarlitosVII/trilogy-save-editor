use imgui::{sys, Ui};
use std::marker::PhantomData;

use super::*;

impl<'ui> Gui<'ui> {
    pub fn set_next_item_open(&self, is_open: bool) {
        unsafe {
            sys::igSetNextItemOpen(is_open, Condition::FirstUseEver as i32);
        }
    }

    pub fn push_tree_node(&self, ident: &str) -> Option<TreeNodeToken> {
        let label = if let Some(label) = ident.split("##").next() { label } else { ident };

        self.ui.align_text_to_frame_padding();
        TreeNode::new(&ImString::new(ident))
            .label(&ImString::new(label))
            .flags(TreeNodeFlags::SPAN_AVAIL_WIDTH)
            .push(self.ui)
    }

    pub fn begin_table(&self, ident: &ImStr, column: i32) -> Option<TableToken> {
        const FLAGS: u32 = sys::ImGuiTableFlags_RowBg
            | sys::ImGuiTableFlags_BordersOuterH
            | sys::ImGuiTableFlags_BordersOuterV;
        if unsafe {
            sys::igBeginTable(ident.as_ptr(), column, FLAGS as i32, [0.0, 0.0].into(), 0.0)
        } {
            Some(TableToken::new(self.ui))
        } else {
            None
        }
    }

    pub fn table_next_row(&self) {
        unsafe {
            sys::igTableNextRow(sys::ImGuiTableRowFlags_None as i32, 1.0);
        }
    }

    pub fn table_next_column(&self) -> bool {
        unsafe { sys::igTableNextColumn() }
    }
}

#[must_use]
pub struct TableToken<'ui>(PhantomData<Ui<'ui>>);

impl<'ui> TableToken<'ui> {
    /// Creates a new token type.
    pub fn new(_: &Ui<'ui>) -> Self {
        Self(std::marker::PhantomData)
    }
}

impl Drop for TableToken<'_> {
    fn drop(&mut self) {
        unsafe { sys::igEndTable() }
    }
}
