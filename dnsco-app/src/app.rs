use actix_web::error::BlockingError;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use futures::Future;
use log::error;
use strava::oauth::{ClientConfig as OauthConfig, RedirectQuery as OauthQuery};
use url::Url;

use dnsco_data::{DataError, Database};

use crate::service::Service;
use crate::AppError;

pub struct Config {
    pub db_url: String,
    pub strava_client_id: String,
    pub strava_client_secret: String,
    pub port: u16,
    pub host: String,
}

pub fn run_config(conf: Config) {
    let site_urls = SiteUrls::new(conf.host);
    let pool = dnsco_data::Database::create(conf.db_url);

    let oauth_config = OauthConfig {
        client_id: conf.strava_client_id,
        client_secret: conf.strava_client_secret,
        redirect_url: site_urls.oauth_redirect(),
    };

    run(pool, oauth_config, site_urls, conf.port)
}

pub fn run(db: Database, strava: OauthConfig, urls: SiteUrls, port: u16) {
    let service = web::Data::new(Service::new(db, strava, urls.clone()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .register_data(service.clone())
            .service(web::resource("/").to(index))
            .route(urls.activities().path(), web::get().to_async(activities))
            .route(
                urls.update_activities().path(),
                web::get().to_async(update_activities),
            )
            .route(urls.oauth_redirect().path(), web::get().to_async(oauth))
    })
    .bind(format!("0.0.0.0:{}", port))
    .unwrap()
    .run()
    .unwrap();
}

pub fn index(service: web::Data<Service>) -> AppResult {
    TemplateResponse::new(service.hello_world()).into()
}

pub fn activities(
    service: web::Data<Service>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    render_template(move || service.activities())
}

fn render_template<F, T>(f: F) -> impl Future<Item = HttpResponse, Error = actix_web::Error>
where
    T: Template + Send + 'static,
    F: FnOnce() -> Result<T, AppError> + Send + 'static,
{
    web::block(f)
        .map(|t| AppResult::from(TemplateResponse::new(t)))
        .then(|result| match result {
            Ok(Ok(temp)) => temp,
            err => err.into(),
        })
}

pub fn update_activities(
    service: web::Data<Service>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    let redirect_path = service.urls.activities().path().to_owned();

    web::block(move || service.update_activities()).then(|res| match res {
        Ok(_) => HttpResponse::Found()
            .header(http::header::LOCATION, redirect_path)
            .finish(),
        Err(e) => AppError::from(e).error_response(),
    })
}

pub fn oauth(
    oauth_resp: web::Query<OauthQuery>,
    service: web::Data<Service>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    web::block(move || {
        let redirect_path = service.urls.update_activities().path().to_owned();
        service.update_oauth_token(&oauth_resp)?;
        Ok(redirect_path)
    })
    .then(|res| match res {
        Ok(redirect_path) => HttpResponse::Found()
            .header(http::header::LOCATION, redirect_path)
            .finish(),
        Err(e) => AppError::from(e).error_response(),
    })
}

pub type AppResult = Result<HttpResponse, AppError>;

impl From<BlockingError<AppError>> for AppError {
    fn from(e: BlockingError<AppError>) -> AppError {
        match e {
            BlockingError::Error(e) => e,
            BlockingError::Canceled => AppError::ThreadCanceled,
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DataError(DataError::NotAuthenticated(redirect_url)) => HttpResponse::Found()
                .header(http::header::LOCATION, redirect_url.to_string())
                .finish(),
            e => {
                error!("Unhandled Error: {}", e);
                HttpResponse::InternalServerError().body("Something Went Wrong")
            }
        }
    }
}

pub struct TemplateResponse<T: Template>(T);

impl<T: Template> TemplateResponse<T> {
    pub fn new(template: T) -> Self {
        TemplateResponse(template)
    }
}

impl<T: Template> From<TemplateResponse<T>> for AppResult {
    fn from(t: TemplateResponse<T>) -> AppResult {
        let rendered =
            t.0.render()
                .map_err(|e| AppError::TemplateError(Box::new(e)))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
    }
}

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

    //    pub fn site_url(&self) -> Url {
    //        self.base.clone()
    //    }

    pub fn activities(&self) -> Url {
        self.base.join("/activities").unwrap()
    }

    pub fn update_activities(&self) -> Url {
        self.base.join("/update_activities").unwrap()
    }

    pub fn oauth_redirect(&self) -> Url {
        self.base.join("/oauth").unwrap()
    }
}
