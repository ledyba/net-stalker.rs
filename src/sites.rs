use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::body::Body;
use axum::{body, Extension};
use axum::extract::Path;
use axum::response::{IntoResponse, Response};

mod kouan;
mod hmc;

pub trait Site {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>>;
}

pub struct Entry {
  site: Box<dyn Site + Send + Sync>,
  cache: Option<Cache>,
}

struct Cache {
  instant: Instant,
  content: String,
}

#[derive(Clone)]
pub struct Service {
  sites: Arc<std::sync::RwLock<HashMap<String, Arc<tokio::sync::RwLock<Entry>>>>>,
}

impl Service {
  pub fn new() -> Self {
    let mut sites: HashMap<String, Arc<tokio::sync::RwLock<Entry>>> = Default::default();
    sites.insert("kouan".to_string(), Arc::new(tokio::sync::RwLock::new(Entry {
      site: Box::new(kouan::Kouan{}),
      cache: None,
    })));
    sites.insert("hmc".to_string(), Arc::new(tokio::sync::RwLock::new(Entry {
      site: Box::new(hmc::HMC{}),
      cache: None,
    })));
    Self {
      sites: Arc::new(std::sync::RwLock::new(sites)),
    }
  }

  pub async fn serve(&mut self, name: String) -> Response<Body> {
    let site = { // Try read from cache
      let sites = self.sites.read().unwrap();
      let site = sites.get(&name);
      if site.is_none() {
        return Response::builder()
          .status(404)
          .body(body::Body::from("Not found"))
          .unwrap();
      }
      site.unwrap().clone()
    };
    // check cache
    {
      let entry = site.read().await;
      if let Some(cache) = &entry.cache {
        if (Instant::now() - cache.instant) < Duration::from_secs(3600) {
          return build_resp(&cache.content);
        }
      }
    }
    // read and store
    let mut entry = site.write().await;
    let content = entry.site.fetch().await;
    if content.is_err() {
      return Response::builder()
        .status(500)
        .body(body::Body::from(content.unwrap_err().to_string()))
        .unwrap();
    }
    let content = content.unwrap();
    entry.cache = Some(Cache {
      instant: Instant::now(),
      content: content.clone(),
    });
    build_resp(&content)
  }
}

fn build_resp(content: &str) -> Response<Body> {
  Response::builder()
    .status(200)
    .header("content-type", "application/rss+xml")
    .body(body::Body::from(content.to_string()))
    .unwrap()
}

pub async fn serve(
  Path(name): Path<String>,
  Extension(mut service): Extension<Service>,
) -> impl IntoResponse {
  service.serve(name).await
}
