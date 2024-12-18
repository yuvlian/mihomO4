use crate::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AvatarData {
    pub id: String,
    pub name: String,
    pub icon: String,
}
