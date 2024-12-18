use crate::Deserialize;
use crate::character::{AttributeData, MainAffixData, PathData, PropertyData, SubAffixData};

#[derive(Deserialize, Debug, Clone)]
pub struct LightConeData {
    pub id: String,
    pub name: String,
    pub rarity: u8,
    pub rank: u8,
    pub level: u8,
    pub promotion: u8,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
    pub path: PathData,
    pub attributes: Vec<AttributeData>,
    pub properties: Vec<PropertyData>,
}

impl LightConeData {
    pub fn max_level(&self) -> u8 {
        20 + 10 * self.promotion
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct RelicData {
    pub id: String,
    pub name: String,
    pub set_id: String,
    pub set_name: String,
    pub rarity: u8,
    pub level: u8,
    pub main_affix: MainAffixData,
    #[serde(rename = "sub_affix")]
    pub sub_affixes: Vec<SubAffixData>,
    pub icon: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RelicSetData {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub num: u8,
    pub desc: String,
    pub properties: Vec<PropertyData>,
}
