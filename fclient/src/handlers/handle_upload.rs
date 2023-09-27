use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::anyhow;
use axum::extract::Multipart;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Extension;
use reqwest::multipart::Form;
use serde::Deserialize;
use shared::tree::node::Node;
use shared::types::error::AppError;
use tokio::sync::Mutex;
use crate::consts::{DEFAULT_EXTERNAL_SERVICE_URL, EXTERNAL_SERVICE_URL_ENV_VAR};

use crate::db::DB;
use crate::types::upload_metadata::UploadMetadata;

#[derive(Debug, Clone, Deserialize)]
pub struct ExternalUploadResult {
    pub root: String,
    pub files: Vec<String>,
}

pub async fn handle_upload(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    let (form, file_hashes, files) = process_form(&mut multipart).await?;

    let tree = Node::create_tree(file_hashes)?;
    let upload_result = upload_to_external(form).await?;

    if tree.hash != upload_result.root {
        return Err(AppError::from(anyhow!("Root hashes do not match!")));
    }

    let metadata = process_upload_result(files, tree, upload_result)?;
    store_metadata(db, &metadata).await?;

    let mut headers = HeaderMap::new();
    headers.insert("Location", "/".parse()?);

    Ok((StatusCode::FOUND, headers, ()).into_response())
}

async fn process_form(
    multipart: &mut Multipart,
) -> Result<(Form, Vec<String>, Vec<String>), AppError> {
    let mut form = Form::new();
    let mut file_hashes = vec![];
    let mut files = vec![];

    while let Some(field) = multipart.next_field().await? {
        let file_name = field
            .file_name()
            .ok_or(anyhow!("Failed to extract field name"))?
            .to_string();
        let data = field.bytes().await?;

        let hash = shared::file::hash::compute(&data.to_vec());
        let part = reqwest::multipart::Part::bytes(data.to_vec())
            .file_name(file_name.clone())
            .mime_str("application/octet-stream")?;

        form = form.part(file_name.clone(), part);
        files.push(file_name);
        file_hashes.push(hash);
    }
    Ok((form, file_hashes, files))
}

async fn store_metadata(db: Arc<Mutex<DB>>, metadata: &UploadMetadata) -> Result<(), AppError> {
    let mut db = db.lock().await;
    db.store_upload_metadata(metadata)?;
    Ok(())
}

fn process_upload_result(
    files: Vec<String>,
    tree: Node,
    upload_result: ExternalUploadResult,
) -> Result<UploadMetadata, AppError> {
    let uploaded_files = upload_result
        .files
        .iter()
        .enumerate()
        .map(|(idx, v)| {
            let file_name = files
                .get(idx)
                .ok_or(anyhow!("File not found at idx {:?}", idx))?;
            Ok((file_name, v))
        })
        .try_fold(
            HashMap::default(),
            |mut p: HashMap<String, String>, c: anyhow::Result<(&String, &String)>| {
                c.map(|(k, v)| {
                    p.insert(k.clone(), v.clone());
                    p
                })
            },
        )?;

    let metadata = UploadMetadata {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        files: uploaded_files,
        root_hash: tree.hash,
    };
    Ok(metadata)
}

async fn upload_to_external(form: Form) -> anyhow::Result<ExternalUploadResult> {
    let external_service_url = env::var(EXTERNAL_SERVICE_URL_ENV_VAR).unwrap_or_else(|_| String::from(DEFAULT_EXTERNAL_SERVICE_URL));
    let client = reqwest::Client::new();

    let res = client
        .post(&format!("{}/upload", external_service_url))
        .multipart(form)
        .send()
        .await;

    let v: ExternalUploadResult = res?.json().await?;

    Ok(v)
}
