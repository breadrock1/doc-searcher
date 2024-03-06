use redis::Client;
use std::sync::{Arc, Mutex};

pub struct RedisClient {
    client: Arc<Mutex<Client>>,
}
