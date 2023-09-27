use std::net::SocketAddr;
use std::sync::Arc;

use axum::{extract::DefaultBodyLimit, routing::get, Extension, Router};
use shared::tracing::tracing::{info, init_tracer};
use tokio::sync::Mutex;
use tower_http::limit::RequestBodyLimitLayer;

use crate::db::DB;
use crate::handlers::get_file::get_file;
use crate::handlers::get_upload_form::get_upload_form;
use crate::handlers::handle_upload::handle_upload;
use crate::handlers::show_uploads_list::get_upload_list;

mod consts;
mod db;
mod handlers;
mod types;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracer("fclient");
    let db = Arc::new(Mutex::new(DB::new()?));

    // build our application with some routes
    let app = Router::new()
        .route("/", get(get_upload_list))
        .route("/upload", get(get_upload_form).post(handle_upload))
        .route("/get-file", get(get_file))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(Extension(db));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
