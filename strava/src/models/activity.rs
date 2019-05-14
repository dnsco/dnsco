use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use std::fmt::{Display, Formatter, Result as FormatResult};

use crate::models::athlete;
use uom::si::f64::Length;
use uom::si::length::{foot, meter, mile};

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
    pub distance: Distance,
    pub total_elevation_gain: Distance,
    pub athlete: athlete::Meta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Distance(f64);
impl Distance {
    fn length(&self) -> Length {
        Length::new::<meter>(self.0)
    }

    pub fn miles(&self) -> String {
        format!("{:.2} {}", self.length().get::<mile>(), "miles")
    }

    pub fn feet(&self) -> String {
        format!("{} {}", self.length().get::<foot>().round(), "feet")
    }
}

impl Display for Distance {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "({:.2} meters)", self.0)
    }
}
