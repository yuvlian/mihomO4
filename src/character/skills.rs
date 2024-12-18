use crate::Deserialize;
use crate::character::ElementData;

#[derive(Deserialize, Debug, Clone)]
pub struct SkillData {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub max_level: u8,
    pub element: Option<ElementData>,
    pub r#type: String,
    pub type_text: String,
    pub effect: String,
    pub effect_text: String,
    pub simple_desc: String,
    pub desc: String,
    pub icon: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SkillTreeData {
    pub id: String,
    pub level: u8,
    pub max_level: u8,
    pub icon: String,
    pub anchor: String,
    pub parent: Option<String>,
}
