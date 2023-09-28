mod context;
mod endpoints;
mod errors;
mod wrappers;

use actix_web::{web, App, HttpServer, Scope};
use crate::context::SearchContext;
use crate::endpoints::elastic::{create_index, find_index};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let elastic_addr = "https://localhost:9200";
    let cxt = SearchContext::_new(elastic_addr).unwrap();
    HttpServer::new(move || {
        let cxt = cxt.clone();
        App::new()
            .app_data(web::Data::new(cxt))
            .service(build_service())
    })
    .bind(("127.0.0.1", 45678))?
    .run()
    .await
}

fn build_service() -> Scope {
    web::scope("/home")
        .service(create_index)
        .service(find_index)
}
