use serde::{Deserialize, Serialize};

pub mod activity;
pub mod athlete;

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
