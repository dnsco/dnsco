#[macro_use]
extern crate tower_web;
#[macro_use]
extern crate serde_json;

extern crate oauth2;
extern crate reqwest;
extern crate tokio;
extern crate url;

use tower_web::view::Handlebars;
use tower_web::ServiceBuilder;

mod auth;
mod service;

use service::Webserver;
use std::env;

pub fn main() {
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("8080".to_owned());
    let addr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address");
    println!("Listening on http://{}", addr);

    let token = auth::authenticate("http://localhost:8080").0;

    dbg!(&token);

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
        fn hello_world(&self) -> Result<TemplateResponse, ()> {
            let page = self.service.hello_world().unwrap();
            Ok(TemplateResponse { page })
        }

        #[get("/activities")]
        fn activities(&self) -> Result<serde_json::Value, ()> {
            self.service.activities()
        }
    }
}
