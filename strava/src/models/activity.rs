use serde::{Deserialize, Serialize};

use crate::models::athlete;
use chrono::{DateTime, FixedOffset, Utc};

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Summary {
    pub id: usize,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub start_date_local: DateTime<FixedOffset>,
    pub distance: f32,
    pub total_elevation_gain: f32,
    pub athlete: athlete::Meta,
}
