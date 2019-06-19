use askama::Template;

use crate::app;
use dnsco_data::models;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub events: Vec<models::Event>,
    pub urls: &'a app::SiteUrls,
}

impl<'a> IndexTemplate<'a> {
    pub fn new(events: Vec<models::Event>, urls: &app::SiteUrls) -> IndexTemplate {
        IndexTemplate { events, urls }
    }
}
