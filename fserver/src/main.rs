use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::routing::post;
use axum::{Extension, Router};
use shared::tracing::tracing::{info, init_tracer};
use tokio::sync::Mutex;
use tower_http::limit::RequestBodyLimitLayer;

use crate::consts::FILES_DIRECTORY;
use crate::db::DB;
use crate::handlers::retrieve::retrieve;
use crate::handlers::upload::upload;

mod consts;
mod db;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  init_tracer("fserver");
  ensure_dir_exists(FILES_DIRECTORY)?;

  let db = Arc::new(Mutex::new(DB::new()?));

  let app = Router::new()
    .route("/upload", post(upload))
    .route("/retrieve", post(retrieve))
    .layer(DefaultBodyLimit::disable())
    .layer(RequestBodyLimitLayer::new(250 * 1024 * 1024 /* 250mb */))
    .layer(tower_http::trace::TraceLayer::new_for_http())
    .layer(Extension(db));

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

  info!("listening on {}", addr);

  axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
  Ok(())
}

fn ensure_dir_exists<P: AsRef<Path>>(dir_path: P) -> std::io::Result<()> {
  if !dir_path.as_ref().exists() {
    fs::create_dir_all(dir_path)?
  }
  Ok(())
}
