use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ErrorResponse {
    message: String,
    errors: Vec<HashMap<String, String>>,
}

use std::collections::HashMap;
use std::fmt;

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    name: String,
    distance: f32,
    total_elevation_gain: f32,
    athlete: Athlete,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Athlete {
    id: usize,
}
