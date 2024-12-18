use crate::Deserialize;

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
