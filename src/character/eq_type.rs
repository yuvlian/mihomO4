use crate::Deserialize;

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

// MainAffix == PropertyData
// SubAffix == EqProData + count & step

#[derive(Deserialize, Debug, Clone)]
pub struct MainAffixData {
    #[serde(flatten)]
    pub property: PropertyData,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SubAffixData {
    #[serde(flatten)]
    pub property: PropertyData,
    pub count: u8,
    pub step: u8,
}
