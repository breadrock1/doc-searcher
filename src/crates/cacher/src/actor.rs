use crate::cacher::RedisService;
use crate::values::from_redis_value_to_doc_vec;
use crate::values::{from_redis_value_to_doc_vec_2, VecCacherDocuments};
use actix::fut::ok;
use actix::{Actor, Context, Handler, Message, ResponseFuture};
use futures_util::future::{BoxFuture, LocalBoxFuture};
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client, Commands, RedisError, RedisResult, Value};
use std::rc::Rc;
pub use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::sync::RwLock;
use wrappers::document::Document;
use wrappers::search_params::SearchParams;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error(transparent)]
    InternalError(Box<dyn std::error::Error + Send>),
    #[error(transparent)]
    ConnectionError(Box<dyn std::error::Error + Send>),
}

impl From<RedisError> for BackendError {
    fn from(err: RedisError) -> Self {
        BackendError::InternalError(Box::new(err))
    }
}

#[derive(Message, Debug, Clone, PartialEq)]
#[rtype(result = "Result<VecCacherDocuments, BackendError>")]
pub struct Get {
    pub key: String,
}

pub struct RedisClient {
    client: Arc<Mutex<Client>>,
}

impl Default for RedisClient {
    fn default() -> Self {
        let address = "redis://127.0.0.1:6379/";
        let redis_client = redis::Client::open(address);
        let client_arc = Arc::new(Mutex::new(redis_client.unwrap()));
        RedisClient { client: client_arc }
    }
}

impl Actor for RedisClient {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Cache actor started");
    }
}

impl Handler<Get> for RedisClient {
    type Result = ResponseFuture<Result<VecCacherDocuments, BackendError>>;

    fn handle(&mut self, msg: Get, _: &mut Self::Context) -> Self::Result {
        let cxt = self.client.lock().unwrap();
        let mut conn_result = cxt.get_connection().unwrap();
        let result = async move {
            let redis_value: Result<Value, BackendError> =
                conn_result.get(msg.key).map_err(BackendError::from);

            let value = &redis_value.unwrap();
            from_redis_value_to_doc_vec(value).map_err(BackendError::from)
        };

        Box::pin(result)
    }
}

#[derive(Clone)]
pub struct RedisClientAsync {
    client: Arc<RwLock<redis::Client>>,
}

impl Default for RedisClientAsync {
    fn default() -> Self {
        let address = "redis://127.0.0.1:6379/";
        let redis_client = redis::Client::open(address);
        let client_arc = Arc::new(RwLock::new(redis_client.unwrap()));
        RedisClientAsync { client: client_arc }
    }
}

impl Actor for RedisClientAsync {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Cache actor started");
    }
}

#[derive(Message, Debug, Clone, PartialEq)]
#[rtype(result = "Result<VecCacherDocuments, BackendError>")]
pub struct Get2 {
    pub key: String,
}

impl Handler<Get2> for RedisClientAsync {
    type Result = ResponseFuture<Result<VecCacherDocuments, BackendError>>;

    fn handle(&mut self, msg: Get2, _: &mut Self::Context) -> Self::Result {
        let test = self.client.clone();
        let result = async move {
            let cxt = test.read().await;
            let mut conn_result = cxt.get_async_connection().await.unwrap();
            let redis_value: RedisResult<Value> = conn_result.get(msg.key).await;

            let value = redis_value.unwrap();
            from_redis_value_to_doc_vec_2(value).map_err(BackendError::from)
        };

        Box::pin(result)
    }
}
