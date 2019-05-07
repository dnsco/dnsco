use askama::Template;

use crate::{config, service::models};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub events: Vec<models::Event>,
    pub urls: &'a config::SiteUrls,
}
