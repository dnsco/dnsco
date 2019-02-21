#[macro_use]
extern crate tower_web;

extern crate tokio;

use tower_web::ServiceBuilder;
use tower_web::view::Handlebars;

mod service;

use service::Webserver;
use std::env;


pub fn main() {
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("8080".to_owned());
    let addr = format!("{}:{}", host, port).parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(RouteMacro { service: Webserver::new() })
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
    page: service::IndexResponse
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
    }
}
