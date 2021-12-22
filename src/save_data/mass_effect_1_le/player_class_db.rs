use serde::Deserialize;

use crate::gui::RcUi;

use super::player::{ComplexTalent, Item, Me1LeClass, SimpleTalent};

#[derive(Deserialize)]
pub struct Me1LeSpecializationBonus {
    pub id: i32,
    pub label: String,
}

#[derive(Deserialize)]
pub struct Me1LePlayerClass {
    pub player_class: Me1LeClass,
    pub localized_class_name: i32,
    pub auto_levelup_template_id: i32,
    pub simple_talents: Vec<RcUi<SimpleTalent>>,
    pub complex_talents: Vec<RcUi<ComplexTalent>>,
    pub armor: Item,
    pub omni_tool: Item,
    pub bio_amp: Item,
}

#[derive(Deserialize, Deref)]
pub struct Me1LePlayerClassDb(Vec<Me1LePlayerClass>);

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;
    use anyhow::Result;

    #[test]
    fn deserialize_player_class_db() -> Result<()> {
        let input = fs::read_to_string("databases/me1_le_player_class_db.ron")?;
        let _me1_le_player_class_db: Me1LePlayerClassDb = ron::from_str(&input)?;

        Ok(())
    }
}
