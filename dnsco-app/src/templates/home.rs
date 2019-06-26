use askama::Template;

use crate::app;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub urls: &'a app::SiteUrls,
}

impl<'a> Index<'a> {
    pub fn new(urls: &app::SiteUrls) -> Index {
        Index { urls }
    }
}
