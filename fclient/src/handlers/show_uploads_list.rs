use std::sync::Arc;

use anyhow::anyhow;
use axum::response::Html;
use axum::Extension;
use shared::types::error::AppError;
use tera::{Context, Tera};
use tokio::sync::Mutex;

use crate::db::DB;

pub async fn get_upload_list(
    Extension(db): Extension<Arc<Mutex<DB>>>,
) -> Result<Html<String>, AppError> {
    let tera = Tera::new("templates/*")?;

    let mut context = Context::new();

    let uploads = db.lock().await.list_upload_metadata()?;

    context.insert("uploads", &uploads);

    let rendered = tera
        .render("home.html", &context)
        .map_err(|e| anyhow!("Failed to render the template {:?}", e))?;

    Ok(Html(rendered))
}
