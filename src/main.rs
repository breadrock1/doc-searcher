mod context;
mod endpoints;
mod errors;
mod es_client;
mod wrappers;

use crate::context::SearchContext;
use crate::es_client::{build_elastic, build_service, init_service_parameters};

use actix_web::{web, App, HttpServer};
use elasticsearch::http::transport::BuildError;

#[actix_web::main]
async fn main() -> Result<(), BuildError> {
    let service_parameters = init_service_parameters()?;
    let es_host = service_parameters.es_host();
    let es_user = service_parameters.es_user();
    let es_passwd = service_parameters.es_passwd();
    let service_port = service_parameters.service_port();
    let service_addr = service_parameters.service_address();

    let elastic = build_elastic(es_host, es_user, es_passwd)?;
    let search_context = SearchContext::_new(elastic);

    HttpServer::new(move || {
        let cxt = search_context.clone();
        App::new()
            .app_data(web::Data::new(cxt))
            .service(build_service())
    })
    .bind((service_addr, service_port))?
    .run()
    .await?;

    Ok(())
}
