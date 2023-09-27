use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use anyhow::anyhow;
use axum::Extension;
use axum::Json;
use serde::{Deserialize, Serialize};
use shared::tree::node::{Node, Position};
use shared::types::error::AppError;
use tokio::sync::Mutex;

use crate::db::DB;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RetrieveFileBody {
  pub root: String,
  pub file_hash: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct RetrieveFileResponse {
  pub file: Vec<u8>,
  pub proof: Vec<(String, Position)>,
}

pub async fn retrieve(
  Extension(db): Extension<Arc<Mutex<DB>>>,
  data: Json<RetrieveFileBody>,
) -> Result<Json<RetrieveFileResponse>, AppError> {
  let (tree, file_path) = get_tree_and_file_map_from_db(db, &data).await?;
  let file_bytes = get_file_content(&file_path)?;
  let hash = shared::file::hash::compute(&file_bytes);
  let proof = tree.generate_proof(&hash).ok_or(AppError::from(anyhow!("Failed to calculate proof")))?;

  Ok(Json(RetrieveFileResponse { proof, file: file_bytes }))
}

fn get_file_content(file_path: &String) -> Result<Vec<u8>, AppError> {
  let mut file = File::open(&file_path)?;
  let mut file_bytes: Vec<u8> = vec![];
  file.read_to_end(&mut file_bytes)?;
  Ok(file_bytes)
}

async fn get_tree_and_file_map_from_db(
  db: Arc<Mutex<DB>>,
  data: &Json<RetrieveFileBody>,
) -> Result<(Node, String), AppError> {
  let (tree, tree_files) = {
    let mut db = db.lock().await;
    let tree = db.get_tree(&data.root)?;
    let tree_files = db.get_files(&data.root)?;
    (tree, tree_files)
  };

  let file_path = tree_files.get(&data.file_hash).ok_or(anyhow!("File does not exists under provided root"))?;

  Ok((tree, file_path.to_string()))
}
