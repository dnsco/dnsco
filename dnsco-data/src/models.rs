use serde::Serialize;

use crate::schema::activities;
#[derive(Queryable)]
pub struct Activity {
    pub id: i32,
    pub description: Option<String>,
    pub distance: Option<f64>,
    pub name: String,
    pub remote_athlete_id: i32,
    pub remote_id: i32,
}

#[derive(AsChangeset, Insertable)]
#[table_name = "activities"]
pub struct NewActivity<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub distance: Option<f64>,
    pub remote_athlete_id: i32,
    pub remote_id: i32,
}

#[derive(Serialize, Clone, Debug)]
pub struct Event {
    pub name: &'static str,
    pub time: &'static str,
    pub info: Race,
}

#[derive(Serialize, Clone, Debug)]
pub struct Race {
    pub distance: &'static str,
}
