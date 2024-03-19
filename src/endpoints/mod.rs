pub mod buckets;
pub mod clusters;
pub mod documents;
pub mod hello;
pub mod loader;
pub mod paginator;
pub mod searcher;
pub mod similarities;

use crate::services::cacher::client::RedisService;
use crate::services::{CacherClient, SearcherService};

use actix_web::web;

pub type SearcherData = web::Data<Box<dyn SearcherService>>;
pub type CacherData = web::Data<Box<CacherClient<RedisService>>>;
