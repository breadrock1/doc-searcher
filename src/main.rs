mod endpoints;
mod errors;
mod es_client;
mod searcher;
mod wrappers;

use crate::es_client::{build_cors_config, build_elastic, build_service, init_service_parameters};
use crate::searcher::elastic::context::ElasticContext;
use crate::searcher::service_client::ServiceClient;

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
    let cors_origin = service_parameters.cors_origin();

    let elastic = build_elastic(es_host, es_user, es_passwd)?;
    let search_context = ElasticContext::_new(elastic);

    HttpServer::new(move || {
        let cxt = search_context.clone();
        let box_cxt: Box<dyn ServiceClient> = Box::new(cxt);
        let cors_cln = cors_origin.clone();
        let cors = build_cors_config(cors_cln.as_str());
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(box_cxt))
            .service(build_service())
    })
    .bind((service_addr, service_port))?
    .run()
    .await?;

    Ok(())
}
