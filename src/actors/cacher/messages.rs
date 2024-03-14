use actix::{Actor, ActorFutureExt, Context, Handler, Message, ResponseFuture};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use futures::future::LocalBoxFuture;
use std::pin::Pin;
use tokio::sync::RwLock;

pub(crate) struct GetValue {
    key: String,
}

impl Message for GetValue {
    type Result = Option<String>;
}

impl GetValue {
    pub fn new(key: String) -> Self {
        GetValue {
            key
        }
    }
}

pub(crate) struct SetValue {
    key: String,
    value: String,
}

impl Message for SetValue {
    type Result = ();
}

impl SetValue {
    pub fn new(key: String, value: String) -> Self {
        SetValue {
            key,
            value,
        }
    }
}

#[derive(Default)]
pub struct CacheActor {
    cache: Arc<RwLock<HashMap<String, String>>>,
}

impl CacheActor {
    pub fn get_cache_ref(&self) -> Arc<RwLock<HashMap<String, String>>> {
        self.cache.clone()
    }
}

impl Actor for CacheActor {
    type Context = Context<Self>;
}

impl Handler<GetValue> for CacheActor {
    type Result = ResponseFuture<Option<String>>;

    fn handle(&mut self, value: GetValue, _cxt: &mut Context<Self>) -> Self::Result {
        let cacher_client = self.get_cache_ref();
        Box::pin(async move {
            let cacher = cacher_client.read().await;
            match cacher.get(&value.key).cloned() {
                None => None,
                Some(val) => {
                    log::info!("{}", val);
                    Some(val)
                }
            }
        })
    }
}

impl Handler<SetValue> for CacheActor {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, value: SetValue, _cxt: &mut Context<Self>) -> Self::Result {
        let cache_client = self.get_cache_ref();
        Box::pin(async move {
            let mut cacher = cache_client.write().await;
            match cacher.insert(value.key, value.value) {
                None => {
                    log::info!("{}", "None");
                },
                Some(val) => {
                    log::info!("{}", val);
                }
            }
        })
    }
}
