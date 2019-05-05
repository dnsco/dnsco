use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    message: String,
    errors: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    name: String,
    distance: f32,
    total_elevation_gain: f32,
    athlete: Athlete,
}

#[derive(Serialize, Deserialize)]
pub struct Athlete {
    id: usize,
}
