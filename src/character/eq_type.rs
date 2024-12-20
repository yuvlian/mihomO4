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

pub trait AttributeLike {
    fn field(&self) -> &String;
    fn name(&self) -> &String;
    fn icon(&self) -> &String;
    fn value(&self) -> f64;
    fn display(&self) -> &String;
    fn is_percent(&self) -> bool;
}

impl AttributeLike for AttributeData {
    fn field(&self) -> &String {
        &self.field
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn icon(&self) -> &String {
        &self.icon
    }
    fn value(&self) -> f64 {
        self.value
    }
    fn display(&self) -> &String {
        &self.display
    }
    fn is_percent(&self) -> bool {
        self.is_percent
    }
}

impl AttributeLike for PropertyData {
    fn field(&self) -> &String {
        &self.field
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn icon(&self) -> &String {
        &self.icon
    }
    fn value(&self) -> f64 {
        self.value
    }
    fn display(&self) -> &String {
        &self.display
    }
    fn is_percent(&self) -> bool {
        self.is_percent
    }
}
