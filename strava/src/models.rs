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

pub mod activity {
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
}

pub mod athlete {
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
}
