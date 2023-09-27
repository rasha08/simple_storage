use std::env;
use crate::db::DB;
use crate::types::upload_metadata::UploadMetadata;
use anyhow::anyhow;
use axum::extract::Query;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use serde::{Deserialize, Serialize};
use shared::tree::node::{Node, Position};
use shared::types::error::AppError;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::consts::{DEFAULT_EXTERNAL_SERVICE_URL, EXTERNAL_SERVICE_URL_ENV_VAR};

#[derive(Clone, Debug, Deserialize)]
pub struct ExternalRetrieveFileResponse {
    pub file: Vec<u8>,
    pub proof: Vec<(String, Position)>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RetrieveFileQueryParams {
    pub root: String,
    pub file_hash: String,
}

pub async fn get_file(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    query: Query<RetrieveFileQueryParams>,
) -> Result<Response, AppError> {
    let metadata = get_metadata_from_storage(db, &query.root).await?; // Validate if root exists in storage
    let filename = get_file_name(&metadata, &query.file_hash)?;

    let res = retrieve_external(&query).await?;

    validate_file(&metadata.root_hash, &res)?;

    let mut headers = HeaderMap::new();

    headers.insert(
        "CONTENT_DISPOSITION",
        format!("attachment; filename=\"{}\"", filename).parse()?,
    );

    Ok((headers, res.file).into_response())
}

async fn retrieve_external(
    data: &RetrieveFileQueryParams,
) -> anyhow::Result<ExternalRetrieveFileResponse> {
    let external_service_url = env::var(EXTERNAL_SERVICE_URL_ENV_VAR).unwrap_or_else(|_| String::from(DEFAULT_EXTERNAL_SERVICE_URL));
    let client = reqwest::Client::new();

    let res = client
        .post(&format!("{}/retrieve", external_service_url))
        .json(data)
        .send()
        .await;

    let v: ExternalRetrieveFileResponse = res.unwrap().json().await.unwrap();

    Ok(v)
}

async fn get_metadata_from_storage(
    db: Arc<Mutex<DB>>,
    root: &str,
) -> anyhow::Result<UploadMetadata> {
    let mut db = db.lock().await;
    db.get_upload_metadata(root)
}

fn validate_file(
    root: &str,
    external_response: &ExternalRetrieveFileResponse,
) -> anyhow::Result<()> {
    let hash = shared::file::hash::compute(&external_response.file);
    if Node::verify_proof(&hash, &external_response.proof, root) {
        Ok(())
    } else {
        Err(anyhow!("File was tempered after upload"))
    }
}

fn get_file_name(metadata: &UploadMetadata, file_id: &String) -> anyhow::Result<String> {
    let (filename, _) = metadata
        .files
        .iter()
        .find(|(_, v)| v == &file_id)
        .ok_or(anyhow!(
            "File does not exist in upload set identified by provided metadata"
        ))?;

    Ok(filename.to_string())
}
