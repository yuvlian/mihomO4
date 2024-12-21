use crate::client::fetch_json;
use serde::Deserialize;
use std::error::Error;

pub enum Language {
    Cht,
    Cn,
    De,
    En,
    Es,
    Fr,
    Id,
    Jp,
    Kr,
    Pt,
    Ru,
    Th,
    Vi,
}

impl Language {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Cht => "cht",
            Self::Cn => "cn",
            Self::De => "de",
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::Id => "id",
            Self::Jp => "jp",
            Self::Kr => "kr",
            Self::Pt => "pt",
            Self::Ru => "ru",
            Self::Th => "th",
            Self::Vi => "vi",
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Mihomo {
    pub player: PlayerData,
    pub characters: Vec<CharacterData>,
}

impl Mihomo {
    pub async fn fetch_user(
        uid: u32,
        language: &Language,
        client: &reqwest::Client,
    ) -> Result<Self, Box<dyn Error>> {
        let url = format!(
            "https://api.mihomo.me/sr_info_parsed/{}?lang={}",
            uid,
            language.as_str()
        );
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_nickname(&self) -> &str {
        &self.player.nickname
    }

    pub fn get_lv_and_wlv(&self) -> (u8, u8) {
        (self.player.level, self.player.world_level)
    }

    pub fn get_friend_cnt(&self) -> u8 {
        self.player.friend_count
    }

    pub fn get_pfp_url(&self) -> &str {
        &self.player.avatar.icon
    }

    pub fn get_signature(&self) -> &str {
        &self.player.signature
    }

    pub fn is_display(&self) -> bool {
        self.player.is_display
    }

    pub fn get_space_info(&self) -> &SpaceInfoData {
        &self.player.space_info
    }

    pub fn get_character_ids(&self) -> Vec<&str> {
        self.characters.iter().map(|c| c.id.as_str()).collect()
    }

    pub fn get_character_names(&self) -> Vec<&str> {
        self.characters.iter().map(|c| c.name.as_str()).collect()
    }

    pub fn get_character_by_id(&self, id: &str) -> Option<&CharacterData> {
        self.characters.iter().find(|c| c.id == id)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerData {
    pub uid: String,
    pub nickname: String,
    pub level: u8,
    pub world_level: u8,
    pub friend_count: u8,
    pub avatar: AvatarData,
    pub signature: String,
    pub is_display: bool,
    pub space_info: SpaceInfoData,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AvatarData {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpaceInfoData {
    pub memory_data: MemoryOfChaosData,
    pub universe_level: u8,
    pub avatar_count: u16,
    pub light_cone_count: u16,
    pub relic_count: u16,
    pub achievement_count: u16,
    pub book_count: u16,
    pub music_count: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MemoryOfChaosData {
    pub level: u8,
    pub chaos_id: Option<String>,
    pub chaos_level: u8,
    pub chaos_star_count: u8,
}

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

#[derive(Deserialize, Debug, Clone)]
pub struct PathData {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElementData {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: String,
}

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

#[derive(Deserialize, Debug, Clone)]
pub struct AttributeData {
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    pub display: String,
    #[serde(rename = "percent")]
    pub is_percent: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PropertyData {
    pub r#type: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    pub display: String,
    #[serde(rename = "percent")]
    pub is_percent: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MainAffixData {
    pub r#type: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    pub display: String,
    #[serde(rename = "percent")]
    pub is_percent: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SubAffixData {
    pub r#type: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    pub display: String,
    #[serde(rename = "percent")]
    pub is_percent: bool,
    pub count: u8,
    pub step: u8,
}
