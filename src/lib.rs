pub use reqwest::Client;
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
    pub fn as_str(&self) -> &'static str {
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

pub mod character;
pub mod player;
pub mod util;

use character::CharacterData;
use player::PlayerData;
use util::fetch_json;

#[derive(Deserialize, Debug, Clone)]
pub struct Mihomo {
    pub player: PlayerData,
    pub characters: Vec<CharacterData>,
}

impl Mihomo {
    pub async fn fetch_user(
        uid: u32,
        language: &Language,
        client: &Client,
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

    pub fn get_space_info(&self) -> &player::SpaceInfoData {
        &self.player.space_info
    }

    pub fn get_character_ids(&self) -> Vec<&str> {
        self.characters.iter().map(|c| c.id.as_str()).collect()
    }

    pub fn get_character_names(&self) -> Vec<&str> {
        self.characters.iter().map(|c| c.name.as_str()).collect()
    }

    pub fn get_character_by_id(&self, id: &str) -> Option<CharacterData> {
        self.characters.iter().cloned().find(|c| c.id == id)
    }
}
