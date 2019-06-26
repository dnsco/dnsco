use askama::Template;

use crate::app;
use dnsco_data::models;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub events: Vec<models::Event>,
    pub urls: &'a app::SiteUrls,
}

impl<'a> Index<'a> {
    pub fn new(events: Vec<models::Event>, urls: &app::SiteUrls) -> Index {
        Index { events, urls }
    }
}
