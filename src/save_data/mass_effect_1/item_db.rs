use derive_more::Deref;
use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Hash)]
pub struct DbItem {
    pub item_id: i32,
    pub manufacturer_id: i32,
}

#[derive(Deserialize, Deref)]
pub struct Me1ItemDb(IndexMap<DbItem, String>);

#[cfg(test)]
mod test {
    use anyhow::Result;
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn deserialize_item_db() -> Result<()> {
        let mut input = String::new();
        {
            let mut file = File::open("databases/me1_item_db.ron")?;
            file.read_to_string(&mut input)?;
        }

        let _me1_item_db: Me1ItemDb = ron::from_str(&input)?;

        Ok(())
    }
}
