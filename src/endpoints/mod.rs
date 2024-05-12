pub mod clusters;
pub mod documents;
pub mod folders;
pub mod hello;
pub mod paginator;
pub mod searcher;
pub mod similarities;
pub mod watcher;

use crate::services::cacher::CacherClient;
use crate::services::redis_cache::client::RedisService;
use crate::services::searcher::SearcherService;

use actix_web::web;

pub type SearcherData = web::Data<Box<dyn SearcherService>>;
pub type CacherData = web::Data<Box<CacherClient<RedisService>>>;
