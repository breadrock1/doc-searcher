pub mod buckets;
pub mod clusters;
pub mod documents;
pub mod hello;
pub mod loader;
pub mod searcher;
pub mod similarities;

use crate::searcher::service_client::ServiceClient;
use actix_web::web;

pub type ContextData = web::Data<Box<dyn ServiceClient>>;
