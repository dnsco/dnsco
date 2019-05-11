use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub id: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Summary {
    pub id: usize,
    #[serde(rename = "firstname")]
    pub first_name: String,
    #[serde(rename = "lastname")]
    pub last_name: String,
    pub city: String,
    pub country: String,
}
