use askama::Template;

use crate::config;
use dnsco_data::models;

pub fn new(events: Vec<models::Event>, urls: &config::SiteUrls) -> Index {
    Index { events, urls }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub events: Vec<models::Event>,
    pub urls: &'a config::SiteUrls,
}
