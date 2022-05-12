use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod kouan;

pub struct CacheEntry {
  instant: std::time::Instant,
  content: String,
}

#[derive(Default)]
pub struct State {
  cache: HashMap<String, CacheEntry>,
}

pub type SharedState = Arc<Mutex<State>>;
