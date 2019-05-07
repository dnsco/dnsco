use url::Url;

#[derive(Clone)]
pub struct SiteUrls {
    base: Url,
}

impl SiteUrls {
    pub fn new(host: String) -> Self {
        let host_with_scheme = format!("http://{}", host);

        Self {
            base: url::Url::parse(&host_with_scheme).unwrap(),
        }
    }

    pub fn site_url(&self) -> Url {
        self.base.clone()
    }

    pub fn activities(&self) -> Url {
        self.base.join("/activities").unwrap()
    }

    pub fn oauth_redirect(&self) -> Url {
        self.base.join("/oauth").unwrap()
    }
}
