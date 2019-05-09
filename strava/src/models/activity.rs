use serde::{Deserialize, Serialize};

use crate::models::athlete;

#[derive(Serialize, Deserialize)]
pub struct Meta {
    id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Summary {
    id: usize,
    name: String,
    distance: f32,
    total_elevation_gain: f32,
    athlete: athlete::Meta,
}
