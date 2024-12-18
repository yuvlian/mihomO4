use crate::Deserialize;

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
