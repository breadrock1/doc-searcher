pub mod clusters;
pub mod documents;
pub mod folders;
pub mod hello;
pub mod paginator;
pub mod searcher;
pub mod watcher;

use crate::services::cacher::CacherClient;
use crate::services::redis_cache::client::RedisService;

use actix_web::web;

pub type CacherData = web::Data<Box<CacherClient<RedisService>>>;
