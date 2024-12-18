use crate::Deserialize;

mod avatar;
mod space_info;

pub use avatar::AvatarData;
pub use space_info::{MemoryOfChaosData, SpaceInfoData};

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
