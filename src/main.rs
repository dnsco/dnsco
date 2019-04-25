#[macro_use]
extern crate tower_web;

use serde_json;
use tower_web::view::Handlebars;
use tower_web::ServiceBuilder;

mod service;
mod strava;

use service::Webserver;
use std::env;

pub fn main() {
    let strava_client_id =
        env::var("STRAVA_CLIENT_ID").expect("Missing the STRAVA_CLIENT_ID environment variable.");
    let strava_client_secret = env::var("STRAVA_CLIENT_SECRET")
        .expect("Missing the STRAVA_CLIENT_SECRET environment variable.");
    let strava_oauth_redirct_url =
        env::var("STRAVA_OAUTH_REDIRECT").unwrap_or("http://localhost:8080".to_owned());
    let strava_access_token = env::var("STRAVA_OAUTH_TOKEN").ok();

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("8080".to_owned());
    let addr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address");

    println!("Listening on http://{}", addr);

    let token = strava::authenticate(
        strava_access_token,
        strava::OauthConfig {
            client_id: strava_client_id,
            client_secret: strava_client_secret,
            redirect_url: strava_oauth_redirct_url,
        },
    );

    println!("go to http://localhost:8080/activities");

    ServiceBuilder::new()
        .resource(RouteMacro {
            service: Webserver::new(token),
        })
        .serializer(Handlebars::new())
        .run(&addr)
        .unwrap();
}

#[derive(Clone, Debug)]
pub struct RouteMacro {
    service: Webserver,
}

#[derive(Response, Debug)]
struct TemplateResponse {
    page: service::IndexResponse,
}

impl_web! {
    impl RouteMacro {
        #[get("/")]
        #[content_type("html")]
        #[web(template = "index")]
        fn hello_world( & self ) -> Result<TemplateResponse, ()> {
            let page = self.service.hello_world().unwrap();
            Ok(TemplateResponse { page })
        }

        #[get("/activities")]
        fn activities(& self ) -> Result<serde_json::Value, ()> {
            self.service.activities()
        }
    }
}
