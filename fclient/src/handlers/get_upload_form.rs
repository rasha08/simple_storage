use anyhow::anyhow;
use axum::response::Html;
use shared::types::error::AppError;
use tera::{Context, Tera};

pub async fn get_upload_form() -> Result<Html<String>, AppError> {
    let tera = Tera::new("templates/*")?;
    let context = Context::new();
    let rendered = tera
        .render("upload.html", &context)
        .map_err(|_| anyhow!("Failed to load the template"))?;
    Ok(Html(rendered))
}
