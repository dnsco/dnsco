use crate::config;
use askama::Template;
use dnsco_data::models;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub events: Vec<models::Event>,
    pub urls: &'a config::SiteUrls,
}
