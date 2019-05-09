use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Meta {
    id: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Summary {
    id: usize,
    #[serde(rename = "firstname")]
    first_name: String,
    #[serde(rename = "lastname")]
    last_name: String,
    city: String,
    country: String,
}
