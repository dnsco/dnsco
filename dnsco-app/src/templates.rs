use crate::errors::{AppError, AppResult};
use actix_web::HttpResponse;
use dnsco_service::Template;

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
