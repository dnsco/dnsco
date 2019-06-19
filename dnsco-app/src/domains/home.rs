use askama::Template;

use crate::config;
use dnsco_data::models;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub events: Vec<models::Event>,
    pub urls: &'a config::SiteUrls,
}

impl<'a> IndexTemplate<'a> {
    pub fn new(events: Vec<models::Event>, urls: &config::SiteUrls) -> IndexTemplate {
        IndexTemplate { events, urls }
    }
}
