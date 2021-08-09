use derive_more::{Deref, From};
use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DbItem {
    pub item_id: i32,
    pub manufacturer_id: i32,
}

#[derive(Deserialize, Deref, From, PartialEq, Eq)]
pub struct Me1ItemDb(IndexMap<DbItem, String>);

#[cfg(test)]
mod test {
    use anyhow::Result;
    use std::fs;

    use super::*;

    #[test]
    fn deserialize_item_db() -> Result<()> {
        let input = fs::read_to_string("databases/me1_item_db.ron")?;
        let _me1_item_db: Me1ItemDb = ron::from_str(&input)?;

        Ok(())
    }
}
