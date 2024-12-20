use crate::Deserialize;

mod ch_type;
mod eq_type;
mod equipment;
mod skills;

pub use ch_type::{ElementData, PathData};
pub use eq_type::{AttributeData, AttributeLike, MainAffixData, PropertyData, SubAffixData};
pub use equipment::{LightConeData, RelicData, RelicSetData};
pub use skills::{SkillData, SkillTreeData};

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterData {
    pub id: String,
    pub name: String,
    pub rarity: u8,
    pub level: u8,
    pub promotion: u8,
    pub rank: u8,
    pub rank_icons: Vec<String>,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
    pub path: PathData,
    pub element: ElementData,
    pub skills: Vec<SkillData>,
    pub skill_trees: Vec<SkillTreeData>,
    pub light_cone: Option<LightConeData>,
    pub relics: Vec<RelicData>,
    pub relic_sets: Vec<RelicSetData>,
    pub attributes: Vec<AttributeData>,
    pub additions: Vec<AttributeData>,
    pub properties: Vec<PropertyData>,
}

impl CharacterData {
    pub fn max_level(&self) -> u8 {
        20 + 10 * self.promotion
    }
}
