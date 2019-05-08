use actix_web::HttpResponse;

pub fn into_response<T: dnsco_service::Template>(
    template: T,
) -> Result<HttpResponse, actix_web::Error> {
    let rsp = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template parsing error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rsp))
}
