use crate::actors::cacher::messages::{CacheActor, GetValue, SetValue};

use actix::Addr;
use actix_web::{HttpResponse, Responder, web};

async fn get_value(
    key: web::Path<String>,
    cache: web::Data<Addr<CacheActor>>
) -> impl Responder {
    match cache.send(GetValue::new(key.into_inner())).await {
        Ok(result) => match result {
            Some(value) => HttpResponse::Ok().body(value),
            None => HttpResponse::NotFound().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn set_value(
    info: web::Json<(String, String)>,
    cache: web::Data<Addr<CacheActor>>,
) -> impl Responder {
    let (key, value) = info.into_inner();
    match cache.send(SetValue::new(key, value)).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
