use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use anyhow::anyhow;
use axum::body::Bytes;
use axum::extract::Multipart;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use shared::tree::node::Node;
use shared::types::error::AppError;
use tokio::sync::Mutex;

use crate::consts::FILES_DIRECTORY;
use crate::db::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResult {
  pub root: String,
  pub files: Vec<String>,
}

pub async fn upload(
  Extension(db): Extension<Arc<Mutex<DB>>>,
  mut multipart: Multipart,
) -> Result<Json<UploadResult>, AppError> {
  let (file_hashes, file_hash_map) = process_upload_form(&mut multipart).await?;

  let tree = shared::tree::node::Node::create_tree(file_hashes.clone())?;
  store_tree_file_hash_map(db, file_hash_map, &tree).await?;
  Ok(Json(UploadResult { root: tree.hash, files: file_hashes }))
}

async fn process_upload_form(multipart: &mut Multipart) -> Result<(Vec<String>, HashMap<String, String>), AppError> {
  let mut file_hashes = vec![];
  let mut file_hash_map: HashMap<String, String> = HashMap::new();

  while let Some(field) = multipart.next_field().await.unwrap() {
    let file_name = field.file_name().ok_or(anyhow!("Failed to get file name"))?.to_string();
    let data = field.bytes().await?;
    let (hash, file_path) = save_file(&file_name, &data)?;

    file_hash_map.insert(hash.to_string(), file_path.to_string());
    file_hashes.push(hash)
  }
  Ok((file_hashes, file_hash_map))
}

async fn store_tree_file_hash_map(
  db: Arc<Mutex<DB>>,
  file_hash_map: HashMap<String, String>,
  tree: &Node,
) -> Result<(), AppError> {
  let mut db = db.lock().await;
  db.store_files(&tree.hash, &file_hash_map)?;
  db.store_tree(&tree)?;
  Ok(())
}

fn save_file(file_name: &String, data: &Bytes) -> Result<(String, String), AppError> {
  let extension = get_extension_from_filename(file_name).ok_or(anyhow!("failed to get file extension"))?;
  let hash = shared::file::hash::compute(&data.to_vec());

  let file_path = format!("{}/{}.{}", FILES_DIRECTORY, &hash, &extension);

  let mut file = File::create(&file_path)?;
  file.write_all(data)?;
  Ok((hash, file_path))
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
  Path::new(filename).extension().and_then(OsStr::to_str)
}
